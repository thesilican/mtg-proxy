use anyhow::{bail, Context, Result};
use image::{
    codecs::png::PngDecoder, imageops::overlay, DynamicImage, GenericImageView, ImageDecoder, Rgba,
    RgbaImage,
};
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};
use log::debug;
use lopdf::{
    content::{Content, Operation},
    dictionary, Document, Object, Stream,
};
use serde::Deserialize;
use std::{collections::HashMap, io::BufReader, sync::Arc, time::Duration};
use tokio::{join, task::spawn_blocking, time::sleep};
use uuid::Uuid;

#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum CardFace {
    Front,
    Back,
}
impl CardFace {
    pub fn as_str(&self) -> &'static str {
        match self {
            CardFace::Front => "front",
            CardFace::Back => "back",
        }
    }
}

#[derive(Deserialize, Copy, Clone, PartialEq, Eq, Hash)]
pub struct CardOptions {
    pub id: Uuid,
    pub face: CardFace,
    pub quantity: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct CardKey {
    id: Uuid,
    face: CardFace,
}

impl From<CardOptions> for CardKey {
    fn from(value: CardOptions) -> Self {
        CardKey {
            id: value.id,
            face: value.face,
        }
    }
}

/// Default options when printing
pub const DEFAULT_PAGE_OPTIONS: PageOptions = PageOptions {
    // 3x3 by default
    rows: 3,
    cols: 3,
    // Default line options
    line_len: 40,
    line_width: 1,
    line_color: Rgba([0x7f, 0x7f, 0x7f, 0xff]),
    black_bleed: 8,
    // Letter size paper
    page_width: 595,
    page_height: 792,
};

/// Measurements for laying cards on a page
#[derive(Clone, Copy)]
pub struct PageOptions {
    /// Number of rows of cards per page
    pub rows: u32,
    /// Number of cols of cards per page
    pub cols: u32,
    /// Length of cut guide lines
    pub line_len: u32,
    /// Width of cut guide lines
    pub line_width: u32,
    /// Color of cut guide lines
    pub line_color: Rgba<u8>,
    /// Number of pixels of black border bleed
    pub black_bleed: u32,
    /// Width of the page in PDF units
    pub page_width: u32,
    /// Height of the page in PDF units
    pub page_height: u32,
}

impl PageOptions {
    fn card_count(&self) -> u32 {
        self.rows * self.cols
    }
}

fn mb_string(bytes: usize) -> String {
    format!("{:0.2}MB", bytes as f64 / 1_000_000.0)
}

pub struct Printer;

impl Printer {
    pub async fn print(cards: &[CardOptions], options: &PageOptions) -> Result<Vec<u8>> {
        // Fetch cards
        let card_pngs = Printer::fetch_cards(cards).await?;

        // Create chunks of keys
        let chunk_size = options.card_count() as usize;
        let mut chunks = Vec::new();
        let mut chunk = Vec::new();
        for &card in cards {
            let key = CardKey::from(card);
            for _ in 0..card.quantity {
                chunk.push(key);
                if chunk.len() == chunk_size {
                    chunks.push(chunk.clone());
                    chunk.clear();
                }
            }
        }
        if chunk.len() != 0 {
            chunks.push(chunk);
        }

        // Build pages
        let card_pngs = Arc::new(card_pngs);
        let page_count = chunks.len();
        let mut pages = Vec::new();
        for (i, chunk) in chunks.into_iter().enumerate() {
            debug!("creating page {}/{}", i + 1, page_count);
            let page = Printer::create_page(chunk, card_pngs.clone(), options).await?;
            pages.push(page);
        }
        let byte_count = pages.iter().map(|x| x.as_bytes().len()).sum::<usize>();
        debug!("created {} pages", pages.len());
        debug!("total size: {}", mb_string(byte_count));
        drop(card_pngs);

        let pdf = Printer::create_pdf(pages, options).await?;
        debug!("pdf size: {}", mb_string(pdf.len()));
        Ok(pdf)
    }

    async fn fetch_cards(cards: &[CardOptions]) -> Result<HashMap<CardKey, Vec<u8>>> {
        debug!("downloading {} cards", cards.len());
        let mut output = HashMap::<CardKey, Vec<u8>>::new();
        for &card in cards {
            let key = CardKey::from(card);
            if output.contains_key(&key) {
                continue;
            }

            // Wait at least 50ms between downloads
            let dl = Printer::download_card(&card);
            let wait = sleep(Duration::from_millis(50));
            let (a, _) = join!(dl, wait);
            let png = a?;
            output.insert(key, png.clone());
        }
        let byte_count = output.iter().map(|(_, png)| png.len()).sum::<usize>();
        debug!("downloaded {} images", output.len());
        debug!("total size: {}", mb_string(byte_count));
        Ok(output)
    }

    async fn download_card(card: &CardOptions) -> Result<Vec<u8>> {
        let id = card.id;
        let face = card.face.as_str();
        let url =
            format!("https://api.scryfall.com/cards/{id}?format=image&version=png&face={face}");
        let png = reqwest::get(&url)
            .await?
            .bytes()
            .await
            .context(format!("error downloading card {}", card.id))?
            .to_vec();
        Ok(png)
    }

    async fn create_page(
        keys: Vec<CardKey>,
        pngs: Arc<HashMap<CardKey, Vec<u8>>>,
        options: &PageOptions,
    ) -> Result<DynamicImage> {
        let options = options.clone();
        spawn_blocking(move || Printer::create_page_sync(keys, pngs, &options)).await?
    }

    fn create_page_sync(
        keys: Vec<CardKey>,
        pngs: Arc<HashMap<CardKey, Vec<u8>>>,
        options: &PageOptions,
    ) -> Result<DynamicImage> {
        // Width of a card image in pixels
        const WIDTH: u32 = 745;
        // Height of a card image in pixels
        const HEIGHT: u32 = 1040;

        let &PageOptions {
            rows,
            cols,
            line_len,
            line_color,
            line_width,
            ..
        } = options;
        let card_count = options.card_count();

        // Convert images from png
        let mut images = Vec::new();
        for key in keys {
            let Some(png) = pngs.get(&key) else {
                bail!("pngs missing {}", key.id);
            };
            let mut buf = RgbaImage::new(WIDTH, HEIGHT);
            let buf_reader = BufReader::new(png.as_slice());
            PngDecoder::new(buf_reader)?
                .read_image(&mut buf)
                .context("Could not decode image")?;
            images.push(buf);
        }

        // Create padding images
        let mut padding = RgbaImage::new(WIDTH, HEIGHT);
        for pixel in padding.pixels_mut() {
            *pixel = Rgba([0xff, 0xff, 0xff, 0xff]);
        }
        while images.len() < card_count as usize {
            images.push(padding.clone());
        }

        // Create page image
        let mut page = RgbaImage::new(WIDTH * cols + line_len * 2, HEIGHT * rows + line_len * 2);
        for pixel in page.pixels_mut() {
            *pixel = Rgba([0xff, 0xff, 0xff, 0xff]);
        }

        // Overlay black background
        let pad = options.black_bleed;
        let mut black = RgbaImage::new(WIDTH * cols + (2 * pad), HEIGHT * rows + (2 * pad));
        for pixel in black.pixels_mut() {
            *pixel = Rgba([0x00, 0x00, 0x00, 0xff]);
        }
        overlay(
            &mut page,
            &black,
            (line_len - pad) as i64,
            (line_len - pad) as i64,
        );
        drop(black);

        // Overlay images
        for j in 0..rows {
            for i in 0..cols {
                let x = (i * WIDTH + line_len) as i64;
                let y = (j * HEIGHT + line_len) as i64;
                let idx = (j * cols + i) as usize;
                overlay(&mut page, &images[idx], x, y);
            }
        }

        // Draw cut lines
        for i in 0..=cols {
            for j in 0..=rows {
                let x = line_len + i * WIDTH;
                let y = line_len + j * HEIGHT;
                let rect = Rect::at((x - line_len) as i32, (y - line_width) as i32)
                    .of_size(line_len * 2, line_width * 2);
                draw_filled_rect_mut(&mut page, rect, line_color);
                let rect = Rect::at((x - line_width) as i32, (y - line_len) as i32)
                    .of_size(line_width * 2, line_len * 2);
                draw_filled_rect_mut(&mut page, rect, line_color);
            }
        }
        let image = DynamicImage::ImageRgba8(page);

        Ok(DynamicImage::from(image.into_rgb8()))
    }

    async fn create_pdf(images: Vec<DynamicImage>, options: &PageOptions) -> Result<Vec<u8>> {
        let options = options.clone();
        spawn_blocking(move || Printer::create_pdf_sync(images, &options)).await?
    }

    fn create_pdf_sync(images: Vec<DynamicImage>, options: &PageOptions) -> Result<Vec<u8>> {
        debug!("creating pdf");

        // Set up document
        let mut doc = Document::new();

        // Pages object id
        let pages_id = doc.new_object_id();

        // List of page ids
        let mut page_ids = Vec::<Object>::new();

        let image_count = images.len();
        for (i, img) in images.into_iter().enumerate() {
            // Image stream object
            let (width, height) = img.dimensions();
            let img_dict = dictionary! {
                "Type" => "XObject",
                "Subtype" => "Image",
                "Width" => width,
                "Height" => height,
                "ColorSpace" => "DeviceRGB",
                "BitsPerComponent" => 8,
            };
            let mut img_stream = Stream::new(img_dict, img.as_bytes().to_vec());
            drop(img);
            debug!("compressing image stream {}/{}", i + 1, image_count);
            img_stream.compress()?;
            let img_id = doc.add_object(img_stream);

            // Resources object
            let resources_dict = dictionary! {
                "ProcSet" => vec!["PDF".into(), "ImageB".into()],
                "XObject" => dictionary! {
                    "I1" => img_id,
                },
            };
            let resources_id = doc.add_object(resources_dict);

            let card_width = width * 72 / 298;
            let card_height = height * 72 / 297;
            let card_tx = (options.page_width - card_width) / 2;
            let card_ty = (options.page_height - card_height) / 2;

            // Page content instructions
            let content = Content {
                operations: vec![
                    Operation::new(
                        "cm",
                        vec![
                            card_width.into(),
                            0.into(),
                            0.into(),
                            card_height.into(),
                            card_tx.into(),
                            card_ty.into(),
                        ],
                    ),
                    Operation::new("Do", vec!["I1".into()]),
                ],
            };
            let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode().unwrap()));

            // Page object
            let page_dict = dictionary! {
                "Type" => "Page",
                "Parent" => pages_id,
                "Contents" => content_id,
                "Resources" => resources_id,
            };
            let page_id = doc.add_object(page_dict);
            page_ids.push(page_id.into());
        }

        // Pages object
        let page_count = page_ids.len() as i64;
        let pages = dictionary! {
            "Type" => "Pages",
            "Kids" => page_ids,
            "Count" => page_count,
            "MediaBox" =>
                vec![
                    0.into(),
                    0.into(),
                    options.page_width.into(),
                    options.page_height.into(),
                ],
        };
        doc.set_object(pages_id, pages);

        // Catalog object
        let catalog_id = doc.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });
        doc.trailer.set("Root", catalog_id);

        // Write document to stream
        let mut buffer = Vec::new();
        doc.save_to(&mut buffer)?;
        Ok(buffer)
    }
}

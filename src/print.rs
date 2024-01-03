use anyhow::{Context, Result};
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
use std::{io::BufReader, time::Duration};
use tokio::{join, task::spawn_blocking, time::sleep};
use uuid::Uuid;

use crate::cache::Cache;

/// Width of a card image in pixels
const WIDTH: u32 = 745;

/// Height of a card image in pixels
const HEIGHT: u32 = 1040;

/// How long to keep images in cache
const TTL: Duration = Duration::from_secs(3600);

/// Default options when printing
pub const DEFAULT_OPTIONS: PrintOptions = PrintOptions {
    // 3x3 by default
    rows: 3,
    cols: 3,
    // Default line options
    line_len: 40,
    line_width: 1,
    line_color: Rgba([0x7f, 0x7f, 0x7f, 0xff]),
    // Letter size paper
    page_width: 595,
    page_height: 792,
};

/// Options for printing cards
#[derive(Clone)]
pub struct PrintOptions {
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
    /// Width of the page in PDF units
    pub page_width: u32,
    /// Height of the page in PDF units
    pub page_height: u32,
}

impl PrintOptions {
    fn card_count(&self) -> u32 {
        self.rows * self.cols
    }
}

#[derive(Clone)]
pub struct Printer {
    cache: Cache,
}

impl Printer {
    pub fn new() -> Self {
        Printer {
            cache: Cache::new(),
        }
    }

    pub fn prune_cache(&self) -> Result<()> {
        self.cache.prune()
    }

    pub async fn print(&self, card_ids: &[Uuid], options: &PrintOptions) -> Result<Vec<u8>> {
        // Fetch cards
        let mut cards = self.fetch_cards(card_ids).await?;
        // Reverse in place, so that they're popped in the right order
        cards.reverse();
        debug!("card count: {}", cards.len());
        debug!(
            "cards size: {} bytes",
            cards.iter().map(|card| card.len()).sum::<usize>()
        );

        let mut pages = Vec::new();
        let chunk_size = options.card_count() as usize;

        // Consume pngs directly to save some memory
        while !cards.is_empty() {
            let mut chunk = Vec::new();
            let mut count = 0;
            while let Some(card) = cards.pop() {
                chunk.push(card);
                count += 1;
                if count == chunk_size {
                    break;
                }
            }
            let png = self.create_page_async(chunk, options).await?;
            pages.push(png);
        }
        debug!("pages count: {}", pages.len());
        debug!(
            "pages size: {} bytes",
            pages
                .iter()
                .map(|page| page.as_bytes().len())
                .sum::<usize>()
        );
        let pdf = self.create_pdf_async(pages, options).await?;
        debug!("pdf size: {}", pdf.len());
        Ok(pdf)
    }

    async fn fetch_cards(&self, card_ids: &[Uuid]) -> Result<Vec<Vec<u8>>> {
        let mut output = Vec::new();
        for &card_id in card_ids {
            if let Some(card) = self.cache.get(card_id)? {
                output.push(card);
            } else {
                // Wait at least 50ms between downloads
                let dl = self.download_card(card_id);
                let wait = sleep(Duration::from_millis(50));
                let (a, _) = join!(dl, wait);
                let png = a?;
                self.cache.insert(card_id, &png, TTL)?;
                output.push(png);
            }
        }
        Ok(output)
    }

    async fn download_card(&self, card_id: Uuid) -> Result<Vec<u8>> {
        let url = format!("https://api.scryfall.com/cards/{card_id}?format=image&version=png");
        let png = reqwest::get(&url)
            .await?
            .bytes()
            .await
            .context(format!("error downloading {card_id}"))?
            .to_vec();
        Ok(png)
    }

    async fn create_page_async(
        &self,
        pngs: Vec<Vec<u8>>,
        options: &PrintOptions,
    ) -> Result<DynamicImage> {
        let pngs = pngs.to_vec();
        let options = options.clone();
        spawn_blocking(move || Self::create_page(pngs, &options)).await?
    }

    fn create_page(pngs: Vec<Vec<u8>>, options: &PrintOptions) -> Result<DynamicImage> {
        debug!("creating page png");
        let &PrintOptions {
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
        for png in pngs {
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
        let mut black = RgbaImage::new(WIDTH * cols, HEIGHT * rows);
        for pixel in black.pixels_mut() {
            *pixel = Rgba([0x00, 0x00, 0x00, 0xff]);
        }
        overlay(&mut page, &black, line_len as i64, line_len as i64);
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

    async fn create_pdf_async(
        &self,
        images: Vec<DynamicImage>,
        options: &PrintOptions,
    ) -> Result<Vec<u8>> {
        let images = images.to_vec();
        let options = options.clone();
        spawn_blocking(move || Self::create_pdf(images, &options)).await?
    }

    fn create_pdf(images: Vec<DynamicImage>, options: &PrintOptions) -> Result<Vec<u8>> {
        debug!("creating pdf");

        // Set up document
        let mut doc = Document::new();

        // Pages object id
        let pages_id = doc.new_object_id();

        // List of page ids
        let mut page_ids = Vec::<Object>::new();

        for img in images {
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
            img_stream.compress()?;
            debug!("compressing image stream");
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

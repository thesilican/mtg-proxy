use std::{io::Cursor, mem, sync::Arc};

use anyhow::{Context, Result};
use image::{
    codecs::png::PngDecoder, imageops::overlay, DynamicImage, GenericImageView, ImageDecoder, Rgba,
    RgbaImage,
};
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};
use js_sys::Function;
use lopdf::{
    content::{Content, Operation},
    dictionary, Document, Object, Stream,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

struct Card {
    data: Arc<[u8]>,
    count: u32,
}

#[wasm_bindgen]
pub struct PrintJob {
    cards: Vec<Card>,
    callback: Option<Function>,
}

#[wasm_bindgen]
impl PrintJob {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        PrintJob {
            cards: Vec::new(),
            callback: None,
        }
    }

    #[wasm_bindgen]
    pub fn add_card(&mut self, count: u32, data: Box<[u8]>) {
        let data = Arc::from(data);
        let card = Card { data, count };
        self.cards.push(card);
    }

    #[wasm_bindgen]
    pub fn add_callback(&mut self, callback: Function) {
        self.callback = Some(callback)
    }

    #[wasm_bindgen]
    pub fn run(&mut self) -> Result<Vec<u8>, String> {
        self.print().map_err(|err| err.to_string())
    }
}

// Layout of cards on the page
const ROWS: u32 = 3;
const COLS: u32 = 3;
const CARD_COUNT: u32 = ROWS * COLS;
// Line options
const LINE_LEN: u32 = 40;
const LINE_WIDTH: u32 = 1;
const LINE_COLOR: Rgba<u8> = Rgba([0x7f, 0x7f, 0x7f, 0xff]);
const BLACK_BLEED: u32 = 8;
// Width of a card image in pixels
const WIDTH: u32 = 745;
const HEIGHT: u32 = 1040;
// Dimensions of letter paper
const PAGE_WIDTH: u32 = 595;
const PAGE_HEIGHT: u32 = 792;

impl PrintJob {
    fn report_progress(&mut self, message: &str) -> Result<()> {
        if let Some(callback) = &mut self.callback {
            let _ = callback.call1(&JsValue::null(), &JsValue::from(message));
        }
        Ok(())
    }
    pub fn print(&mut self) -> Result<Vec<u8>> {
        // Create chunks of 9 cards each
        let mut chunks = Vec::new();
        let mut chunk = Vec::new();
        for card in self.cards.iter() {
            for _ in 0..card.count {
                chunk.push(card.data.clone());
                if chunk.len() == 9 {
                    let full_chunk = mem::replace(&mut chunk, Vec::new());
                    chunks.push(full_chunk);
                }
            }
        }
        if chunk.len() != 0 {
            chunks.push(chunk);
        }

        // Create a page for each chunk
        let mut pages = Vec::new();
        let len = chunks.len();
        for (i, chunk) in chunks.into_iter().enumerate() {
            self.report_progress(&format!("Generating page images ({} / {})", i + 1, len))?;
            let page = self.create_page(chunk)?;
            pages.push(page);
        }

        // Create final pdf
        let pdf = self.create_pdf(pages)?;
        Ok(pdf)
    }
    fn create_page(&self, pngs: Vec<Arc<[u8]>>) -> Result<DynamicImage> {
        // Convert images from png
        let mut images = Vec::new();
        for png in pngs {
            let mut buf = RgbaImage::new(WIDTH, HEIGHT);
            let buf_reader = Cursor::new(&*png);
            PngDecoder::new(buf_reader)?
                .read_image(&mut buf)
                .context("could not decode image")?;
            images.push(buf);
        }

        // Create padding images
        let mut padding = RgbaImage::new(WIDTH, HEIGHT);
        for pixel in padding.pixels_mut() {
            *pixel = Rgba([0xff, 0xff, 0xff, 0xff]);
        }
        while images.len() < CARD_COUNT as usize {
            images.push(padding.clone());
        }

        // Create page image
        let mut page = RgbaImage::new(WIDTH * COLS + LINE_LEN * 2, HEIGHT * ROWS + LINE_LEN * 2);
        for pixel in page.pixels_mut() {
            *pixel = Rgba([0xff, 0xff, 0xff, 0xff]);
        }

        // Overlay black background
        let mut black = RgbaImage::new(
            WIDTH * COLS + (2 * BLACK_BLEED),
            HEIGHT * ROWS + (2 * BLACK_BLEED),
        );
        for pixel in black.pixels_mut() {
            *pixel = Rgba([0x00, 0x00, 0x00, 0xff]);
        }
        overlay(
            &mut page,
            &black,
            (LINE_LEN - BLACK_BLEED) as i64,
            (LINE_LEN - BLACK_BLEED) as i64,
        );
        drop(black);

        // Overlay images
        for j in 0..ROWS {
            for i in 0..COLS {
                let x = (i * WIDTH + LINE_LEN) as i64;
                let y = (j * HEIGHT + LINE_LEN) as i64;
                let idx = (j * COLS + i) as usize;
                overlay(&mut page, &images[idx], x, y);
            }
        }

        // Draw cut lines
        for i in 0..=COLS {
            for j in 0..=ROWS {
                let x = LINE_LEN + i * WIDTH;
                let y = LINE_LEN + j * HEIGHT;
                let rect = Rect::at((x - LINE_LEN) as i32, (y - LINE_WIDTH) as i32)
                    .of_size(LINE_LEN * 2, LINE_WIDTH * 2);
                draw_filled_rect_mut(&mut page, rect, LINE_COLOR);
                let rect = Rect::at((x - LINE_WIDTH) as i32, (y - LINE_LEN) as i32)
                    .of_size(LINE_WIDTH * 2, LINE_LEN * 2);
                draw_filled_rect_mut(&mut page, rect, LINE_COLOR);
            }
        }
        let image = DynamicImage::ImageRgba8(page);

        Ok(DynamicImage::from(image.into_rgb8()))
    }

    fn create_pdf(&mut self, pages: Vec<DynamicImage>) -> Result<Vec<u8>> {
        // Set up document
        let mut doc = Document::new();

        // Pages object id
        let pages_id = doc.new_object_id();

        // List of page ids
        let mut page_ids = Vec::<Object>::new();

        let len = pages.len();
        for (i, img) in pages.into_iter().enumerate() {
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
            self.report_progress(&format!("Compressing pages ({} / {})", i + 1, len))?;
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
            let card_tx = (PAGE_WIDTH - card_width) / 2;
            let card_ty = (PAGE_HEIGHT - card_height) / 2;

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
                    PAGE_WIDTH.into(),
                    PAGE_HEIGHT.into(),
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

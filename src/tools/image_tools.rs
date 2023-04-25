use std::fs;
use image::{ImageBuffer, ImageFormat, Rgb};
use screenshots::Screen;
use leptess::LepTess;

pub async fn ocr(img: image::DynamicImage) -> String {
    let mut lt = LepTess::new(None, "eng").unwrap();
    lt.set_image_from_dynamic_image(&img);
    lt.get_utf8_text().unwrap()
}
    
pub fn screenshot(x: u32, y: u32, width: u32, height: u32) -> image::DynamicImage {
    let screen = Screen::from_point(0, 0).unwrap();
    let scr = screen.capture().unwrap();
    let buffer = scr.buffer();
    let img = ImageBuffer::from_raw(screen.width, screen.height, buffer).unwrap();

    img.crop(x, y, width, height)
}
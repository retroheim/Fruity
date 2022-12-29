use std::{fs};
use image::{ImageFormat};
use screenshots::Screen;
pub async fn ocr() -> String {

    // Prepares Optical Character Recognition
    let mut lt = leptess::LepTess::new(None, "eng").unwrap();
    
    // Reads the Cropped Screenshotted Image.
    lt.set_image("images/roblox.png").unwrap();
    
    // Returns Read Recognized Characters
    return lt.get_utf8_text().unwrap();
}
    
pub fn screenshot(_x:&u32, _y:&u32, _width:&u32, _height:&u32) {
    
    // Takes a Screenshot of the Main Monitor
    let screen = Screen::from_point(0,0).unwrap();
    let scr = screen.capture().unwrap();
    let buffer = scr.buffer();
    
    // Writes the Screenshot to roblox.png
    fs::write("images/roblox.png", &buffer).unwrap();
    
    // Re-Opens the Screenshot as Image
    let mut img = image::open("images/roblox.png").unwrap();
    
    // Crops the Image to a configured size and relative location (for Better Accuracy)
    let x = screen.width / _x;
    let y = screen.height / _y;
    let width = screen.width / _width;
    let height = screen.height / _height;
    img = img.crop(x, y, width, height);
    
    // Overwrites the previously Saved Image
    let path = format!("images/roblox.png");
    img
        .save_with_format(&path, ImageFormat::Png)
        .unwrap();
}
extern crate image;

use std::io;
use std::error::Error;
use image::io::Reader as ImageReader;

fn main() {
    println!("Converting image to rgb565 format");
    test_load_img().unwrap();
}

fn test_load_img() {
    let img = ImageReader::open("colere_sven.png")?.decode()?;
    img.save("colere_sven.jpg")?;
}



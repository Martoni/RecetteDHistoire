extern crate image;

use std::env;
use std::fs::File;
use std::io::Write;

use image::GenericImageView;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("debug: args {:?}", args);
    println!("Converting image to rgb565 format");
    test_load_img(&args[1]);
    println!("Ok converted");
}

fn test_load_img(imgpath: &String) {
    println!("Open {:?}", imgpath);
    let img = image::open(imgpath).unwrap();
    let (wimg, himg) = img.dimensions();
    println!("{:?}", img.color());
    let imgrgb = img.into_rgb8();
    println!("width {:?}, height {:?}", wimg, himg);

    /* convert rgb888 24Bits to rgb565 16Bits values */
    let mut outfile = File::create("rgb565.bin").unwrap();
    for pix in imgrgb.pixels() {
        println!("Pixel {:?} -> {:x}", pix, pix[0]);
        let pix565: u16 =  (((pix[0] as u16) & 0x00f8) << 8u16)
                          |(((pix[1] as u16) & 0x00fc) << 3u16)
                          |(((pix[2] as u16) & 0x00f8) >> 3u16);
        let hpix565: u8 = (pix565 >> 8) as u8;
        let lpix565: u8 = (pix565 & 0x00ff) as u8;
        outfile.write(&[hpix565, lpix565]).unwrap();
    }
}



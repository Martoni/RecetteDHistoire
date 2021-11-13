extern crate image;
use std::fs::File;
use std::io::Write;

use image::GenericImageView;

fn main() {
    println!("Converting image to rgb565 format");
    test_load_img();
    println!("Ok converted");
}

fn test_load_img() {
    println!("Open colere_sven.png");
    let img = image::open("colere_sven.png").unwrap();
    let (wimg, himg) = img.dimensions();
    println!("{:?}", img.color());
    img.save("colere_sven.jpg").unwrap();

    let imgrgb = img.into_rgb8();

    for pix in imgrgb.pixels() {
        println!("Pixel {:?} -> {:x}", pix, pix[0]);
    }

    println!("width {:?}, height {:?}", wimg, himg);

    let mut outfile = File::create("output.bin").unwrap();
    outfile.write(&[0x40u8]).unwrap();
}



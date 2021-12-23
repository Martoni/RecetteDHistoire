extern crate image;
extern crate clap;

use std::env;
use std::fs::File;
use std::io::Write;

use itertools::Itertools;
use image::GenericImageView;
use image::RgbImage;

fn rgb888_to_rgb565(imgrgb: &RgbImage, outputname: &str){
    let mut outfile = File::create(outputname).unwrap();
    for pix in imgrgb.pixels() {
        let pix565: u16 =  (((pix[0] as u16) & 0x00f8) << 8u16)
                          |(((pix[1] as u16) & 0x00fc) << 3u16)
                          |(((pix[2] as u16) & 0x00f8) >> 3u16);
        let hpix565: u8 = (pix565 >> 8) as u8;
        let lpix565: u8 = (pix565 & 0x00ff) as u8;
        outfile.write(&[hpix565, lpix565]).unwrap();
    }
}

fn write_to_rgb565(imgpath: &String) -> String {
    //TODO: check extension
    let (corename, extname) = imgpath.split(".").collect_tuple().unwrap();
    println!("extension {:?}, corename {:?}", extname, corename);
    let outputfilename = format!("{}.bin", corename);

    println!("Open {:?}", imgpath);
    let img = image::open(imgpath).unwrap();
    let (_wimg, _himg) = img.dimensions();
    let imgrgb = img.into_rgb8();
    /* convert rgb888 24Bits to rgb565 16Bits values */
    rgb888_to_rgb565(&imgrgb, &outputfilename[..]);
    outputfilename
}

fn usages() {
    println!("convertrgb565 imagefilename.ext");
    println!("Convert image in raw binary rgb565 format");
    println!("Output file is imagefilename.bin");
}

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usages();
        panic!("Give image pathname");
    }

    let outputfilename = write_to_rgb565(&args[1]);
    println!("{:?} written", outputfilename);
}



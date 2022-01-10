use std::fs::File;
use std::io::Write;

use itertools::Itertools;
use image::GenericImageView;
use image::RgbImage;

pub fn rgb888_to_rgb565(imgrgb: &RgbImage, outputname: &str){
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

pub fn write_to_rgb565(imgpath: &str) -> String {
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

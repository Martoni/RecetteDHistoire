extern crate image;

use image::GenericImageView;

fn main() {
    println!("Converting image to rgb565 format");
    test_load_img();
    println!("Ok converted");
}

fn test_load_img() {
    println!("Open colere_sven.png");
    let img = image::open("colere_sven.png").unwrap();
    println!("dimensions {:?}", img.dimensions());
    println!("{:?}", img.color());
    img.save("colere_sven.jpg").unwrap();
}



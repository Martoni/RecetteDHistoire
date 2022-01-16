extern crate image;
extern crate clap;

use clap::{Arg, App};

use rdhist::rdimage;

fn main() {

    let mut app = App::new("Convertisseur d'image au format binaire rgb565")
                .version("0.1")
                .author("Fabien Marteau <mail@fabienm.eu>")
                .about("Raconte des histoires")
                .arg(Arg::with_name("finput")
                        .short("i")
                        .long("input")
                        .value_name("FINPUT")
                        .help("Fichier image d'entrée")
                        .takes_value(true));

    let mut helpmsg = Vec::new();
    app.write_long_help(&mut helpmsg)
            .expect("Impossible d'écrire le message d'aide");
    let helpmsg = std::str::from_utf8(&helpmsg)
            .expect("Impossible de convertir le message en utf8");

    let matches = app.get_matches();

    let finput = matches.value_of("finput")
                .expect(&format!("\nErreur de nom de fichier d'entrée\n\n{}",
                                 helpmsg));


    let outputfilename = rdimage::write_to_rgb565(&finput);
    println!("{:?} written", outputfilename);
}



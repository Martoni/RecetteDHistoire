extern crate clap;
extern crate yaml_rust;

use clap::{Arg, App};
use yaml_rust::{YamlLoader};
use std::fs;
use std::str;

fn main() {

    let mut app = App::new("Recette d'Histoire")
                    .version("0.1")
                    .author("Fabien Marteau <mail@fabienm.eu>")
                    .about("Tous les ustensiles nécessaire pour cuisiner des histoires")
                    .arg(Arg::new("listrecette")
                         .short('l')
                         .long("listrecette")
                         .value_name("LISTRECETTE")
                         .help("Liste les recettes disponibles")
                         .takes_value(false));

    let mut helpmsg = Vec::new();
    app.write_long_help(&mut helpmsg)
        .expect("Impossible d'écrire le message d'aide");
    let _helpmsg = str::from_utf8(&helpmsg)
        .expect("Impossible de convertir le message en utf8");

    let matches = app.get_matches();

    if matches.is_present("listrecette") {
        println!("TODO: lister les recettes disponibles");
    } else {
        /* XXX some try */
        let filename = String::from("recettes/grande_histoire_pomme_dapi/2021_04_Lili_et_la_graine_magique.rdist");
        println!("Recettes D'Histoires");
        println!("Recette : {}", filename);
        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
        let docs = YamlLoader::load_from_str(&contents).unwrap();
        println!("{:?}", docs);
    }
}

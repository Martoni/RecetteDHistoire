extern crate clap;
extern crate yaml_rust;

use clap::{Arg, App};
//use yaml_rust::{YamlLoader};
use std::{str, process};

use rdhist::rdmain;

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

    let cfg = if matches.is_present("listrecette") {
        rdmain::Config::new().set_cmdlist()
    } else {
        rdmain::Config::new()
    };

    if let Err(e) = rdmain::run(cfg) {
        eprintln!("Erreur durant l'exécution du programme: {}", e);
        process::exit(1);
    }

}

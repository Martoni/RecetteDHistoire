extern crate clap;
extern crate yaml_rust;
extern crate main_error;

use clap::{Arg, App};
//use yaml_rust::{YamlLoader};
use std::{str, process};
use main_error::MainError;

use rdhist::rdmain;

fn main() -> Result<(), MainError> {

    let mut app = App::new("Recette d'Histoire")
                    .version("0.1")
                    .author("Fabien Marteau <mail@fabienm.eu>")
                    .about("Tous les ustensiles nécessaire pour cuisiner des histoires")
                    .arg(Arg::new("listerecettes")
                         .short('l')
                         .long("listerecettes")
                         .value_name("LISTERECETTES")
                         .help("Liste les recettes disponibles")
                         .takes_value(false))
                    .arg(Arg::new("recolter")
                        .short('r')
                        .long("recolter")
                        .value_name("RECETTE")
                        .help("Récolte les ingrédients de la recette donnée en argument")
                        .takes_value(true));


    let mut helpmsg = Vec::new();
    app.write_long_help(&mut helpmsg)
        .expect("Impossible d'écrire le message d'aide");
    let _helpmsg = str::from_utf8(&helpmsg)
        .expect("Impossible de convertir le message en utf8");

    let matches = app.get_matches();

    let cfg = if matches.is_present("listerecettes") {
        rdmain::Config::new()?.set_cmdlist()
    } else {
        rdmain::Config::new()?
    };

    let recolterval = matches.value_of("recolter").unwrap_or("None").to_string();
    let cfg = cfg.set_recette_filename(recolterval);

    if let Err(e) = rdmain::run(cfg) {
        eprintln!("Erreur durant l'exécution du programme: {}", e);
        process::exit(1);
    }

    Ok(())
}

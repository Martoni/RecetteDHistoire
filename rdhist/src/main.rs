extern crate clap;
extern crate main_error;

use clap::{Arg, App};
use std::str;
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
                    .arg(Arg::new("listappareils")
                        .short('a')
                        .long("listappareils")
                        .value_name("LISTAPPAREILS")
                        .help("Liste des appareils disponibles")
                        .takes_value(false))
                    .arg(Arg::new("caprice")
                        .short('c')
                        .long("caprice")
                        .value_name("CAPRICE")
                        .help("Lancer la ligne de commande «caprice»")
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
        rdmain::RdMainConfig::new()?.set_cmdlist()
    } else if matches.is_present("caprice") {
        rdmain::RdMainConfig::new()?.set_caprice()
    } else if matches.is_present("listappareils") {
        rdmain::RdMainConfig::new()?.set_listappareils()
    } else {
        rdmain::RdMainConfig::new()?
    };

    let recolterval = matches.value_of("recolter")
                            .unwrap_or("None")
                            .to_string();
    let cfg = cfg.set_recette_filename(recolterval);

    Ok(rdmain::run(cfg)?)
}

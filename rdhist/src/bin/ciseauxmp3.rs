extern crate clap;

use clap::{Arg, App};
use std::process::Command;
use std::str;
use std::fs::File;

/*
concatenation:
cat fichier1.mp3 fichier2.mp3 > out.mp3

split:
ffmpeg -i fichier1.mp3 -acodec copy -ss 00:00:34 -t 00:00:02 out.mp3
*/

fn main() {

    let mut app = App::new("Assembleur/Découpeur de fichiers mp3")
                        .version("0.1")
                        .author("Fabien Marteau <mail@fabienm.eu>")
                        .about("Raconte des histoires")
                        .arg(Arg::with_name("finput")
                                .short("i")
                                .long("input")
                                .multiple(true)
                                .value_name("FINPUT")
                                .help("fichier mp3 d'entrée")
                                .takes_value(true))
                        .arg(Arg::with_name("foutput")
                                .short("o")
                                .long("output")
                                .value_name("FOUTPUT")
                                .help("nom du fichier de sortie")
                                .takes_value(true))
                        .arg(Arg::with_name("concat")
                                .short("c")
                                .long("concat")
                                .value_name("CONCAT")
                                .takes_value(false)
                                .help("Assemble les fichiers en un seul"))
                        .arg(Arg::with_name("split")
                                .short("s")
                                .long("split")
                                .value_name("DEBUT:DUREE")
                                .takes_value(true)
                                .help(concat!(
                                    "découpe le mp3 pour en extraire ",
                                    "une partie (temps en secondes",
                                    ")")));

    let mut helpmsg = Vec::new();
    app.write_long_help(&mut helpmsg)
                    .expect("Impossible d'écrire le message d'aide");
    let helpmsg = str::from_utf8(&helpmsg)
                    .expect("Impossible de convertir le message en utf8");

    let matches = app.get_matches();

    let inputs_num = match matches.occurrences_of("finput") {
        0 => panic!("Donnez au moins un fichier source"),
        a => a};

    let finputs = matches.values_of("finput")
        .expect("Erreur: impossible de récupérer les noms des fichiers d'entrée");
    let finputsv: Vec<&str> = finputs.collect();

    let foutname = matches.value_of("foutput")
        .expect(&format!(
        "\nErreur: Donnez un nom de fichier de sortie (-o)\n\n{}", helpmsg));

    let foutput = File::create(&foutname)
                        .expect(&format!("Impossible de créer le fichier {:?}", foutname));

    let concat = matches.is_present("concat");
    let split = matches.is_present("split");
    if concat {
        if inputs_num < 2 {
            panic!(format!("\nErreur: Donnez au moins deux fichiers d'entrées\n\n{}", helpmsg));
        }
        let _cmdret = Command::new("cat")
                                .args(finputsv)
                                .stdout(foutput)
                                .output()
                                .expect("La commande 'cat' a échouée");
    } else if split {
        if inputs_num != 1 {
            panic!(format!("\nErreur: Donnez un seul nom de fichier en entrée\n\n{}", helpmsg));
        }
    } else {
            panic!(format!("\nErreur: Voulez vous concatener (-c) ou découper (-s) ?\n\n{}", helpmsg));
    }
    println!("Fichier {:?} créé", foutname);
}

extern crate clap;

use clap::{Arg, App, SubCommand};

/*
concatenation:
cat fichier1.mp3 fichier2.mp3 > out.mp3

split:
ffmpeg -i fichier1.mp3 -acodec copy -ss 00:00:34 -t 00:00:02 out.mp3
*/

fn main() {

    let matches = App::new("Assembleur/Découpeur de fichiers mp3")
                        .version("0.1")
                        .author("Fabien Marteau <mail@fabienm.eu>")
                        .about("Raconte des histoires")
                        .arg(Arg::with_name("finput")
                                .short("i")
                                .long("input")
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
                                .help("Assemble les fichiers en un seul"))
                        .arg(Arg::with_name("split")
                                .short("s")
                                .long("split")
                                .value_name("SPLIT")
                                .help("découpe le mp3 pour en extraire une partie (temps en secondes)"))
                        .get_matches();

    usages();
}

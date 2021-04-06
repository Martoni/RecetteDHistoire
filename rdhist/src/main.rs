extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter};
use std::fs;

fn main() {
    let filename = String::from("recettes/grande_histoire_pomme_dapi/2021_04_Lili_et_la_graine_magique.rdist");
    println!("Recettes D'Histoires");
    println!("Recette : {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let docs = YamlLoader::load_from_str(&contents).unwrap();
    println!("{:?}", docs);
}

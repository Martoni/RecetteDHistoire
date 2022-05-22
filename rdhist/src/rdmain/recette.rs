extern crate serde;

use std::error::Error;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Ingredients {
    media_type: String,
    nom: String,
    format: String,
    url: String,
    md5sum: String,
    dimensions: Option<String>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Opération {
    media_type: String,
    fichier_source: String,
    début: String,
    fin: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Contenus {
    media_type: Vec<Opération>
    }

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Recette {
    titre: String,
    date: String,
    collection: String,
    numéro: u32,
    codebarre: Option<u64>,
    ingrédients: Vec<Ingredients>,
    opérations: Option<Vec<Opération>>
}


impl Recette {
    pub fn new(filename: String) -> Result<Recette, Box<dyn Error>> {
        let rawcontent = std::fs::read_to_string(filename)?;
        let yrecette = serde_yaml::from_str(&rawcontent)?;
        Ok(yrecette)
    }
}

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn recette_ext_check(filename: &String) -> bool {
    let vecfile = filename.split(".").collect::<Vec<&str>>();
    let ext = *vecfile.last().expect("Pas d'extensions");

    if ext.eq("rdhist") {
        true
    } else {
        false
    }
}

extern crate serde;

use std::error::Error;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};

use crate::rdmain::RDHIST_EXT;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SourceTypes {
    Url,
    Cdaudio
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FormatTypes {
    Cdaudio,
    Mp3,
    Jpeg
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Ingredients {
    pub media_type: String,
    pub nom: String,
    pub format: Option<FormatTypes>,
    pub source: Option<SourceTypes>,
    pub url: Option<String>,
    pub md5sum: String,
    pub dimensions: Option<String>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Opération {
    pub media_type: String,
    pub fichier_source: String,
    pub début: String,
    pub fin: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Contenus {
    pub media_type: Vec<Opération>
    }

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Recette {
    pub titre: String,
    pub date: String,
    pub collection: String,
    pub numéro: u32,
    pub codebarre: Option<u64>,
    pub ingrédients: Vec<Ingredients>,
    pub opérations: Option<Vec<Opération>>
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

    if ext.eq(RDHIST_EXT) {
        true
    } else {
        false
    }
}

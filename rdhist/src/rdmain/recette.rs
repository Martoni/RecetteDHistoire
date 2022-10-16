extern crate serde;

/// Description du format de fichier pour les cahiers de recette
/// Un fichier de recette porte l'extension `.rdhist` et est au
/// format yaml

use std::fs;
use std::error::Error;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};

use crate::rdmain::RDHIST_EXT;

use crate::rdmain::ingredients::Ingredients;

/// Type de source à aller chercher
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SourceTypes {
    Url,
    Cdaudio
}

/// Format de la source
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FormatTypes {
    Cdaudio,
    Mp3,
    Webp,
    Jpeg,
    Png,
}

impl FormatTypes {
    /// Donne l'extension du format de fichier
    pub fn extension(&self) -> String {
        match self {
            FormatTypes::Cdaudio => "au".to_string(),
            FormatTypes::Mp3 => "mp3".to_string(),
            FormatTypes::Jpeg => "jpg".to_string(),
            FormatTypes::Webp => "webp".to_string(),
            FormatTypes::Png => "png".to_string(),
        }
    }
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

    pub fn create_cagette_dir(&self, dir_path: String) -> Result<String, Box<dyn Error>> {
        let mut full_path:String = String::from("");
        full_path.push_str(&dir_path);
        full_path.push_str("/");
        full_path.push_str(&self.date.replace("-","_"));
        full_path.push_str("_");
        full_path.push_str(&self.titre.replace(" ","_"));
        fs::create_dir_all(&full_path)?;
        Ok(full_path)
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

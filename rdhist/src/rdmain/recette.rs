extern crate serde;

use std::error::Error;
use std::fs::File;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};


#[derive(Debug, Serialize, Deserialize)]
pub struct Ingredients {
    media_type: String,
    nom: String,
    format: String,
    url: String,
    md5sum: String,
    dimensions: String
}

pub struct Opération {
    media_type: String,
    fichier_source: String,
    début: String,
    fin: String,
}

pub struct Contenus {
    media_type: Vec<Opération>
    }
pub struct YRecette {
    titre: String,
    date: String,
    collection: String,
    numéro: u32,
    codebarre: u64,
    ingrédients: Vec<Ingredients>,
    opérations: Vec<Opération>
}


pub struct Recette {
    pub path_recette_filename: String
}

impl Recette {
    pub fn new(filename: String) -> Result<Recette, Box<dyn Error>> {
        let recette = Recette {
            path_recette_filename: filename
        };

        Ok(recette)
    }

    pub fn load(mut self) -> Result<Recette, Box<dyn Error>> {
        let f = std::fs::File::open(&self.path_recette_filename)?;
        Ok(self)
    }

    pub fn display(self) {
       println!("{}", self.path_recette_filename);
    }
}

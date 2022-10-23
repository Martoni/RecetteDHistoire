extern crate serde;

/// Description du format de fichier pour les cahiers de recette
/// Un fichier de recette porte l'extension `.rdhist` et est au
/// format yaml

use std::fs;
use std::error::Error;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};

use crate::rdmain::RDHIST_EXT;
use crate::rdmain::outils::aplatir_chaine;

use crate::rdmain::ingredients::Ingredients;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Opération {
    pub media_type: String,
    pub fichier_source: String,
    pub début: String,
    pub fin: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Recette {
    pub ftype: String,
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
        let rawcontent = std::fs::read_to_string(&filename)?;
        let yaml_ret = serde_yaml::from_str(&rawcontent);
        let yrecette: Recette = match yaml_ret {
            Ok(recette) => recette,
            Err(msg) => return Err(format!("\t\n    Erreur dans le fichier {}:\t\n {}",
                                           &filename, msg).into()),
        };
        if !yrecette.ftype.eq("Recette") {
            Err(format!("{} n'est pas un fichier de recette ({})", filename, yrecette.ftype).into())
        } else {
            Ok(yrecette)
        }
    }
    
    pub fn get_collection_dir(&self, dir_path: String) -> Result<String, Box<dyn Error>> {
        let mut collection_dir: String = String::from(&dir_path);
        collection_dir.push_str("/");
        collection_dir.push_str(&aplatir_chaine(&self.collection));
        Ok(collection_dir)
    }

    pub fn create_collection_dir(&self, dir_path: String) -> Result<String, Box<dyn Error>> {
        let collection_dir = self.get_collection_dir(dir_path)?;
        fs::create_dir_all(&collection_dir)?;
        Ok(collection_dir)
    }

    pub fn get_cagette_dir(&self, collection_path: String) -> Result<String, Box<dyn Error>> {
        let mut full_path:String = String::from("");
        full_path.push_str(&collection_path);
        full_path.push_str("/");
        full_path.push_str(&aplatir_chaine(&self.date));
        full_path.push_str("_");
        full_path.push_str(&aplatir_chaine(&self.titre));
        Ok(full_path)
    }

    pub fn create_cagette_dir(&self, collection_path: String) -> Result<String, Box<dyn Error>> {
        let full_path = self.get_cagette_dir(collection_path)?;
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

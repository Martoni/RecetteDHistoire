extern crate serde;

use std::error::Error;
use serde::{Deserialize, Serialize};

use crate::rdmain::ingredients::Ingredients;

/* Description du format de fichier yaml pour la gestion des menus
 */

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NoeudMenu {
    pub ftype: String,
    pub collection: String,
    pub ingrédients: Vec<Ingredients>
}

impl NoeudMenu {
    pub fn new(filename: String) -> Result<NoeudMenu, Box<dyn Error>> {
        let rawcontent = std::fs::read_to_string(&filename)?;
        let yaml_ret = serde_yaml::from_str(&rawcontent);
        let ynoeud: NoeudMenu = match yaml_ret {
            Ok(noeud) => noeud,
            Err(msg) => return Err(format!("\t\n    Erreur dans le fichier {}:\t\n {}",
                                           &filename, msg).into()),
        };
        if !ynoeud.ftype.eq("Recette") {
            Err(format!("{} n'est pas un fichier de noeud ({})", filename, ynoeud.ftype).into())
        } else {
            Ok(ynoeud)
        }
    }
}

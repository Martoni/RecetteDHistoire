extern crate serde;
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

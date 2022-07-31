extern crate serde;

use std::fmt;
use serde::{Deserialize, Serialize};

use crate::rdmain::recette::{FormatTypes, SourceTypes};

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

impl fmt::Display for Ingredients {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.nom, self.media_type)
    }     
}

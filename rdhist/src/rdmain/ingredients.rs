extern crate serde;

use std::error::Error;
use std::fmt;
use std::process::Command;
use std::path::Path;

use serde::{Deserialize, Serialize};
use crate::rdmain::recette::{FormatTypes, SourceTypes};
 
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Ingredients {
    pub media_type: String,
    pub nom: String,
    pub format: Option<FormatTypes>,
    pub source: SourceTypes,
    pub url: Option<String>,
    pub md5sum: Option<String>,
    pub dimensions: Option<String>,
    pub temps: Option<f32>
}

impl fmt::Display for Ingredients {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.nom, self.media_type)
    } 
}

impl Ingredients {

    fn télécharger_dans(&self, dir_path: &String) -> Result<String, Box<dyn Error>> {
       if let Some(surl) = &self.url {
           println!("debug downloading {:?} to {:?}", &surl, dir_path);
           let filepath = Path::new(&dir_path)
               .join(self.nom.to_owned() +
               "." +
               &self.format.as_ref().ok_or("No format")?.extension());
           println!("filepath {:?}", filepath);
           let _cmdret = Command::new("curl")
               .arg("-o").arg(&filepath)
               .arg("-L").arg(&surl)
               .output()
               .expect(format!("Impossible de télécharger {}", &surl).as_str());
           return Ok(surl.to_string());
       } else {
           return Err("Un média de type 'Url' doit avoir une URL !".into());
       }
    }

    fn ripper_cd_dans(&self, dir_path: &String) -> Result<String, Box<dyn Error>> {
        Err("TODO: ripper le CD audio".into())
    }

    pub fn recolter_dans(&self, dir_path: &String) -> Result<String, Box<dyn Error>> {
        match self.source {
            SourceTypes::Url => {self.télécharger_dans(dir_path)}
            SourceTypes::Cdaudio => {self.ripper_cd_dans(dir_path)}
        }
    }
}

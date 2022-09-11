extern crate serde;

use std::error::Error;
use std::fmt;
use std::process::Command;
use std::path::Path;
use serde::{Deserialize, Serialize};

use crate::rdmain::recette::{FormatTypes, SourceTypes};
use crate::rdsound::cdaudio::*;
 
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Ingredients {
    pub media_type: String,
    pub nom: String,
    pub format: Option<FormatTypes>,
    pub source: SourceTypes,
    pub url: Option<String>,
    pub md5sum: Option<String>,
    pub dimensions: Option<String>,
    pub temps: Option<f32>,
    pub discid: Option<String>,
    pub tracks: Option<Vec<f32>>,
}

impl fmt::Display for Ingredients {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.nom, self.media_type)
    } 
}

impl Ingredients {

    fn télécharger_dans(&self, dir_path: &String) -> Result<String, Box<dyn Error>> {
       if let Some(surl) = &self.url {
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

    fn ripper_cd_dans(&self, _dir_path: &String)
                    -> Result<String, Box<dyn Error>> {
        /// On vérifie d'abord que le cd présent dans le lecteur est le bon
        if self.discid == None {
            return Err("La recette ne contient pas de signature de disque 'discid'".into());
        }
        let str_discid = self.discid.as_ref().unwrap();
        let ingr_discid = DiscId::from_string(str_discid.to_string())?;
        let drive_discid = DiscId::in_drive()?;
        if ingr_discid != drive_discid {
            let _ret = disc_eject().expect("Impossible d'éjecter le cd du lecteur");
            return Err("Le cd dans le lecteur ne correspond pas à l'histoire".into());
        }

        /// On récupère la table des matières
        let message = "TODO: ripper le CD audio";
        println!("{}", message);
        Err(message.into())
    }

    pub fn recolter_dans(&self, dir_path: &String) -> Result<String, Box<dyn Error>> {
        match self.source {
            SourceTypes::Url => {self.télécharger_dans(dir_path)}
            SourceTypes::Cdaudio => {self.ripper_cd_dans(dir_path)}
        }
    }
}

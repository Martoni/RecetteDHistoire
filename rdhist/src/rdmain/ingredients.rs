extern crate serde;

use std::error::Error;
use std::fmt;
use std::env;
use std::process::Command;
use std::path::Path;

use serde::{Deserialize, Serialize};
use crate::rdmain::recette::{FormatTypes, SourceTypes};

// https://georgik.rocks/how-to-download-binary-file-in-rust-by-reqwest/
use std::io::Cursor;
 
pub fn fetch_url(url: &String, file_name: String) -> Result<(), Box<dyn Error>> {
    let response = reqwest::blocking::get(url)?;
    let mut file = std::fs::File::create(file_name)?;
    let mut content =  Cursor::new(response.bytes()?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Ingredients {
    pub media_type: String,
    pub nom: String,
    pub format: Option<FormatTypes>,
    pub source: SourceTypes,
    pub url: Option<String>,
    pub md5sum: String,
    pub dimensions: Option<String>
}

impl fmt::Display for Ingredients {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.nom, self.media_type)
    } 
}

impl Ingredients {
    pub fn recolter_dans(&self, dir_path: &String) -> Result<String, Box<dyn Error>> {
        if self.source == SourceTypes::Url {
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
        Err(format!("Impossible de récolter l'ingrédient {:?}", self.nom).into())
    }
}

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

    fn ripper_cd_dans(&self, dir_path: String)
                    -> Result<String, Box<dyn Error>> {
        // On vérifie d'abord que le cd présent dans le lecteur est le bon
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

        // On récupère la table des matières et on vérifie que c'est la même
        // que dans la recette
        let cdtoc = CdToc::in_drive()?;
        if cdtoc.ntracks != ingr_discid.tracks_number()? {
            return Err("Le cd n'a pas le même nombre de piste que la recette".into());
        }
        let ingr_tracks = self.tracks.as_ref().expect("Pas de tracks dans les ingrédients");
        for n in 0..ingr_discid.tracks_number()? {
            let n: usize = n as usize; //Les indices de slice doivent être des usize
            if ingr_tracks[n] != cdtoc.tracks[n].as_secs_f32() {
                return Err(format!("La piste {} du CD fait {:?} sec, \
                                    alors qu'il est noté {:?} sec dans la recette.",
                                    n, cdtoc.tracks[n], ingr_tracks[n]).into());
            }
        }

        // on rippe le cd en wave dans le répertoire /tmp/
        let _ripwave = cdtoc.rip2wave("/tmp".to_string())?;

        // on converti les fichiers en mp3 dans la cagette
        let _wav2mp3 = cdtoc.save2mp3("/tmp".to_string(), dir_path)?;

        Ok("ok".into())
    }

    pub fn recolter_dans(&self, dir_path: &String) -> Result<String, Box<dyn Error>> {
        match self.source {
            SourceTypes::Url => {self.télécharger_dans(dir_path)}
            SourceTypes::Cdaudio => {self.ripper_cd_dans(dir_path.to_string())}
        }
    }
}

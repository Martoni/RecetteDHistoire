extern crate serde;

use std::error::Error;
use std::fmt;
use std::process::Command;
use std::path::Path;
use serde::{Deserialize, Serialize};

use crate::rdsound::cdaudio::*;

const CD_TRACK_FILENAME: &str = "track";

/* status de l'ingrédient */
pub enum StatusIngrédient {
    // L'ingrédient n'est pas téléchargé
    Absent,
    // L'ingrédient est téléchargé mais il y a un problème (message)
    Cagette(String),
    // L'ingrédient est téléchargé et valide
    Valide,
}

impl fmt::Display for StatusIngrédient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StatusIngrédient::Absent => write!(f, "Absent"),
            StatusIngrédient::Cagette(msg) => write!(f, "Présent : {}", msg),
            StatusIngrédient::Valide => write!(f, "Valide"),
        }
    }
}


/// Type de source à aller chercher
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SourceTypes {
    Url,
    Cdaudio
}

// Format de la source
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FormatTypes {
    Cdaudio,
    Mp3,
    Webp,
    Jpeg,
    Png,
}

impl FormatTypes {
    // Donne l'extension du format de fichier
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
pub enum MediaType {
    Image,
    Son,
}

impl fmt::Display for MediaType {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MediaType::Image => write!(formatter, "Image"),
            MediaType::Son => write!(formatter, "Son"),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Ingredients {
    pub media_type: MediaType,
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

    /* Fonction «générique» qui permet de récolter le média en fonction
     * du type de source */
    pub fn recolter_dans(&self, cagette_path: &String)
            -> Result<String, Box<dyn Error>> {
        match self.source {
            SourceTypes::Url => {self.télécharger_dans(cagette_path)}
            SourceTypes::Cdaudio => {self.ripper_cd_dans(cagette_path.to_string())}
        }
    }

    /* Status du media */
    pub fn status(&self, cagette_path: &String)
            -> Result<StatusIngrédient, Box<dyn Error>> {
        match self.media_type {
            MediaType::Image => {self.get_image_status(cagette_path)}
            MediaType::Son => {self.get_sound_status(cagette_path)}
        }
    }

    pub fn get_image_status(&self, recette_path: &String)
            -> Result<StatusIngrédient, Box<dyn Error>> {
        let image_path = format!("{}/{}.{}",
                                 recette_path,
                                 self.nom,
                                 self.format.as_ref().ok_or("No format")?.extension());
        let b = Path::new(&image_path).exists();
        if !b {
            return Ok(StatusIngrédient::Absent);
        }
        // TODO: vérifier les dimensions
        // TODO: vérifier le md5sum
        Ok(StatusIngrédient::Valide)
    }

    fn verifier_fichier_son_d_url(&self, recette_path: &String)
            -> Result<StatusIngrédient, Box<dyn Error>> {

        let track_path = format!("{}/{}.{}",
            recette_path,
            self.nom,
            self.format.as_ref().ok_or("No format")?.extension());

        let b = Path::new(&track_path).exists();
        if !b {
            return Ok(StatusIngrédient::Absent);
        }
        Ok(StatusIngrédient::Valide)
    }

    fn verifier_fichier_son_cdaudio(&self, recette_path: &String)
            -> Result<StatusIngrédient, Box<dyn Error>> {
        for track_num in 1..=self.tracks.as_ref().ok_or("No tracks")?.len() {
            let track_path = format!("{}/{}{:02}.{}",
                recette_path,
                CD_TRACK_FILENAME,
                track_num,
                self.format.as_ref().ok_or("No format")?.extension());
            let b = Path::new(&track_path).exists();
            if !b {
                if track_num == 1 {
                    return Ok(StatusIngrédient::Absent);
                } else {
                    return Ok(StatusIngrédient::Cagette(format!("Piste {} manquante", track_num)));
                }
            }
        }
                Ok(StatusIngrédient::Valide)
    }

    pub fn get_sound_status(&self, recette_path: &String)
            -> Result<StatusIngrédient, Box<dyn Error>> {
        let result = match self.source {
            SourceTypes::Url => {self.verifier_fichier_son_d_url(recette_path)},
            SourceTypes::Cdaudio => {self.verifier_fichier_son_cdaudio(recette_path)},
        };
        result
    }

    /* Si la source du média est une URL on la télécharge */
    fn télécharger_dans(&self, cagette_path: &String)
            -> Result<String, Box<dyn Error>> {
       if let Some(surl) = &self.url {
           let filepath = Path::new(&cagette_path)
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

    /* si la source du média est un CD audio, on le rip
     * et on le converti en mp3 */
    fn ripper_cd_dans(&self, cagette_path: String)
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
        let _wav2mp3 = cdtoc.save2mp3("/tmp".to_string(), cagette_path)?;

        Ok("ok".into())
    }

}

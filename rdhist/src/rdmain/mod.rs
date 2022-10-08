/* Manage main rdhist command line program */
extern crate dirs;

use std::error::Error;
use std::result::Result::Err;
use std::fs;

pub mod recette;
pub mod ingredients;
pub mod rdhistcli;
pub mod test_recette;
pub mod rdhistcaprice;

use crate::rdappareils;

use rdhistcli::RdhistCli;
use rdhistcaprice::RdhistCaprice;
use recette::Recette;

const DATA_DIR_PATH: &str = ".local/share/rdhist";
const DATA_DIR_RECETTES: &str = "recettes";
const DATA_DIR_CAGETTES: &str = "cagettes";
const _DATA_DIR_SERVICES: &str = "services";
const RDHIST_EXT: &str = "rdhist";

pub struct RdMainConfig {
    pub path: String,
    pub cmdlist: bool,
    pub caprice: bool,
    pub listappareils: bool,
    pub recette_filename: String
}

impl RdMainConfig {
    pub fn new() -> Result<RdMainConfig, Box<dyn Error>> {
        /* ouch */
        let home_dir = dirs::home_dir().unwrap().to_str().unwrap().to_owned();

        let conf = RdMainConfig {
            path: home_dir + "/" + &DATA_DIR_PATH,
            cmdlist: false,
            caprice: false,
            listappareils: false,
            recette_filename: "None".to_string()
        };

        Ok(conf)
    }

    /* set list command to true */
    pub fn set_cmdlist(self) -> Self {RdMainConfig {cmdlist: true, ..self}}
    pub fn set_caprice(self) -> Self {RdMainConfig {caprice: true, ..self}}
    pub fn set_listappareils(self) -> Self {RdMainConfig {listappareils: true, ..self}}

    pub fn set_recette_filename(self, recette_filename: String) -> Self {
        RdMainConfig {
            recette_filename,
            ..self
        }
    }

    pub fn get_list_files_recettes(&self) -> Result<Vec<Recette>, Box<dyn Error>> {
        let mut recettelist: Vec<Recette> = vec![];
        let recettes_dir = format!("{}/{}", &self.path, DATA_DIR_RECETTES);
        for file in fs::read_dir(recettes_dir)? {
            let file = file?;
            let path = file.path();
            if path.is_dir() {
                for subfile in fs::read_dir(path)? {
                    let rfilepath = subfile?.path().display().to_string();
                    if recette::recette_ext_check(&rfilepath) {
                        recettelist.push(Recette::new(rfilepath)?);
                    }
                }
            } else {
                let rfilepath = file.path().display().to_string();
                if recette::recette_ext_check(&rfilepath) {
                    recettelist.push(Recette::new(rfilepath)?);
                }
            }
        }
        Ok(recettelist)
    }

    pub fn get_recette_by_title(&self, arg_titre: &String)
                    -> Result<Recette, Box<dyn Error>> {
        for rec in self.get_list_files_recettes()? {
            if &rec.titre == arg_titre {
                return Ok(rec);
            }
        }
        Err(format!("La recette {:?} est introuvable", arg_titre).into())
    }

    pub fn afficher_recettes_disponibles(&self) -> Result<String, Box<dyn Error>> {
        let list_recettes = self.get_list_files_recettes()?;
        let retstr = list_recettes.iter()
                .map(|x| {format!("{} : {}", &x.collection, &x.titre).to_string()})
                .collect::<Vec<String>>()
                .join("\r\n");
        Ok(retstr.into())
    }

    pub fn list_appareils(&self) -> Result<String, Box<dyn Error>> {
        let rd_apps = rdappareils::RdAppareils::new()?;
        let retstr = rd_apps.listapp.iter()
                            .map(|x| x.nom().to_string())
                            .collect::<Vec<String>>()
                            .join("\r\n");
        Ok(retstr)
    }

    pub fn recolter_ingredients(&self, titre_recette: &String)
                    -> Result<bool, Box<dyn Error>> {
        let rec = self.get_recette_by_title(titre_recette)?;
        let cagette_dir = rec.create_cagette_dir(format!("{}/{}", self.path, DATA_DIR_CAGETTES))?;
        println!("Répertoire créé pour faire les courses:\n{}\n", &cagette_dir);
        for ingredient in rec.ingrédients.iter() {
            println!("nom: {}", ingredient);
            let _fichier_ingredient = ingredient.recolter_dans(&cagette_dir)?;
        }
        Ok(true)
    }
}

pub fn run(cfg: RdMainConfig) -> Result<(), Box<dyn Error>> {
    if cfg.cmdlist {
        let ret = cfg.afficher_recettes_disponibles()?;
        println!("{}", &ret);
        Ok(())
    } else if cfg.listappareils {
        let ret = cfg.list_appareils()?;
        println!("{}", &ret);
        Ok(())
    } else if cfg.recette_filename != "None" {
        if let Err(e) = cfg.recolter_ingredients(&cfg.recette_filename) {
          return Err(format!("Impossible de récolter les éléments : {}", e).into())
        }
        Ok(())
    } else if cfg.caprice {
        // lancement de la console «caprice»
        let rdc = RdhistCaprice::new(cfg)?;
        match rdc.cli() {
            Ok(_) => {
                println!("Salut, et pas de caprices!");
                Ok(())}
            Err(e) => Err(format!("On avait dit pas de caprices : {}", e).into())
        }
    } else {
        // lancement de la console «classique»
        let rdc = RdhistCli::new(cfg)?;
        match rdc.cli() {
            Ok(_) => {println!("Au revoir"); Ok(())}
            Err(e) => Err(format!("Il y a eu un problème avec la ligne de commande : {}", e).into())
        }

    }
}

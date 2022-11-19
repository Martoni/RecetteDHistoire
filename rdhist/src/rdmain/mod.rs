/* Manage main rdhist command line program */
extern crate dirs;

use std::error::Error;
use std::result::Result::Err;
use std::fs;

pub mod recette;
pub mod ingredients;
pub mod rdhistcli;
pub mod rdhisthelper;
pub mod outils;
pub mod noeud_menu;
pub mod test_recette;
pub mod test_rdmain;

use crate::rdappareils;

use rdhistcli::RdhistCli;
use recette::Recette;
use noeud_menu::NoeudMenu;

const DATA_DIR_PATH: &str = ".local/share/rdhist";
const DATA_DIR_RECETTES: &str = "recettes";
const DATA_DIR_NOEUDS: &str = "noeuds";
const DATA_DIR_CAGETTES: &str = "cagettes";
const _DATA_DIR_SERVICES: &str = "services";
const RDHIST_EXT: &str = "rdhist";

/* commandes */
/* TODO: liste <type> (liste recette, liste appareil, ...) */ 
pub const CMD_LISTE_RECETTES: &str  = "listrec";
pub const CMD_LISTE_APPAREILS: &str = "listapp";
pub const CMD_LISTE_NOEUDS: &str = "listnoeuds";
pub const CMD_RECOLTER: &str = "recolter";
pub const CMD_INFORECETTE: &str = "inforec";
pub const CMD_SERVIR: &str = "servir";

pub struct RdMainConfig {
    pub path: String,
    pub cmdlist: bool,
    pub listappareils: bool,
    pub recette_titre: String
}

impl RdMainConfig {
    pub fn new() -> Result<RdMainConfig, Box<dyn Error>> {
        /* ouch */
        let home_dir = dirs::home_dir().unwrap().to_str().unwrap().to_owned();

        let conf = RdMainConfig {
            path: home_dir + "/" + &DATA_DIR_PATH,
            cmdlist: false,
            listappareils: false,
            recette_titre: "None".to_string()
        };

        Ok(conf)
    }

    /* set list command to true */
    pub fn set_cmdlist(self) -> Self {RdMainConfig {cmdlist: true, ..self}}
    pub fn set_listappareils(self) -> Self {RdMainConfig {listappareils: true, ..self}}

    pub fn set_titre_recette(self, recette_titre: String) -> Self {
        RdMainConfig {
            recette_titre,
            ..self
        }
    }

    pub fn list_noeuds(&self) -> Result<String, Box<dyn Error>> {
        let mut noeudslist: Vec<NoeudMenu> = vec![];
        let noeuds_dir = format!("{}/{}", &self.path, DATA_DIR_NOEUDS);
        for file in fs::read_dir(&noeuds_dir)? {
            let file = file?;
            let path = file.path();
            if path.is_dir() {
                for subfile in fs::read_dir(path)? {
                    let rfilepath = subfile?.path().display().to_string();
                    if recette::recette_ext_check(&rfilepath) {
                        noeudslist.push(NoeudMenu::new(rfilepath)?);
                    }
                }
            } else {
                let rfilepath = file.path().display().to_string();
                if recette::recette_ext_check(&rfilepath) {
                    noeudslist.push(NoeudMenu::new(rfilepath)?);
                }
            }
        }
        Ok(noeuds_dir)
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

    fn get_data_dir_cagettes(&self) -> String {
        format!("{}/{}", self.path, DATA_DIR_CAGETTES)
    }

    pub fn recolter_ingredients(&self, titre_recette: &String)
                    -> Result<bool, Box<dyn Error>> {
        let rec = self.get_recette_by_title(titre_recette)?;
        let collection_dir = rec.create_collection_dir(self.get_data_dir_cagettes())?;
        let cagette_dir = rec.create_cagette_dir(collection_dir)?;
        println!("Répertoire créé pour faire les courses:\n{}\n", &cagette_dir);
        for ingredient in rec.ingrédients.iter() {
            println!("nom: {}", ingredient);
            let _fichier_ingredient = ingredient.recolter_dans(&cagette_dir)?;
        }
        println!("ÀFaire: copier la recette dans la cagette");
        Ok(true)
    }

    pub fn info_recette(&self, titre_recette: &String)
                    -> Result<bool, Box<dyn Error>> {
        let rec = self.get_recette_by_title(titre_recette)?;
        println!("Titre: {}", rec.titre);
        println!("Collection: {}", rec.collection);
        println!("Numéro: {}", rec.numéro);
        if let Some(codebarre) = rec.codebarre {
            println!("codebarre: {}", codebarre);
        }
        println!("Date de parution: {}", rec.date);
        println!("");
        println!("{} Ingrédients :", rec.ingrédients.len());
        let collection_path = rec.get_collection_dir(self.get_data_dir_cagettes())?;
        let cagette_path = rec.get_cagette_dir(collection_path)?;
        for ingrédient in rec.ingrédients {
            let status = ingrédient.status(&cagette_path)?;
            println!("{} -> {}", ingrédient, status); // À faire : indiquer si l'ingrédient est valide
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
    } else if cfg.recette_titre != "None" {
        if let Err(e) = cfg.recolter_ingredients(&cfg.recette_titre) {
          return Err(format!("Impossible de récolter les éléments : {}", e).into())
        }
        Ok(())
    } else {
        // lancement de la console «classique»
        let rdc = RdhistCli::new(cfg)?;
        match rdc.cli() {
            Ok(_) => {println!("Au revoir"); Ok(())}
            Err(e) => Err(format!("Il y a eu un problème avec la ligne de commande : {}", e).into())
        }

    }
}

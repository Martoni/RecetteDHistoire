/* Manage main rdhist command line program */
extern crate dirs;

use std::error::Error;
use std::result::Result::Err;
use std::fs;

pub mod recette;

use recette::Recette;

const CONFIG_DIR_PATH: &str = ".rdhist";
const CONFIG_DIR_RECETTES: &str = "recettes";
const CONFIG_DIR_CAGETTES: &str = "cagettes";
const CONFIG_DIR_SORTIES: &str = "sorties";
const RDHIST_EXT: &str = "rdhist";

pub struct Config {
    pub path: String,
    pub cmdlist: bool,
    pub recette_filename: String
}

impl Config {
    pub fn new() -> Result<Config, Box<dyn Error>> {
        /* ouch */
        let home_dir = dirs::home_dir().unwrap().to_str().unwrap().to_owned();

        let conf = Config {
            path: home_dir + "/" + &CONFIG_DIR_PATH,
            cmdlist: false,
            recette_filename: "None".to_string()
        };

        Ok(conf)
    }

    /* set list command to true */
    pub fn set_cmdlist(self) -> Self {
        Config {
            cmdlist: true,
            ..self
        }
    }

    pub fn set_recette_filename(self, recette_filename: String) -> Self {
        Config {
            recette_filename,
            ..self
        }
    }

    pub fn get_files_recettes(&self) -> Result<Vec<Recette>, Box<dyn Error>> {
        let mut recettelist: Vec<Recette> = vec![];
        for file in fs::read_dir(format!("{}/{}",
                                        &self.path,
                                        CONFIG_DIR_RECETTES))? {
            let file = file?;
            let path = file.path();
            if path.is_dir() {
                for subfile in fs::read_dir(path)? {
                    let rfilepath = subfile?.path().display().to_string();
                    recettelist.push(Recette::new(rfilepath)?);
                }
            } else {
                let rfilepath = file.path().display().to_string();
                recettelist.push(Recette::new(rfilepath)?);
            }
        }
        Ok(recettelist)
    }
}

pub fn run(cfg: Config) -> Result<Config, Box<dyn Error>> {
    if cfg.cmdlist {
        let list_recettes = cfg.get_files_recettes()?;
        for recette in list_recettes {
            recette.display();
        }
        Ok(cfg)
    } else if cfg.recette_filename != "None" {
        print!("TODO: Vérifier puis allez chercher les ingrédients de {:?}", cfg.recette_filename);
        Ok(cfg)
    } else {
        Err("Donnez au moins une option".into())
    }
}

/* Manage main rdhist command line program */
extern crate dirs;

use std::error::Error;
use std::result::Result::Err;
use std::fs;

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

    pub fn set_cmdlist(mut self) -> Self {
        Config {
            cmdlist: true,
            ..self
        }
    }

    pub fn set_recette_filename(mut self, recette_filename: String) -> Self {
        Config {
            recette_filename,
            ..self
        }
    }
}

pub fn run(cfg: Config) -> Result<Config, Box<dyn Error>> {
    if cfg.cmdlist {
        for file in fs::read_dir(format!("{}/{}",
                                        &cfg.path,
                                        CONFIG_DIR_RECETTES))? {
            let file = file?;
            let path = file.path();
            if path.is_dir() {
                for subfile in fs::read_dir(path)? {
                    println!("{}", subfile?.path().display());
                }

            } else {
                println!("{}", file.path().display());
            }
        }
        Ok(cfg)
    } else if cfg.recette_filename != "None" {
        print!("TODO: Vérifier puis allez chercher les ingrédients de {:?}", cfg.recette_filename);
        Ok(cfg)
    } else {
        Err("Donnez au moins une option".into())
    }
}

/* Manage main rdhist command line program */
extern crate dirs;

use std::error::Error;
use std::result::Result::Err;
use std::fs;

const CONFIG_DIR_PATH: &str = ".rdhist";
const CONFIG_DIR_RECETTES: &str = "recettes";
const CONFIG_DIR_CAGETTES: &str = "cagettes";
const CONFIG_DIR_SORTIES: &str = "sorties";


pub struct Config {
    pub path: String,
    pub cmdlist: bool
}

impl Config {
    pub fn new() -> Config {
        /* ouch */
        let home_dir = dirs::home_dir().unwrap().to_str().unwrap().to_owned();

        Config {
            path: home_dir + "/" + &CONFIG_DIR_PATH,
            cmdlist: false
        }
    }

    pub fn set_cmdlist(mut self) -> Self {
        self.cmdlist = true;
        self
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
        Err("TODO: lister les recettes disponibles".into())
    } else {
        Ok(cfg)
    }
}

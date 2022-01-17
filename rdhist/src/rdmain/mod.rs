/* Manage main rdhist command line program */
extern crate dirs;

use std::error::Error;
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
        Config {
        path: dirs::home_dir().unwrap().to_str().unwrap().to_owned() + "/" + &CONFIG_DIR_PATH,
        cmdlist: false }
    }

    pub fn set_cmdlist(mut self) -> Self {
        self.cmdlist = true;
        self
    }
}

pub fn run(cfg: Config) -> Result<Config, Box<dyn Error>> {
    if cfg.cmdlist {
        println!("TODO: lister les recettes disponibles");

        for file in fs::read_dir(format!("{}/{}", &cfg.path,CONFIG_DIR_RECETTES))? {
            println!("{}", file.unwrap().path().display());
        }
        Ok(cfg)
    } else {
        Ok(cfg)
    }
}

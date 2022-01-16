/* Manage main rdhist command line program */
use std::error::Error;

const CONFIG_DIR_PATH: &str = "~/.rdhist";

pub struct Config {
    pub path: &'static str,
    pub cmdlist: bool
}

impl Config {
    pub fn new() -> Config {
        Config { path: CONFIG_DIR_PATH, cmdlist: false }
    }

    pub fn set_cmdlist(mut self) -> Self {
        self.cmdlist = true;
        self
    }
}

pub fn run(cfg: Config) -> Result<Config, Box<dyn Error>> {
    if cfg.cmdlist {
        println!("TODO: lister les recettes disponibles");
        Ok(cfg) //XXX Return Error 
    } else {
        Ok(cfg)
    }
}

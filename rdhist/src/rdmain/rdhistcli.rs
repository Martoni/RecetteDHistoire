use rustyline::error::ReadlineError;
use rustyline::Editor;
use crate::rdmain::Error;
use crate::rdmain::RdMainConfig;
use crate::rdmain::{CMD_LISTE_APPAREILS,
                    CMD_LISTE_RECETTES,
                    CMD_RECOLTER};

const RDHISTORY_FILENAME: &str = "rdhistory.txt";
const RDHISTPROMPT: &str = "rdhist> ";

const LIST_CMD: &'static [&'static str] = &[
    "exit",
    "help",
    CMD_LISTE_APPAREILS,
    CMD_LISTE_RECETTES,
    CMD_RECOLTER];

pub struct RdhistCli {
    pub history_filename: String,
    pub prompt: String,
    pub maincfg: RdMainConfig,
}

impl RdhistCli {

    pub fn new(cfg: RdMainConfig) -> Result<RdhistCli, Box<dyn Error>> {
        let rdhistcli = RdhistCli {
            history_filename: RDHISTORY_FILENAME.into(),
            prompt: RDHISTPROMPT.into(),
            maincfg: cfg,
        };
        Ok(rdhistcli)
    }

    pub fn cli(self) -> Result<RdhistCli, Box<dyn Error>> {
        // `()` can be used when no completer is required
        let mut rl = Editor::<()>::new();
        if rl.load_history(&self.history_filename).is_err() {
            println!("No previous history.");
        }
        loop {
            let readline = rl.readline(&self.prompt);
            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str());
                    let args: Vec<&str> = line.split_whitespace().collect(); //XXX: doit prendre en compte les quote "
                    if args.len() != 0 {
                        match args[0] {
                            "exit" => {break}
                            "help" => {let _ = &self.help()?;}
                            CMD_LISTE_RECETTES => {let _ = &self.list()?;}
                            CMD_LISTE_APPAREILS => {let _ = &self.list_appareils()?;}
                            CMD_RECOLTER => {let _ = &self.recolter(args)?;}
                            _ => {println!("args {:?}", args)}
                        }
                    }
                },
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break;
                },
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break
                },
                Err(err) => {
                    println!("Error: {:?}", err);
                    break
                }
            }
        }
        rl.save_history(&self.history_filename).unwrap();
        Ok(self)
    }

    // commandes
    fn list(&self) -> Result<(), Box<dyn Error>>{
        let ret = self.maincfg.afficher_recettes_disponibles()?;
        println!("{}", &ret);
        Ok(())
    }

    fn help(&self) -> Result<(), Box<dyn Error>>{
         println!("Liste des commandes disponibles:\r\n{}\r\n",
                    LIST_CMD.join("\r\n"));
        Ok(())
    }

    fn list_appareils(&self) -> Result<(), Box<dyn Error>>{
        let ret = self.maincfg.list_appareils()?;
        println!("{}", &ret);
        Ok(())
    }

    fn recolter(&self, args: Vec<&str>) -> Result<(), Box<dyn Error>>{
        let ret = self.maincfg.recolter_ingredients(&args[1].to_string())?;
        println!("{}", &ret);
        Ok(())
    }
}

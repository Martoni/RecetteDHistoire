use rustyline::error::ReadlineError;
use rustyline::Editor;
use crate::rdmain::Error;
use crate::rdmain::RdMainConfig;

const RDHISTORY_FILENAME: &str = "rdhistory.txt";
const RDHISTPROMPT: &str = "rdhist> ";

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
                    let args: Vec<&str> = line.split_whitespace().collect();
                    if args.len() != 0 {
                        match args[0] {
                            "exit" => {break}
                            "list" => {let _ = &self.list()?;}
                            "listapp" => {let _ = &self.list_appareils()?;}
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

    fn list_appareils(&self) -> Result<(), Box<dyn Error>>{
        let ret = self.maincfg.list_appareils()?;
        println!("{}", &ret);
        Ok(())
    }
}

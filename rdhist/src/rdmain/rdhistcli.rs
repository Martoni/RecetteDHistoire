extern crate shell_words;

// example from https://github.com/kkawakam/rustyline/blob/master/examples/example.rs
use rustyline::error::ReadlineError;
use rustyline::{CompletionType, Config, EditMode, Editor};
use rustyline::highlight::MatchingBracketHighlighter;
use rustyline::completion::FilenameCompleter;

use crate::rdmain::rdhisthelper::{RdHistHelper, rdhist_hints};
use crate::rdmain::Error;
use crate::rdmain::RdMainConfig;
use crate::rdmain::{CMD_LISTE_APPAREILS,
                    CMD_LISTE_RECETTES,
                    CMD_SERVIR,
                    CMD_INFORECETTE,
                    CMD_RECOLTER};

const RDHISTORY_FILENAME: &'static str = "rdhistory.txt";
const RDHISTPROMPT: &'static str = "rdhist> ";
const RDHISTPROMPT_COLOR: &'static str = "\x1b[1;32mrdhist> \x1b[0m";

pub const LIST_CMD: &'static [&'static str] = &[
    "exit",
    "help",
    CMD_INFORECETTE,
    CMD_LISTE_APPAREILS,
    CMD_LISTE_RECETTES,
    CMD_RECOLTER,
    CMD_SERVIR];

pub struct RdhistCli {
    pub history_filename: String,
    pub prompt: String,
    pub maincfg: RdMainConfig,
}

impl RdhistCli {

    pub fn new(cfg: RdMainConfig) -> Result<RdhistCli, Box<dyn Error>> {
        let rdhistcli = RdhistCli {
            history_filename: RDHISTORY_FILENAME.into(),
            prompt: RDHISTPROMPT_COLOR.into(),
            maincfg: cfg,
        };
        Ok(rdhistcli)
    }

    pub fn cli(self) -> Result<RdhistCli, Box<dyn Error>> {
        // `()` can be used when no completer is required
        let config = Config::builder()
            .edit_mode(EditMode::Vi)
            .history_ignore_space(true)
            .completion_type(CompletionType::List)
            .build();

        let h = RdHistHelper {
            hints: rdhist_hints(),
            highlighter: MatchingBracketHighlighter::new(),
            completer: FilenameCompleter::new(),
            colored_prompt: RDHISTPROMPT.to_owned(),
        };

        let mut rl = Editor::with_config(config);
        rl.set_helper(Some(h));
        if rl.load_history(&self.history_filename).is_err() {
            println!("No previous history.");
        }
        loop {
            let readline = rl.readline(&self.prompt);
            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str());
                    let args = shell_words::split(&line)?;
                    if args.len() != 0 {
                        match args[0].as_str() {
                            "exit" => {
                                break}
                            "help" => {
                                let _ = &self.help()?;}
                            CMD_INFORECETTE => {
                                let _ = &self.inforecette(args)?;}
                            CMD_LISTE_RECETTES => {
                                let _ = &self.list()?;}
                            CMD_LISTE_APPAREILS => {
                                let _ = &self.list_appareils()?;}
                            CMD_RECOLTER => {
                                let _ = &self.recolter(args)?;}
                            CMD_SERVIR => {
                                let _ = &self.servir()?;}
                            _ => {println!("Erreur: commande inconnue {:?}", args)}
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

    fn inforecette(&self, args: Vec<String>) -> Result<(), Box<dyn Error>>{
        let ret = self.maincfg.info_recette(&args[1].to_string());
        match ret {
            Ok(_)  => {},
            Err(msg) => {println!("Erreur: {}", &msg)},
        }
        Ok(())
    }

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
        let ret = self.maincfg.list_appareils();
        match ret {
            Ok(ret) => {println!("{}", &ret)},
            Err(msg) =>{println!("Erreur: {}", &msg)},
        }
        Ok(())
    }

    fn recolter(&self, args: Vec<String>) -> Result<(), Box<dyn Error>>{
        let ret = self.maincfg.recolter_ingredients(&args[1].to_string());
        match ret {
            Ok(_) => {},
            Err(msg) => {println!("Erreur: {}", &msg)},
        }
        Ok(())
    }

    fn servir(&self) -> Result<(), Box<dyn Error>>{
        println!("ÀFAIRE: servir");
        Ok(())
    }
}

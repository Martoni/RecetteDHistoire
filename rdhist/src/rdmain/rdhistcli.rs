use std::borrow::Cow::{self, Borrowed, Owned};
// example from https://github.com/kkawakam/rustyline/blob/master/examples/example.rs
use rustyline::completion::FilenameCompleter;
use rustyline::error::ReadlineError;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::HistoryHinter;
use rustyline::validate::MatchingBracketValidator;
use rustyline::{CompletionType, Config, EditMode, Editor};
use rustyline_derive::{Completer, Helper, Hinter, Validator};
use crate::rdmain::Error;
use crate::rdmain::RdMainConfig;
use crate::rdmain::{CMD_LISTE_APPAREILS,
                    CMD_LISTE_RECETTES,
                    CMD_RECOLTER};

extern crate shell_words;

const RDHISTORY_FILENAME: &str = "rdhistory.txt";
const RDHISTPROMPT: &str = "rdhist> ";

const LIST_CMD: &'static [&'static str] = &[
    "exit",
    "help",
    CMD_LISTE_APPAREILS,
    CMD_LISTE_RECETTES,
    CMD_RECOLTER];


#[derive(Helper, Completer, Hinter, Validator)]
struct MyHelper {
    #[rustyline(Completer)]
    completer: FilenameCompleter,
    highlighter: MatchingBracketHighlighter,
    #[rustyline(Validator)]
    validator: MatchingBracketValidator,
    #[rustyline(Hinter)]
    hinter: HistoryHinter,
    colored_prompt: String,
}

impl Highlighter for MyHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Borrowed(&self.colored_prompt)
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.highlighter.highlight_char(line, pos)
    }
}

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
        let config = Config::builder()
            .edit_mode(EditMode::Vi)
            .history_ignore_space(true)
            .completion_type(CompletionType::List)
            .build();
        let h = MyHelper {
            completer: FilenameCompleter::new(),
            highlighter: MatchingBracketHighlighter::new(),
            hinter: HistoryHinter {},
            colored_prompt: RDHISTPROMPT.to_owned(),
            validator: MatchingBracketValidator::new(),
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
            Ok(msg) => {println!("{}", &msg)},
            Err(msg) => {println!("Erreur: {}", &msg)},
        }
        Ok(())
    }
}

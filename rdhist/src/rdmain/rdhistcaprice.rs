use crate::rdmain::Error;
use crate::rdmain::RdMainConfig;
use crate::rdmain::{CMD_LISTE_APPAREILS,
                    CMD_LISTE_RECETTES,
                    CMD_RECOLTER,
                    CMD_SERVIR};

use caprice::{Caprice, CapriceCommand};

static PROMPT: &str = "rdhist :!";

pub struct RdhistCaprice {
    pub rdmaincfg: RdMainConfig,
}

impl RdhistCaprice {
  pub fn new(cfg: RdMainConfig) -> Result<RdhistCaprice, Box<dyn Error>> {
    Ok(RdhistCaprice {
        rdmaincfg: cfg,
    })
  }

  pub fn cli(self) -> Result<RdhistCaprice, Box<dyn Error>> {
      let caprice = Caprice::new()
          .set_prompt(PROMPT) // set the prompt
          .disable_ctrl_c() // pressing control + c won't close the caprice console
          .set_keywords(vec![  // set some tokens
              CMD_LISTE_APPAREILS.to_owned(),
              CMD_LISTE_RECETTES.to_owned(),
              CMD_RECOLTER.to_owned(),
              CMD_SERVIR.to_owned(),
              "exit".to_owned(), // an exit keyword
          ])
          .init(); // initialises the caprice terminal
      // caprice.run() will run the caprice in a separate thread.
      // you can use the returned tx and rx channels for receiving and sending messages
      // to caprice instance
      let (tx, rx, caprice_handle) = caprice.run().unwrap();
      // our main application runs here
      // for this example we will simply print back
      // the tokens send by caprice
      loop {
          // if we received a token from caprice
          if let Ok(token) = rx.try_recv() {
              match token.as_str() {
                  // leave if the user types exit
                  "exit" => {
                      tx.send(CapriceCommand::Exit).unwrap();
                      caprice_handle.join()
                          .expect("couldn't join thread")
                          .expect("Caprice run has encountered an error");
                       // at this point caprice has already exited,
                       //let the main process do as well
                       break},
                  // else send back the token to be printed
                  CMD_LISTE_RECETTES => {
                    let ret = self.rdmaincfg.afficher_recettes_disponibles()?;
                    tx.send(CapriceCommand::Println(ret.into())).unwrap();
                  }
                  CMD_LISTE_APPAREILS => {
                    let ret = self.rdmaincfg.list_appareils()?;
                    tx.send(CapriceCommand::Println(ret.into())).unwrap();
                  }
                  _ => {
                      let print_token = format!("Got {} from Caprice", token);
                      tx.send(CapriceCommand::Println(print_token)).unwrap();
                  }
              }
          }
      }
      Ok(self)
  }
}

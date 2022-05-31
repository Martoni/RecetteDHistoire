use rustyline::error::ReadlineError;
use rustyline::Editor;

const RDHISTORY_FILENAME: &str = "rdhistory.txt";
const RDHISTPROMPT: &str = "rdhist> ";

pub fn rdhistcli() {
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    if rl.load_history(RDHISTORY_FILENAME).is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(RDHISTPROMPT);
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("Line: {}", line);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
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
    rl.save_history(RDHISTORY_FILENAME).unwrap();
}

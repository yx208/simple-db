use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

const HISTORY_FILE: &str = "d://demo/mydb-rs/history.txt";

fn main() -> Result<()> {

    let mut rl = Editor::<()>::new()?;
    if rl.load_history(HISTORY_FILE).is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("Line: {}", line);

                if line == "exit" {
                    break
                }
            }
            Err(ReadlineError::Interrupted) => {
                // CTRL-C so just skip
            }
            Err(ReadlineError::Eof) => {
                // CTRL-D so exit
                break
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }

    rl.save_history(HISTORY_FILE)

}
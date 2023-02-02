use miette::GraphicalReportHandler;
use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};
use sql_parser::ast::SqlQuery;
use sql_parser::parse::Parse;

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
                match SqlQuery::parse_format_error(line.as_ref()) {
                    Ok(query) => println!("{query:?}"),
                    Err(err) => {
                        let mut s = String::new();
                        GraphicalReportHandler::new()
                            .render_report(&mut s, &err)
                            .unwrap();
                        println!("{s}");
                    }
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

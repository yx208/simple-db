mod display;

use miette::GraphicalReportHandler;
use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};
use sql_parser::ast::SqlQuery;
use sql_parser::parse::Parse;
use crate::display::display_response;

const HISTORY_FILE: &str = "d://demo/mydb-rs/history.txt";

fn main() -> Result<()> {

    let mut rl = Editor::<()>::new()?;
    if rl.load_history(HISTORY_FILE).is_err() {
        println!("No previous history.");
    }

    let mut exec = sql_execution::Execution::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {

                rl.add_history_entry(line.as_str());
                let line: &str = line.as_ref();
                let res = exec.parse_and_run(line);
                match res {
                    Ok(exec_res) => display_response(exec_res),
                    Err(err) => {
                        let mut s = String::new();
                        GraphicalReportHandler::new()
                            .with_cause_chain()
                            .with_context_lines(10)
                            .render_report(&mut s, &err)
                            .unwrap();
                        println!("{s}");
                    }
                }
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

mod cd;
mod complete;
mod completion;
mod constant;
mod echo;
mod parser;
mod pwd;
mod syscmd;
mod typecmd;
mod utils;
use rustyline::completion::FilenameCompleter;
use rustyline::config::Configurer;
use rustyline::error::ReadlineError;
use rustyline::{CompletionType, Editor, Result};

use crate::completion::CompletionHelper;

fn main() -> Result<()> {
    let mut rl = Editor::new()?;
    rl.set_completion_type(CompletionType::List);
    rl.set_helper(Some(CompletionHelper {
        file_completer: FilenameCompleter::new(),
    }));

    loop {
        let readline = rl.readline("$ ");
        match readline {
            Ok(line) => {
                let continue_reading = parser::handle_line(line);
                if !continue_reading {
                    break;
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}

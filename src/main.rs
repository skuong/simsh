mod cd;
mod complete;
mod completion;
mod constant;
mod echo;
mod jobs;
mod parser;
mod pwd;
mod syscmd;
mod typecmd;
mod utils;
use std::collections::{BTreeMap, HashMap};

use parser::HandleLineParams;
use rustyline::completion::FilenameCompleter;
use rustyline::config::Configurer;
use rustyline::error::ReadlineError;
use rustyline::{CompletionType, Editor, Result};

use crate::completion::CompletionHelper;

pub(crate) struct Job {
    pub(crate) pid: u32,
    pub(crate) command: String,
    pub(crate) status: String,
}

fn main() -> Result<()> {
    let mut rl = Editor::new()?;
    rl.set_completion_type(CompletionType::List);
    rl.set_helper(Some(CompletionHelper {
        file_completer: FilenameCompleter::new(),
        registered_specs: HashMap::<String, String>::new(),
    }));

    let mut jobs = BTreeMap::<usize, Job>::new();

    loop {
        let readline = rl.readline("$ ");

        match readline {
            Ok(line) => {
                let completion_helper = rl.helper_mut().expect("Failed to get registered specs");
                let continue_reading = parser::handle_line(HandleLineParams {
                    line,
                    registered_specs: &mut completion_helper.registered_specs,
                    jobs: &mut jobs,
                });
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

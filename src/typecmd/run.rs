use std::env;
use std::path::Path;

use crate::constant::BUILTIN_COMMANDS;
use crate::syscmd::is_cmd_exists_and_executable;

pub fn run(cmd: &str) {
    match cmd {
        builtin if BUILTIN_COMMANDS.iter().any(|cmd| *cmd == builtin) => {
            println!("{} is a shell builtin", builtin)
        }
        maybe_cmd => match env::var("PATH") {
            Ok(paths) => {
                for path in paths.split(":") {
                    let potential_path = Path::new(path).join(maybe_cmd);

                    if is_cmd_exists_and_executable(&potential_path) {
                        println!("{} is {}", maybe_cmd, potential_path.display());
                        return;
                    }
                }

                println!("{}: not found", maybe_cmd);
            }

            Err(_) => {}
        },
    }
}

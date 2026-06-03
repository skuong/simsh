use std::env;
use std::path::Path;

use crate::syscmd::is_cmd_exists_and_executable;

pub fn run(cmd: &str) {
    match cmd {
        "echo" => println!("{} is a shell builtin", cmd),
        "exit" => println!("{} is a shell builtin", cmd),
        "type" => println!("{} is a shell builtin", cmd),
        "pwd" => println!("{} is a shell builtin", cmd),
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

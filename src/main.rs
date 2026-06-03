#[allow(unused_imports)]
use std::io::{self, Write};
mod echo;
mod typecmd;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();

        io::stdin().read_line(&mut command).unwrap_or_default();

        let command = command.trim();

        if command.starts_with("echo ") {
            echo::run(&command[5..]);
            continue;
        }

        if command.starts_with("type ") {
            typecmd::run(&command[5..]);
            continue;
        }

        match command {
            "" => {
                continue;
            }
            "exit" => {
                break;
            }
            _ => {
                println!("{command}: command not found");
            }
        }
    }
}

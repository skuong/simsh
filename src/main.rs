#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();

        io::stdin().read_line(&mut command).unwrap_or_default();

        let command = command.trim();

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

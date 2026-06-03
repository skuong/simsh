#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();

        io::stdin().read_line(&mut command).unwrap_or_default();

        let command = command.trim();

        if command.starts_with("echo") {
            let message = command.splitn(2, " ").collect::<Vec<&str>>();
            if message.len() == 2 {
                println!("{}", message[1]);
            }
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

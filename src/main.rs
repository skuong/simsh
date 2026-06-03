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

        if command.starts_with("type ") {
            let interpreted_command = &command[5..];

            match interpreted_command {
                "echo" => println!("{} is a shell builtin", interpreted_command),
                "exit" => println!("{} is a shell builtin", interpreted_command),
                "type" => println!("{} is a shell builtin", interpreted_command),
                _ => println!("{}: not found", interpreted_command),
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

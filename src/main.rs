#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;

use crate::syscmd::is_cmd_exists_in_path_and_executable;
mod echo;
mod syscmd;
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
            potential_system_command => {
                let potential_command: Vec<&str> = potential_system_command.split(" ").collect();

                if is_cmd_exists_in_path_and_executable(potential_command[0]) {
                    Command::new(potential_command[0])
                        .args(&potential_command[1..])
                        .status()
                        .expect("Failed to execute command");
                } else {
                    println!("{potential_system_command}: command not found");
                }
            }
        }
    }
}

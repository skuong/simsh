#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;

use crate::{parser::command_input_parser, syscmd::is_cmd_exists_in_path_and_executable};
mod cd;
mod echo;
mod parser;
mod pwd;
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

        if command.starts_with("cd ") {
            cd::run(&command[3..]);
            continue;
        }

        if command == "pwd" {
            pwd::run();
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
                let parts: Vec<&str> = potential_system_command.splitn(2, " ").collect();
                let cmd = parts[0];
                let args_str = parts.get(1).unwrap_or(&"");

                if is_cmd_exists_in_path_and_executable(cmd) {
                    Command::new(cmd)
                        .args(command_input_parser(args_str))
                        .status()
                        .expect("Failed to execute command");
                } else {
                    println!("{potential_system_command}: command not found");
                }
            }
        }
    }
}

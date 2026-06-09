use std::io::{self, Write};

use crate::syscmd::is_cmd_exists_in_path_and_executable;
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
                let (args, file_descriptor, redirect_file_name) =
                    parser::command_input_parser(command);

                if is_cmd_exists_in_path_and_executable(&args[0]) {
                    syscmd::handle_system_command(
                        &args[0],
                        args[1..].to_vec(),
                        file_descriptor,
                        redirect_file_name,
                    );

                    continue;
                }

                println!("{potential_system_command}: command not found");
            }
        }
    }
}

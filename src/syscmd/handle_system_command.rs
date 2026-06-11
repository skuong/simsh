use std::process::{Command, Stdio};

use crate::{parser::OutputRedirectType, utils::create_a_file_to_redirect_output_to};

pub fn handle_system_command(
    cmd: &String,
    args: Vec<String>,
    file_descriptor: Option<char>,
    redirect_file_name: String,
    output_redirect_type: Option<OutputRedirectType>,
) {
    if let Some(fd) = file_descriptor {
        let output_file =
            create_a_file_to_redirect_output_to(output_redirect_type, redirect_file_name)
                .expect("Failed to open file");

        let mut command = Command::new(cmd);
        command.args(args);

        match fd {
            '1' => {
                command
                    .stdout(Stdio::from(output_file))
                    .status()
                    .expect("Failed to execute command");
            }
            '2' => {
                command
                    .stderr(Stdio::from(output_file))
                    .status()
                    .expect("Failed to execute command");
            }
            _ => {}
        }
    } else {
        Command::new(cmd)
            .args(args)
            .status()
            .expect("Failed to execute command");
    }
}

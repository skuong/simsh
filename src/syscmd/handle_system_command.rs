use std::process::{Command, Stdio};

use crate::{parser::CommandParserOutput, utils::create_a_file_to_redirect_output_to};

pub fn handle_system_command(params: CommandParserOutput) {
    let cmd = &params.args[0];
    let args = params.args[1..].to_vec();

    if let Some(fd) = params.file_descriptor {
        let output_file =
            create_a_file_to_redirect_output_to(params.write_type, params.redirect_filename)
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

use std::process::{Command, Stdio};

use crate::{
    parser::CommandParserOutput, syscmd::spawn_command_in_the_background,
    utils::create_a_file_to_redirect_output_to,
};

pub fn handle_system_command(params: CommandParserOutput) -> Option<u32> {
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
                let command = command.stdout(Stdio::from(output_file));
                if params.is_background {
                    return Some(spawn_command_in_the_background(command));
                }

                command.status().expect("Failed to execute command");
                return None;
            }
            '2' => {
                let command = command.stderr(Stdio::from(output_file));
                if params.is_background {
                    return Some(spawn_command_in_the_background(command));
                }

                command.status().expect("Failed to execute command");
                return None;
            }
            _ => return None,
        }
    } else {
        let mut command = Command::new(cmd);
        command.args(args);

        if params.is_background {
            return Some(spawn_command_in_the_background(&mut command));
        }

        command.status().expect("Failed to execute command");
        return None;
    }
}

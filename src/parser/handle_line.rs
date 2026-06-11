use std::collections::HashMap;

use crate::{
    cd, complete, echo, parser, pwd,
    syscmd::{self, is_cmd_exists_in_path_and_executable},
    typecmd,
};

pub fn handle_line(line: String, registered_specs: &mut HashMap<String, String>) -> bool {
    if line.starts_with("complete ") {
        complete::run(&line[9..], registered_specs);
        return true;
    }

    if line.starts_with("echo ") {
        echo::run(&line[5..]);
        return true;
    }

    if line.starts_with("type ") {
        typecmd::run(&line[5..]);
        return true;
    }

    if line.starts_with("cd ") {
        cd::run(&line[3..]);
        return true;
    }

    if line == "pwd" {
        pwd::run();
        return true;
    }

    match line.as_str() {
        "" => {
            return true;
        }
        "exit" => {
            return false;
        }
        potential_system_command => {
            let (args, file_descriptor, redirect_file_name, output_redirect_type) =
                parser::command_input_parser(&line);

            if is_cmd_exists_in_path_and_executable(&args[0]) {
                syscmd::handle_system_command(
                    &args[0],
                    args[1..].to_vec(),
                    file_descriptor,
                    redirect_file_name,
                    output_redirect_type,
                );

                return true;
            }

            println!("{potential_system_command}: command not found");

            return true;
        }
    }
}

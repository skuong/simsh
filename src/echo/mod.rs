use crate::{parser::command_input_parser, utils::create_a_file_to_redirect_output_to};
use std::io::Write;

pub(crate) fn run(message: &str) {
    let (args, file_descriptor, redirect_file_name, output_redirect_type) =
        command_input_parser(message);

    let output = args.join(" ");

    if let Some(fd) = file_descriptor {
        let mut output_file =
            create_a_file_to_redirect_output_to(output_redirect_type, redirect_file_name)
                .expect("Failed to open file");

        match fd {
            '1' => {
                let _ = output_file.write_all(output.as_bytes());
                let _ = output_file.write_all("\n".as_bytes());
            }
            '2' => {
                let _ = output_file.write("".as_bytes());
                println!("{output}");
            }
            _ => {}
        }
    } else {
        println!("{output}");
    }
}

use crate::{parser::command_input_parser, utils::create_a_file_to_redirect_output_to};
use std::io::Write;

pub(crate) fn run(input: &str) {
    let parsed_input = command_input_parser(input);

    let output = parsed_input.args.join(" ");

    if let Some(fd) = parsed_input.file_descriptor {
        let mut output_file = create_a_file_to_redirect_output_to(
            parsed_input.write_type,
            parsed_input.redirect_filename,
        )
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

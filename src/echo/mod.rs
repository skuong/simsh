use crate::parser::command_input_parser;
use std::{fs::File, io::Write};

pub(crate) fn run(message: &str) {
    let (args, file_descriptor, redirect_file_name) = command_input_parser(message);
    let output = args.join(" ");

    if !redirect_file_name.is_empty() {
        if file_descriptor == Some('2') {
            File::create(&redirect_file_name).expect("Failed to create redirect file");
            println!("{}", output);
        } else if file_descriptor == Some('1') {
            let mut file =
                File::create(&redirect_file_name).expect("Failed to create redirect file");

            file.write_all(output.as_bytes()).ok();
            file.write_all(b"\n").ok();
        }
    } else {
        println!("{}", output);
    }
}

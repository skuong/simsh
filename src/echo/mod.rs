use crate::parser::command_input_parser;
use std::fs;

pub(crate) fn run(message: &str) {
    let (mut args, redirect_stdout_file_name) = command_input_parser(message);

    if !redirect_stdout_file_name.is_empty() {
        args.push("\n".to_string());
        let _ = fs::write(redirect_stdout_file_name, args.join(" "));
    } else {
        println!("{}", args.join(" "));
    }
}

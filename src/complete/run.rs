use crate::parser;
use std::{
    fs::{self, OpenOptions},
    io::Read,
};

pub fn run(input: &str) {
    let args = parser::command_input_parser(input).0;

    let mut open_options = OpenOptions::new();
    open_options.create(true).read(true).append(true);

    let name = format!("/tmp/{}", std::process::id());
    let file_name = name.as_str();

    let file_result = open_options.open(file_name);
    let mut specs = String::new();
    match file_result {
        Ok(mut file) => {
            let _ = file.read_to_string(&mut specs);
        }
        Err(err) => {
            println!("{}", err);
        }
    }

    match args[0].as_str() {
        "-p" => {
            let command = args[1].clone();

            for line in specs.lines() {
                let args_from_specs: Vec<&str> = line.splitn(2, " ").collect();
                if command == args_from_specs[0] {
                    println!("complete -C '{}' {}", args_from_specs[1], command);
                    return;
                }
            }

            println!("complete: {}: no completion specification", command);
        }
        "-C" => {
            specs.push_str("\n");
            specs.push_str(format!("{} {}", args[2], args[1]).as_str());
            let _ = fs::write(file_name, specs);
        }
        _ => {}
    };
}

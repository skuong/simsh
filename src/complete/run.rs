use crate::parser;
use std::collections::HashMap;

pub fn run(input: &str, registered_specs: &mut HashMap<String, String>) {
    let args = parser::command_input_parser(input).args;

    match args[0].as_str() {
        "-p" => {
            let command = args[1].clone();

            if let Some(completer_script_path) = registered_specs.get(&command) {
                println!("complete -C '{}' {}", completer_script_path, command);
                return;
            }

            println!("complete: {}: no completion specification", command);
        }
        "-C" => {
            registered_specs.insert(args[2].clone(), args[1].clone());
        }
        "-r" => {
            let command = args[1].clone();
            registered_specs.remove(&command);
        }
        _ => {}
    };
}

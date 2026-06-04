use crate::parser::command_input_parser;

pub(crate) fn run(message: &str) {
    let parsed_args = command_input_parser(message);
    println!("{}", parsed_args.join(" "));
}

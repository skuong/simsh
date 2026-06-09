mod handle_back_slash;
mod handle_double_quote;
mod handle_single_quote;
mod handle_whitespace;
use handle_back_slash::handle_back_slash;
use handle_double_quote::handle_double_quote;
use handle_single_quote::handle_single_quote;
use handle_whitespace::handle_whitespace;

pub(crate) fn command_input_parser(input: &str) -> (Vec<String>, String) {
    let mut args = Vec::new();
    let mut redirect_file_name = String::new();

    let mut current_arg = String::new();
    let mut last_quote = None;
    let mut last_slash = None;

    let mut is_file_descriptor = false;
    let mut is_last_whitespace = false;
    let mut is_stdout_redirect = false;

    for character in input.chars() {
        match (character, last_quote, last_slash) {
            ('\\', quote, slash) => {
                (last_quote, last_slash, current_arg) =
                    handle_back_slash(character, quote, slash, current_arg.clone());
            }

            ('"', quote, slash) => {
                (last_quote, last_slash, current_arg) =
                    handle_double_quote(character, quote, slash, current_arg.clone());
            }

            ('\'', quote, slash) => {
                (last_quote, last_slash, current_arg) =
                    handle_single_quote(character, quote, slash, current_arg.clone());
            }

            ('1', None, None) => {
                if is_last_whitespace {
                    is_file_descriptor = true;
                } else {
                    current_arg.push('1');
                }
            }

            ('>', None, None) => {
                if is_last_whitespace || is_file_descriptor {
                    is_stdout_redirect = true;
                }
            }

            (char, quote, slash) if char.is_whitespace() => {
                let (quote, slash, arg) =
                    handle_whitespace(character, quote, slash, current_arg.clone());

                last_quote = quote;
                last_slash = slash;

                if arg.is_empty() && !current_arg.is_empty() {
                    if is_stdout_redirect {
                        redirect_file_name = current_arg;
                    } else {
                        args.push(current_arg);
                    }
                    current_arg = String::new();
                } else {
                    current_arg = arg;
                }

                is_last_whitespace = true
            }

            (char, _, _) => {
                current_arg.push(char);
                last_slash = None;

                is_last_whitespace = false;
            }
        }
    }

    if !current_arg.is_empty() {
        if is_stdout_redirect {
            redirect_file_name = current_arg;
        } else {
            args.push(current_arg);
        }
    }

    (args, redirect_file_name)
}

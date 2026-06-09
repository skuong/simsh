mod handle_back_slash;
use handle_back_slash::handle_back_slash;

pub(crate) fn command_input_parser(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut last_quote = None;
    let mut last_slash = None;

    for character in input.chars() {
        match (character, last_quote, last_slash) {
            ('\\', quote, slash) => {
                let (quote, slash, arg) =
                    handle_back_slash(character, quote, slash, current_arg.clone());
                last_quote = quote;
                last_slash = slash;
                current_arg = arg;
            }

            ('"', None, slash) => {
                if let Some(_) = slash {
                    current_arg.push(character);
                    last_slash = None;
                    continue;
                }
                last_quote = Some('"');
            }
            ('"', Some('"'), slash) => {
                if let Some(_) = slash {
                    current_arg.push(character);
                    last_slash = None;
                    continue;
                }
                last_quote = None;
            }
            ('\'', Some('"'), _) => {
                current_arg.push(character);
            }
            ('\'', None, slash) => {
                if let Some(_) = slash {
                    current_arg.push(character);
                    last_slash = None;
                    continue;
                }
                last_quote = Some('\'');
            }
            ('\'', Some('\''), _) => {
                last_quote = None;
            }
            (char, None, slash) if char.is_whitespace() => {
                if let Some(_) = slash {
                    current_arg.push(character);
                    last_slash = None;
                    continue;
                }

                if !current_arg.is_empty() {
                    args.push(current_arg);
                    current_arg = String::new();
                }
            }
            (char, _, _) => {
                current_arg.push(char);
                last_slash = None;
            }
        }
    }

    if !current_arg.is_empty() {
        args.push(current_arg);
    }

    args
}

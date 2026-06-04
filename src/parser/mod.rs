pub(crate) fn command_input_parser(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut last_quote = None;

    for character in input.chars() {
        match (character, last_quote) {
            ('\'', None) => {
                last_quote = Some('\'');
            }
            ('\'', Some('\'')) => {
                last_quote = None;
            }
            (char, None) if char.is_whitespace() => {
                if !current_arg.is_empty() {
                    args.push(current_arg);
                    current_arg = String::new();
                }
            }
            (char, _) => {
                current_arg.push(char);
            }
        }
    }

    if !current_arg.is_empty() {
        args.push(current_arg);
    }

    args
}

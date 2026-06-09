pub fn handle_back_slash(
    character: char,
    quote: Option<char>,
    slash: Option<char>,
    current_arg: String,
) -> (Option<char>, Option<char>, String) {
    let mut slash = slash;
    let mut current_arg = current_arg;

    match (character, quote, slash) {
        ('\\', None, None) => {
            slash = Some('\\');
        }
        ('\\', None, Some('\\')) => {
            slash = None;
            current_arg.push(character);
        }
        ('\\', Some('"'), None) => {
            slash = Some('\\');
        }
        ('\\', Some('\''), _) => {
            current_arg.push(character);
        }
        ('\\', Some('"'), Some('\\')) => {
            slash = None;
            current_arg.push(character);
        }
        _ => {}
    }

    return (quote, slash, current_arg);
}

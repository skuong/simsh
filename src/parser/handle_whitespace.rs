pub fn handle_whitespace(
    character: char,
    quote: Option<char>,
    slash: Option<char>,
    arg: String,
) -> (Option<char>, Option<char>, String) {
    let mut slash = slash;
    let mut arg = arg;

    if let Some(_) = quote {
        arg.push(character);
        slash = None;
    } else if let Some(_) = slash {
        arg.push(character);
        slash = None;
    } else {
        arg = String::new();
    }

    (quote, slash, arg)
}

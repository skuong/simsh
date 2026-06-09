pub fn handle_single_quote(
    character: char,
    quote: Option<char>,
    slash: Option<char>,
    arg: String,
) -> (Option<char>, Option<char>, String) {
    let mut slash = slash;
    let mut quote = quote;
    let mut arg = arg;

    match (character, quote, slash) {
        ('\'', Some('"'), _) => {
            arg.push(character);
        }

        ('\'', Some('\''), _) => {
            quote = None;
        }

        ('\'', None, Some('\\')) => {
            arg.push(character);
            slash = None;
        }

        ('\'', None, None) => {
            quote = Some('\'');
        }
        _ => {}
    }

    (quote, slash, arg)
}

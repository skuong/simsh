use crate::parser::OutputRedirectType;

pub(crate) struct HandleWhitespaceParams<'a> {
    pub(crate) character: char,
    pub(crate) quote: Option<char>,
    pub(crate) slash: &'a mut Option<char>,
    pub(crate) current_arg: &'a mut String,
    pub(crate) redirect_filename: &'a mut String,
    pub(crate) write_type: &'a mut Option<OutputRedirectType>,
    pub(crate) args: &'a mut Vec<String>,
}

pub fn handle_whitespace(
    HandleWhitespaceParams {
        character,
        quote,
        slash,
        current_arg,
        redirect_filename,
        write_type,
        args,
    }: HandleWhitespaceParams,
) {
    if let Some(_) = quote {
        current_arg.push(character);
        *slash = None;
    } else if let Some(_) = slash {
        current_arg.push(character);
        *slash = None;
    } else {
        if let Some(_) = write_type
            && redirect_filename.is_empty()
        {
            *redirect_filename = current_arg.clone();
        } else {
            if !current_arg.is_empty() {
                args.push(current_arg.clone());
            }
        }

        *current_arg = String::new();
    }
}

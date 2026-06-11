use std::fs::{File, OpenOptions};

use crate::parser::OutputRedirectType;

pub fn create_a_file_to_redirect_output_to(
    redirect_type: Option<OutputRedirectType>,
    file_name: String,
) -> Result<File, String> {
    let mut open_file_option = OpenOptions::new();
    open_file_option.create(true);

    if let Some(OutputRedirectType::Append) = redirect_type {
        open_file_option.append(true);
    } else {
        open_file_option.write(true).truncate(true);
    }

    open_file_option
        .open(file_name)
        .map_err(|err| err.to_string())
}

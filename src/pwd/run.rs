use std::env;

pub(crate) fn run() {
    match env::current_dir() {
        Ok(current_dir) => println!("{}", current_dir.display()),
        Err(_) => {}
    }
}

use std::{env, path::Path};
pub fn run(dir: &str) {
    let dir_path = Path::new(dir);
    if env::set_current_dir(dir_path).is_err() {
        println!("cd: {}: No such file or directory", dir);
    }
}

use std::{env, path::Path};
pub fn run(dir: &str) {
    if dir == "~"
        && let Ok(home) = env::var("HOME")
    {
        let home_path = Path::new(&home);

        if env::set_current_dir(home_path).is_err() {
            println!("Can't go home");
        }

        return;
    }

    let dir_path = Path::new(dir);

    if env::set_current_dir(dir_path).is_err() {
        println!("cd: {}: No such file or directory", dir);
    }
}

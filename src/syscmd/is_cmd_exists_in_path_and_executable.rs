use std::env;
use std::path::Path;

use crate::syscmd::is_cmd_exists_and_executable;

pub fn is_cmd_exists_in_path_and_executable(cmd: &str) -> bool {
    match env::var("PATH") {
        Ok(paths) => {
            for path in paths.split(":") {
                let potential_cmd_path = Path::new(path).join(cmd);

                if is_cmd_exists_and_executable(&potential_cmd_path) {
                    return true;
                }
            }
        }

        Err(_) => {}
    };

    return false;
}

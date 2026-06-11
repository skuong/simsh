use std::env;

use crate::syscmd::is_cmd_exists_and_executable;

pub fn is_cmd_exists_in_path_and_executable(cmd: &str) -> bool {
    let path_var = env::var_os("PATH").unwrap_or_default();

    for path in env::split_paths(&path_var) {
        let potential_cmd_path = path.join(cmd);

        if is_cmd_exists_and_executable(&potential_cmd_path) {
            return true;
        }
    }

    return false;
}

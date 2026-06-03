use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

pub fn is_cmd_exists_and_executable(full_path: &PathBuf) -> bool {
    if full_path.exists() {
        match full_path.metadata() {
            Ok(metadata) => {
                let mode = metadata.permissions().mode();
                let is_executable = mode & 0o111 != 0;
                if is_executable {
                    return true;
                }
            }
            Err(_) => {
                return false;
            }
        }
    }

    return false;
}

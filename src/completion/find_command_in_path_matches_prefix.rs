use rustyline::completion::Pair;
use std::{env, fs, os::unix::fs::PermissionsExt};

pub fn find_command_in_path_matches_prefix(prefix: &str) -> Vec<Pair> {
    let path_var = env::var_os("PATH").unwrap_or_default();

    let mut matches: Vec<Pair> = env::split_paths(&path_var)
        .filter_map(|dir| fs::read_dir(dir).ok())
        .flatten()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            let name = entry.file_name().to_string_lossy().into_owned();
            name.starts_with(prefix) &&
            // is a file and executable
            entry
                .metadata()
                .map(|metadata| metadata.is_file() && metadata.permissions().mode() & 0o111 != 0)
                .unwrap_or(false)
        })
        .map(|entry| {
            let name = entry.file_name().to_string_lossy().into_owned();
            Pair {
                display: name.clone(),
                replacement: format!("{name} "),
            }
        })
        .collect();

    matches.sort_by_key(|pair| pair.replacement.clone());

    matches
}

use rustyline::completion::{FilenameCompleter, Pair};
use rustyline::error::ReadlineError;
use std::env;
use std::path::Path;

pub fn get_file_completions(
    file_completer: &FilenameCompleter,
    line: &str,
    pos: usize,
) -> Result<(usize, Vec<Pair>), ReadlineError> {
    file_completer
        .complete_path(line, pos)
        .map(|(position, pairs)| {
            let mut new_pairs: Vec<Pair> = pairs
                .iter()
                .map(|pair| {
                    let cwd = env::current_dir().unwrap();
                    let replacement_path = Path::new(&pair.replacement);
                    let is_dir = cwd.join(replacement_path).is_dir();

                    Pair {
                        display: format!(
                            "{}{}",
                            pair.display.clone(),
                            if !is_dir { "" } else { "/" }
                        ),
                        replacement: format!(
                            "{}{}",
                            pair.replacement,
                            if !is_dir { " " } else { "" }
                        ),
                    }
                })
                .collect();

            new_pairs.sort_by_key(|pair| pair.replacement.clone());

            (position, new_pairs)
        })
}

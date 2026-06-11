use std::env;
use std::path::Path;

use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper};

use crate::completion::find_command_in_path_matches_prefix;
use crate::constant::BUILTIN_COMMANDS;

pub struct CompletionHelper {
    pub file_completer: FilenameCompleter,
}

impl Completer for CompletionHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        let matches: Vec<Pair> = BUILTIN_COMMANDS
            .iter()
            .filter(|cmd| cmd.starts_with(&line[..pos]))
            .map(|cmd| Pair {
                display: cmd.to_string(),
                replacement: format!("{cmd} "),
            })
            .collect();

        if matches.len() > 0 {
            return Ok((0, matches));
        }
        let syscmd_matches = find_command_in_path_matches_prefix(line);
        if syscmd_matches.len() > 0 {
            return Ok((0, syscmd_matches));
        }

        let file_completion =
            self.file_completer
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
                });

        file_completion
    }
}

impl Highlighter for CompletionHelper {}
impl Hinter for CompletionHelper {
    type Hint = String;
    fn hint(&self, _line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<Self::Hint> {
        Some("".to_string())
    }
}
impl Validator for CompletionHelper {}
impl Helper for CompletionHelper {}

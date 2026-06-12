use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper};
use std::collections::HashMap;

use crate::completion::{
    find_command_in_path_matches_prefix, get_builtin_command_completions,
    get_completions_from_registered_spec, get_file_completions,
};
use crate::parser;

pub struct CompletionHelper {
    pub file_completer: FilenameCompleter,
    pub registered_specs: HashMap<String, String>,
}

impl Completer for CompletionHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        let args = parser::command_input_parser(line).0;
        let command = args[0].clone();

        if let Some(path) = self.registered_specs.get(&command) {
            if let Some((pos, pairs)) = get_completions_from_registered_spec(args, pos, path) {
                return Ok((pos, pairs));
            }
        };

        let builtin_commands = get_builtin_command_completions(command.as_str(), pos);
        if let Some(pairs) = builtin_commands {
            return Ok((0, pairs));
        }

        let syscmd_matches = find_command_in_path_matches_prefix(command.as_str());
        if syscmd_matches.len() > 0 {
            return Ok((0, syscmd_matches));
        }

        get_file_completions(&self.file_completer, command.as_str(), pos)
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

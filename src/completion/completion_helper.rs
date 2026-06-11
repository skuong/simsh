use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper};

use crate::constant::BUILTIN_COMMANDS;

pub struct CompletionHelper;

impl Completer for CompletionHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        let matches = BUILTIN_COMMANDS
            .iter()
            .filter(|cmd| cmd.starts_with(&line[..pos]))
            .map(|cmd| Pair {
                display: cmd.to_string(),
                replacement: format!("{cmd} "),
            })
            .collect();

        Ok((0, matches))
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

use crate::constant::BUILTIN_COMMANDS;
use rustyline::completion::Pair;

pub fn get_builtin_command_completions(line: &str, pos: usize) -> Option<Vec<Pair>> {
    let matches: Vec<Pair> = BUILTIN_COMMANDS
        .iter()
        .filter(|cmd| cmd.starts_with(&line[..pos]))
        .map(|cmd| Pair {
            display: cmd.to_string(),
            replacement: format!("{cmd} "),
        })
        .collect();

    if matches.len() > 0 {
        return Some(matches);
    }

    None
}

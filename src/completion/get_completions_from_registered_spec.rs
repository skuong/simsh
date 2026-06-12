use rustyline::completion::Pair;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

pub fn get_completions_from_registered_spec(path: &String) -> Option<Vec<Pair>> {
    let mut child = Command::new(path)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute completion script");

    let mut completions: Vec<String> = vec![];

    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(completion) = line {
                completions.push(completion);
            }
        }
    }

    if completions.len() > 0 {
        return Some(
            completions
                .iter()
                .map(|completion| Pair {
                    display: completion.clone(),
                    replacement: format!("{} ", completion.clone()),
                })
                .collect(),
        );
    }

    None
}

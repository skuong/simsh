use rustyline::completion::Pair;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

pub fn get_completions_from_registered_spec(
    args: Vec<String>,
    pos: usize,
    path: &String,
) -> Option<(usize, Vec<Pair>)> {
    let command = args[0].clone();
    let prefix = if args.len() == 3 {
        args[2].clone()
    } else {
        args[1].clone()
    };
    let subcommand = if args.len() > 2 {
        args[1].clone()
    } else {
        "".to_string()
    };

    let mut child = Command::new(path)
        .args(vec![command, prefix.clone(), subcommand])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute completion script");

    let mut completions: Vec<String> = vec![];

    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(completion) = line
                && completion.starts_with(prefix.as_str())
            {
                completions.push(completion);
            }
        }
    }

    if completions.len() > 0 {
        return Some((
            pos - prefix.len(),
            completions
                .iter()
                .map(|completion| Pair {
                    display: completion.clone(),
                    replacement: format!("{} ", completion.clone()),
                })
                .collect(),
        ));
    }

    None
}

use rustyline::completion::Pair;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

use crate::parser;

pub fn get_completions_from_registered_spec(
    line: &str,
    pos: usize,
    path: &String,
) -> Option<(usize, Vec<Pair>)> {
    let args = parser::command_input_parser(line).0;

    let command = args[0].clone();
    let mut subcommand = String::new();
    let mut prefix = String::new();
    if args.len() > 3 {
        if let Some(arg) = args.last() {
            prefix = arg.clone();
            subcommand = args[args.len() - 2].clone();
        }
    } else if args.len() == 3 {
        subcommand = args[1].clone();
        prefix = args[2].clone();
    } else if args.len() == 2 {
        prefix = args[1].clone();
        subcommand = command.clone();
    } else {
        subcommand = command.clone();
    }

    let is_line_ends_with_space = line.ends_with(" ");
    let mut vars = HashMap::<String, String>::new();
    vars.insert("COMP_LINE".to_string(), line.to_string());
    vars.insert("COMP_POINT".to_string(), pos.to_string());

    let mut child = Command::new(path)
        .envs(vars)
        .args(vec![command, prefix.clone(), subcommand])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute completion script");

    let mut completions: Vec<String> = vec![];

    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line_from_reader in reader.lines() {
            if let Ok(completion) = line_from_reader
                && completion.starts_with(prefix.as_str())
            {
                completions.push(completion);
            }
        }
    }

    let pos = pos
        - if is_line_ends_with_space {
            0
        } else {
            prefix.len()
        };

    if completions.len() > 0 {
        return Some((
            pos,
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

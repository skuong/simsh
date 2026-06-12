use std::collections::{BTreeMap, HashMap};

use crate::{
    Job, cd, complete, echo, jobs, parser, pwd,
    syscmd::{self, is_cmd_exists_in_path_and_executable},
    typecmd,
};

pub(crate) struct HandleLineParams<'a> {
    pub(crate) line: String,
    pub(crate) registered_specs: &'a mut HashMap<String, String>,
    pub(crate) jobs: &'a mut BTreeMap<usize, Job>,
}

pub fn handle_line(
    HandleLineParams {
        line,
        registered_specs,
        jobs,
    }: HandleLineParams,
) -> bool {
    if line.starts_with("complete ") {
        complete::run(&line[9..], registered_specs);
        return true;
    }

    if line.starts_with("echo ") {
        echo::run(&line[5..]);
        return true;
    }

    if line.starts_with("jobs") {
        jobs::run(&line[4..], &jobs);
        return true;
    }

    if line.starts_with("type ") {
        typecmd::run(&line[5..]);
        return true;
    }

    if line.starts_with("cd ") {
        cd::run(&line[3..]);
        return true;
    }

    if line == "pwd" {
        pwd::run();
        return true;
    }

    match line.as_str() {
        "" => {
            return true;
        }
        "exit" => {
            return false;
        }
        potential_system_command => {
            let parser_output = parser::command_input_parser(&line);

            if is_cmd_exists_in_path_and_executable(&parser_output.args[0]) {
                if let Some(job) = syscmd::handle_system_command(parser_output) {
                    let pid = job.pid;

                    let job_number = jobs.len() + 1;
                    jobs.insert(job_number, job);

                    println!("[{}] {}", job_number, pid);
                }

                return true;
            }

            println!("{potential_system_command}: command not found");

            return true;
        }
    }
}

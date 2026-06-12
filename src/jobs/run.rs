use std::collections::BTreeMap;

use crate::Job;

pub fn run(_input: &str, jobs: &BTreeMap<usize, Job>) {
    let len = jobs.len();

    for kv in jobs {
        let job = kv.1;

        let padded_status = format!("{:24}", job.status);

        let marker = if *kv.0 == len {
            '+'
        } else if *kv.0 == len - 1 {
            '-'
        } else {
            ' '
        };

        println!("[{}]{marker}  {padded_status}{}", kv.0, job.command,);
    }
}

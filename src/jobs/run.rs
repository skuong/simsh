use std::collections::HashMap;

use crate::Job;

pub fn run(_input: &str, jobs: &HashMap<u32, Job>) {
    for kv in jobs {
        let job = kv.1;
        let padded_status = format!("{:24}", job.status);
        println!("[{}]+  {}{}", kv.0, padded_status, job.command);
    }
}

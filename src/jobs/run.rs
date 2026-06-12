use crate::Job;
use nix::{
    sys::wait::{WaitPidFlag, WaitStatus, waitpid},
    unistd::Pid,
};
use std::collections::BTreeMap;

pub fn run(_input: &str, jobs: &mut BTreeMap<usize, Job>) {
    let len = jobs.len();
    let mut done_jobs: Vec<usize> = vec![];

    for (i, kv) in jobs.iter_mut().enumerate() {
        let job = kv.1;

        match waitpid(Pid::from_raw(job.pid as i32), Some(WaitPidFlag::WNOHANG)) {
            Ok(WaitStatus::Exited(_, code)) => {
                job.status = format!("{:24}", "Done");
                job.exit_code = Some(code);
                done_jobs.push(*kv.0);
            }
            Ok(WaitStatus::StillAlive) => {
                job.status = format!("{:24}", "Running");
            }
            _ => {}
        };

        let padded_status = format!("{:24}", job.status);

        let marker = if i == len - 1 {
            '+'
        } else if i == len - 2 {
            '-'
        } else {
            ' '
        };

        println!("[{}]{marker}  {padded_status}{}", kv.0, job.command,);
    }

    for pid in done_jobs {
        jobs.remove(&pid);
    }
}

use std::{
    fs::File,
    process::{Command, Stdio},
};

pub fn handle_system_command(
    cmd: &String,
    args: Vec<String>,
    file_descriptor: Option<char>,
    redirect_file_name: String,
) {
    match file_descriptor {
        Some('1') => {
            let output_file =
                File::create(redirect_file_name).expect("Failed to create output file");

            Command::new(cmd)
                .args(args)
                .stdout(Stdio::from(output_file))
                .status()
                .expect("Failed to execute command");
        }
        Some('2') => {
            let output_file =
                File::create(redirect_file_name).expect("Failed to create output file");

            Command::new(cmd)
                .args(args)
                .stderr(Stdio::from(output_file))
                .status()
                .expect("Failed to execute command");
        }
        None => {
            Command::new(cmd)
                .args(args)
                .status()
                .expect("Failed to execute command");
        }
        _ => {}
    }
}

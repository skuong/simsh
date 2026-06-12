use std::process::Command;

pub fn spawn_command_in_the_background(command: &mut Command) -> u32 {
    let child = command.spawn().expect("Failed to execute command");
    return child.id();
}

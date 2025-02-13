use std::process::Command;

pub fn terminal_info() -> String {
    let terminal_info_command = Command::new("sh")
        .arg("-c")
        .arg("echo $TERM")
        .output()
        .expect("Failed to execute echo $TERM command");

    let res = String::from_utf8_lossy(&terminal_info_command.stdout)
        .trim()
        .to_string();

    res
}

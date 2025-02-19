use std::process::Command;

pub fn shell_info() -> String {
    let package_info_command = Command::new("sh")
        .arg("-c")
        .arg("echo $SHELL")
        .output()
        .expect("Failed to execute echo $SHELL command");

    let res = String::from_utf8_lossy(&package_info_command.stdout)
        .split("/")
        .last()
        .expect("Error spliting the string")
        .trim()
        .to_string();

    res
}

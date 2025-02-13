use std::process::Command;

pub fn host_info() -> String {
    let hostname_info_command = Command::new("hostname")
        .output()
        .expect("Failed to execute uptime command");

    let res = String::from_utf8_lossy(&hostname_info_command.stdout)
        .trim()
        .to_string();

    res
}

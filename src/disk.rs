use std::process::Command;

pub fn disk_info() -> String {
    let memory_info_command = Command::new("df")
        .arg("-h")
        .output()
        .expect("Failed to execute df command");

    let binding = String::from_utf8_lossy(&memory_info_command.stdout);
    let vec: Vec<&str> = binding.lines().collect();
    let values: Vec<&str> = vec[1].split_whitespace().collect();

    let res = format!("{} / {}", values[2], values[1]);
    res
}

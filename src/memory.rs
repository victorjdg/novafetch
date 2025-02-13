use std::process::Command;

pub fn memory_info(tp: &str) -> String {
    let memory_info_command = Command::new("free")
        .arg("-m")
        .output()
        .expect("Failed to execute free command");

    let mem_type = match tp {
        _ if tp.eq("memory") => 1,
        _ if tp.eq("swap") => 2,
        _ => return "Memory type not supported".to_string(),
    };

    let binding = String::from_utf8_lossy(&memory_info_command.stdout);
    let vec: Vec<&str> = binding.lines().collect();

    let mut values = Vec::new();
    let mut current_value = String::new();

    for c in vec[mem_type].chars() {
        if c.is_digit(10) {
            current_value.push(c);
        } else if !current_value.is_empty() {
            values.push(current_value.clone());
            current_value.clear();
        }
    }

    if !current_value.is_empty() {
        values.push(current_value);
    }

    let res = format!("{} / {} Mi", values[1], values[0]);
    res
}

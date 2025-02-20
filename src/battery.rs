use std::{collections::HashMap, process::Command};

pub fn battery_info() -> String {
    let memory_info_command = Command::new("upower")
        .arg("-d")
        .output()
        .expect("Failed to execute upower command");

    let binding = String::from_utf8_lossy(&memory_info_command.stdout);
    let vec: Vec<&str> = binding.lines().collect();
    let mut values_map: HashMap<&str, &str> = HashMap::new();

    for info in vec.iter() {
        if info.contains("model") {
            let details: Vec<&str> = info.split(":").collect();
            values_map.insert("model", details[1].trim());
        } else if info.contains("state") {
            let details: Vec<&str> = info.split(":").collect();
            values_map.insert("state", details[1].trim());
        } else if info.contains("percentage") {
            let details: Vec<&str> = info.split(":").collect();
            values_map.insert("percentage", details[1].trim());
        } else if info.contains("time") {
            let details: Vec<&str> = info.split(":").collect();
            values_map.insert("time", details[1].trim());
        }
    }

    let res = format!(
        "{} [{}] | Time to empty {} | {}",
        &values_map
            .get("percentage")
            .expect("Failed to get percentage value"),
        &values_map.get("state").expect("Failed to get state value"),
        &values_map.get("time").expect("Failed to get time value"),
        &values_map.get("model").expect("Failed to get model value")
    );
    res
}

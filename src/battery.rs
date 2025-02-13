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
            values_map.insert(details[0].trim(), details[1].trim());
        } else if info.contains("state") {
            let details: Vec<&str> = info.split(":").collect();
            values_map.insert(details[0].trim(), details[1].trim());
        } else if info.contains("percentage") {
            let details: Vec<&str> = info.split(":").collect();
            values_map.insert(details[0].trim(), details[1].trim());
        } else if info.contains("time") {
            let details: Vec<&str> = info.split(":").collect();
            values_map.insert(details[0].trim(), details[1].trim());
        }
    }

    for (key, val) in &values_map {
        println!("{}: {}", key, val);
    }

    let res = format!(
        "{} [{}] | Time to empty {} | {}",
        &values_map.get("percentage:").unwrap().to_string(),
        &values_map.get("state:").unwrap().to_string(),
        &values_map.get("tte:").unwrap().to_string(),
        &values_map.get("model:").unwrap().to_string()
    );
    res
}

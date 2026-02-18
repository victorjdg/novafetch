use std::process::Command;

pub fn battery_info() -> String {
    let output = Command::new("upower")
        .arg("-d")
        .output();
    
    let stdout = match output {
        Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
        Err(_) => return "N/A".to_string(),
    };

    let mut percentage = "N/A";
    let mut state = "N/A";
    let mut time = "N/A";
    let mut model = "Unknown";

    for line in stdout.lines() {
        let line = line.trim();
        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim();
            let value = value.trim();
            
            if key == "percentage" {
                percentage = value;
            } else if key == "state" {
                state = value;
            } else if key == "time to empty" || key == "time to full" {
                time = value;
            } else if key == "model" {
                model = value;
            }
        }
    }

    format!("{} [{}] | Time: {} | {}", percentage, state, time, model)
}

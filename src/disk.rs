use std::process::Command;

pub fn disk_info() -> String {
    let output = Command::new("df")
        .arg("-h")
        .arg("/")
        .output()
        .expect("Failed to execute df command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    
    if let Some(line) = lines.get(1) {
        let values: Vec<&str> = line.split_whitespace().collect();
        if values.len() >= 5 {
            let used = values[2];
            let total = values[1];
            let percent = values[4];
            return format!("{} / {} ({})", used, total, percent);
        }
    }
    
    "N/A".to_string()
}

use std::process::Command;

pub fn memory_info(tp: &str) -> String {
    let output = match Command::new("free").arg("-m").output() {
        Ok(out) => out,
        Err(_) => return "N/A".to_string(),
    };

    let line_idx = match tp {
        "memory" => 1,
        "swap" => 2,
        _ => return "Memory type not supported".to_string(),
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    
    let line = match lines.get(line_idx) {
        Some(line) => line,
        None => return "N/A".to_string(),
    };
    let values: Vec<u64> = line
        .split_whitespace()
        .skip(1)
        .filter_map(|s| s.parse().ok())
        .collect();
    
    if values.len() >= 2 {
        let used = values[1];
        let total = values[0];
        if tp == "swap" && total == 0 {
            "No swap".to_string()
        } else {
            let percentage = if total > 0 {
                (used * 100) / total
            } else {
                0
            };
            format!("{} / {} Mi ({}%)", used, total, percentage)
        }
    } else {
        "N/A".to_string()
    }
}

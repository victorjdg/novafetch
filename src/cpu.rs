use std::collections::HashMap;
use std::process::Command;

fn parse_cpu_info(info: String) -> HashMap<String, String> {
    let mut res_hs = HashMap::new();

    for line in info.lines() {
        if let Some((key, value)) = line.split_once(':') {
            res_hs.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    res_hs
}

fn pretty_cpu_info(cpu_info: HashMap<String, String>) -> String {
    let model_name = cpu_info.get("Model name").map(|s| s.as_str()).unwrap_or("Unknown");
    let core_count = cpu_info.get("CPU(s)").map(|s| s.as_str()).unwrap_or("?");
    let max_frequency = cpu_info.get("CPU max MHz").map(|s| s.as_str()).unwrap_or("?");
    
    format!("{} ({}) @ {} MHz", model_name, core_count, max_frequency)
}

pub fn cpu_info() -> String {
    match Command::new("lscpu").output() {
        Ok(output) => {
            let cpu_info = parse_cpu_info(String::from_utf8_lossy(&output.stdout).to_string());
            pretty_cpu_info(cpu_info)
        }
        Err(_) => "N/A".to_string(),
    }
}

use std::collections::HashMap;

fn parse_os_info(info: String) -> HashMap<String, String> {
    let mut res: HashMap<String, String> = HashMap::new();

    for line in info.lines() {
        if let Some((key, value)) = line.split_once('=') {
            let value_clean = value.trim_matches('"').to_string();
            res.insert(key.to_string(), value_clean);
        }
    }

    res
}

fn pretty_os_info(os_info: HashMap<String, String>) -> String {
    os_info.get("PRETTY_NAME")
        .cloned()
        .unwrap_or_else(|| "Unknown OS".to_string())
}

use std::fs;

pub fn os_info() -> String {
    match fs::read_to_string("/etc/os-release") {
        Ok(content) => {
            let os_info = parse_os_info(content);
            pretty_os_info(os_info)
        }
        Err(_) => "Unknown OS".to_string(),
    }
}

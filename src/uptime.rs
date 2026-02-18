use std::process::Command;

pub fn uptime_info() -> String {
    match Command::new("uptime").arg("-p").output() {
        Ok(output) => {
            let info = String::from_utf8_lossy(&output.stdout);
            info.strip_prefix("up ")
                .map(|s| s.trim())
                .unwrap_or(info.trim())
                .to_string()
        }
        Err(_) => "N/A".to_string(),
    }
}

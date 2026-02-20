use crate::error::FetchError;
use std::process::Command;

pub fn uptime_info() -> Result<String, FetchError> {
    let output = Command::new("uptime")
        .arg("-p")
        .output()
        .map_err(|e| FetchError::CommandFailed(format!("uptime: {}", e)))?;
    let info = String::from_utf8_lossy(&output.stdout);
    Ok(info
        .strip_prefix("up ")
        .map(|s| s.trim())
        .unwrap_or(info.trim())
        .to_string())
}

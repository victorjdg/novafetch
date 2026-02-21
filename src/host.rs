use crate::error::FetchError;
use std::process::Command;

pub fn host_info() -> Result<String, FetchError> {
    let output = Command::new("hostname")
        .output()
        .map_err(|e| FetchError::CommandFailed(format!("hostname: {}", e)))?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

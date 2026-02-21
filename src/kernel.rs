use crate::error::FetchError;
use std::process::Command;

pub fn kernel_info() -> Result<String, FetchError> {
    let output = Command::new("uname")
        .arg("-r")
        .output()
        .map_err(|e| FetchError::CommandFailed(format!("uname: {}", e)))?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

use crate::error::FetchError;
use std::process::Command;

pub fn gpu_info() -> Result<String, FetchError> {
    let output = Command::new("lspci")
        .args(["-d", "::0300", "-vmm"])
        .output()
        .map_err(|e| FetchError::CommandFailed(format!("lspci: {}", e)))?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        if line.starts_with("Device:") {
            if let Some(device) = line.split('\t').nth(1) {
                return Ok(device
                    .split('[')
                    .next()
                    .map(|s| s.trim().to_string())
                    .unwrap_or_else(|| device.to_string()));
            }
        }
    }

    gpu_info_fallback()
}

fn gpu_info_fallback() -> Result<String, FetchError> {
    let output = Command::new("lspci")
        .output()
        .map_err(|e| FetchError::CommandFailed(format!("lspci: {}", e)))?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        if line.contains("VGA") {
            if let Some(gpu) = line.split("VGA compatible controller:").nth(1) {
                return Ok(gpu
                    .split('(')
                    .next()
                    .map(|s| s.trim().to_string())
                    .unwrap_or_else(|| "Unknown GPU".to_string()));
            }
        }
    }

    Err(FetchError::NotFound)
}

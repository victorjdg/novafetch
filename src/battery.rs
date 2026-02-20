use crate::error::FetchError;
use std::process::Command;

pub fn battery_info() -> Result<String, FetchError> {
    let output = Command::new("upower")
        .arg("-d")
        .output()
        .map_err(|e| FetchError::CommandFailed(format!("upower: {}", e)))?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    let mut percentage = "N/A";
    let mut state = "N/A";
    let mut time = "N/A";
    let mut model = "Unknown";

    for line in stdout.lines() {
        let line = line.trim();
        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim();
            let value = value.trim();

            match key {
                "percentage" => percentage = value,
                "state" => state = value,
                "time to empty" | "time to full" => time = value,
                "model" => model = value,
                _ => {}
            }
        }
    }

    if percentage == "N/A" && state == "N/A" {
        return Err(FetchError::NotFound);
    }

    Ok(format!(
        "{} [{}] | Time: {} | {}",
        percentage, state, time, model
    ))
}

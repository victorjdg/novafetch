use crate::error::FetchError;
use std::process::Command;

pub fn disks_info() -> Result<String, FetchError> {
    let output = Command::new("df")
        .arg("-h")
        .output()
        .map_err(|e| FetchError::CommandFailed(format!("df: {}", e)))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut results = Vec::new();

    for (i, line) in stdout.lines().enumerate() {
        if i == 0 {
            continue;
        }

        let values: Vec<&str> = line.split_whitespace().collect();
        if values.len() >= 6 {
            let filesystem = values[0];
            let size = values[1];
            let used = values[2];
            let percent = values[4];
            let mountpoint = values[5];

            if filesystem.starts_with("/dev/") || filesystem.contains("mapper") {
                results.push(format!("{}: {} / {} ({})", mountpoint, used, size, percent));
            }
        }
    }

    if results.is_empty() {
        Err(FetchError::NotFound)
    } else {
        Ok(results.join(", "))
    }
}

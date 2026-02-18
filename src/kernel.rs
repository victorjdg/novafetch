use std::process::Command;

pub fn kernel_info() -> String {
    match Command::new("uname").arg("-r").output() {
        Ok(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),
        Err(_) => "N/A".to_string(),
    }
}

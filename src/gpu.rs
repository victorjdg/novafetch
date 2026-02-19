use std::process::Command;

pub fn gpu_info() -> String {
    let output = match Command::new("lspci")
        .args(&["-d", "::0300", "-vmm"])
        .output() 
    {
        Ok(out) => out,
        Err(_) => return "N/A".to_string(),
    };
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    for line in stdout.lines() {
        if line.starts_with("Device:") {
            if let Some(device) = line.split('\t').nth(1) {
                return device.split('[').next()
                    .map(|s| s.trim().to_string())
                    .unwrap_or_else(|| device.to_string());
            }
        }
    }
    
    gpu_info_fallback()
}

fn gpu_info_fallback() -> String {
    let output = match Command::new("lspci").output() {
        Ok(out) => out,
        Err(_) => return "N/A".to_string(),
    };
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    for line in stdout.lines() {
        if line.contains("VGA") {
            if let Some(gpu) = line.split("VGA compatible controller:").nth(1) {
                return gpu.split('(').next()
                    .map(|s| s.trim().to_string())
                    .unwrap_or_else(|| "Unknown GPU".to_string());
            }
        }
    }
    
    "Unknown GPU".to_string()
}

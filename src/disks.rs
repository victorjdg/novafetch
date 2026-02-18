use std::process::Command;

pub fn disks_info() -> String {
    let output = match Command::new("df").arg("-h").output() {
        Ok(out) => out,
        Err(_) => return "N/A".to_string(),
    };
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut results = Vec::new();
    
    for (i, line) in stdout.lines().enumerate() {
        if i == 0 {
            continue; // Saltar header
        }
        
        let values: Vec<&str> = line.split_whitespace().collect();
        if values.len() >= 6 {
            let filesystem = values[0];
            let size = values[1];
            let used = values[2];
            let _available = values[3];
            let percent = values[4];
            let mountpoint = values[5];
            
            // Filtrar sistemas virtuales y tmpfs
            if filesystem.starts_with("/dev/") || filesystem.contains("mapper") {
                // Mostrar formato: /mountpoint: used / size (percent)
                results.push(format!("{}: {} / {} ({})", mountpoint, used, size, percent));
            }
        }
    }
    
    if results.is_empty() {
        "N/A".to_string()
    } else {
        results.join(", ")
    }
}

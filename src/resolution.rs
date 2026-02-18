use std::process::Command;

pub fn resolution_info() -> String {
    // Intentar con xrandr (X11)
    if let Ok(output) = Command::new("xrandr").arg("--current").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Buscar líneas con '*' que indican la resolución activa
        for line in stdout.lines() {
            if line.contains('*') {
                // Formato típico: "   1920x1080     60.00*+  59.94"
                let parts: Vec<&str> = line.trim().split_whitespace().collect();
                if let Some(res) = parts.first() {
                    // Extraer la frecuencia (el número antes del *)
                    let freq = parts.get(1).map(|s| s.trim_matches('+').trim_matches('*'))
                        .unwrap_or("?");
                    return format!("{} @ {}Hz", res, freq);
                }
            }
        }
    }
    
    // Intentar con wlr-randr (Wayland - compositors wlroots)
    if let Ok(output) = Command::new("wlr-randr").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.contains("Enabled") && line.contains("px") {
                // Formato: "  Enabled: yes  1920x1080 @ 60Hz"
                if let Some(pos) = line.find("px") {
                    let start = line[..pos].rfind(' ').map(|i| i + 1).unwrap_or(0);
                    let res = &line[start..pos + 2];
                    if let Some(hz_pos) = line.find("Hz") {
                        let hz_start = line[..hz_pos].rfind(' ').map(|i| i + 1).unwrap_or(0);
                        let hz = &line[hz_start..hz_pos];
                        return format!("{} @ {}Hz", res.trim(), hz.trim());
                    }
                    return res.to_string();
                }
            }
        }
    }
    
    // Intentar con swaymsg (Sway)
    if let Ok(output) = Command::new("swaymsg").args(["-t", "get_outputs"]).output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Some(pos) = stdout.find("\"current_mode\"") {
            let sub = &stdout[pos..pos + 100.min(stdout.len() - pos)];
            if let Some(w) = sub.find("\"width\":") {
                if let Some(h) = sub.find("\"height\":") {
                    let width: i32 = sub[w + 8..sub[w + 8..].find(',').map(|i| i + w + 8).unwrap_or(sub.len())]
                        .trim().parse().unwrap_or(0);
                    let height: i32 = sub[h + 9..sub[h + 9..].find(',').map(|i| i + h + 9).unwrap_or(sub.len())]
                        .trim().parse().unwrap_or(0);
                    if width > 0 && height > 0 {
                        return format!("{}x{}", width, height);
                    }
                }
            }
        }
    }
    
    "N/A".to_string()
}

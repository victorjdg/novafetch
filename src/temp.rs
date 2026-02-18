use std::process::Command;

pub fn temp_info() -> String {
    // Intentar con sensors (lm-sensors) - método más común
    if let Ok(output) = Command::new("sensors").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Buscar temperaturas de CPU (Core, Tdie, Tctl, etc.)
        let mut temps: Vec<f64> = Vec::new();
        
        for line in stdout.lines() {
            let line = line.trim();
            
            // Buscar líneas como: "Core 0:        +45.0°C"
            if line.contains("Core") || line.contains("Tdie") || line.contains("Tctl") || 
               line.contains("Package id") || line.contains("Physical id") {
                if let Some(temp_str) = extract_temp(line) {
                    if let Ok(temp) = temp_str.parse::<f64>() {
                        temps.push(temp);
                    }
                }
            }
        }
        
        if !temps.is_empty() {
            // Calcular promedio
            let avg: f64 = temps.iter().sum::<f64>() / temps.len() as f64;
            let max = temps.iter().fold(f64::NAN, |m, &v| v.max(m));
            
            if temps.len() > 1 {
                return format!("{:.1}°C (avg) / {:.1}°C (max)", avg, max);
            } else {
                return format!("{:.1}°C", avg);
            }
        }
        
        // Si no encontramos cores específicos, buscar cualquier temperatura
        for line in stdout.lines() {
            if line.contains("°C") {
                if let Some(temp_str) = extract_temp(line) {
                    if let Ok(temp) = temp_str.parse::<f64>() {
                        return format!("{:.1}°C", temp);
                    }
                }
            }
        }
    }
    
    // Intentar leer de thermal zones (sysfs) - funciona sin lm-sensors
    for i in 0..10 {
        let path = format!("/sys/class/thermal/thermal_zone{}/temp", i);
        if let Ok(content) = std::fs::read_to_string(&path) {
            if let Ok(millidegrees) = content.trim().parse::<f64>() {
                let celsius = millidegrees / 1000.0;
                if celsius > 0.0 && celsius < 150.0 {  // Rango razonable
                    // Intentar obtener el tipo
                    let type_path = format!("/sys/class/thermal/thermal_zone{}/type", i);
                    if let Ok(zone_type) = std::fs::read_to_string(&type_path) {
                        let zone_type = zone_type.trim();
                        return format!("{:.1}°C ({})", celsius, zone_type);
                    }
                    return format!("{:.1}°C", celsius);
                }
            }
        }
    }
    
    // Intentar con /sys/class/hwmon (otra interfaz común)
    if let Ok(entries) = std::fs::read_dir("/sys/class/hwmon") {
        for entry in entries.flatten() {
            let hwmon_path = entry.path();
            let name_path = hwmon_path.join("name");
            
            if let Ok(name) = std::fs::read_to_string(&name_path) {
                let name = name.trim();
                // Buscar dispositivos de temperatura comunes
                if name.contains("coretemp") || name.contains("k10temp") || 
                   name.contains("zenpower") || name.contains("acpitz") {
                    // Buscar archivos temp*_input
                    if let Ok(files) = std::fs::read_dir(&hwmon_path) {
                        for file in files.flatten() {
                            let file_name = file.file_name().to_string_lossy().to_string();
                            if file_name.starts_with("temp") && file_name.ends_with("_input") {
                                if let Ok(content) = std::fs::read_to_string(file.path()) {
                                    if let Ok(millidegrees) = content.trim().parse::<f64>() {
                                        let celsius = millidegrees / 1000.0;
                                        if celsius > 0.0 && celsius < 150.0 {
                                            return format!("{:.1}°C", celsius);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    "N/A".to_string()
}

fn extract_temp(line: &str) -> Option<&str> {
    // Buscar patrón +XX.X°C o -XX.X°C
    if let Some(start) = line.find(['+', '-']) {
        let substr = &line[start..];
        if let Some(end) = substr.find('°') {
            let temp_str = &substr[..end];
            // Limpiar espacios
            let temp_str = temp_str.trim();
            return Some(temp_str);
        }
    }
    None
}

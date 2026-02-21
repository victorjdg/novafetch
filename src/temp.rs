use crate::error::FetchError;
use std::process::Command;

pub fn temp_info() -> Result<String, FetchError> {
    if let Ok(output) = Command::new("sensors").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);

        let mut temps: Vec<f64> = Vec::new();

        for line in stdout.lines() {
            let line = line.trim();

            if line.contains("Core")
                || line.contains("Tdie")
                || line.contains("Tctl")
                || line.contains("Package id")
                || line.contains("Physical id")
            {
                if let Some(temp_str) = extract_temp(line) {
                    if let Ok(temp) = temp_str.parse::<f64>() {
                        temps.push(temp);
                    }
                }
            }
        }

        if !temps.is_empty() {
            let avg: f64 = temps.iter().sum::<f64>() / temps.len() as f64;
            let max = temps.iter().fold(f64::NAN, |m, &v| v.max(m));

            if temps.len() > 1 {
                return Ok(format!("{:.1}°C (avg) / {:.1}°C (max)", avg, max));
            } else {
                return Ok(format!("{:.1}°C", avg));
            }
        }

        for line in stdout.lines() {
            if line.contains("°C") {
                if let Some(temp_str) = extract_temp(line) {
                    if let Ok(temp) = temp_str.parse::<f64>() {
                        return Ok(format!("{:.1}°C", temp));
                    }
                }
            }
        }
    }

    for i in 0..10 {
        let path = format!("/sys/class/thermal/thermal_zone{}/temp", i);
        if let Ok(content) = std::fs::read_to_string(&path) {
            if let Ok(millidegrees) = content.trim().parse::<f64>() {
                let celsius = millidegrees / 1000.0;
                if celsius > 0.0 && celsius < 150.0 {
                    let type_path = format!("/sys/class/thermal/thermal_zone{}/type", i);
                    if let Ok(zone_type) = std::fs::read_to_string(&type_path) {
                        let zone_type = zone_type.trim();
                        return Ok(format!("{:.1}°C ({})", celsius, zone_type));
                    }
                    return Ok(format!("{:.1}°C", celsius));
                }
            }
        }
    }

    if let Ok(entries) = std::fs::read_dir("/sys/class/hwmon") {
        for entry in entries.flatten() {
            let hwmon_path = entry.path();
            let name_path = hwmon_path.join("name");

            if let Ok(name) = std::fs::read_to_string(&name_path) {
                let name = name.trim();
                if name.contains("coretemp")
                    || name.contains("k10temp")
                    || name.contains("zenpower")
                    || name.contains("acpitz")
                {
                    if let Ok(files) = std::fs::read_dir(&hwmon_path) {
                        for file in files.flatten() {
                            let file_name = file.file_name().to_string_lossy().to_string();
                            if file_name.starts_with("temp") && file_name.ends_with("_input") {
                                if let Ok(content) = std::fs::read_to_string(file.path()) {
                                    if let Ok(millidegrees) = content.trim().parse::<f64>() {
                                        let celsius = millidegrees / 1000.0;
                                        if celsius > 0.0 && celsius < 150.0 {
                                            return Ok(format!("{:.1}°C", celsius));
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

    Err(FetchError::NotFound)
}

fn extract_temp(line: &str) -> Option<&str> {
    if let Some(start) = line.find(['+', '-']) {
        let substr = &line[start..];
        if let Some(end) = substr.find('°') {
            let temp_str = &substr[..end];
            let temp_str = temp_str.trim();
            return Some(temp_str);
        }
    }
    None
}

use crate::error::FetchError;
use crate::parser::parse_key_value;
use std::collections::HashMap;
use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CpuInfo {
    pub model: String,
    pub cores: String,
    pub max_frequency: String,
}

impl CpuInfo {
    pub fn from_map(info: &HashMap<String, String>) -> Result<Self, FetchError> {
        let model = info
            .get("Model name")
            .ok_or_else(|| FetchError::ParseFailed("Model name not found".to_string()))?
            .clone();
        let cores = info
            .get("CPU(s)")
            .ok_or_else(|| FetchError::ParseFailed("CPU count not found".to_string()))?
            .clone();
        let max_frequency = info
            .get("CPU max MHz")
            .ok_or_else(|| FetchError::ParseFailed("CPU max MHz not found".to_string()))?
            .clone();

        Ok(Self {
            model,
            cores,
            max_frequency,
        })
    }
}

impl std::fmt::Display for CpuInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}) @ {} MHz",
            self.model, self.cores, self.max_frequency
        )
    }
}

pub fn cpu_info() -> Result<CpuInfo, FetchError> {
    let output = Command::new("lscpu")
        .output()
        .map_err(|e| FetchError::CommandFailed(format!("lscpu: {}", e)))?;

    let cpu_map = parse_key_value(&String::from_utf8_lossy(&output.stdout), ':', false);
    CpuInfo::from_map(&cpu_map)
}

pub fn format_cpu_info() -> Result<String, FetchError> {
    let info = cpu_info()?;
    Ok(info.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_info_display() {
        let cpu = CpuInfo {
            model: "Intel i7".to_string(),
            cores: "8".to_string(),
            max_frequency: "4800".to_string(),
        };
        assert_eq!(cpu.to_string(), "Intel i7 (8) @ 4800 MHz");
    }

    #[test]
    fn test_cpu_info_from_map_success() {
        let mut map = HashMap::new();
        map.insert("Model name".to_string(), "AMD Ryzen 9".to_string());
        map.insert("CPU(s)".to_string(), "16".to_string());
        map.insert("CPU max MHz".to_string(), "5000".to_string());

        let cpu = CpuInfo::from_map(&map).unwrap();
        assert_eq!(cpu.model, "AMD Ryzen 9");
        assert_eq!(cpu.cores, "16");
        assert_eq!(cpu.max_frequency, "5000");
    }

    #[test]
    fn test_cpu_info_from_map_missing_model() {
        let mut map = HashMap::new();
        map.insert("CPU(s)".to_string(), "16".to_string());
        map.insert("CPU max MHz".to_string(), "5000".to_string());

        let result = CpuInfo::from_map(&map);
        assert!(result.is_err());
    }

    #[test]
    fn test_cpu_info_from_map_missing_cores() {
        let mut map = HashMap::new();
        map.insert("Model name".to_string(), "AMD Ryzen 9".to_string());
        map.insert("CPU max MHz".to_string(), "5000".to_string());

        let result = CpuInfo::from_map(&map);
        assert!(result.is_err());
    }

    #[test]
    fn test_cpu_info_from_map_missing_frequency() {
        let mut map = HashMap::new();
        map.insert("Model name".to_string(), "AMD Ryzen 9".to_string());
        map.insert("CPU(s)".to_string(), "16".to_string());

        let result = CpuInfo::from_map(&map);
        assert!(result.is_err());
    }
}

use crate::error::FetchError;
use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiskInfo {
    pub used: String,
    pub total: String,
    pub percentage: String,
}

impl DiskInfo {
    pub fn new(used: impl Into<String>, total: impl Into<String>, percentage: impl Into<String>) -> Self {
        Self {
            used: used.into(),
            total: total.into(),
            percentage: percentage.into(),
        }
    }
}

impl std::fmt::Display for DiskInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} / {} ({})", self.used, self.total, self.percentage)
    }
}

pub fn disk_info() -> Result<DiskInfo, FetchError> {
    let output = Command::new("df")
        .arg("-h")
        .arg("/")
        .output()
        .map_err(|e| FetchError::CommandFailed(format!("df: {}", e)))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    let line = lines
        .get(1)
        .ok_or_else(|| FetchError::ParseFailed("No disk info found".to_string()))?;

    let values: Vec<&str> = line.split_whitespace().collect();
    if values.len() >= 5 {
        Ok(DiskInfo::new(values[2], values[1], values[4]))
    } else {
        Err(FetchError::ParseFailed("Failed to parse disk info".to_string()))
    }
}

pub fn format_disk_info() -> Result<String, FetchError> {
    let info = disk_info()?;
    Ok(info.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disk_info_new() {
        let info = DiskInfo::new("65G", "417G", "17%");
        assert_eq!(info.used, "65G");
        assert_eq!(info.total, "417G");
        assert_eq!(info.percentage, "17%");
    }

    #[test]
    fn test_disk_info_display() {
        let info = DiskInfo::new("65G", "417G", "17%");
        assert_eq!(info.to_string(), "65G / 417G (17%)");
    }

    #[test]
    fn test_disk_info_from_strings() {
        let used = String::from("100G");
        let total = String::from("500G");
        let percentage = String::from("20%");
        
        let info = DiskInfo::new(used, total, percentage);
        assert_eq!(info.to_string(), "100G / 500G (20%)");
    }
}

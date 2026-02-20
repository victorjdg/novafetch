use crate::error::FetchError;
use std::process::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryType {
    Memory,
    Swap,
}

impl MemoryType {
    fn line_idx(self) -> usize {
        match self {
            MemoryType::Memory => 1,
            MemoryType::Swap => 2,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryInfo {
    pub used: u64,
    pub total: u64,
    pub percentage: u8,
}

impl MemoryInfo {
    pub fn new(used: u64, total: u64) -> Self {
        let percentage = if total > 0 {
            ((used * 100) / total) as u8
        } else {
            0
        };
        Self {
            used,
            total,
            percentage,
        }
    }

    pub fn is_empty_swap(&self) -> bool {
        self.total == 0
    }
}

impl std::fmt::Display for MemoryInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} / {} Mi ({}%)",
            self.used, self.total, self.percentage
        )
    }
}

pub fn memory_info(memory_type: MemoryType) -> Result<MemoryInfo, FetchError> {
    let output = Command::new("free")
        .arg("-m")
        .output()
        .map_err(|e| FetchError::CommandFailed(format!("free: {}", e)))?;

    let line_idx = memory_type.line_idx();

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    let line = lines
        .get(line_idx)
        .ok_or_else(|| FetchError::ParseFailed("Line not found in free output".to_string()))?;

    let values: Vec<u64> = line
        .split_whitespace()
        .skip(1)
        .filter_map(|s| s.parse().ok())
        .collect();

    if values.len() >= 2 {
        let total = values[0];
        let used = values[1];
        Ok(MemoryInfo::new(used, total))
    } else {
        Err(FetchError::ParseFailed("Failed to parse memory values".to_string()))
    }
}

pub fn format_memory_info(memory_type: MemoryType) -> Result<String, FetchError> {
    let info = memory_info(memory_type)?;

    if memory_type == MemoryType::Swap && info.is_empty_swap() {
        Ok("No swap".to_string())
    } else {
        Ok(info.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_info_new() {
        let info = MemoryInfo::new(4096, 16384);
        assert_eq!(info.used, 4096);
        assert_eq!(info.total, 16384);
        assert_eq!(info.percentage, 25);
    }

    #[test]
    fn test_memory_info_percentage_zero() {
        let info = MemoryInfo::new(0, 0);
        assert_eq!(info.percentage, 0);
    }

    #[test]
    fn test_memory_info_percentage_full() {
        let info = MemoryInfo::new(8192, 8192);
        assert_eq!(info.percentage, 100);
    }

    #[test]
    fn test_memory_info_display() {
        let info = MemoryInfo::new(4096, 16384);
        assert_eq!(info.to_string(), "4096 / 16384 Mi (25%)");
    }

    #[test]
    fn test_memory_info_is_empty_swap_true() {
        let info = MemoryInfo::new(0, 0);
        assert!(info.is_empty_swap());
    }

    #[test]
    fn test_memory_info_is_empty_swap_false() {
        let info = MemoryInfo::new(1024, 2048);
        assert!(!info.is_empty_swap());
    }

    #[test]
    fn test_memory_type_line_idx() {
        assert_eq!(MemoryType::Memory.line_idx(), 1);
        assert_eq!(MemoryType::Swap.line_idx(), 2);
    }
}

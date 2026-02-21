use crate::error::FetchError;
use crate::parser::parse_key_value;
use std::fs;

fn pretty_os_info(os_info: std::collections::HashMap<String, String>) -> Result<String, FetchError> {
    os_info
        .get("PRETTY_NAME")
        .cloned()
        .ok_or_else(|| FetchError::ParseFailed("PRETTY_NAME not found".to_string()))
}

pub fn os_info() -> Result<String, FetchError> {
    let content = fs::read_to_string("/etc/os-release")?;
    let os_info = parse_key_value(&content, '=', true);
    pretty_os_info(os_info)
}

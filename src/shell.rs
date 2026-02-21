use crate::error::FetchError;
use std::env;

pub fn shell_info() -> Result<String, FetchError> {
    let shell = env::var("SHELL")
        .map_err(|e| FetchError::IoError(std::io::Error::new(std::io::ErrorKind::NotFound, e)))?;
    Ok(shell
        .split('/')
        .last()
        .map(|s| s.to_string())
        .unwrap_or_else(|| shell))
}

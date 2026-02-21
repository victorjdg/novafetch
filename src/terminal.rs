use crate::error::FetchError;
use std::env;

pub fn terminal_info() -> Result<String, FetchError> {
    env::var("TERM")
        .map_err(|e| FetchError::IoError(std::io::Error::new(std::io::ErrorKind::NotFound, e)))
}

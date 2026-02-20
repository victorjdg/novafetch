use std::fmt;
use std::io;

#[derive(Debug)]
pub enum FetchError {
    CommandFailed(String),
    IoError(io::Error),
    ParseFailed(String),
    NotFound,
    NotSupported(String),
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FetchError::CommandFailed(cmd) => write!(f, "Command failed: {}", cmd),
            FetchError::IoError(e) => write!(f, "IO error: {}", e),
            FetchError::ParseFailed(msg) => write!(f, "Parse failed: {}", msg),
            FetchError::NotFound => write!(f, "Not found"),
            FetchError::NotSupported(msg) => write!(f, "Not supported: {}", msg),
        }
    }
}

impl From<io::Error> for FetchError {
    fn from(e: io::Error) -> Self {
        FetchError::IoError(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_error_command_failed() {
        let err = FetchError::CommandFailed("lscpu not found".to_string());
        assert_eq!(err.to_string(), "Command failed: lscpu not found");
    }

    #[test]
    fn test_fetch_error_parse_failed() {
        let err = FetchError::ParseFailed("invalid format".to_string());
        assert_eq!(err.to_string(), "Parse failed: invalid format");
    }

    #[test]
    fn test_fetch_error_not_found() {
        let err = FetchError::NotFound;
        assert_eq!(err.to_string(), "Not found");
    }

    #[test]
    fn test_fetch_error_not_supported() {
        let err = FetchError::NotSupported("Windows".to_string());
        assert_eq!(err.to_string(), "Not supported: Windows");
    }

    #[test]
    fn test_fetch_error_from_io_error() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let fetch_err: FetchError = io_err.into();
        match fetch_err {
            FetchError::IoError(_) => (),
            _ => panic!("Expected IoError variant"),
        }
    }
}

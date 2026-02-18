use std::env;

pub fn terminal_info() -> String {
    env::var("TERM").unwrap_or_else(|_| "unknown".to_string())
}

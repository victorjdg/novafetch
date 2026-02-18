use std::env;

pub fn shell_info() -> String {
    env::var("SHELL")
        .unwrap_or_else(|_| "/bin/sh".to_string())
        .split('/')
        .last()
        .unwrap_or("sh")
        .to_string()
}

use std::process::Command;

pub fn packages_info(os: String) -> String {
    let (command, args) = match os {
        _ if os.contains("Fedora") => ("dnf", vec!["list", "--installed"]),
        _ if os.contains("Debian") => ("apt", vec!["list", "--installed"]),
        _ => return "OS not supported".to_string(),
    };

    let package_info_command = Command::new(command)
        .args(&args)
        .output()
        .expect("Failed to execute package manager command");

    let res = String::from_utf8_lossy(&package_info_command.stdout)
        .lines()
        .count()
        .to_string();

    res
}

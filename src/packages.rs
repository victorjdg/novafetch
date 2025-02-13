use std::process::Command;

pub fn packages_info(os: String) -> String {
    let mut res = "SO not supported".to_string();

    if os.contains("Fedora") {
        let package_info_command = Command::new("dnf")
            .arg("list")
            .arg("--installed")
            .output()
            .expect("Failed to execute uptime command");

        res = String::from_utf8_lossy(&package_info_command.stdout)
            .lines()
            .count()
            .to_string();
    } else if os.contains("Ubuntu") {
        let package_info_command = Command::new("apt")
            .arg("list")
            .arg("--installed")
            .output()
            .expect("Failed to execute uptime command");

        res = String::from_utf8_lossy(&package_info_command.stdout)
            .lines()
            .count()
            .to_string();
    }
    res
}

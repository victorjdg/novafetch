use std::process::Command;

fn pretty_kernel_info(info: String) -> String {
    let kernel_info = &info;
    let res = format!("{}", kernel_info.trim_end());

    res
}

pub fn kernel_info() -> String {
    let kernel_info_command = Command::new("uname")
        .arg("-r")
        .output()
        .expect("Failed to execute uname command");
    let kernel_info =
        pretty_kernel_info(String::from_utf8_lossy(&kernel_info_command.stdout).to_string());

    kernel_info
}

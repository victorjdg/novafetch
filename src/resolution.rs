use std::process::{Command};

fn parse_res_info(info: String) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();

    let a_info: Vec<&str> = info.split(",").collect();
    res.push(a_info[0].to_string());
    res.push(a_info[1].to_string().replace("\n", ""));

    res
}

fn pretty_res_info(info: Vec<String>) -> String {
    let resolution_info = &info;
    let res = format!("{}x{}", resolution_info[0], resolution_info[1]);

    res
}

pub fn resolution_info() -> String {
    let session_info_command = Command::new("cat")
        .arg("/sys/class/graphics/fb0/virtual_size")
        .output()
        .expect("Failed to find /sys/class/graphics/fb0/virtual_size file");
    let session_info = parse_res_info(String::from_utf8_lossy(&session_info_command.stdout).to_string());
    let res = pretty_res_info(session_info);
    res
}

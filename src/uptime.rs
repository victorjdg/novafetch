use std::process::Command;

fn parse_uptime_info(info: String) -> String {
    let upt_info: Vec<&str> = info.split("up ").collect();
    let res = upt_info[1].to_string();

    res
}

fn pretty_uptime_info(info: String) -> String {
    let uptime_info = &info;
    let res = format!("{}", uptime_info.trim_end());

    res
}

pub fn uptime_info() -> String {
    let uptime_info_command = Command::new("uptime")
        .arg("-p")
        .output()
        .expect("Failed to execute uptime command");
    let uptime_info =
        parse_uptime_info(String::from_utf8_lossy(&uptime_info_command.stdout).to_string());
    let res = pretty_uptime_info(uptime_info);

    res
}

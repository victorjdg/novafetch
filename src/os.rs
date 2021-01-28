use std::process::Command;

fn parse_os_info(info: String) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();

    let info_vec: Vec<&str> = info.lines().collect();

    // OS name
    let long_os_name = info_vec[0].to_string();
    let os_name: Vec<&str> = long_os_name.split("\"").collect();
    res.push(os_name[1].to_string());

    res
}

fn pretty_os_info(vec_info: Vec<String>) -> String {
    let os_name = &vec_info[0];
    let res = format!("{}", os_name);

    res
}

pub fn os_info() -> String {
    let os_info_command = Command::new("cat").arg("/etc/os-release").output().expect("Failed to execute cat command");
    let os_info = parse_os_info(String::from_utf8_lossy(&os_info_command.stdout).to_string());
    let res = pretty_os_info(os_info);

    res
}

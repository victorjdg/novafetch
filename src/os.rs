use std::collections::HashMap;
use std::process::Command;

fn parse_os_info(info: String) -> HashMap<String, String> {
    let mut res: HashMap<String, String> = HashMap::new();

    let info_vec: Vec<&str> = info.lines().collect();
    for info in &info_vec {
        let pair: Vec<&str> = info.split("=").collect();
        res.insert(pair[0].to_string(), pair[1].to_string());
    }

    res
}

fn pretty_os_info(os_info: HashMap<String, String>) -> String {
    let mut res = "";
    match os_info.get("PRETTY_NAME") {
        Some(pretty_name) => res = &pretty_name[1..pretty_name.len() - 1],
        _ => println!("Unable to get OS name"),
    }

    res.to_string()
}

pub fn os_info() -> String {
    let os_info_command = Command::new("cat")
        .arg("/etc/os-release")
        .output()
        .expect("Failed to execute cat command");
    let os_info = parse_os_info(String::from_utf8_lossy(&os_info_command.stdout).to_string());
    let res = pretty_os_info(os_info);

    res
}

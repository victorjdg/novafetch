use std::process::Command;
use std::collections::HashMap;

fn parse_cpu_info(info: String) -> HashMap<String, String> {
    let mut res_hs = HashMap::new();

    let info_vec: Vec<&str> = info.lines().collect();

    // Split lines by ":" and store data in hashmap
    for line in &info_vec{
        let mut split: Vec<&str> = line.split(":").collect();
        res_hs.insert(
            split[0].to_string(),
            split[1].to_string(),
        );
    }

    res_hs
}

fn pretty_cpu_info(cpu_info: HashMap<String, String>) -> String {
    let model_name: Vec<&str> = cpu_info["Model name"].split("                      ").collect();
    let core_count: Vec<&str> = cpu_info["CPU(s)"].split("                          ").collect();
    let max_frequency: Vec<&str> = cpu_info["CPU max MHz"].split("                     ").collect();
    let res = format!("{} ({}) @ {} MHz", model_name[1], core_count[1], max_frequency[1]);

    res
}

pub fn cpu_info() -> String {
    let cpu_info_command = Command::new("lscpu").output().expect("Failed to execute lscpu command");
    let cpu_info = parse_cpu_info(String::from_utf8_lossy(&cpu_info_command.stdout).to_string());
    let res = pretty_cpu_info(cpu_info);
    res
}

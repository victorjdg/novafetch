use std::process::Command;

fn parse_cpu_info(info: String) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();

    let info_vec: Vec<&str> = info.lines().collect();
    
    // CPU core count
    let cpu_long_core_count = info_vec[4].to_string();
    let cpu_core_count: Vec<&str> = cpu_long_core_count.split_whitespace().collect();
    res.push(cpu_core_count[cpu_core_count.len() - 1].to_string());
    // CPU name
    let cpu_long_name = info_vec[13].to_string();
    let cpu_name: Vec<&str> = cpu_long_name.split("  ").collect();
    let name_check = cpu_name[cpu_name.len() - 1].trim_start().to_string();
    let vendor_check: Vec<&str> = name_check.split("@").collect();
    if vendor_check.len() > 1 {
        // Intel
        res.push(vendor_check[0].trim_end().to_string());
        // CPU max freq
        let cpu_long_max_freq = info_vec[16].to_string();
        let cpu_max_freq_str: Vec<&str> = cpu_long_max_freq.split_whitespace().collect();
        let cpu_max_freq_f: f32 = cpu_max_freq_str[cpu_max_freq_str.len() - 1]
            .to_string()
            .replace(",", ".")
            .parse().unwrap();
        res.push((cpu_max_freq_f / 1000 as f32).to_string());

    } else {
        // AMD
        res.push(vendor_check[0].to_string());
        // CPU max freq
        let cpu_long_max_freq = info_vec[17].to_string();
        let cpu_max_freq_str: Vec<&str> = cpu_long_max_freq.split_whitespace().collect();
        let cpu_max_freq_f: f32 = cpu_max_freq_str[cpu_max_freq_str.len() - 1]
            .to_string()
            .replace(",", ".")
            .parse().unwrap();
        res.push((cpu_max_freq_f / 1000 as f32).to_string());
    }

    res
}

fn pretty_cpu_info(vec_info: Vec<String>) -> String {
    let model_name = &vec_info[1];
    let core_count = &vec_info [0];
    let max_frequency = &vec_info[2];
    let res = format!("{} ({}) @ {} GHz", model_name, core_count, max_frequency);

    res
}

pub fn cpu_info() -> String {
    let cpu_info_command = Command::new("lscpu").output().expect("Failed to execute lscpu command");
    let cpu_info = parse_cpu_info(String::from_utf8_lossy(&cpu_info_command.stdout).to_string());
    let res = pretty_cpu_info(cpu_info);
    res
}

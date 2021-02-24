use std::process::Command;

fn parse_memory_info(info: String) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();

    let info_vec: Vec<&str> = info.lines().collect();
    let m_info: Vec<&str> = info_vec[1].split_whitespace().collect();

    res.push(m_info[1].to_string());
    res.push(m_info[6].to_string());
    
    res
}

fn pretty_memory_info(mem_info: Vec<String>) -> String {
    let total_mem = mem_info[0].parse::<i32>().unwrap();
    let avaliable_mem = mem_info[1].parse::<i32>().unwrap();

    let used_mem = total_mem - avaliable_mem;

    let res = format!("{}MiB / {}MiB", used_mem, total_mem);
    
    res
}

pub fn memory_info() -> String {
    let memory_info_command = Command::new("free").arg("-m").output().expect("Failed to execute free command");

    let memory_info = parse_memory_info(String::from_utf8_lossy(&memory_info_command.stdout).to_string());
    
    let res = pretty_memory_info(memory_info);

    res
}

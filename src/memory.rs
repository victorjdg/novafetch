use std::process::Command;
use std::collections::HashMap;

fn parse_memory_info(info: String) -> HashMap<String, String> {
    let mut res = HashMap::new();

    let info_vec: Vec<&str> = info.lines().collect();
    
    // Split lines by ":" and store data in hashmap
    for line in &info_vec{
        let split: Vec<&str> = line.split(":").collect();
        res.insert(
            split[0].to_string(),
            split[1].to_string(),
        );
    }
    res
}

fn pretty_memory_info(mem_info: HashMap<String, String>) -> String {
    let total_mem: Vec<&str> = mem_info["MemTotal"].split("kB").collect();
    let available_mem: Vec<&str> = mem_info["MemAvailable"].split("kB").collect();
    
    let total_mem_kib = total_mem[0].replace(" ", "").parse::<i32>().unwrap();
    let available_mem_kib = available_mem[0].replace(" ", "").parse::<i32>().unwrap();

    let total_mem_mib = total_mem_kib/1024;
    let available_mem_mib = available_mem_kib/1024;

    let mem_used = total_mem_mib - available_mem_mib;

    let res = format!("{} MiB / {} MiB", mem_used, total_mem_mib);
    
    res
}

pub fn memory_info() -> String {
    let memory_info_command = Command::new("cat").arg("/proc/meminfo").output()
        .expect("Failed to read /proc/meminfo file");

    let memory_info = parse_memory_info(String::from_utf8_lossy(&memory_info_command.stdout).to_string());
    
    let res = pretty_memory_info(memory_info);

    res
}

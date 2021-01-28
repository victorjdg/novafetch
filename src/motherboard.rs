use std::process::Command;

fn pretty_motherboard_info(info: String) -> String {
    let motherboard_name = info;
    let res = format!("{}", motherboard_name);

    res
}

pub fn motherboard_info() -> String {
    let motherboard_info_command = Command::new("cat").arg("/sys/devices/virtual/dmi/id/board_name")
        .output()
        .expect("Failed to execute cat command");
    let res;
    let motherboard_name = String::from_utf8_lossy(&motherboard_info_command.stdout).trim_end().to_string();
    let test_output: Vec<&str> = motherboard_name.split_whitespace().collect();
    if test_output.len() > 1 {
        res = pretty_motherboard_info(motherboard_name); 
    } else {
        let motherboard_info_command = Command::new("cat").arg("/sys/devices/virtual/dmi/id/chassis_version")
            .output()
            .expect("Failed to execute cat command");
        let motherboard_name = String::from_utf8_lossy(&motherboard_info_command.stdout).trim_end().to_string();
        let test_output: Vec<&str> = motherboard_name.split_whitespace().collect();
        if test_output.len() > 1 {
            res = pretty_motherboard_info(motherboard_name);
        } else {
            let motherboard_info_command = Command::new("cat").arg("/sys/devices/virtual/dmi/id/product_name")
                .output()
                .expect("Failed to execute cat command");
            let motherboard_name = String::from_utf8_lossy(&motherboard_info_command.stdout).trim_end().to_string();
            res = pretty_motherboard_info(motherboard_name);
        }
    }

    res
}

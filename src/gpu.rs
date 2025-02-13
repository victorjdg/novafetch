use std::io::{Read, Write};
use std::process::{Command, Stdio};

fn parse_gpu_info(info: String) -> String {
    let p_info: Vec<&str> = info.split("VGA compatible controller: ").collect();
    let gpu_info: Vec<&str> = p_info[1].split("(").collect();
    let res = gpu_info[0].to_string();

    res
}

fn pretty_gpu_info(info: String) -> String {
    let gpu_info = &info;
    let res = format!("{}", gpu_info.trim_end());

    res
}

pub fn gpu_info() -> String {
    let mut gpu_info_command_lspci = Command::new("lspci")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let mut gpu_info_command_grep = Command::new("grep")
        .arg("VGA")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    if let Some(ref mut stdout) = gpu_info_command_lspci.stdout {
        if let Some(ref mut stdin) = gpu_info_command_grep.stdin {
            let mut buf: Vec<u8> = Vec::new();
            stdout.read_to_end(&mut buf).unwrap();
            stdin.write_all(&buf).unwrap();
        }
    }
    let gpu_info = parse_gpu_info(
        String::from_utf8_lossy(&gpu_info_command_grep.wait_with_output().unwrap().stdout)
            .to_string(),
    );
    let res = pretty_gpu_info(gpu_info);

    res
}

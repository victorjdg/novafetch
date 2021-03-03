use std::process::{Command, Stdio};
use std::io::{Read, Write};

fn parse_res_info(info: String) -> String {
    let a_info: Vec<&str> = info.split(":").collect();
    let res_info: Vec<&str> = a_info[1].split("pixels").collect();
    let res = res_info[0].to_string();

    res
}

fn pretty_res_info(info: String) -> String {
    let resolution_info = &info;
    let res = format!("{}", resolution_info.trim_start());

    res
}

pub fn resolution_info() -> String {
    let mut res = "".to_string();
    
    let session_info_command = Command::new("loginctl")
        .output()
        .expect("Failed to execute loginctl command");
    let session_info = String::from_utf8_lossy(&session_info_command.stdout);
    let session_info_vec: Vec<&str> = session_info.split_whitespace().collect();
    let session = session_info_vec[5];
    
    let display_server_command = Command::new("loginctl")
        .arg("show-session")
        .arg(session)
        .arg("-p")
        .arg("Type")
        .output()
        .expect("Failed to execute loginctl command");
    let display_server_info = String::from_utf8_lossy(&display_server_command.stdout);
    let display_server_vec: Vec<&str> = display_server_info.split("=").collect();
    let display_server = display_server_vec[1].to_string();
    
    if display_server.trim_end() == "x11" {
        let mut res_info_command_xdpyinfo = Command::new("xdpyinfo")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

        let mut res_info_command_grep = Command::new("grep")
            .arg("dimensions")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        if let Some(ref mut stdout) = res_info_command_xdpyinfo.stdout {
            if let Some(ref mut stdin) = res_info_command_grep.stdin {
                let mut buf: Vec<u8> = Vec::new();
                stdout.read_to_end(&mut buf).unwrap();
                stdin.write_all(&buf).unwrap();
            }
        }

        let res_info = parse_res_info(String::from_utf8_lossy(&res_info_command_grep
                                                          .wait_with_output()
                                                          .unwrap()
                                                          .stdout)
                                      .to_string());
        res = pretty_res_info(res_info);

    } else if display_server == "wayland" {
        let mut res_info_command_xdpyinfo = Command::new("xdpyinfo")
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let mut res_info_command_grep = Command::new("grep")
            .arg("dimensions")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        if let Some(ref mut stdout) = res_info_command_xdpyinfo.stdout {
            if let Some(ref mut stdin) = res_info_command_grep.stdin {
                let mut buf: Vec<u8> = Vec::new();
                stdout.read_to_end(&mut buf).unwrap();
                stdin.write_all(&buf).unwrap();
            }
        }

        let res_info = parse_res_info(String::from_utf8_lossy(&res_info_command_grep
                                                          .wait_with_output()
                                                          .unwrap()
                                                          .stdout)
                                      .to_string());
        res = pretty_res_info(res_info);

    } else {
        res = "Display server not supported".to_string();
    }
    res
}

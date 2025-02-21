mod battery;
mod cpu;
mod disk;
mod gpu;
mod host;
mod kernel;
mod memory;
mod os;
mod packages;
mod shell;
mod terminal;
mod uptime;

use battery::battery_info;
use cpu::cpu_info;
use disk::disk_info;
use gpu::gpu_info;
use host::host_info;
use kernel::kernel_info;
use memory::memory_info;
use os::os_info;
use packages::packages_info;
use shell::shell_info;
use std::env;
use terminal::terminal_info;
use uptime::uptime_info;

fn main() {
    let options: Vec<String> = env::args().collect();
    if options.len() > 1 {
        let valid_options = options[1..options.len()].to_vec();
        for option in valid_options {
            match option.as_str() {
                "-o" | "-O" | "-os" | "-OS" => println!("OS: {}", os_info()),
                "-h" | "-H" | "-host" | "-HOST" => println!("Host: {}", host_info()),
                "-k" | "-K" | "-kernel" | "-KERNEL" => println!("Kernel: {}", kernel_info()),
                "-u" | "-U" | "-uptime" | "-UPTIME" => println!("Uptime: {}", uptime_info()),
                "-p" | "-P" | "-packages" | "-PACKAGES" => {
                    println!("Packages: {}", packages_info(os_info()))
                }
                "-s" | "-S" | "-shell" | "-SHELL" => println!("Shell: {}", shell_info()),
                "-t" | "-T" | "-terminal" | "-TERMINAL" => {
                    println!("Terminal: {}", terminal_info())
                }
                "-c" | "-C" | "-cpu" | "-CPU" => println!("CPU: {}", cpu_info()),
                "-g" | "-G" | "-gpu" | "-GPU" => println!("GPU: {}", gpu_info()),
                "-m" | "-M" | "-memory" | "-MEMORY" => {
                    println!("Memory: {}", memory_info("memory"))
                }
                "-ms" | "-MS" | "-swap" | "-SWAP" => println!("Swap: {}", memory_info("swap")),
                "-d" | "-D" | "-disk" | "-DISK" => println!("Disk (/): {}", disk_info()),
                "-b" | "-B" | "-battery" | "-BATTERY" => println!("Battery: {}", battery_info()),
                _ => println!("Invalid option {}", option),
            }
        }
    } else {
        println!("OS: {}", os_info());
        println!("Host: {}", host_info());
        println!("Kernel: {}", kernel_info());
        println!("Uptime: {}", uptime_info());
        println!("Packages: {}", packages_info(os_info()));
        println!("Shell: {}", shell_info());
        println!("Terminal: {}", terminal_info());
        println!("CPU: {}", cpu_info());
        println!("GPU: {}", gpu_info());
        println!("Memory: {}", memory_info("memory"));
        println!("Swap: {}", memory_info("swap"));
        println!("Disk (/): {}", disk_info());
        println!("Battery: {}", battery_info());
    }
}

mod cpu;
mod os;
mod motherboard;
mod kernel;
mod uptime;
mod gpu;
mod memory;

use std::env;
use cpu::cpu_info;
use os::os_info;
use motherboard::motherboard_info;
use kernel::kernel_info;
use uptime::uptime_info;
use gpu::gpu_info;
use memory::memory_info;

fn main() {
    let options: Vec<String> = env::args().collect();
    if options.len() > 1 {
        let valid_options = options[1 .. options.len()].to_vec();
        for option in valid_options {
            match option.as_str() {
                "-o" | "-O" | "-os" | "-OS" => println!("OS: {}", os_info()),
                "-h" | "-H" | "-host" | "-HOST" => println!("Host: {}", motherboard_info()),
                "-k" | "-K" | "-kernel" | "-KERNEL" => println!("Kernel: {}", kernel_info()),
                "-u" | "-U" | "-uptime" | "-UPTIME" => println!("Uptime: {}", uptime_info()),
                "-c" | "-C" | "-cpu" | "-CPU" => println!("CPU: {}", cpu_info()),
                "-m" | "-M" | "-memory" | "-MEMORY" => println!("Memory: {}", memory_info()),
                "-g" | "-G" | "-gpu" | "-GPU" => println!("GPU: {}", gpu_info()),
                _ => println!("Invalid option {}", option),
            }
        }
    } else {
        println!("OS: {}", os_info());
        println!("Host: {}", motherboard_info());
        println!("Kernel: {}", kernel_info());
        println!("Uptime: {}", uptime_info());
        println!("CPU: {}", cpu_info());
        println!("Memory: {}", memory_info());
        println!("GPU: {}", gpu_info());
    }
}

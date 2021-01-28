mod cpu;
mod os;
mod motherboard;
mod kernel;

use cpu::cpu_info;
use os::os_info;
use motherboard::motherboard_info;
use kernel::kernel_info;

fn main() {
    println!("OS: {}", os_info());
    println!("Host: {}", motherboard_info());
    println!("Kernel: {}", kernel_info());
    println!("CPU: {}", cpu_info());
}

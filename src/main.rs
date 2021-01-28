mod cpu;
mod os;
mod motherboard;

use cpu::cpu_info;
use os::os_info;
use motherboard::motherboard_info;

fn main() {
    println!("OS: {}", os_info());
    println!("Host: {}", motherboard_info());
    println!("CPU: {}", cpu_info());
}

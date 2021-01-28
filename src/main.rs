mod cpu;
mod os;

use cpu::cpu_info;
use os::os_info;

fn main() {
    println!("OS: {}", os_info());
    println!("CPU: {}", cpu_info());
}

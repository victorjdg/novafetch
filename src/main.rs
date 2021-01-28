mod cpu;
use cpu::cpu_info;

fn main() {
    println!("{}", cpu_info());
}

mod battery;
mod cpu;
mod desktop;
mod disk;
mod disks;
mod gpu;
mod host;
mod kernel;
mod memory;
mod os;
mod packages;
mod resolution;
mod shell;
mod temp;
mod terminal;
mod theme;
mod uptime;

use battery::battery_info;
use cpu::cpu_info;
use desktop::desktop_info;
use disk::disk_info;
use disks::disks_info;
use gpu::gpu_info;
use host::host_info;
use kernel::kernel_info;
use memory::memory_info;
use os::os_info;
use packages::packages_info;
use resolution::resolution_info;
use shell::shell_info;
use temp::temp_info;
use terminal::terminal_info;
use theme::theme_info;
use uptime::uptime_info;

// Códigos de color ANSI
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const CYAN: &str = "\x1b[36m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const MAGENTA: &str = "\x1b[35m";
const RED: &str = "\x1b[31m";
const BLUE: &str = "\x1b[34m";

const VERSION: &str = "0.3.0";

fn print_info(label: &str, value: String, label_color: &str) {
    println!("{}{}{}: {}{}", BOLD, label_color, label, RESET, value);
}

fn show_help() {
    println!("{}novafetch {}{}- System information fetch tool{}", BOLD, VERSION, CYAN, RESET);
    println!();
    println!("Usage: novafetch [OPTIONS]");
    println!();
    println!("Options:");
    println!("  -o, -os        Operating system");
    println!("  -h, -host      Host name");
    println!("  -k, -kernel    Kernel version");
    println!("  -u, -uptime    System uptime");
    println!("  -p, -packages  Package count");
    println!("  -s, -shell     Current shell");
    println!("  -t, -terminal  Terminal type");
    println!("  -de, -desktop  Desktop environment / WM");
    println!("  -r, -res       Screen resolution");
    println!("  -th, -theme    GTK theme and icons");
    println!("  -c, -cpu       CPU information");
    println!("  -temp          CPU temperature");
    println!("  -g, -gpu       GPU information");
    println!("  -m, -memory    Memory usage");
    println!("  -ms, -swap     Swap usage");
    println!("  -d, -disk      Disk usage (root)");
    println!("  -ds, -disks    All disk partitions");
    println!("  -b, -battery   Battery status");
    println!("  -v, --version  Show version");
    println!("  --help         Show this help message");
}

fn main() {
    let options: Vec<String> = std::env::args().collect();
    
    // Verificar --version primero
    if options.len() > 1 && (options[1] == "-v" || options[1] == "--version") {
        println!("novafetch {}", VERSION);
        return;
    }
    
    // Verificar --help
    if options.len() > 1 && options[1] == "--help" {
        show_help();
        return;
    }
    
    if options.len() > 1 {
        let valid_options = options[1..options.len()].to_vec();
        for option in valid_options {
            match option.as_str() {
                "-o" | "-O" | "-os" | "-OS" => print_info("OS", os_info(), CYAN),
                "-h" | "-H" | "-host" | "-HOST" => print_info("Host", host_info(), CYAN),
                "-k" | "-K" | "-kernel" | "-KERNEL" => print_info("Kernel", kernel_info(), CYAN),
                "-u" | "-U" | "-uptime" | "-UPTIME" => print_info("Uptime", uptime_info(), CYAN),
                "-p" | "-P" | "-packages" | "-PACKAGES" => {
                    print_info("Packages", packages_info(os_info()), CYAN)
                }
                "-s" | "-S" | "-shell" | "-SHELL" => print_info("Shell", shell_info(), CYAN),
                "-t" | "-T" | "-terminal" | "-TERMINAL" => {
                    print_info("Terminal", terminal_info(), CYAN)
                }
                "-de" | "-DE" | "-desktop" | "-DESKTOP" => {
                    print_info("DE/WM", desktop_info(), GREEN)
                }
                "-r" | "-R" | "-res" | "-RES" | "-resolution" | "-RESOLUTION" => {
                    print_info("Resolution", resolution_info(), GREEN)
                }
                "-th" | "-TH" | "-theme" | "-THEME" => print_info("Theme", theme_info(), GREEN),
                "-c" | "-C" | "-cpu" | "-CPU" => print_info("CPU", cpu_info(), YELLOW),
                "-temp" | "-TEMP" | "-temperature" | "-TEMPERATURE" => {
                    print_info("CPU Temp", temp_info(), RED)
                }
                "-g" | "-G" | "-gpu" | "-GPU" => print_info("GPU", gpu_info(), MAGENTA),
                "-m" | "-M" | "-memory" | "-MEMORY" => {
                    print_info("Memory", memory_info("memory"), BLUE)
                }
                "-ms" | "-MS" | "-swap" | "-SWAP" => {
                    print_info("Swap", memory_info("swap"), BLUE)
                }
                "-d" | "-D" | "-disk" | "-DISK" => print_info("Disk", disk_info(), BLUE),
                "-ds" | "-DS" | "-disks" | "-DISKS" => {
                    print_info("Disks", disks_info(), BLUE)
                }
                "-b" | "-B" | "-battery" | "-BATTERY" => {
                    print_info("Battery", battery_info(), GREEN)
                }
                _ => println!("{}Invalid option: {}{}", RED, option, RESET),
            }
        }
    } else {
        // Mostrar toda la información con colores
        print_info("OS", os_info(), CYAN);
        print_info("Host", host_info(), CYAN);
        print_info("Kernel", kernel_info(), CYAN);
        print_info("Uptime", uptime_info(), CYAN);
        print_info("Packages", packages_info(os_info()), CYAN);
        print_info("Shell", shell_info(), CYAN);
        print_info("Terminal", terminal_info(), CYAN);
        print_info("DE/WM", desktop_info(), GREEN);
        print_info("Resolution", resolution_info(), GREEN);
        print_info("Theme", theme_info(), GREEN);
        print_info("CPU", cpu_info(), YELLOW);
        print_info("CPU Temp", temp_info(), RED);
        print_info("GPU", gpu_info(), MAGENTA);
        print_info("Memory", memory_info("memory"), BLUE);
        print_info("Swap", memory_info("swap"), BLUE);
        print_info("Disk", disk_info(), BLUE);
        print_info("Battery", battery_info(), GREEN);
    }
}

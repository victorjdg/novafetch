mod battery;
mod cpu;
mod desktop;
mod disk;
mod disks;
mod error;
mod gpu;
mod host;
mod kernel;
mod memory;
mod os;
mod packages;
mod parser;
mod resolution;
mod shell;
mod temp;
mod terminal;
mod theme;
mod uptime;

use battery::battery_info;
use cpu::format_cpu_info;
use desktop::desktop_info;
use disk::format_disk_info;
use disks::disks_info;
use gpu::gpu_info;
use host::host_info;
use kernel::kernel_info;
use memory::{format_memory_info, MemoryType};
use os::os_info;
use packages::packages_info;
use resolution::resolution_info;
use shell::shell_info;
use temp::temp_info;
use terminal::terminal_info;
use theme::theme_info;
use uptime::uptime_info;

use clap::Parser;

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const CYAN: &str = "\x1b[36m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const MAGENTA: &str = "\x1b[35m";
const RED: &str = "\x1b[31m";
const BLUE: &str = "\x1b[34m";

#[derive(Parser)]
#[command(
    name = "novafetch",
    about = "System information fetch tool",
    version = env!("CARGO_PKG_VERSION"),
    disable_help_flag = true,
    help_template = "{name} {version}\n{about-section}

{usage-heading} {usage}

{all-args}"
)]
struct Cli {
    #[arg(long, action = clap::ArgAction::SetTrue)]
    help: bool,
    #[arg(short, long)]
    os: bool,
    #[arg(short = 'h', long)]
    host: bool,
    #[arg(short, long)]
    kernel: bool,
    #[arg(short, long)]
    uptime: bool,
    #[arg(short, long)]
    packages: bool,
    #[arg(short, long)]
    shell: bool,
    #[arg(short, long)]
    terminal: bool,
    #[arg(long)]
    desktop: bool,
    #[arg(short, long)]
    resolution: bool,
    #[arg(long)]
    theme: bool,
    #[arg(short, long)]
    cpu: bool,
    #[arg(long)]
    temp: bool,
    #[arg(short, long)]
    gpu: bool,
    #[arg(short, long)]
    memory: bool,
    #[arg(long)]
    swap: bool,
    #[arg(short, long)]
    disk: bool,
    #[arg(long)]
    disks: bool,
    #[arg(short, long)]
    battery: bool,
}

fn print_info(label: &str, value: Result<String, error::FetchError>, label_color: &str) {
    let display_value = match value {
        Ok(v) => v,
        Err(_) => "N/A".to_string(),
    };
    println!("{}{}{}: {}{}", BOLD, label_color, label, RESET, display_value);
}

fn show_all_info() {
    print_info("OS", os_info(), CYAN);
    print_info("Host", host_info(), CYAN);
    print_info("Kernel", kernel_info(), CYAN);
    print_info("Uptime", uptime_info(), CYAN);
    let os_result = os_info().unwrap_or_default();
    print_info("Packages", packages_info(&os_result), CYAN);
    print_info("Shell", shell_info(), CYAN);
    print_info("Terminal", terminal_info(), CYAN);
    print_info("DE/WM", desktop_info(), GREEN);
    print_info("Resolution", resolution_info(), GREEN);
    print_info("Theme", theme_info(), GREEN);
    print_info("CPU", format_cpu_info(), YELLOW);
    print_info("CPU Temp", temp_info(), RED);
    print_info("GPU", gpu_info(), MAGENTA);
    print_info("Memory", format_memory_info(MemoryType::Memory), BLUE);
    print_info("Swap", format_memory_info(MemoryType::Swap), BLUE);
    print_info("Disk", format_disk_info(), BLUE);
    print_info("Battery", battery_info(), GREEN);
}

fn main() {
    let cli = Cli::parse();

    if cli.help {
        let mut cmd = <Cli as clap::CommandFactory>::command();
        cmd.print_help().unwrap();
        println!();
        return;
    }

    let no_args = !cli.os
        && !cli.host
        && !cli.kernel
        && !cli.uptime
        && !cli.packages
        && !cli.shell
        && !cli.terminal
        && !cli.desktop
        && !cli.resolution
        && !cli.theme
        && !cli.cpu
        && !cli.temp
        && !cli.gpu
        && !cli.memory
        && !cli.swap
        && !cli.disk
        && !cli.disks
        && !cli.battery;

    if no_args {
        show_all_info();
        return;
    }

    let os_result = os_info().unwrap_or_default();

    if cli.os {
        print_info("OS", os_info(), CYAN);
    }
    if cli.host {
        print_info("Host", host_info(), CYAN);
    }
    if cli.kernel {
        print_info("Kernel", kernel_info(), CYAN);
    }
    if cli.uptime {
        print_info("Uptime", uptime_info(), CYAN);
    }
    if cli.packages {
        print_info("Packages", packages_info(&os_result), CYAN);
    }
    if cli.shell {
        print_info("Shell", shell_info(), CYAN);
    }
    if cli.terminal {
        print_info("Terminal", terminal_info(), CYAN);
    }
    if cli.desktop {
        print_info("DE/WM", desktop_info(), GREEN);
    }
    if cli.resolution {
        print_info("Resolution", resolution_info(), GREEN);
    }
    if cli.theme {
        print_info("Theme", theme_info(), GREEN);
    }
    if cli.cpu {
        print_info("CPU", format_cpu_info(), YELLOW);
    }
    if cli.temp {
        print_info("CPU Temp", temp_info(), RED);
    }
    if cli.gpu {
        print_info("GPU", gpu_info(), MAGENTA);
    }
    if cli.memory {
        print_info("Memory", format_memory_info(MemoryType::Memory), BLUE);
    }
    if cli.swap {
        print_info("Swap", format_memory_info(MemoryType::Swap), BLUE);
    }
    if cli.disk {
        print_info("Disk", format_disk_info(), BLUE);
    }
    if cli.disks {
        print_info("Disks", disks_info(), BLUE);
    }
    if cli.battery {
        print_info("Battery", battery_info(), GREEN);
    }
}

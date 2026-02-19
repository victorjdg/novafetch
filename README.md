# novafetch

A command-line system information fetch tool written in Rust. Displays hardware and software information about your Linux system, similar to neofetch.

![License](https://img.shields.io/badge/license-GPL--3.0-blue.svg)
![Language](https://img.shields.io/badge/language-Rust-orange.svg)
![Platform](https://img.shields.io/badge/platform-Linux-lightgrey.svg)

## Features

novafetch can display the following system information:

| Category | Description |
|----------|-------------|
| OS | Operating system name and version |
| Host | System hostname |
| Kernel | Linux kernel version |
| Uptime | System uptime |
| Packages | Number of installed packages (supports multiple package managers) |
| Shell | Current shell |
| Terminal | Terminal emulator |
| Desktop | Desktop Environment or Window Manager |
| Resolution | Screen resolution (X11/Wayland) |
| Theme | GTK theme, icon theme, and cursor |
| CPU | Processor information |
| Temperature | CPU temperature |
| GPU | Graphics card information |
| Memory | RAM usage |
| Swap | Swap usage |
| Disk | Root partition usage (/) |
| Disks | All disk partitions |
| Battery | Battery status and percentage |

## Installation

### Arch Linux (AUR)

```bash
yay -S novafetch-git
# or
paru -S novafetch-git
```

### From Source

**Requirements:**
- Rust toolchain (cargo)
- make (optional)

```bash
# Clone the repository
git clone https://github.com/UnversedBlood/novafetch.git
cd novafetch

# Build
make
# or
cargo build --release

# Install system-wide
sudo make install
```

### Uninstall

```bash
sudo make uninstall
```

## Usage

### Show all information

```bash
novafetch
```

### Show specific information

```bash
novafetch -c -g -m    # Show only CPU, GPU, and Memory
```

### Available Options

| Short | Long Options | Description |
|-------|--------------|-------------|
| `-o` | `-os`, `-OS` | Operating system |
| `-h` | `-host`, `-HOST` | Host name |
| `-k` | `-kernel`, `-KERNEL` | Kernel version |
| `-u` | `-uptime`, `-UPTIME` | System uptime |
| `-p` | `-packages`, `-PACKAGES` | Package count |
| `-s` | `-shell`, `-SHELL` | Current shell |
| `-t` | `-terminal`, `-TERMINAL` | Terminal type |
| `-de` | `-desktop`, `-DESKTOP` | Desktop Environment / Window Manager |
| `-r` | `-res`, `-resolution` | Screen resolution |
| `-th` | `-theme`, `-THEME` | GTK theme, icons, cursor |
| `-c` | `-cpu`, `-CPU` | CPU information |
| `-temp` | `-TEMP` | CPU temperature |
| `-g` | `-gpu`, `-GPU` | GPU information |
| `-m` | `-memory`, `-MEMORY` | Memory usage |
| `-ms` | `-swap`, `-SWAP` | Swap usage |
| `-d` | `-disk`, `-DISK` | Disk usage (/) |
| `-ds` | `-disks`, `-DISKS` | All disk partitions |
| `-b` | `-battery`, `-BATTERY` | Battery status |
| `-v` | `--version` | Show version |
| | `--help` | Show help message |

### Development

```bash
# Run without installing
cargo run

# Run with arguments
cargo run -- -c -g -u
```

## Dependencies

### Required
- `hostname`, `uname`, `uptime`
- `lscpu` (CPU info)
- `lspci` (GPU info)
- `free` (memory info)
- `df` (disk info)

### Optional
- `sensors` (CPU temperature - falls back to sysfs if unavailable)
- `upower` (battery info)
- `xrandr` or `wlr-randr` (display resolution)
- `gsettings` or `xfconf-query` (theme detection)
- Package manager commands (`dnf`, `apt`, `pacman`, `zypper`, `rpm`, `apk`, etc.)

## Supported Package Managers

| Distribution | Package Manager |
|--------------|-----------------|
| Fedora | dnf |
| Debian/Ubuntu/Linux Mint/Pop!_OS | dpkg |
| Arch/Manjaro/EndeavourOS | pacman |
| openSUSE | zypper |
| RHEL/CentOS/Rocky/Alma | rpm |
| Alpine Linux | apk |
| Gentoo | qlist |
| Void Linux | xbps-query |
| Solus | eopkg |
| NixOS | nix-store |

## Example Output

```
OS: Fedora Linux 40 (Workstation Edition)
Host: myhostname
Kernel: 6.8.5-301.fc40.x86_64
Uptime: 3 hours, 42 minutes
Packages: 2150 (dnf)
Shell: /bin/bash
Terminal: xterm-256color
Desktop: GNOME
Resolution: 1920x1080
Theme: Adwaita-dark [GTK2/3]
Icons: Papirus [GTK2/3]
CPU: AMD Ryzen 7 5800X (16) @ 3.800GHz
Temperature: 45°C
GPU: NVIDIA GeForce RTX 3070
Memory: 6423 MiB / 32036 MiB (20%)
Swap: No swap
Disk (/): 45G / 100G (45%)
Battery: 85% [Discharging]
```

## Special Thanks

Thanks to my friend [vbaenal](https://github.com/vbaenal) for helping with Rust questions and guidance.

## License

This project is licensed under the GPL-3.0 License - see the [LICENSE](LICENSE) file for details.

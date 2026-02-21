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

| Short | Long | Description |
|-------|------|-------------|
| `-o` | `--os` | Operating system |
| `-h` | `--host` | Host name |
| `-k` | `--kernel` | Kernel version |
| `-u` | `--uptime` | System uptime |
| `-p` | `--packages` | Package count |
| `-s` | `--shell` | Current shell |
| `-t` | `--terminal` | Terminal type |
| | `--desktop` | Desktop Environment / Window Manager |
| `-r` | `--resolution` | Screen resolution |
| | `--theme` | GTK theme, icons, cursor |
| `-c` | `--cpu` | CPU information |
| | `--temp` | CPU temperature |
| `-g` | `--gpu` | GPU information |
| `-m` | `--memory` | Memory usage |
| | `--swap` | Swap usage |
| `-d` | `--disk` | Disk usage (/) |
| | `--disks` | All disk partitions |
| `-b` | `--battery` | Battery status |
| `-v` | `--version` | Show version |
| | `--help` | Show help message |

### Development

```bash
# Run without installing
cargo run

# Run with arguments
cargo run -- -c -g -u

# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

## Dependencies

### Build Dependencies
- Rust toolchain (cargo >= 1.56)
- make (optional)

### Runtime Dependencies (Required)
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
OS: Linux Mint 22.2
Host: myhostname
Kernel: 6.14.0-37-generic
Uptime: 3 hours, 42 minutes
Packages: 2199
Shell: bash
Terminal: xterm-256color
DE/WM: Cinnamon
Resolution: 3440x1440 @ 60.00Hz
Theme: Theme: Mint-Y-Purple | Icons: Mint-Y-Purple | Cursor: Bibata-Modern-Classic
CPU: Intel(R) Core(TM) i7-1185G7 @ 3.00GHz (8) @ 4800 MHz
CPU Temp: 59.4°C (avg) / 64.0°C (max)
GPU: TigerLake-LP GT2
Memory: 8617 / 31881 Mi (27%)
Swap: No swap
Disk: 64G / 417G (17%)
Battery: 80% [charging] | Time: N/A | Wireless Keyboard
```

## Special Thanks

Thanks to my friend [vbaenal](https://github.com/vbaenal) for helping with Rust questions and guidance.

## License

This project is licensed under the GPL-3.0 License - see the [LICENSE](LICENSE) file for details.

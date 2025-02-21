novafetch is a rust tool to get information about your computer-

It is currently able to show the following informaton:
* OS
* Host
* Kernel
* Uptime
* Number of packages installed
* Shell
* Terminal
* CPU
* GPU
* Memory [RAM & Swap]
* Disk
* Battery

## Installing / Unistalling
If you are an Arch Linux user you can install Novafetch through the Arch User Repository. Otherwise you can install it using the following commands:

Installing:
```bash
make
sudo make install
```

Unistalling
```bash
sudo make unistall
```

## Usage
### Show all avaliable information
```bash
cargo run
```
If you choose to install novafetch you can access the information with:
```bash
novafetch
```

### Arguments supported
| Short Options | Long Options       | Description                          | Command Executed                          |
|---------------|--------------------|--------------------------------------|-------------------------------------------|
| `-o`          | `-O`, `-os`, `-OS` | Displays operating system information | `println!("OS: {}", os_info())`           |
| `-h`          | `-H`, `-host`, `-HOST` | Displays host information           | `println!("Host: {}", host_info())`       |
| `-k`          | `-K`, `-kernel`, `-KERNEL` | Displays kernel information         | `println!("Kernel: {}", kernel_info())`   |
| `-u`          | `-U`, `-uptime`, `-UPTIME` | Displays system uptime              | `println!("Uptime: {}", uptime_info())`   |
| `-p`          | `-P`, `-packages`, `-PACKAGES` | Displays package information       | `println!("Packages: {}", uptime_info())` |
| `-s`          | `-S`, `-shell`, `-SHELL` | Displays shell information          | `println!("Shell: {}", uptime_info())`    |
| `-t`          | `-T`, `-terminal`, `-TERMINAL` | Displays terminal information      | `println!("Terminal: {}", uptime_info())` |
| `-c`          | `-C`, `-cpu`, `-CPU` | Displays CPU information             | `println!("CPU: {}", cpu_info())`         |
| `-g`          | `-G`, `-gpu`, `-GPU` | Displays GPU information             | `println!("GPU: {}", gpu_info())`         |
| `-m`          | `-M`, `-memory`, `-MEMORY` | Displays memory information         | `println!("Memory: {}", memory_info("memory"))` |
| `-ms`         | `-MS`, `-swap`, `-SWAP` | Displays swap information            | `println!("Swap: {}", memory_info("swap"))`|
| `-d`          | `-D`, `-disk`, `-DISK` | Displays disk information (/)       | `println!("Disk (/): {}", gpu_info())`    |
| `-b`          | `-B`, `-battery`, `-BATTERY` | Displays battery information       | `println!("Battery: {}", battery_info())` |

It also supports multiple options, example:
```bash
novafetch -c -g -u
```
## Special thanks
I wanted to take a moment to say thanks to my friend [vbaenal](https://github.com/vbaenal) for resolving my stupid questions about rust usage.

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
| Short Options | Long Options       | Description                          |
|---------------|--------------------|--------------------------------------|
| `-o`          | `-O`, `-os`, `-OS` | Displays operating system information |
| `-h`          | `-H`, `-host`, `-HOST` | Displays host information |
| `-k`          | `-K`, `-kernel`, `-KERNEL` | Displays kernel information |
| `-u`          | `-U`, `-uptime`, `-UPTIME` | Displays system uptime |
| `-p`          | `-P`, `-packages`, `-PACKAGES` | Displays package information |
| `-s`          | `-S`, `-shell`, `-SHELL` | Displays shell information |
| `-t`          | `-T`, `-terminal`, `-TERMINAL` | Displays terminal information |
| `-c`          | `-C`, `-cpu`, `-CPU` | Displays CPU information |
| `-g`          | `-G`, `-gpu`, `-GPU` | Displays GPU information |
| `-m`          | `-M`, `-memory`, `-MEMORY` | Displays memory information |
| `-ms`         | `-MS`, `-swap`, `-SWAP` | Displays swap information |
| `-d`          | `-D`, `-disk`, `-DISK` | Displays disk information (/) |
| `-b`          | `-B`, `-battery`, `-BATTERY` | Displays battery information |

It also supports multiple options, example:
```bash
novafetch -c -g -u
```
## Special thanks
I wanted to take a moment to say thanks to my friend [vbaenal](https://github.com/vbaenal) for resolving my stupid questions about rust usage.

rustfetch is a rust tool to get information about your computer-

It is currently able to show the following informaton:
* OS
* Host
* Kernel
* Uptime
* CPU
* GPU

## Usage

### Show all avaliable information
```bash
cargo run
```

### Option support
The application has support for the following options:
#### OS
```bash
cargo run main -o|-os|-O|-OS
```
#### Host
```bash
cargo run main -h|-host|-H|-HOST
```
#### Kernel
```bash
cargo run main -k|-kernel|-K|-KERNEL
```
#### Uptime
```bash
cargo run main -u|-uptime|-U|-UPTIME
```
#### CPU
```bash
cargo run main -c|-cpu|-C|-CPU
```
#### GPU
```bash
cargo run main -g|-gpu|-G|-GPU
```

It also supports multiple options:
```bash
cargo run main -c -g -u
```

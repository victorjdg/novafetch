novafetch is a rust tool to get information about your computer-

It is currently able to show the following informaton:
* OS
* Host
* Kernel
* Uptime
* CPU
* Memory
* GPU

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

### Option support
The application has support for the following options:
#### OS
```bash
novafetch -o|-os|-O|-OS
```
#### Host
```bash
novafetch -h|-host|-H|-HOST
```
#### Kernel
```bash
novafetch -k|-kernel|-K|-KERNEL
```
#### Uptime
```bash
novafetch -u|-uptime|-U|-UPTIME
```
#### CPU
```bash
novafetch -c|-cpu|-C|-CPU
```
#### Memory
```bash
novafetch -m|-memory|-M|-MEMORY
```
#### Resolution
```
novafetch -r|-resolution|-R|-RESOLUTION
```
#### GPU
```bash
novafetch -g|-gpu|-G|-GPU
```

It also supports multiple options:
```bash
novafetch -c -g -u
```

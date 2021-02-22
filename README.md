novafetch is a rust tool to get information about your computer-

It is currently able to show the following informaton:
* OS
* Host
* Kernel
* Uptime
* CPU
* GPU

## Installing / Unistalling
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
#### GPU
```bash
novafetch -g|-gpu|-G|-GPU
```

It also supports multiple options:
```bash
novafetch -c -g -u
```

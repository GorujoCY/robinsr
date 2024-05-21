# Supported Version: 2.2.51, 2.2.52, 2.2.53

Tool website: [https://freesr-tools.pages.dev](https://freesr-tools.pages.dev)

Start battle by entering any calyx in the map, DON'T ATTACK THE ENEMIES, IT WON'T WORK (maybe)

Some scenes might not loaded properly. If you stuck at loading screen, remove `persistent` file.

# RobinSR
Original: https://github.com/amizing25/robinsr


A Server emulator for the game [`Honkai: Star Rail`](https://hsr.hoyoverse.com/en-us/)
![screenshot](image.png)

## Installation

### From Source

#### Requirements

- [Rust](https://www.rust-lang.org/tools/install)

**NOTE**: Nightly Rust is required to build the project. To install it, first install
Rust itself, then run the following command:

```sh
rustup toolchain install nightly
rustup default nightly
```
Android Installation:

```sh
pkg upgrade
pkg install git rust -y
git clone https://github.com/GorujoCY/robinsr.git
cd robinsr
chmod +x run.sh
./run.sh
```

#### Building

```sh
cargo install --path gameserver
cargo install --path sdkserver
```
## Usage

To begin using the server, you need to run `run.bat` file (you can check its source before doing so if you're afraid)

Android: 

```sh
cd ~/robinsr
./run.sh
```

## Connecting
[Get 2.3 beta client](https://autopatchos.starrails.com/client/Beta/20240517111205_PZfNSHVLH509e76v/StarRail_.2.53.zip)

[Consult with me](https://gorujokun.cy/#contact) for the apk version

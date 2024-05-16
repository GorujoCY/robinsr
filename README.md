# Supported Version: 2.2.51

Tool website: [https://freesr-tools.pages.dev](https://freesr-tools.pages.dev)

Start battle by entering any calyx in the map, DON'T ATTACK THE ENEMIES, IT WON'T WORK (maybe)

Some scenes might not loaded properly. If you stuck at loading screen, remove `persistent` file.

# RobinSR
Original: https://github.com/amizing25/robinsr


A Server emulator for the game [`Honkai: Star Rail`](https://hsr.hoyoverse.com/en-us/)
![changing screenshot](image.png)

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

#### Building

```sh
cargo install --path gameserver
cargo install --path sdkserver
```
## Usage

To begin using the server, you need to run `run.bat` file (you can check its source before doing so if you're afraid)

## Connecting
[Get 2.3 beta client](https://autopatchos.starrails.com/client/Beta/20240501125700_dUBAjS7YiX9nF7mJ/StarRail_2.2.51.zip)
~~Replace mhypbase.dll file in your game folder, it will redirect game traffic (and also disable in-game censorship)~~ You will be guided or assume you have a way to proxy already

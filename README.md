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
#### Android Requirements:
- [Termux](https://github.com/termux/termux-app/releases/download/v0.118.0/termux-app_v0.118.0+github-debug_universal.apk)

#### Android Installation:

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

## Online/Offline for Laptops
For laptops it is very similar to the android guide below, start renaming  `versions.json` to `versions-online.json` then `versions-offline.json` to `versions.json` and/or vice versa then copy it on the `sdkserver` folders 
# Android Specific Guides

## Tranferring FreeSR Data from your directory (assuming Download Folder of your phone)
```sh
termux-setup-storage #grant it files permission
cd ~/robinsr 
rm -f freesr-data.json
cd /sdcard/Download #if its not on your download folder then change this to the directory that's in
mv freesr-data.json ~/robinsr
cd ~/robinsr
```
if `termux-setup-storage` does not work manually grant the permission
[tutorial](https://cdn.discordapp.com/attachments/1240737048483332147/1241023572135120997/SjNpYzk.mp4?ex=665099c4&is=664f4844&hm=4b34cd0f7611fbf939a2666ca0db713fb4042dbdcb62346a480afeed29a8705f&)

## Online/Offline functionality toggle
Those are dedicated for areas you cant have Mobile Data or Wi-Fi like airplanes but do them anytime you like you are free after all

### Online to Offline
```sh
cd ~/robinsr
mv versions.json versions-online.json
mv versions-offline.json versions.json
cp versions.json sdkserver
```
If you want to say apply the hotfix again in case of breakage or something
### Offline to Online 
```sh
cd ~/robinsr
mv versions.json versions-offline.json
mv versions-online.json versions.json
cp versions.json sdkserver
```

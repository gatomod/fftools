<img align="left" width="150" height="150" src="assets/FFtools.png">

# FFtools
## FFMPEG-based toolkit for easy media manipulation

<br />

[<img alt="license" src="https://img.shields.io/github/license/gatomo-oficial/path_trav.svg?color=06b6d4&style=for-the-badge&logo=Apache">](https://www.apache.org/licenses/LICENSE-2.0)
[<img alt="crates.io" src="https://img.shields.io/crates/v/fftools.svg?style=for-the-badge&color=fc8d62&logo=rust">](https://crates.io/crates/fftools)
[<img alt="discord" src="https://img.shields.io/discord/880947411432923136?style=for-the-badge&color=blue&logo=discord">](https://gatomo.ga/discord)

FFtools is a FFmpeg command spawner made in Rust that focuses on simplicity in different common tasks (such as trim, optimize, merge, etc.).

In addition, it has several improvements, like automatic stream copy detection (which allows a faster coding) or media optimization via TBN (which can reduce file size with not much loss of quality).


## Table of contents
- [Usage example](#usage-example)
- [Wiki](#wiki)
- [Installation](#installation)
  - [Dependencies](#dependencies)
  - [Standalone (Using Cargo)](#standalone-using-cargo)
  - [Linux](#linux)
    - [Arch Linux](#arch-linux)
    - [Debian/Ubuntu](#debianubuntu)
  - [Windows](#windows)
    - [Manual](#manual)
  - [Manual](#manual-1)
- [Development](#development)
- [Release history](#release-history)
- [License](#license)
- [Contribute](#contribute)

## Usage example


Convert to lossless GIF
```sh
fftools gif -i video.mp4 output.gif
```
https://user-images.githubusercontent.com/63877602/226492973-1edf7625-8301-4be0-a62d-c3146c655b30.mp4

Optimizing video at 5k TBN
```sh
fftools optimize -i video.mp4 output.mp4 5k
```
https://user-images.githubusercontent.com/63877602/226492359-b3155939-99e5-4f76-9bd2-7c4fa3d51238.mp4

You can use flags to extend command functionalities. All commands and arguments are available in the [FFtools Wiki](https://github.com/gatomo-oficial/fftools/wiki).

## Wiki
If you want to see an extended documentation about commands, options and usage, please go to [FFtools Wiki](https://github.com/gatomo-oficial/fftools/wiki).

## Installation
### Dependencies
- [`ffmpeg`](https://ffmpeg.org/)

### Standalone (Using Cargo)
```sh
cargo install fftools
```

### Linux
#### Arch Linux
FFtools is available via AUR.
```sh
yay -S fftools-bin
# or
paru -S fftools-bin
```

#### Debian/Ubuntu
For Debian, Ubuntu and Debian-based distros, download the `.deb` in [`Releases`](https://github.com/gatomo-oficial/fftools/releases/). Then install it with DPKG.
```sh
sudo dpkg -i fftools_1.0.0_amd64.deb
```

### Windows
FFtools is also available on Windows. 

#### Manual
1. Install `FFmpeg`.
2. Download the .exe in [`Releases`](https://github.com/gatomo-oficial/fftools/releases/).
3. Add it to PATH. If you don't know how to do it, [read this post](https://www.architectryan.com/2018/03/17/add-to-the-path-on-windows-10/).

> **Note:** There will be Scoop support soon...

### Manual
If you want to edit code or install it manually you can run these commands.
```sh
git clone https://github.com/gatomo-oficial/fftools.git
cd fftools
cargo build --release
```

## Development
Clone the repository, install dependencies and run it.
```sh
git clone https://github.com/gatomo-oficial/fftools.git
cd fftools
cargo run
```

## Release history
* 1.0.0
  * Initial release

## License
FFtools is licensed under the [Apache 2.0 license](https://www.apache.org/licenses/LICENSE-2.0).

## Contribute
Any PR is welcome! Is a small project, so the guideline is to follow the code style and not make insane purposes.

*Gátomo - Apache 2.0 license*

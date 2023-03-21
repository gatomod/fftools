#!/bin/bash
# this script is weird

cargo install cargo-aur cargo-deb

cargo aur
cargo deb

mkdir -p build

mv target/debian/* build
mv *.tar.gz build

makepkg --printsrcinfo > .SRCINFO

mv PKGBUILD build
mv .SRCINFO build

cargo build --target x86_64-pc-windows-gnu --release
mv target/x86_64-pc-windows-gnu/release/fftools.exe build/fftools-windows-x86_64.exe

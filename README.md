# Launcher

Simple terminal application to operate as a game controller mananged menu to
launch other applications.

## Building

The launcher is written in [Rust](https://www.rust-lang.org/) and requires a
working Rust toolchain to build.

Requires `libudev-dev` to be installed on the system.

e.g. on Rasperry Pi OS/Debian:

    sudo apt install libudev-dev
    
Build release version:

    cargo build --release
# OS-Leela

An experimental operating system built completely with Rust as a practice project.

## Overview

OS-Leela is a bare-metal operating system written in Rust, designed for x86_64 architecture. This project serves as a learning platform for understanding low-level systems programming and operating system development.

## Features

- **Bare-metal Rust**: Built entirely without the standard library (`no_std`)
- **VGA Text Mode**: Direct VGA buffer manipulation for text output
- **Custom Bootloader**: Uses the `bootloader` crate for x86_64 boot process
- **Memory Safe**: Leverages Rust's safety guarantees for systems programming

## Prerequisites

- Rust nightly toolchain
- `bootimage` tool for creating bootable disk images
- QEMU (optional, for testing the OS)

## Building

Install the required tools:

```bash
rustup override set nightly
cargo install bootimage
rustup component add rust-src
rustup component add llvm-tools-preview
```

Build the OS:

```bash
cargo build
```

Create a bootable disk image:

```bash
cargo bootimage
```

## Running

Run the OS in QEMU:

```bash
cargo run
```

Or manually with QEMU:

```bash
qemu-system-x86_64 -drive format=raw,file=target/x86_64-os-leela/debug/bootimage-os-leela.bin
```

## Project Structure

- `src/main.rs` - Entry point and main OS logic
- `src/vga_buffer.rs` - VGA text mode driver for console output
- `x86_64-os-leela.json` - Custom target specification for bare-metal x86_64
- `.cargo/config.toml` - Cargo build configuration

## Architecture

The OS currently implements:

1. **Entry Point** (`_start`): The OS entry point called by the bootloader
2. **Panic Handler**: Custom panic handler for bare-metal environment
3. **VGA Buffer Driver**: Text-mode output directly to VGA memory at 0xb8000
4. **Print Macros**: `print!` and `println!` macros for formatted output

## License

See LICENSE file for details.

## Contributing

This is a practice project, but suggestions and improvements are welcome!

## Resources

- [Writing an OS in Rust](https://os.phil-opp.com/) by Philipp Oppermann
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [OSDev Wiki](https://wiki.osdev.org/)
# Riven OS

A bare-metal operating system written in Rust.

Riven OS is a personal systems programming project exploring operating system development, kernel architecture, and low-level hardware interaction from the ground up.

## About

Built with Rust and targeting x86-64, Riven OS aims to deepen my understanding of how computers work beneath the applications and operating systems we use every day.

The project is an ongoing exploration of kernel development, memory management, and computer architecture.

## Built With

- **Rust** - kernel development
- **Rust Nightly** - custom target support and experimental features
- **Bootimage** - bootable kernel image generation
- **QEMU** — virtual machine testing

## Getting Started

### Prerequisites

- Rust nightly toolchain
- `rust-src` component
- `bootimage` utility
- QEMU for emulation

### Clone the Repository

```bash
git clone https://github.com/RicoLViviers/riven-os.git
cd riven-os
```

### Build

```bash
cargo +nightly bootimage
```

The generated bootable image is placed in the `target` directory.

### Run

Make sure you have `QEMU` installed.

```bash
cargo +nightly run
```

This will open a window running the operating system.

## Goals

- Understand the boot process and kernel initialization
- Explore memory management and hardware interaction
- Learn x86-64 architecture
- Build foundational operating system components
- Strengthen low-level programming skills with Rust

## Status

Riven OS is a work in progress. Features, architecture, and development instructions will evolve as the project grows.

## Author

**Rico Viviers**

[GitHub](https://github.com/RicoLViviers)

# Building & Installing SigmaOS

SigmaOS utilizes a strict `no_std` Rust environment and automates the build process via a root `Justfile`.

## Prerequisites

- `rustup` (Nightly toolchain required)
- `just` command runner (`cargo install just`)
- `qemu-system-x86_64` (for virtualization)

## Setup

1. **Install the `x86_64-unknown-none` target:**
   ```bash
   rustup target add x86_64-unknown-none
   ```

2. **Verify Toolchain:**
   Ensure you are using a nightly compiler as SigmaOS relies on unstable features like `naked_functions` and `asm_const`.

## Build Commands

Run these commands from the root of the repository:

- **Format Code:**
  ```bash
  just fmt
  ```

- **Compile and Check:**
  Validates that all Kernel and Userland code adheres to `no_std` zero-allocation constraints.
  ```bash
  just check
  ```

- **Build Kernel:**
  ```bash
  just build
  ```

- **Run in QEMU:**
  *Note: Bootloader integration is currently in progress.*
  ```bash
  just run
  ```

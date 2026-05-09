# SigmaOS Build Guide

## Prerequisites
- `aarch64-linux-gnu-g++` (cross-compiler for ARM64)
- `make`
- QEMU (for emulation testing)

## Quick Build

```bash
make all
```

## Architecture Matrix Builds

| Target    | Command                      | Output              |
|-----------|------------------------------|---------------------|
| ARM64 RPi | `make ARCH=aarch64`          | `sigma_os.elf`      |
| x86_64    | `make ARCH=x86_64`           | `sigma_os_x64.elf`  |
| RISC-V    | `make ARCH=riscv64`          | `sigma_os_rv.elf`   |

## Shard Manifest

The `SHARDS.manifest` file lists all 600+ shard `.cpp` files compiled by the Makefile. To add a new shard:
1. Create your `.cpp` file under the appropriate module directory.
2. Add its path to `SHARDS.manifest`.
3. Run `make`.

## Compiler Flags

```
-std=c++17 -ffreestanding -fno-exceptions -fno-rtti
-nostdlib -nostdinc++ -Wall -Wextra -Wpedantic
-I include -I include/core -I include/libc
```

> [!IMPORTANT]
> **No stdlib allowed.** Never `#include <iostream>`, `<string>`, or any STL header. Use `SovereignLibC.h` and `SigmaOOP.hpp` exclusively.

## Running in QEMU

```bash
qemu-system-aarch64 -machine raspi4b -kernel sigma_os.elf -serial stdio
```

## CI/CD Pipeline

The GitHub Actions matrix build runs `make ARCH=aarch64` and `make ARCH=x86_64` on every push to `main`. See `.github/workflows/` for full configuration.

## Zero-Dependency Enforcement

The `compile_flags.txt` passes `-nostdinc++` to clangd, ensuring the IDE also enforces the zero-stdlib rule during development.

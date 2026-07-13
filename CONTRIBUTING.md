# Contributing to SigmaOS

<<<<<<< Updated upstream
Thank you for your interest in advancing Sovereign Silicon! Contributing to SigmaOS requires adhering to strict architectural constraints.

## The Prime Directive: Zero Dependencies

SigmaOS guarantees computational sovereignty. Under no circumstances may a contributor:

1. `#include <stdio.h>`, `<stdlib.h>`, `<string.h>`, or any standard library header.

2. Link against `glibc`, `musl`, or any pre-compiled system library.

3. Import external logic that relies on POSIX standards.

## Writing a Sovereign Driver

When writing a driver, integrate it with the Universal Driver Framework (`sigma_driver_fw.cpp`).

1. **Hardware Direct**: Use MMIO or port I/O directly.

2. **Metadata**: Define a `SigmaDriverMetadata` block matching vendor/device IDs.

3. **Registration**: Expose an initialization function that calls `sigma_register_driver()`.

## Writing a Sovereign Tool

When building a new utility (e.g., a clone of a GNU tool):

1. **Standalone**: Create `tools/utilities/sigma_<name>.cpp`.

2. **Interface**: Expose `extern "C" int sigma_<name>_main(int argc, char** argv)`.

3. **I/O**: Only use `sigma_vga_puts()`, `sigma_vga_printf()`, or the VFS read/write functions.

4. **Integration**: Register your tool in the `sigma_sh.cpp` shell dispatcher.

Please open an RFC issue before initiating massive architectural shifts or adding entirely new file systems!
=======
Thank you for your interest in contributing to the Sovereign Lattice!

## Getting Started

### 1. Prerequisites

- `gcc-x86-64-linux-gnu`

- `nasm`

- `make`

- `python3` (for scripts)

- `qemu-system-x86` (for local emulation)

### Alternatively, use the provided `Dockerfile` for a zero-setup, cross-platform build environment

### 2. Building

Run the following to compile the entire OS:

```bash

make all

```

### 3. Architecture Rules

- **No External Dependencies:** We rely on `sigma_libc.h` and the Sovereign SDK. Do not include standard libraries like `<stdio.h>` in the kernel suites.

- **Zero-Trust:** Assume other shards are hostile. Verify inputs when exposing handlers to the Sovereign Event Bus.

## Coding Standards

- Use C11.

- Variables should be `snake_case`. Macros must be `UPPER_SNAKE_CASE`.

- Indentation is 4 spaces.

## PR Workflow

1. Fork the repo and create your branch from `main`.

2. Add your shard to the appropriate `SXX_` suite directory.

3. Ensure the CI passes (we check for dependencies and run static analysis).

4. Issue a PR with a clear description of the shard's purpose.
>>>>>>> Stashed changes

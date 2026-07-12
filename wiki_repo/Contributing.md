# Contributing to SigmaOS

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

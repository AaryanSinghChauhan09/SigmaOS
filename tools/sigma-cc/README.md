# SigmaCC Compiler & Toolchain

The **SigmaCC** toolchain is a sovereign wrapper around standard compiler technologies (like LLVM/Clang) that strictly enforces SigmaOS compilation requirements. 

SigmaOS rejects POSIX/libc dependencies at the kernel level, meaning binaries must be compiled freestanding, with strict memory management policies and custom linker alignments to respect SigmaOS privilege rings.

## Core Features
1. **Zero-Libc Enforcement:** Injects `-nostdlib` and `-ffreestanding` into every compilation.
2. **Sovereign Linker Alignment:** Automatically selects `x86_64-sigma.ld` or `aarch64-sigma.ld` to map the `.text` and `.data` segments correctly for the SigmaOS MLFQ Scheduler.
3. **Strict Memory Boundaries:** Prevents compiling code that attempts to bypass capability-based access control.

## Usage
Instead of using `clang` or `gcc`, developers target SigmaOS by running:
```bash
sigma-cc my_kernel_module.c -o my_kernel_module.o
```
This automatically ensures the module is 100% compliant with SigmaOS constraints.

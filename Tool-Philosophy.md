# 🧰 Tool Philosophy

> "BusyBox minimalism, but fully sovereign and statically built into the lattice."

SigmaOS replaces GNU coreutils and BusyBox with a custom suite of utilities designed around the SigmaOS system call dispatcher.

## 1. No POSIX Assumption
SigmaOS does not guarantee a 1:1 POSIX interface. Tools like `ls` or `cat` do not call `open()`, `read()`, or `printf()`. Instead, they invoke:

- `sovereign_syscall_opendir()`
- `sovereign_syscall_readdir()`
- `sigma_vga_printf()`

## 2. Monolithic Binary (Like BusyBox)
To save space and avoid dynamic linking, all standard utilities are compiled into a single binary (`sigma_sh`). The entry point checks `argv[0]` to determine which tool logic to execute.

## 3. Core Utilities Implemented
- `echo`: Implemented directly in `tools/utilities/sigma_echo.cpp`.
- `ls`: Traverses the VFS using raw syscalls (`tools/utilities/sigma_ls.cpp`).
- `cat`: Reads inodes directly into a buffer and streams to VGA.

## 4. The `sigma_sh` Shell
The shell is the gateway. It doesn't fork/exec standard ELF binaries initially. It executes built-in function pointers for the utilities. In the future, it will execute statically compiled Sovereign ELFs.

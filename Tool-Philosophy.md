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
SigmaOS has 19 fully sovereign, zero-dependency utilities:
- `pwd`: Print working directory (`tools/utilities/sigma_pwd.cpp`)
- `uname`: Display kernel name and system info (`tools/utilities/sigma_uname.cpp`)
- `ps`: Query and list current system processes and ticks (`tools/utilities/sigma_ps.cpp`)
- `top`: Real-time system resource monitor (`tools/utilities/sigma_top.cpp`)
- `kill`: Terminate a process or task (`tools/utilities/sigma_kill.cpp`)
- `cp`: Copy file contents byte-by-byte via system calls (`tools/utilities/sigma_cp.cpp`)
- `mv`: Move/rename directories and files (`tools/utilities/sigma_mv.cpp`)
- `rm`: Remove files/nodes (`tools/utilities/sigma_rm.cpp`)
- `chmod`: Modify file permission/mode flags (`tools/utilities/sigma_chmod.cpp`)
- `df`: Analyze disk space occupancy (`tools/utilities/sigma_df.cpp`)
- `grep`: Match pattern and locate substrings (`tools/utilities/sigma_grep.cpp`)
- `dmesg`: View system logs from kernel ring buffer (`tools/utilities/sigma_dmesg.cpp`)
- `wc`: Count lines, words, and bytes in a file (`tools/utilities/sigma_wc.cpp`)
- `head`: Display first N lines of a file (`tools/utilities/sigma_head.cpp`)
- `hexdump`: Read and display file contents in hex and ASCII (`tools/utilities/sigma_hexdump.cpp`)
- `ifconfig`: View network interfaces, IP addresses, netmasks, MACs (`tools/utilities/sigma_ifconfig.cpp`)
- `ping`: Send ICMP echo requests and track response time (`tools/utilities/sigma_ping.cpp`)
- `mount`: Mount ext2 or fat32 filesystems on partitions (`tools/utilities/sigma_mount.cpp`)
- `lspci`: Enumerate all PCI devices on the system bus (`tools/utilities/sigma_lspci.cpp`)

## 4. The `sigma_sh` Shell
The shell is the gateway. It doesn't fork/exec standard ELF binaries initially. It executes built-in function pointers for the utilities. In the future, it will execute statically compiled Sovereign ELFs.


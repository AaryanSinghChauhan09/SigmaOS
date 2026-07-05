# SigmaOS Developer Guide

Welcome to the SigmaOS project! This guide will help you understand the core architecture, build process, and how to write native services for the Sovereign lattice.

## 🏗️ Building SigmaOS

SigmaOS is designed with absolute sovereignty in mind, which means zero monolithic external dependencies (no glibc, no systemd).

### Prerequisites

- `cmake` (>= 3.20)

- `clang` (>= 15) or GCC (>= 12)

- `qemu-system-x86_64` (for local emulation)

- `mtools` (for creating bootable FAT32 images)

### Compilation

```bash

# 1. Clone the repository

git clone https://github.com/sigma-os/zenith.git
cd zenith

# 2. Configure the build

mkdir build && cd build
cmake .. -DCMAKE_TOOLCHAIN_FILE=../toolchain-x86_64-sigma.cmake

# 3. Compile the Kernel and Userland

make -j$(nproc)

# 4. Generate the bootable ISO

make iso
```

### Running in Emulation

```bash
qemu-system-x86_64 -cdrom sigmaos-zenith.iso -m 2G -enable-kvm -serial stdio
```

## 🧠 Architecture Overview

SigmaOS does not use a monolithic kernel. It operates on a **Lattice Architecture**:

- **Sovereign Init (`sigma-init`)**: Handles dependency-resolved parallel booting and process monitoring.

- **Zero-Trust VFS (`sigma-pam-acl`)**: All resources are denied by default unless an explicit Role-Based Access Control (RBAC) entry allows it.

- **Proton Bridge**: An opt-in compatibility layer (`sigma_proton_bridge`) that translates standard Linux POSIX syscalls into native SigmaOS IPC messages, allowing standard ELF binaries to run.

## 📝 Writing a Native Service

Native SigmaOS applications do not link against `libc`. They use the `sigma_libc` headers which provide thin wrappers around the native syscalls.

### The "Hello World" Service

```cpp
#include <sigma_libc.h>

int main() {
    // 1. Open the system journal (stdout equivalent)
    sigma_u32 fd = sys_file_open("/dev/journal", 0);

    if (fd > 0) {
        const char* msg = "Hello from a Sovereign Service!\n";
        sys_file_write(fd, msg, 32);
        sys_file_close(fd);
    }

    // 2. Exit cleanly
    sys_thread_exit(0);
    return 0;
}
```

### Native Syscall API (x86_64)

Native syscalls are executed via the `syscall` instruction with arguments in `rdi, rsi, rdx, r10, r8, r9`. The syscall number is passed in `rax`.

| Number | Name | Description | Arguments |
| :--- | :--- | :--- | :--- |
| `0x01` | `SYS_IPC_SEND` | Send a synchronous IPC message to a target shard. | `target_id`, `msg_ptr`, `size` |
| `0x02` | `SYS_IPC_RECV` | Block and wait for an IPC message. | `buffer_ptr`, `max_size` |
| `0x10` | `SYS_FILE_OPEN` | Open a VFS resource (enforces RBAC). | `path_ptr`, `flags` |
| `0x11` | `SYS_FILE_READ` | Read from an open VFS resource. | `fd`, `buf_ptr`, `count` |
| `0x12` | `SYS_FILE_WRITE` | Write to an open VFS resource. | `fd`, `buf_ptr`, `count` |
| `0x20` | `SYS_VMM_MAP` | Map a physical memory page into the shard's virtual space. | `virt_addr`, `phys_addr`, `flags` |

---
*For contributing to the kernel core, please refer to the `CONTRIBUTOR_ROADMAP.md`.*

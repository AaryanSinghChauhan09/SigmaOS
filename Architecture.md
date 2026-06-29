# SigmaOS Privilege & Isolation Boundaries

This document defines the strict execution boundaries between the Kernel Space and User Space in the SigmaOS Zenith microkernel.

---

## 🔒 Privilege Ring Separation

SigmaOS enforces a strict boundary using CPU privilege rings (Ring 0 and Ring 3):

```text
       +---------------------------------------------+
       |                 RING 3: USERLAND            |
       |  - sh shell       - coreutils (ls, cat)    |
       |  - UI Renderer    - apps / web applications |
       +----------------------+----------------------+
                              |
                     SYSCALL INTERFACE
                              |
       +----------------------v----------------------+
       |                RING 0: KERNEL SPACE         |
       |  - MLFQ Scheduler    - Page Allocators      |
       |  - VFS Vitals        - TCP/IP Stack         |
       |  - Device Drivers    - Shard Manager        |
       +---------------------------------------------+
```

### 1. Kernel Space (Ring 0 - Privilege)
- **Subsystems**: Memory Manager, Scheduler, Device Drivers, VFS Core, Net Core.
- **Privilege**: Direct access to hardware ports (`outb`/`inb`), page tables, CPU interrupt control registers, and physical disk sectors.
- **Isolation**: Executed in a flat identity-mapped segment, isolated from user task interference.

### 2. User Space (Ring 3 - Non-Privilege)
- **Subsystems**: Omni-Shell, user application shards, system packages, standard libraries.
- **Privilege**: No direct access to memory outside allocated task boundaries. No direct hardware I/O commands.
- **Isolation**: Each process runs in its own virtual address space managed by the Virtual Memory Manager.

---

## 📞 System Call (Syscall) Dispatcher

Communication across the kernel-userland boundary is strictly routed via the Sovereign Syscall Dispatcher (`SovereignSyscall.cpp`), mapped through the `int 0x80` or `syscall` CPU instructions:

| Syscall ID | Name | Source Parameter | Target Action |
|---|---|---|---|
| `0x01` | `sys_write` | String buffer, length | Writes data to the COM1 debug serial or VGA screen. |
| `0x02` | `sys_read` | Input buffer, maximum length | Reads input data from the keyboard queue. |
| `0x05` | `sys_socket` | Domain, type, protocol | Allocates a network socket handler. |
| `0x06` | `sys_pkg_install` | Package name, source | Downloads and registers a software package. |

---

## 🛡️ Boundary Enforcement Policies
1. **Memory Separation**: User space cannot read or write to memory belonging to the kernel. Violation triggers a Page Fault Exception, terminating the user thread.
2. **I/O Isolation**: Any hardware I/O port manipulation from Ring 3 results in a General Protection Fault.
3. **No Monolithic Bloat**: File systems (Ext4/FAT32) and protocol parsers are run inside isolated kernel services and exposed through clean, lightweight wrappers.

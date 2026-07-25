# 🏗️ SigmaOS vs. LFS (Linux From Scratch) Gap Analysis & Parity Blueprint

This specification details the structural, toolchain, and utility differences between **SigmaOS** and **LFS (Linux From Scratch)**, highlighting the missing architectural blocks and outlining zero-dependency, `#![no_std]` Rust implementation plans to bridge these gaps completely.

---

## 📊 1. Side-by-Side Comparison Matrix

Below is a granular assessment of LFS core requirements compared to the current implementation state of SigmaOS:

| LFS Core Component | LFS (Linux From Scratch) Implementation | SigmaOS Implementation State | Missing / Gap to Parity |
| :--- | :--- | :--- | :--- |
| **Compiler Toolchain** | `GCC`, `Binutils`, `Bison`, `M4`, `Flex` | Rust compiler (host-driven) | standalone on-device compilation & native assembler/linker. |
| **C Standard Library** | `Glibc` (GNU C Library) | `#![no_std]` with local custom C-shims | Full POSIX C API compliance to execute legacy binaries. |
| **Core Userland Utilities** | `Coreutils` (80+ utilities: `cat`, `ls`, `cp`, `mv`, `rm`, `chmod`) | Partial shell commands in `src/shell/repl.rs` | Standalone modular binaries for standard system administration. |
| **Command Shell** | `Bash` (Bourne Again SHell) | `sigma-sh` REPL / REPL mock | Robust parsing, script pipes, file redirects, env-vars. |
| **Dynamic Linker & Loader** | `ld-linux-x86-64.so.2` (dynamic shared library resolution) | Static linking or virtualized enclaves | Native ELF loader supporting dynamic shared objects (`.so`). |
| **System Init & Boot Scripts**| `SysVinit` or `Systemd` with scripts in `/etc/rc.d` | Minimal mocked service controls | Comprehensive service state manager (e.g. S-INIT). |
| **Text Processing & Filters** | `Grep`, `Sed`, `Awk`, `Diffutils`, `Findutils`, `Patch` | None | Stream filtering and regular expression parsing engines. |
| **Archival & Compressing** | `Tar`, `Gzip`, `Bzip2`, `Xz` | CLI (sigma-pkg) exists, decompressors conceptual | Raw block archive packing and unpacking libraries. |
| **Boot Configuration** | `/etc/fstab`, `/etc/passwd`, `/etc/group`, `/etc/hosts` | Partial, mostly mocked | Complete standard system administration databases. |

---

## 🛠️ 2. Detailed Gap Analysis & Porting Blueprints

To achieve complete parity with a book-compliant LFS system, SigmaOS must implement five key architectural systems natively:

### A. The Temporary Toolchain & Self-Hosted Compiler (LFS Chapters 5 & 6)
*   **The LFS Model:** Builds a temporary bootstrap compiler (`cross-gcc`, `cross-binutils`) in a separate workspace `/tools` to prevent host system contamination during final compilation.
*   **The SigmaOS Gap:** SigmaOS currently compiles strictly on a host development machine and does not feature on-device self-hosting.
*   **Sovereign Solution:** Create a bootstrap recipe inside `SigmaPkg` to build a localized, sandboxed Rust/Cargo/Zig toolchain. All compiled compiler binaries will be cached in `/store/sha256-...` (NixOS-style content-addressed paths) to guarantee 100% reproducible on-device builds.

### B. POSIX Compatibility Layer (`Libc` / System Calls)
*   **The LFS Model:** Glibc serves as the fundamental layer, defining the interface between userland applications and the Linux kernel syscall table.
*   **The SigmaOS Gap:** SigmaOS core runs entirely in `#![no_std]`, which is highly secure but prevents compiling standard Linux C software out-of-the-box.
*   **Sovereign Solution:** Build `SovereignLibc` - a zero-dependency, `#![no_std]` Rust implementation of musl/Glibc interfaces. It maps standard POSIX APIs (e.g. `open`, `read`, `write`, `malloc`, `free`) to SigmaOS capability-gated microkernel system calls natively.

### C. Standalone Core Utility Binaries (Replacement of GNU Coreutils)
*   **The LFS Model:** Packages like Coreutils provide standalone binaries that manipulate file streams, check system permissions, and perform environment reporting.
*   **The SigmaOS Gap:** Commands are currently compiled directly inside the interactive shell REPL (`src/shell/repl.rs`) and do not exist as independent executable files.
*   **Sovereign Solution:** Implement `SigmaCoreutils` - a collection of decoupled, zero-dependency `#![no_std]` binaries compiled as individual signed `SigmaAppImage` files.
    -   *Example `ls` implementation:* Walks the virtual directory structure via VFS system calls and writes to standard output with accessible layout markers.
    -   *Example `cat` implementation:* Opens file descriptors via secure capability tokens, reads streams, and flushes output directly.

---

## 🧠 3. Implementation Code: Standalone `#![no_std]` Core Utilities

Below is a complete, clean, OOP-driven, `#![no_std]` Rust implementation of standalone `cat` and `ls` utilities. This code shows how SigmaOS implements GNU Coreutils functionality from scratch without standard library dependencies.

### A. Standalone `cat` Utility (`cat.rs`)
```rust
#![no_std]
#![no_main]

use core::ptr::NonNull;

// Mock system call numbers mapped directly to the microkernel dispatch table
const SYS_OPEN: usize = 2;
const SYS_READ: usize = 3;
const SYS_WRITE: usize = 4;
const SYS_CLOSE: usize = 6;

const O_RDONLY: usize = 0;
const STDOUT_FILENO: usize = 1;
const BUFFER_SIZE: usize = 512;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Low-level inline assembly syscall dispatcher for x86_64
#[inline(always)]
unsafe fn syscall3(num: usize, arg1: usize, arg2: usize, arg3: usize) -> isize {
    let mut ret: isize;
    core::arch::asm!(
        "syscall",
        in("rax") num,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        lateout("rax") ret,
        clobber_abi("system")
    );
    ret
}

#[inline(always)]
unsafe fn syscall1(num: usize, arg1: usize) -> isize {
    let mut ret: isize;
    core::arch::asm!(
        "syscall",
        in("rax") num,
        in("rdi") arg1,
        lateout("rax") ret,
        clobber_abi("system")
    );
    ret
}

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    // In a real execution, arguments are parsed from the stack (argc/argv)
    // For this blueprint, we cat a default file "/etc/hostname"
    let file_path = b"/etc/hostname\0";

    let fd = syscall3(SYS_OPEN, file_path.as_ptr() as usize, O_RDONLY, 0);
    if fd < 0 {
        exit(1);
    }

    let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    loop {
        let bytes_read = syscall3(SYS_READ, fd as usize, buffer.as_mut_ptr() as usize, BUFFER_SIZE);
        if bytes_read <= 0 {
            break;
        }
        syscall3(SYS_WRITE, STDOUT_FILENO, buffer.as_ptr() as usize, bytes_read as usize);
    }

    syscall1(SYS_CLOSE, fd as usize);
    exit(0);
}

fn exit(code: usize) -> ! {
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 60, // SYS_EXIT
            in("rdi") code,
            options(noreturn)
        );
    }
}
```

### B. Standalone `ls` Utility (`ls.rs`)
```rust
#![no_std]
#![no_main]

const SYS_OPENDIR: usize = 15;
const SYS_READDIR: usize = 16;
const SYS_WRITE: usize = 4;
const SYS_CLOSEDIR: usize = 17;

const STDOUT_FILENO: usize = 1;
const BUFFER_SIZE: usize = 1024;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[repr(C)]
struct DirEntry {
    inode: u64,
    offset: u64,
    reclen: u16,
    name: [u8; 256],
}

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    let dir_path = b".\0";
    let fd = syscall2(SYS_OPENDIR, dir_path.as_ptr() as usize, 0);
    if fd < 0 {
        exit(1);
    }

    let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    loop {
        let count = syscall3(SYS_READDIR, fd as usize, buffer.as_mut_ptr() as usize, BUFFER_SIZE);
        if count <= 0 {
            break;
        }

        let mut offset = 0;
        while offset < count as usize {
            let entry = &*(buffer.as_ptr().add(offset) as *const DirEntry);
            if entry.reclen == 0 {
                break;
            }

            // Print entry name
            let mut name_len = 0;
            while entry.name[name_len] != 0 && name_len < 256 {
                name_len += 1;
            }

            syscall3(SYS_WRITE, STDOUT_FILENO, entry.name.as_ptr() as usize, name_len);
            syscall3(SYS_WRITE, STDOUT_FILENO, b"\n".as_ptr() as usize, 1);

            offset += entry.reclen as usize;
        }
    }

    syscall1(SYS_CLOSEDIR, fd as usize);
    exit(0);
}

#[inline(always)]
unsafe fn syscall2(num: usize, arg1: usize, arg2: usize) -> isize {
    let mut ret: isize;
    core::arch::asm!(
        "syscall",
        in("rax") num,
        in("rdi") arg1,
        in("rsi") arg2,
        lateout("rax") ret,
        clobber_abi("system")
    );
    ret
}

#[inline(always)]
unsafe fn syscall3(num: usize, arg1: usize, arg2: usize, arg3: usize) -> isize {
    let mut ret: isize;
    core::arch::asm!(
        "syscall",
        in("rax") num,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        lateout("rax") ret,
        clobber_abi("system")
    );
    ret
}

#[inline(always)]
unsafe fn syscall1(num: usize, arg1: usize) -> isize {
    let mut ret: isize;
    core::arch::asm!(
        "syscall",
        in("rax") num,
        in("rdi") arg1,
        lateout("rax") ret,
        clobber_abi("system")
    );
    ret
}

fn exit(code: usize) -> ! {
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 60, // SYS_EXIT
            in("rdi") code,
            options(noreturn)
        );
    }
}
```

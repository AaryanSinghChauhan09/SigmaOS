# gVisor-Inspired Linux Syscall Compatibility

## Overview

SigmaOS implements a **Linux syscall compatibility layer** (`kernel/linux_compat/`) inspired by the gVisor Sentry architecture (Apache-2.0). The goal is to run statically linked Linux ELF binaries on SigmaOS without modification. This is a **cleanroom implementation** — gVisor Go code is not copied; only the architectural approach (syscall translation table + ptrace-style interception) is studied.

---

## Architecture

```
Linux ELF binary (statically linked)
        │  execve()
        ▼
  sigma-linuxcompat loader (kernel/linux_compat/)
        │  ELF parsing → sigma process + address space
        │  redirect Linux syscall numbers → sigma equivalents
        ▼
  sigma kernel primitives (sigma_open, sigma_read, sigma_mmap, etc.)
```

The compatibility layer uses a **syscall translation table** that maps Linux x86_64 syscall numbers to SigmaOS internal IDs.

---

## File Layout

```
kernel/linux_compat/
├── README.md
├── sigma_linuxcompat.rs   # main compat layer
├── syscall_table.rs       # Linux → Sigma syscall map
├── elf_loader.rs          # ELF loader for Linux binaries
└── tests/
    └── hello_static.rs    # integration test
```

---

## Priority Syscall Table

```rust
// kernel/linux_compat/syscall_table.rs

/// Map Linux x86_64 syscall numbers to SigmaOS handlers.
pub static LINUX_SYSCALL_TABLE: &[(u64, &str, SigmaHandler)] = &[
    (0,   "read",    handle_read),
    (1,   "write",   handle_write),
    (2,   "open",    handle_open),
    (3,   "close",   handle_close),
    (9,   "mmap",    handle_mmap),
    (10,  "mprotect",handle_mprotect),
    (11,  "munmap",  handle_munmap),
    (12,  "brk",     handle_brk),
    (57,  "fork",    handle_fork),
    (59,  "execve",  handle_execve),
    (60,  "exit",    handle_exit),
    (61,  "wait4",   handle_wait4),
    (63,  "uname",   handle_uname),
    (102, "getuid",  handle_getuid),
    (158, "arch_prctl", handle_arch_prctl),
    (231, "exit_group", handle_exit_group),
];

type SigmaHandler = fn(&mut SyscallContext) -> i64;
```

---

## sigma_linuxcompat.rs (Stub)

```rust
//! SigmaOS Linux syscall compatibility layer.
//! Intercepts Linux syscalls and translates to SigmaOS primitives.

mod syscall_table;
mod elf_loader;

use syscall_table::LINUX_SYSCALL_TABLE;

pub struct SyscallContext {
    pub nr:   u64,          // Linux syscall number
    pub args: [u64; 6],     // rdi, rsi, rdx, r10, r8, r9
    pub ret:  i64,          // return value
}

/// Dispatch a Linux syscall number to the appropriate SigmaOS handler.
pub fn dispatch(ctx: &mut SyscallContext) {
    for (nr, name, handler) in LINUX_SYSCALL_TABLE {
        if *nr == ctx.nr {
            log::trace!("linux_compat: syscall {} ({})", ctx.nr, name);
            ctx.ret = handler(ctx);
            return;
        }
    }
    log::warn!("linux_compat: unimplemented syscall {}", ctx.nr);
    ctx.ret = -38; // ENOSYS
}

fn handle_read(ctx: &mut SyscallContext) -> i64 {
    let fd  = ctx.args[0] as i32;
    let buf = ctx.args[1] as *mut u8;
    let len = ctx.args[2] as usize;
    // Translate to sigma_read()
    sigma_fs::read(fd, buf, len) as i64
}

fn handle_write(ctx: &mut SyscallContext) -> i64 {
    let fd  = ctx.args[0] as i32;
    let buf = ctx.args[1] as *const u8;
    let len = ctx.args[2] as usize;
    sigma_fs::write(fd, buf, len) as i64
}

fn handle_exit(ctx: &mut SyscallContext) -> i64 {
    let code = ctx.args[0] as i32;
    sigma_process::exit(code);
}

// ... additional handlers for open, close, mmap, fork, execve, wait4
```

---

## ELF Loader

```rust
// kernel/linux_compat/elf_loader.rs (sketch)

pub fn load_linux_elf(path: &str) -> Result<u64, ElfError> {
    let data = sigma_fs::read_all(path)?;
    let elf  = goblin::elf::Elf::parse(&data)?;
    // Map PT_LOAD segments into sigma address space
    for ph in elf.program_headers.iter().filter(|p| p.p_type == PT_LOAD) {
        sigma_mm::mmap(ph.p_vaddr, ph.p_memsz as usize, ph.p_flags)?;
        sigma_mm::copy_to(ph.p_vaddr, &data[ph.p_offset as usize..][..ph.p_filesz as usize]);
    }
    Ok(elf.entry)
}
```

---

## Exit Criteria

- `sigma-run /bin/hello-static` (a statically linked `puts("Hello, Linux!")` binary) runs and prints `Hello, Linux!`.
- `sigma-run /bin/busybox-static ls /` lists the SigmaFS root directory.
- Unimplemented syscalls return `ENOSYS` gracefully without panicking the kernel.

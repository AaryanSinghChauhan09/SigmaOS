# SigmaOS System Call Table

> **Complete reference for all SigmaOS syscalls.** SigmaOS implements a sovereign
> syscall interface inspired by Linux, OpenBSD, and FreeBSD — but with additional
> security-focused syscalls (`sigma_pledge`, `sigma_unveil`, `sigma_jail`, `sigma_capset`).

---

## Syscall Invocation Convention (x86-64)

| Register | Role |
|----------|------|
| `rax` | Syscall number |
| `rdi` | Argument 1 |
| `rsi` | Argument 2 |
| `rdx` | Argument 3 |
| `r10` | Argument 4 |
| `r8` | Argument 5 |
| `r9` | Argument 6 |
| `rax` (return) | Return value / error (negative = errno) |

**Instruction**: `syscall` (AMD64) / `svc #0` (AArch64)

---

## File I/O Syscalls

### `sigma_open` — Open File

| Field | Value |
|-------|-------|
| **Number** | `1` |
| **Signature** | `sigma_open(path: *const u8, flags: u32, mode: u32) -> i64` |
| **Returns** | File descriptor (≥0) on success, negative errno on error |
| **Linux Equiv** | `openat(AT_FDCWD, path, flags, mode)` |
| **Security** | Checked against `unveil()` policy; blocked if path not unveiled |

```c
// Open for read
int fd = sigma_open("/etc/config", O_RDONLY, 0);

// Create for write
int fd = sigma_open("/tmp/out", O_WRONLY | O_CREAT | O_TRUNC, 0644);
```

---

### `sigma_read` — Read from File

| Field | Value |
|-------|-------|
| **Number** | `2` |
| **Signature** | `sigma_read(fd: i32, buf: *mut u8, count: usize) -> i64` |
| **Returns** | Bytes read (0 = EOF), negative errno on error |
| **Linux Equiv** | `read(2)` |

---

### `sigma_write` — Write to File

| Field | Value |
|-------|-------|
| **Number** | `3` |
| **Signature** | `sigma_write(fd: i32, buf: *const u8, count: usize) -> i64` |
| **Returns** | Bytes written, negative errno on error |
| **Linux Equiv** | `write(2)` |

---

### `sigma_close` — Close File Descriptor

| Field | Value |
|-------|-------|
| **Number** | `4` |
| **Signature** | `sigma_close(fd: i32) -> i64` |
| **Linux Equiv** | `close(2)` |

---

### `sigma_lseek` — Seek in File

| Field | Value |
|-------|-------|
| **Number** | `5` |
| **Signature** | `sigma_lseek(fd: i32, offset: i64, whence: i32) -> i64` |
| **Linux Equiv** | `lseek(2)` |

**whence values**: `SEEK_SET=0`, `SEEK_CUR=1`, `SEEK_END=2`

---

### `sigma_stat` — File Metadata

| Field | Value |
|-------|-------|
| **Number** | `6` |
| **Signature** | `sigma_stat(path: *const u8, statbuf: *mut SigmaStat) -> i64` |
| **Linux Equiv** | `stat(2)` / `fstatat(2)` |

---

### `sigma_ioctl` — Device Control

| Field | Value |
|-------|-------|
| **Number** | `7` |
| **Signature** | `sigma_ioctl(fd: i32, request: u64, arg: usize) -> i64` |
| **Linux Equiv** | `ioctl(2)` |
| **Security** | Only allowed on fds from unveiled device paths |

---

## Memory Management Syscalls

### `sigma_mmap` — Map Memory

| Field | Value |
|-------|-------|
| **Number** | `10` |
| **Signature** | `sigma_mmap(addr: usize, len: usize, prot: u32, flags: u32, fd: i32, offset: i64) -> i64` |
| **Returns** | Virtual address on success, `-ENOMEM` on error |
| **Linux Equiv** | `mmap(2)` |
| **Security** | W^X enforced — `PROT_WRITE | PROT_EXEC` rejected |

**prot flags**: `PROT_READ=1`, `PROT_WRITE=2`, `PROT_EXEC=4`, `PROT_NONE=0`

---

### `sigma_munmap` — Unmap Memory

| Field | Value |
|-------|-------|
| **Number** | `11` |
| **Signature** | `sigma_munmap(addr: usize, len: usize) -> i64` |
| **Linux Equiv** | `munmap(2)` |

---

### `sigma_mprotect` — Change Memory Protection

| Field | Value |
|-------|-------|
| **Number** | `12` |
| **Signature** | `sigma_mprotect(addr: usize, len: usize, prot: u32) -> i64` |
| **Linux Equiv** | `mprotect(2)` |
| **Security** | Cannot add EXEC to already-WRITE pages (W^X policy) |

---

## Process Syscalls

### `sigma_fork` — Fork Process

| Field | Value |
|-------|-------|
| **Number** | `20` |
| **Signature** | `sigma_fork() -> i64` |
| **Returns** | Child PID in parent, 0 in child, negative on error |
| **Linux Equiv** | `fork(2)` / `clone(2)` |
| **Security** | Child inherits pledge/unveil restrictions from parent |

---

### `sigma_exec` — Execute Program

| Field | Value |
|-------|-------|
| **Number** | `21` |
| **Signature** | `sigma_exec(path: *const u8, argv: *const *const u8, envp: *const *const u8) -> i64` |
| **Returns** | Does not return on success, negative errno on failure |
| **Linux Equiv** | `execve(2)` |
| **Security** | Path checked against unveil policy; ELF validated |

---

### `sigma_exit` — Terminate Process

| Field | Value |
|-------|-------|
| **Number** | `22` |
| **Signature** | `sigma_exit(status: i32) -> !` |
| **Linux Equiv** | `exit(2)` / `exit_group(2)` |

---

### `sigma_wait` — Wait for Child

| Field | Value |
|-------|-------|
| **Number** | `23` |
| **Signature** | `sigma_wait(pid: i32, status: *mut i32, options: i32) -> i64` |
| **Linux Equiv** | `waitpid(2)` |

---

### `sigma_kill` — Send Signal

| Field | Value |
|-------|-------|
| **Number** | `24` |
| **Signature** | `sigma_kill(pid: i32, sig: i32) -> i64` |
| **Linux Equiv** | `kill(2)` |
| **Security** | Pledge `proc` required to send signals to other processes |

---

### `sigma_getpid` / `sigma_getppid`

| Number | Signature | Linux Equiv |
|--------|-----------|-------------|
| `25` | `sigma_getpid() -> i64` | `getpid(2)` |
| `26` | `sigma_getppid() -> i64` | `getppid(2)` |

---

## Networking Syscalls

### `sigma_socket` — Create Socket

| Field | Value |
|-------|-------|
| **Number** | `40` |
| **Signature** | `sigma_socket(domain: i32, type_: i32, protocol: i32) -> i64` |
| **Linux Equiv** | `socket(2)` |
| **Security** | Pledge `inet` or `unix` required |

---

### `sigma_bind` — Bind Socket

| Field | Value |
|-------|-------|
| **Number** | `41` |
| **Signature** | `sigma_bind(fd: i32, addr: *const SockAddr, len: u32) -> i64` |
| **Linux Equiv** | `bind(2)` |

---

### `sigma_connect` — Connect Socket

| Field | Value |
|-------|-------|
| **Number** | `42` |
| **Signature** | `sigma_connect(fd: i32, addr: *const SockAddr, len: u32) -> i64` |
| **Linux Equiv** | `connect(2)` |

---

### `sigma_accept` — Accept Connection

| Field | Value |
|-------|-------|
| **Number** | `43` |
| **Signature** | `sigma_accept(fd: i32, addr: *mut SockAddr, len: *mut u32) -> i64` |
| **Linux Equiv** | `accept4(2)` |

---

### `sigma_send` / `sigma_recv`

| Number | Signature | Linux Equiv |
|--------|-----------|-------------|
| `44` | `sigma_send(fd, buf, len, flags) -> i64` | `send(2)` |
| `45` | `sigma_recv(fd, buf, len, flags) -> i64` | `recv(2)` |

---

## Security Syscalls

### `sigma_pledge` — Restrict Process Capabilities (OpenBSD-Inspired)

| Field | Value |
|-------|-------|
| **Number** | `100` |
| **Signature** | `sigma_pledge(promises: *const u8, execpromises: *const u8) -> i64` |
| **OpenBSD Equiv** | `pledge(2)` |
| **Source** | `src/pledge.rs`, `src/security/mac.rs` |

**Pledge promise strings**:

| Promise | Allows |
|---------|--------|
| `stdio` | Basic I/O (read, write, close, fstat) |
| `rpath` | Read-only filesystem access |
| `wpath` | Write-only filesystem access |
| `cpath` | File creation |
| `inet` | TCP/UDP network sockets |
| `unix` | Unix domain sockets |
| `proc` | fork, exec, kill |
| `exec` | execve only |
| `id` | UID/GID changes |
| `pf` | PF firewall rules |

```rust
// After initialization, restrict to stdio + rpath only
sigma_pledge("stdio rpath", "stdio");
// Now any syscall outside stdio/rpath causes SIGKILL
```

---

### `sigma_unveil` — Restrict Filesystem Visibility (OpenBSD-Inspired)

| Field | Value |
|-------|-------|
| **Number** | `101` |
| **Signature** | `sigma_unveil(path: *const u8, permissions: *const u8) -> i64` |
| **OpenBSD Equiv** | `unveil(2)` |
| **Source** | `src/security/mac.rs` |

**Permission strings**: `r` (read), `w` (write), `x` (exec), `c` (create)

```rust
sigma_unveil("/etc", "r");          // Allow reading /etc
sigma_unveil("/tmp", "rwc");        // Allow read/write/create in /tmp
sigma_unveil(NULL, NULL);           // Lock unveil — no more paths allowed
```

---

### `sigma_capset` — Set Capability Token

| Field | Value |
|-------|-------|
| **Number** | `102` |
| **Signature** | `sigma_capset(pid: i32, caps: *const CapabilitySet) -> i64` |
| **Linux Equiv** | `capset(2)` (extended) |
| **Source** | `src/security/capability.rs` |

Capability tokens restrict what kernel resources a process can access,
independently of UID (root is not special — capabilities are what matter).

---

### `sigma_jail` — Create Process Jail (FreeBSD-Inspired)

| Field | Value |
|-------|-------|
| **Number** | `103` |
| **Signature** | `sigma_jail(params: *const JailParams) -> i64` |
| **FreeBSD Equiv** | `jail(2)` |
| **Source** | `src/security/mac.rs`, `src/container/` |

Creates a new jail with isolated filesystem root, PID namespace, and network namespace.

```rust
let params = JailParams {
    path: "/var/jail/myjail\0".as_ptr(),
    hostname: "myjail\0".as_ptr(),
    ip4addr: [10, 0, 0, 1],
    flags: JAIL_DYING | JAIL_ATTACH,
};
sigma_jail(&params);
```

---

## IPC Syscalls

| Number | Syscall | Description | Linux Equiv |
|--------|---------|-------------|-------------|
| `60` | `sigma_pipe(fds)` | Create pipe pair | `pipe2(2)` |
| `61` | `sigma_eventfd(init, flags)` | Event notification fd | `eventfd(2)` |
| `62` | `sigma_futex(uaddr, op, val, ...)` | Fast user-space mutex | `futex(2)` |
| `63` | `sigma_shm_open(name, flags, mode)` | Shared memory | `shm_open(3)` |
| `64` | `sigma_shm_unlink(name)` | Remove shared memory | `shm_unlink(3)` |

---

## Kernel/System Syscalls

| Number | Syscall | Description | Linux Equiv |
|--------|---------|-------------|-------------|
| `80` | `sigma_sysinfo(info)` | System statistics | `sysinfo(2)` |
| `81` | `sigma_uname(buf)` | OS name/version | `uname(2)` |
| `82` | `sigma_sysctl(name, namelen, old, oldlen, new, newlen)` | Kernel tunables | `sysctl(2)` |
| `83` | `sigma_kexec(img, flags)` | Load new kernel | `kexec_load(2)` |
| `84` | `sigma_reboot(magic, cmd)` | Reboot/halt | `reboot(2)` |

---

## Error Codes

| Code | Name | Meaning |
|------|------|---------|
| `-1` | `EPERM` | Operation not permitted |
| `-2` | `ENOENT` | No such file or directory |
| `-4` | `EINTR` | Interrupted system call |
| `-9` | `EBADF` | Bad file descriptor |
| `-12` | `ENOMEM` | Out of memory |
| `-13` | `EACCES` | Permission denied |
| `-22` | `EINVAL` | Invalid argument |
| `-38` | `ENOSYS` | Function not implemented |
| `-100` | `EPLEDGE` | Pledge violation (SigmaOS-specific) |
| `-101` | `EUNVEIL` | Unveil violation (SigmaOS-specific) |
| `-102` | `ECAP` | Capability violation (SigmaOS-specific) |
| `-103` | `EJAIL` | Jail operation error (SigmaOS-specific) |

---

## Implementation

All syscalls are dispatched through `src/kernel/syscall/table.rs`:

```rust
pub fn dispatch(nr: u64, args: &SyscallArgs) -> i64 {
    match nr {
        1  => sys_open(args),
        2  => sys_read(args),
        3  => sys_write(args),
        // ...
        100 => sys_pledge(args),
        101 => sys_unveil(args),
        102 => sys_capset(args),
        103 => sys_jail(args),
        _   => -38, // ENOSYS
    }
}
```

Security checks run **before** dispatch for pledge/unveil violations:

```rust
fn check_pledge(task: &Task, nr: u64) -> Result<(), i64> {
    if task.pledge_promises.allows_syscall(nr) {
        Ok(())
    } else {
        // Send SIGKILL — pledge violation is fatal (OpenBSD behavior)
        task.signal(SIGKILL);
        Err(-100) // EPLEDGE
    }
}
```

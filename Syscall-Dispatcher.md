# Syscall Dispatcher

The SigmaOS Syscall Dispatcher (`SovereignSyscall.cpp`) is the sole communication channel between Ring 3 userland and Ring 0 kernel space. Every system call flows through it — no direct kernel function calls from user processes.

---

## Mechanism

Userland code triggers a syscall using either `int 0x80` (legacy 32-bit compatibility) or the `syscall` instruction (x86_64 fast path):

```asm
; Userland: call sigma_write(1, "Hello", 5)
mov rax, 0x01      ; syscall number: sys_write
mov rdi, 1         ; fd = 1 (stdout)
mov rsi, msg       ; buffer pointer
mov rdx, 5         ; length
syscall            ; transfer to Ring 0
```

On `syscall`, the CPU:
1. Saves `rip` and `rflags` in `rcx` and `r11`.
2. Loads the kernel `rip` from the `LSTAR` MSR (set during `sigma_idt_init`).
3. Switches to Ring 0 and the kernel stack.

The dispatcher then:
1. Reads `rax` (syscall number).
2. Validates it is within the table bounds.
3. Looks up the handler via `sigma_syscall_table[rax]` — an O(1) function pointer dereference.
4. Calls the handler with `rdi`, `rsi`, `rdx`, `r10`, `r8`, `r9` as arguments (Linux ABI).
5. Returns the result in `rax` and restores user context via `sysretq`.

---

## Syscall Table

```c
// syscalls.h
#define SYSCALL_WRITE       0x01
#define SYSCALL_READ        0x02
#define SYSCALL_OPEN        0x03
#define SYSCALL_CLOSE       0x04
#define SYSCALL_SOCKET      0x05
#define SYSCALL_PKG_INSTALL 0x06
#define SYSCALL_GETPID      0x07
#define SYSCALL_SPAWN       0x08
#define SYSCALL_MMAP        0x09
#define SYSCALL_MUNMAP      0x0A
#define SYSCALL_EXIT        0x0B
```

The table is a fixed-size array of function pointers:

```c
// dispatcher.c
static sigma_syscall_fn sigma_syscall_table[] = {
    [SYSCALL_WRITE]       = sys_write_handler,
    [SYSCALL_READ]        = sys_read_handler,
    [SYSCALL_OPEN]        = sys_open_handler,
    [SYSCALL_CLOSE]       = sys_close_handler,
    [SYSCALL_SOCKET]      = sys_socket_handler,
    [SYSCALL_PKG_INSTALL] = sys_pkg_install_handler,
    [SYSCALL_GETPID]      = sys_getpid_handler,
    [SYSCALL_SPAWN]       = sys_spawn_handler,
    [SYSCALL_MMAP]        = sys_mmap_handler,
    [SYSCALL_MUNMAP]      = sys_munmap_handler,
    [SYSCALL_EXIT]        = sys_exit_handler,
};
```

An out-of-range syscall number returns `-ENOSYS` (`-38`) without consulting the table.

---

## Syscall Reference

### `sys_write` (0x01)

```c
sigma_u64 sys_write(sigma_i32 fd, const void* buf, sigma_size_t count);
```

Writes `count` bytes from `buf` to the file descriptor `fd`.

- `fd = 1` (stdout): Writes to the VGA text console.
- `fd = 2` (stderr): Writes to the COM1 serial debug output.
- Any other fd: Routes to the VFS write callback for the open file.

Returns bytes written on success, `-EBADF` if `fd` is not open, `-EFAULT` if `buf` is not in valid user memory.

---

### `sys_read` (0x02)

```c
sigma_u64 sys_read(sigma_i32 fd, void* buf, sigma_size_t count);
```

Reads up to `count` bytes into `buf` from file descriptor `fd`.

- `fd = 0` (stdin): Blocks until keyboard input is available, then reads from the PS/2 keyboard buffer.
- Any other fd: Routes to the VFS read callback.

Returns bytes read on success, `0` on EOF, `-EINTR` if interrupted by a signal.

---

### `sys_socket` (0x05)

```c
sigma_i32 sys_socket(sigma_i32 domain, sigma_i32 type, sigma_i32 protocol);
```

Allocates a socket. See [Networking](Networking) for the full socket API.

---

### `sys_spawn` (0x08)

```c
sigma_i32 sys_spawn(const char* path, const char** argv, const char** envp);
```

Creates a new process. The kernel:
1. Allocates a new task struct and virtual address space.
2. Loads the ELF binary from `path`.
3. Sets up the user stack with `argv` and `envp`.
4. Adds the task to the scheduler run queue.

Returns the new process's PID on success, negative error code on failure.

---

### `sys_mmap` (0x09)

```c
void* sys_mmap(void* addr, sigma_size_t length, sigma_i32 prot, sigma_i32 flags, sigma_i32 fd, sigma_u64 offset);
```

Maps memory into the process's virtual address space.

- `prot`: Combination of `PROT_READ`, `PROT_WRITE`, `PROT_EXEC`.
- `flags`: `MAP_ANONYMOUS` (no file backing), `MAP_PRIVATE`, `MAP_SHARED`.
- Returns: Virtual address of the mapping, or `(void*)-1` on failure.

---

### `sys_exit` (0x0B)

```c
[[noreturn]] void sys_exit(sigma_i32 status);
```

Terminates the calling process. The kernel:
1. Frees all pages in the process's address space.
2. Closes all open file descriptors.
3. If the process is a child, delivers `SIGCHLD` to the parent.
4. Marks the task as a zombie until the parent calls `wait`.

This syscall never returns.

---

## Adding a New Syscall

1. Add a `#define SYSCALL_MYOP 0xNN` to `syscalls.h`.
2. Implement `sys_myop_handler` in the appropriate kernel source file.
3. Add it to `sigma_syscall_table` in `dispatcher.c`.
4. Expose a C wrapper in `lib/libc/sigma_libc.c`:

```c
sigma_i32 sigma_myop(sigma_i32 arg) {
    return (sigma_i32)__syscall(SYSCALL_MYOP, arg);
}
```

5. Declare the prototype in `sigma_libc.h`.
6. Write a test in `tests/` and verify it passes: `npm run test`.

---

*See also: [Kernel](Kernel) · [Architecture Overview](Architecture-Overview) · [API Reference](API-Reference)*

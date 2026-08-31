# Sovereign Process, ABI & Terminal Architecture (Linux & BSD Inspirations)

## Overview & Philosophy
SigmaOS combines the process isolation of OpenBSD, the storage topology and networking stack of FreeBSD, the portability of NetBSD, the performance scheduling of CachyOS/Zen, and the declarative reproducibility of NixOS into a single, unified `#![no_std]` Rust kernel ecosystem.

---

## 1. Process Management & State Machine (`src/process/sovereign_process_engine.rs`)

Inspired by Linux `task_struct`, POSIX process states, and BSD Ksh/csh job control:

### 1.1 Execution States
- **Ready**: Enqueued in high-performance priority scheduler.
- **Running**: Active CPU execution frame.
- **BackgroundRunning**: Ksh/BSD background execution job (`pgid`), disowned from interactive TTY input.
- **Waiting**: Non-blocking waiting state during I/O stream drainage.
- **Aborted**: Immediate termination on unhandled signal or security violation.
- **Cancelled**: Graceful cancellation signal dispatched to execution context.
- **Terminated(i32)**: Clean exit with status code recorded in kernel process table.

### 1.2 Non-Blocking Stream I/O & Buffering
- Separate `stdin_buffer`, `stdout_buffer`, and `stderr_buffer` byte vectors per process.
- Non-blocking read (`sovereign_read`) and write (`sovereign_write`) primitives avoiding thread block stalls.

### 1.3 Zero-Copy IPC Channels
- Ring buffer IPC architecture (`ZeroCopyIpcChannel`) supporting zero-copy message transfers.
- 64KB capacity limit (`capacity_bytes = 65536`) to protect physical frame allocation.
- Atomic event notification tracking (`event_notifications_count`).

---

## 2. Cross-Kernel ABI & Syscall Alignment (`src/compatibility/abi_translator.rs`, `src/compatibility/abi_extended.rs`)

### 2.1 Linux vDSO (Virtual Dynamic Shared Object) Fast Syscalls
- Maps virtual page addresses for high-frequency userland system calls:
  - `clock_gettime` / `__vdso_clock_gettime` (`0x7fff_ffff_f000`)
  - `gettimeofday` / `__vdso_gettimeofday` (`0x7fff_ffff_f200`)
  - `time` / `__vdso_time` (`0x7fff_ffff_f400`)
  - `getcpu` / `__vdso_getcpu` (`0x7fff_ffff_f600`)

### 2.2 ELF Auxiliary Vectors (`auxv`)
- Constructs binary execution auxiliary vectors (`ElfAuxVectorEntry`) for glibc, musl, and FreeBSD `ld-elf.so` dynamic loaders.

### 2.3 System Calling Conventions
- Translates register param layouts across AMD64 System V, Windows x64, ARM64 AAPCS, and RISC-V 64-bit calling conventions.

---

## 3. Pseudo-Terminal (PTY) & Line Discipline (`src/shell/terminal_emulator.rs`)

### 3.1 PTY Session Control (`PtySessionController`)
- Handles master/slave file descriptors (`master_fd`, `slave_fd`).
- Signal propagation (`SIGINT`, `SIGTSTP`, `SIGQUIT`, `SIGWINCH`, `SIGHUP`) directly to foreground process group (`foreground_pgid`).
- Raw and Canonical termios line discipline toggle (`set_raw_mode`).

### 3.2 Visual Bell & TrueColor Palette
- DEC private mode and OSC escape code parsing.
- BSD console themes: Dracula, Nord, Solarized, Gruvbox, VT100 classic.

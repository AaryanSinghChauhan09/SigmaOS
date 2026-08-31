# Sovereign Linux & BSD Distribution Parity & Architectural Absorption Specification

## Executive Summary & Vision
SigmaOS achieves sovereign OS supremacy by studying, absorbing, and transcending the foundational design paradigms of leading Linux distributions and BSD operating systems. Through zero-dependency, bare-metal Rust implementation (`#![no_std]`), SigmaOS unifies Linux process management, OpenBSD pledge/unveil sandboxing, FreeBSD ULE scheduling & GEOM storage topology, NetBSD Rump hypercalls, and Gentoo Portage USE flag masking engines.

---

## 1. Process & Subsystem Execution Paradigms (Linux & BSD Inspirations)

### 1.1 Process Lifecycle & State Machine
SigmaOS processes transition through POSIX-compliant and BSD-extended state models managed by `SovereignProcessManager`:
- **Ready**: Process initialized in process tree, waiting for CPU allocation.
- **Running**: Active execution in foreground process group (`pgid`).
- **BackgroundRunning**: Ksh/BSD-style job control background execution (`is_foreground = false`).
- **Waiting**: Non-blocking waiting on I/O streams or child process events.
- **Aborted / Cancelled**: Immediate signal abortion or soft cancellation.
- **Terminated(i32)**: Clean exit with return code tracking.

### 1.2 Zero-Copy Inter-Process Communication (IPC)
Inspired by Linux `splice(2)`, `vmsplice(2)`, and FreeBSD zero-copy socket buffers:
- Ring-buffer based IPC channels (`ZeroCopyIpcChannel`) with fixed 64KB atomic page frames.
- Overflow bounds enforcement (`capacity_bytes = 65536`) to prevent memory exhaustion.
- Event notification counters tracking asynchronous packet dispatch without kernel-user context switches.

---

## 2. Cross-Kernel ABI & Syscall Alignment

### 2.1 Linux vDSO (Virtual Dynamic Shared Object) Integration
- Fast user-space syscall mapping (`LinuxVdsoTableMap`) eliminating kernel trap overhead for time-critical calls:
  - `clock_gettime` / `__vdso_clock_gettime` (`0x7fff_ffff_f000`)
  - `gettimeofday` / `__vdso_gettimeofday` (`0x7fff_ffff_f200`)
  - `time` / `__vdso_time` (`0x7fff_ffff_f400`)
  - `getcpu` / `__vdso_getcpu` (`0x7fff_ffff_f600`)

### 2.2 ELF Auxiliary Vector (`auxv`) Engine
- Generates `ElfAuxVectorEntry` mappings (`AT_PHDR`, `AT_PHENT`, `AT_PHNUM`, `AT_PAGESZ`, `AT_BASE`, `AT_FLAGS`, `AT_ENTRY`, `AT_UID`, `AT_EUID`, `AT_GID`, `AT_EGID`, `AT_SYSINFO_EHDR`) required by glibc, musl, and FreeBSD `ld-elf.so`.

---

## 3. Terminal Emulation & PTY Session Controls

### 3.1 Pseudo-Terminal (PTY) Controller (`PtySessionController`)
- Master/Slave PTY pair management (`master_fd`, `slave_fd`).
- Foreground process group signal propagation (`SIGINT`, `SIGTSTP`, `SIGQUIT`, `SIGWINCH`, `SIGHUP`).
- Termios line discipline supporting Raw, Canonical, and Cbreak modes.

### 3.2 Visual & Color Palette Innovations
- TrueColor 24-bit RGB escape sequence parsing.
- Theme presets inspired by BSD Console & Xterm: Dracula, Nord, Solarized, Monokai, Gruvbox, VT100 classic.
- Sixel and Kitty graphics protocol handling with visual bell flashes.

---

## 4. Distro Absorption Matrix

| Distribution / OS | Feature Absorbed | SigmaOS Implementation Component |
| :--- | :--- | :--- |
| **OpenBSD** | `pledge` & `unveil` Sandboxing | `src/security/sigma_unveil.rs`, `src/pledge.rs` |
| **FreeBSD** | GEOM Topology & VNET Stack | `src/storage/geom.rs`, `src/network/` |
| **NetBSD** | Rump Kernel Hypercalls | `src/compatibility/netbsd.rs` |
| **Gentoo** | Portage USE Flags & Package Masking | `src/sigpkg/gentoo_use_flags.rs`, `src/unimplemented_features.rs` |
| **CachyOS / Zen** | BORE & EEVDF Interactive Scheduler | `src/kernel/bore.rs`, `src/kernel/scheduler.rs` |
| **Fedora** | TPM 2.0 PCR Measured Boot & IMA Logs | `src/boot/firmware.rs`, `src/unimplemented_features.rs` |
| **Clear Linux** | Stateless `/usr` Overlay Manager | `src/unimplemented_features.rs` |
| **Kali Linux** | Anonsurf Anonymous Routing & Killswitch | `src/unimplemented_features.rs` |

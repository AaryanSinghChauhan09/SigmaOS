# SigmaOS Zenith: Core Subsystems & Open-Source Inspirations (v15.2)

To transform SigmaOS Zenith into an industrial-grade sovereign microkernel, we have absorbed and synthesized critical kernel subsystems from leading open-source operating systems and network protocols.

These subsystems operate entirely within the C11 standalone environment, ensuring absolute silicon-direct execution without reliance on POSIX bloat.

---

## 1. Inter-Process Communication (IPC)
**Inspirations:** Linux `ipc/`, POSIX IPC, GNU Hurd `mach_msg()`
**Implementation:** `kernel/core/ipc/sigma_ipc.c`

SigmaOS provides a deterministic IPC lattice featuring:
* **Message Queues:** Asynchronous, typed message passing.
* **Shared Memory:** Zero-copy memory regions with explicit lifecycle attachment.
* **Counting Semaphores:** Hardware-backed concurrency primitives.

## 2. Unix-Style Pipes
**Inspirations:** Linux `fs/pipe.c`, FreeBSD `sys_pipe.c`, Plan 9 Channels
**Implementation:** `kernel/core/ipc/sigma_pipe.c`

A rigorous byte-stream pipe implementation allowing producer-consumer execution chains between sovereign shards, featuring automatic EOF handling and SIGPIPE broken-pipe detection.

## 3. Signal Handling
**Inspirations:** Linux `kernel/signal.c`, OpenBSD W^X Hardening
**Implementation:** `kernel/core/process/sigma_signal.c`

A highly secure, masked signal delivery system. By default, it enforces strict POSIX constraints (e.g., `SIGKILL` and `SIGSTOP` cannot be caught or blocked) while allowing shards to register isolated exception handlers.

## 4. Hardware Watchdog Timer
**Inspirations:** Linux `watchdog_core.c`, Intel `iTCO_wdt`, systemd watchdog
**Implementation:** `kernel/core/diag/sigma_watchdog.c`

A critical fail-safe for autonomous edge and IoT deployments. The kernel must actively "pet" the watchdog within the configured timeout, or the hardware will trigger an unmaskable system reset. Supports `CONFIG_WATCHDOG_NOWAYOUT` to prevent malicious disablement.

## 5. ProcFS Virtual Filesystem
**Inspirations:** Linux `fs/proc/`, Plan 9 Synthetic FS, FreeBSD `linprocfs`
**Implementation:** `fs/procfs.c`

Exposes the internal runtime state of the Zenith microkernel via a readable synthetic filesystem. Modules include:
* `/proc/cpuinfo`: Hardware capabilities, cache layouts, and active flags.
* `/proc/meminfo`: Slab allocation, paging, and available physical memory.
* `/proc/loadavg` & `/proc/uptime`: Kernel scheduling metrics.

## 6. Kernel Ring Buffer (dmesg)
**Inspirations:** Linux `printk`, FreeBSD `msgbuf`
**Implementation:** `kernel/core/diag/sigma_klog.c`

A highly efficient, lock-free circular buffer for kernel diagnostics. Supports Syslog-compatible severity levels (EMERG, ALERT, CRIT, ERR, WARN, NOTICE, INFO, DEBUG) and tracks dropped packets during extreme load spikes.

## 7. Network: ARP & DHCP Client
**Inspirations:** Linux `net/ipv4/arp.c`, systemd-networkd, RFC 2131/826
**Implementation:** `net/arp.c`, `net/dhcp.c`

SigmaOS abandons user-space daemon reliance, migrating essential network bootstrapping directly into the kernel:
* **ARP Resolver:** Maintains reachable, stale, and delayed state machines for Ethernet-to-IPv4 address resolution.
* **DHCP Engine:** Executes the strict DORA (Discover, Offer, Request, Acknowledge) handshake to lease IP configurations autonomously on boot.

## 8. Sovereign Environment Variable Registry
**Inspirations:** glibc `setenv.c`, musl libc `src/env/`
**Implementation:** `usr/sigma_env.c`

A high-performance registry mapping essential runtime environments for execution domains. Pre-populated with core paths, system architecture declarations, and terminal capabilities.

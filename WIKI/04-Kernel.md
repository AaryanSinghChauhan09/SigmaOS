# Kernel

SigmaOS kernel is a microkernel-inspired design with modular subsystems, implementing W^X memory hardening, EEVDF/CFS scheduling, UTS namespace isolation, Control Flow Integrity (CFI), and kernel pointer restriction (kptr_restrict).

## Kernel Architecture

### Core Components

- **Task Scheduler**: CPU scheduling with EEVDF (Earliest Eligible Virtual Deadline First) and CFS fallback, RT, and IDLE policies
- **Memory Management**: Buddy allocator, slab allocator, transparent huge pages, W^X enforcement
- **IPC**: Inter-process communication with shared memory, message queues, and HelenOS-inspired async IPC
- **VFS**: Virtual filesystem layer with pluggable filesystem drivers
- **Security**: Capability-based security (Capsicum-inspired), pledge/unveil sandboxing, CFI forward-edge validation
- **Namespaces**: UTS, PID, mount, network, user, and IPC namespace isolation
- **Drivers**: Hardware abstraction layer for device drivers

## Kernel Modules

### Loading Modules

Load kernel modules dynamically:

```bash
# Load module
sigmod load module-name

# Unload module
sigmod unload module-name

# List loaded modules
sigmod list

# Show module information
sigmod info module-name
```

### Kernel Parameters

View and modify kernel parameters:

```bash
# View all parameters
sysctl -a

# View specific parameter
sysctl kernel.hostname

# Set parameter
sysctl kernel.hostname="new-hostname"

# Persist parameter
echo "kernel.hostname=new-hostname" >> /etc/sysctl.conf
```

## Process Management

### Process Monitoring

View process information:

```bash
# List all processes
sigps aux

# Show process tree
sigps tree

# Show process details
sigps show pid
```

### Process Control

```bash
# Kill process
sigkill pid

# Set process priority
renice priority pid

# Run process with nice level
nice -n 10 command
```

## Memory Management

### Memory Information

View memory usage:

```bash
# Show memory statistics
sigmem stat

# Show memory map for process
sigmem map pid

# Show swap usage
sigmem swap
```

### Transparent Huge Pages

Configure huge pages:

```bash
# Enable huge pages
echo 1 > /proc/sys/vm/nr_hugepages

# Set huge page size
echo 2048 > /proc/sys/vm/hugepages_treat_as
```

## Filesystem Encryption

### fscrypt

Enable per-directory encryption:

```bash
# Create encrypted directory
fscrypt encrypt /path/to/directory

# Lock directory
fscrypt lock /path/to/directory

# Unlock directory
fscrypt unlock /path/to/directory
```

### AutoFS

Configure on-demand mounting:

```bash
# Register mount trigger
autofs register /mnt/data /dev/sda1 ext4

# Trigger mount
autofs trigger /mnt/data

# Configure idle timeout
autofs set-timeout /mnt/data 300
```

## Security Mitigations

See dedicated sections above for [W^X Memory Hardening](#wx-memory-hardening), [CFI](#control-flow-integrity-cfi), and [kptr_restrict](#kernel-pointer-restriction-kptr_restrict).

### Kernel Pointer Restriction (Quick Reference)

Restrict kernel pointer exposure:

```bash
# Set kptr_restrict level
sysctl kernel.kptr_restrict=2

# Set dmesg_restrict level
sysctl kernel.dmesg_restrict=1
```

### Module Loading Control

Control kernel module loading:

```bash
# Disable module loading
sysctl kernel.modules_disabled=1

# Enable module loading
sysctl kernel.modules_disabled=0
```

## Device Drivers

### PCI/PCIe

Manage PCIe devices:

```bash
# List PCIe devices
sigpci list

# Scan for new devices
sigpci scan

# Configure device
sigpci configure bus:device:function
```

### NVMe

Manage NVMe storage:

```bash
# List NVMe devices
sgnvme list

# Format NVMe device
sgnvme format /dev/nvme0n1

# Show NVMe SMART data
sgnvme smart /dev/nvme0n1
```

## Kernel Debugging

### System Logs

View kernel logs:

```bash
# View kernel messages
dmesg

# View systemd journal
journalctl -k

# Follow kernel logs
journalctl -kf
```

### Kernel Debugging

Enable kernel debugging:

```bash
# Enable debug symbols
echo 1 > /proc/sys/kernel/sysrq

# Trigger sysrq
echo t > /proc/sysrq-trigger
```

## Linux-Compatible Syscalls

### Syscall Compatibility

Linux syscall compatibility layer:

```bash
# Use Linux syscalls directly
# via compatibility layer

# Standard Linux syscalls supported:
# - read, write, open, close
# - fork, clone, execve, exit
# - mkdir, rmdir, unlink, chmod
# - getpid, getppid
# - And many more
```

## Sysfs - Kernel Parameters

### Sysfs Operations

Linux sysfs-inspired kernel parameter management:

```bash
# Create kobject
sysfs create-kobject /kernel

# Add attribute
sysfs add-attr hostname sigmaos

# Read attribute
sysfs read /kernel/hostname

# Write attribute
sysfs write /kernel/hostname newhost

# List children
sysfs list /kernel
```

### Standard Parameters

- **kernel.hostname**: System hostname
- **kernel.osrelease**: OS release version
- **kernel.version**: Kernel version
- **vm.swappiness**: VM swappiness parameter
- **vm.dirty_ratio**: Dirty page ratio
- **net.ipv4.ip_forward**: IP forwarding

## Process Scheduler

### EEVDF Scheduler

SigmaOS implements EEVDF (Earliest Eligible Virtual Deadline First), the scheduler introduced in Linux 6.6 as the primary replacement for CFS. EEVDF provides better latency and fairness guarantees by tracking per-task virtual deadlines.

Key properties:
- **Eligibility**: A task is eligible when its virtual start time ≤ current virtual time
- **Virtual Deadline**: `vdeadline = vstart + (slice / weight)`
- **Lag tracking**: Unused service credit is tracked as lag to prevent starvation

```bash
# View current scheduler policy for a process
cat /proc/<pid>/sched

# Set SCHED_DEADLINE policy (EEVDF-backed)
chrt --deadline --sched-runtime 5000000 --sched-deadline 10000000 --sched-period 10000000 -p <pid>
```

### CFS Scheduler

Linux CFS-inspired Completely Fair Scheduler (fallback for non-deadline tasks):

```bash
# Create process
scheduler create normal 0

# Pick next task
scheduler pick-next

# Update vruntime
scheduler update-vruntime 1 1000

# Put task to sleep
scheduler sleep 1

# Wake up task
scheduler wake 1
```

### Scheduler Types

| Policy | Description | Use Case |
|--------|-------------|----------|
| `SCHED_EEVDF` | Virtual deadline earliest-eligible | Default; latency-sensitive tasks |
| `SCHED_CFS` | vruntime-based fair scheduling | CPU-bound background tasks |
| `SCHED_RT` | Priority-based real-time scheduling | Interrupt handlers, audio |
| `SCHED_IDLE` | Lowest priority idle tasks | Maintenance background work |
| `SCHED_DEADLINE` | CBS/EDF hard real-time | Deterministic timing requirements |

### Energy-Aware Scheduling

```bash
# Update thermal state
scheduler thermal-state 85

# Get CPU frequency
scheduler frequency

# Get thermal state
scheduler thermal
```

## W^X Memory Hardening

W^X (Write XOR Execute) ensures no memory page is simultaneously writable and executable. This mitigates JIT-spray and code-injection attacks.

### Enforcement

- All memory mappings default to either writable (`PROT_WRITE`) or executable (`PROT_EXEC`), never both
- Kernel enforces W^X at page-table level on all user and kernel mappings
- JIT compilers must use a write-then-mprotect pattern: allocate RW, write code, mprotect to RX before execution

```bash
# Verify W^X enforcement is active
sysctl kernel.wx_enforce

# Example: check process memory map for WX pages (should be empty)
cat /proc/<pid>/maps | awk '$2 ~ /wx/ {print "WARNING: WX page:", $0}'
```

### Configuration

```bash
# Enable strict W^X (blocks mmap(PROT_WRITE|PROT_EXEC))
sysctl kernel.wx_strict=1

# Log W^X violations without blocking (audit mode)
sysctl kernel.wx_strict=0
sysctl kernel.wx_audit=1
```

## UTS Namespaces

UTS (Unix Time-sharing System) namespaces isolate `hostname` and `domainname` per container or process group, enabling container isolation without affecting the host.

```bash
# Create new UTS namespace
unshare --uts /bin/bash

# Set hostname inside namespace
hostname container-hostname

# Verify isolation
hostname  # shows container-hostname
# (host hostname is unchanged)
```

### Namespace Hierarchy

SigmaOS supports the full Linux namespace suite:

| Namespace | Flag | Isolates |
|-----------|------|----------|
| UTS | `CLONE_NEWUTS` | Hostname, NIS domain name |
| PID | `CLONE_NEWPID` | Process IDs |
| Mount | `CLONE_NEWNS` | Filesystem mount points |
| Network | `CLONE_NEWNET` | Network interfaces, routing |
| User | `CLONE_NEWUSER` | UID/GID mappings |
| IPC | `CLONE_NEWIPC` | SysV IPC, POSIX MQ |
| Cgroup | `CLONE_NEWCGROUP` | Cgroup root |

## Control Flow Integrity (CFI)

CFI prevents control-flow hijacking by validating indirect call targets against a compile-time allowlist of valid function addresses.

### Forward-Edge CFI

Forward-edge CFI validates all indirect function calls (`call *rax`, vtable dispatches):

```bash
# Register valid CFI target (function pointer allowlisting)
cfi register 0x1000 function_name

# Validate indirect call — returns true if target is allowlisted
cfi validate 0x2000 0x1000

# View CFI violations log
cfi violations

# Clear violations
cfi reset
```

### CFI Build Requirements

Kernel and drivers compiled with `-fsanitize=cfi -flto` (LLVM CFI). Requires:
- LTO (Link-Time Optimization) for cross-module type graph
- `-fvisibility=hidden` to prevent CFI bypass via exported symbols

## Kernel Pointer Restriction (kptr_restrict)

`kptr_restrict` controls whether kernel addresses are exposed to unprivileged users via `/proc/kallsyms`, `dmesg`, and similar interfaces.

| Level | Behavior |
|-------|----------|
| `0` | No restriction — all pointers visible (debug only) |
| `1` | Pointers hidden from non-CAP_SYSLOG processes |
| `2` | All kernel pointers replaced with `0` for all users |

```bash
# Recommended hardened setting
sysctl kernel.kptr_restrict=2

# Also restrict dmesg to privileged users
sysctl kernel.dmesg_restrict=1

# Persist across reboots
echo "kernel.kptr_restrict=2" >> /etc/sysctl.d/99-hardening.conf
echo "kernel.dmesg_restrict=1" >> /etc/sysctl.d/99-hardening.conf
sysctl --system
```

## Interrupt Handling

### Interrupt Controller

Linux and BSD-inspired interrupt handling:

```bash
# Allocate interrupt vector
interrupt allocate irq

# Register handler
interrupt register 32 handler

# Enable interrupt
interrupt enable 32

# Handle interrupt
interrupt handle 32

# Register IRQ
interrupt register-irq 1 edge

# Enable IRQ
interrupt enable-irq 1
```

### Interrupt Types

- **Exception**: Processor exceptions
- **IRQ**: Hardware interrupts
- **Software Interrupt**: Software-generated interrupts
- **Trap**: Debugging traps

## Process Descriptors

### Pidfd/Procdesc

Linux pidfd and FreeBSD Capsicum procdesc integration:

```bash
# Open pidfd for process
pidfd_open pid > /proc/self/fd/3

# Send signal via pidfd
pidfd_send_signal 3 SIGTERM

# Get file descriptor from process
pidfd_getfd 3 4
```

### Subreaper

Orphan process containment:

```bash
# Set process as subreaper
prctl --set-child-subreaper 1

# Reparent orphans
# Automatically handled by kernel
```

## Control Flow Integrity

### CFI Engine

Forward-edge CFI for control flow hijacking prevention:

```bash
# Register CFI target
cfi register 0x1000 function_name

# Validate indirect call
cfi validate 0x2000 0x1000

# View violations
cfi violations
```

## Next Steps

- [Filesystems](05-Filesystems.md) - Filesystem management
- [Networking](06-Networking.md) - Network configuration
- [Security](07-Security.md) - Security hardening

## AI Agent Maintenance Instructions

- **Bolt ⚡**: Ensure documentation of any new zero-allocation optimizations or performance improvements are added concisely without marketing fluff.
- **Palette 🎨**: Maintain Arch Linux wiki style: clear, factual, one page per topic, using appropriate markdown formatting and tables where necessary.
- **Sentinel 🛡️**: Verify that no hardcoded credentials or unvetted cryptographic algorithms are documented as production-ready. Ensure security limitations are accurately stated.
- **General**: Keep pages up-to-date with current repository capabilities. Remove redundant files when consolidating information.

---

## Oct 2026 Additions

### W^X Memory Hardening (`src/kernel/wx_pte_hardening.rs`)
Inspired by OpenBSD KARL (Kernel Address Layout Randomization). Every memory page is either writable OR executable — never both.

| Flag Constant | Value | Description |
|--------------|-------|-------------|
| `PageFlags::PRESENT` | bit 0 | PTE present bit |
| `PageFlags::WRITE` | bit 1 | Page writable |
| `PageFlags::USER` | bit 2 | User-accessible |
| `PageFlags::ACCESSED` | bit 5 | Recently accessed |
| `PageFlags::NO_EXECUTE` | bit 63 | NX bit — not executable |

Enforcement: when mapping pages, `WRITE` and `EXECUTE` bits are mutually exclusive. Any violation triggers a CFI violation record.

### POSIX Compatibility Stubs (`src/syscall/posix_compat.rs`)
New POSIX syscall stubs for application compatibility:
- `sys_prctl` — PR_SET_NAME, PR_GET_DUMPABLE, PR_SET_SECCOMP
- `sys_madvise` — MADV_NORMAL, MADV_RANDOM, MADV_DONTNEED, MADV_FREE
- `sys_pread64` / `sys_pwrite64` — positional file I/O
- `sys_sigaction` / `sys_sigprocmask` — signal handling
- `sys_getpid` / `sys_getppid` / `sys_exit_group`

### Hardware Entropy (`src/crypto/entropy.rs`)
XorShift64-based entropy pool with RDRAND-ready interface. Mix external entropy via `mix_entropy(value: u64)`. Get random bytes via `get_entropy_bytes(buf: &mut [u8])`.

---

## Maintenance Instructions for AI Agents

1. Run `cargo check 2>&1 | grep '^error' | wc -l` → 0 before any commit
2. All `unsafe` blocks require `// SAFETY:` comments
3. Syscall numbers must match Linux x86_64 ABI
4. W^X invariants must be preserved in any new memory-mapping code
5. When adding scheduler policies, implement for both CfsScheduler and RtScheduler

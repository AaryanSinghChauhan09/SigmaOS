# BSD & Linux Feature Inspirations in SigmaOS

SigmaOS draws from the best features across the Unix ecosystem.

## OpenBSD Inspirations

### pledge()

See [Pledge & Capsicum](Pledge-and-Capsicum)

### unveil()

See `src/security/sigma_unveil.rs`

### W^X (Write XOR Execute)

All memory is either writable or executable, never both. Implemented in `src/kernel/mm/`.

### ASLR

Address Space Layout Randomization with high entropy randomization.

### Pledge-based system daemons

All SigmaOS system daemons pledge after startup:

*   `sigma-dns` pledges: `stdio inet dns`
*   `sigma-sshd` pledges: `stdio inet proc id auth`
*   `sigma-ntp` pledges: `stdio inet`

## FreeBSD Inspirations

### Capsicum

See [Pledge & Capsicum](Pledge-and-Capsicum)

### jails

BSD jail-inspired process containers. See `src/container/`.

### ZFS

See [ZFS Compatible Filesystem](ZFS-Compatible-Filesystem)

### Ports-style package manager

`sigma-pkg` inspired by FreeBSD ports:

*   Source-based builds
*   Dependency resolution
*   Flavors (build variants)

### kqueue

See [Kqueue Event Notification](Kqueue-Event-Notification)

### DTrace

See [DTrace Tracing](DTrace-Tracing)

## Linux Inspirations

### eBPF

Kernel-level tracing (see DTrace-Tracing for SigmaOS equivalent).

### cgroups

Resource isolation for containers. See `src/resource/`.

### namespaces

Isolation primitives:

*   PID namespaces
*   Network namespaces
*   Mount namespaces
*   User namespaces

### systemd-inspired init

Service management with:

*   Declarative unit files
*   Dependency ordering
*   Socket activation
*   Crash recovery

### Btrfs features

Copy-on-write, snapshots, RAID modes (parallel to ZFS compat).

### io\_uring

High-performance async I/O (roadmap).

### seccomp

System call filtering (integrated into sandbox subsystem).

## macOS/Darwin Inspirations

### Mach-O compatibility

Binary format support for running macOS binaries.

### Grand Central Dispatch

Task-parallel dispatch queues. See `src/scheduler/`.

### Sandbox.d profiles

Declarative sandbox profiles.

## Plan 9 Inspirations

### Everything is a file

Devices, processes, network connections exposed as filesystem paths.

### 9P protocol

Distributed filesystem protocol for cluster operation.

### Namespace per process

Each process has its own view of the filesystem namespace.

## Gentoo/NixOS Inspirations

### Reproducible builds

All packages can be rebuilt exactly from source with identical results.

### USE flags (Gentoo-style)

Build-time feature flags for minimal installations.

### Declarative system configuration (NixOS-style)

Entire system configuration in a single declarative file:

    sigma {
      services.sshd.enable = true;
      security.pledge.enable = true;
      filesystem.zfs.enable = true;
      kernel.dtrace.enable = true;
    }

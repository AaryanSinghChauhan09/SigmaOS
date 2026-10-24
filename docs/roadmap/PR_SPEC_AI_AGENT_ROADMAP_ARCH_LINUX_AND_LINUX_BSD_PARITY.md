# PR Specification: AI Agent Future Development Roadmap for Arch Linux, Systemd, and Linux/BSD Parity

## Component
- [x] Packaging / Pacman ALPM Hooks & AUR Transpilation
- [x] Boot / Systemd-boot & Initramfs CPIO Pipelines
- [x] Kernel / `sched_ext` BPF Schedulers
- [x] Security / FreeBSD Capsicum, OpenBSD Pledge/Unveil, Landlock V6

## Upstream study
- **Projects**: Arch Linux, FreeBSD, OpenBSD, NetBSD, Linux Kernel, Systemd
- **Research note**: Integration of Arch Linux package management paradigms (ALPM hooks, PKGBUILD transpilation to native `.sigpkg`, clean chroot builds) alongside Linux/BSD kernel security and init system advancements.

## What SigmaOS adopts
- Declarative ALPM transaction hooks and automated cache scrubbing.
- Clean-room initramfs CPIO archive synthesis with DEFLATE/Zstd compression.
- BPF-programmable CPU scheduling (`sched_ext`) and real-time PREEMPT_RT latency guarantees.
- FreeBSD Capsicum sandbox restrictions for unprivileged processes.
- OpenBSD `pledge()` and `unveil()` system call and path restriction contracts.

## What SigmaOS changes or rejects
- Rejects imperative unvalidated shell installation scripts from untrusted package repositories.
- Rejects monolithic init system lock-in; supports pluggable init engines (OpenRC, runit, BSD rc.subr, systemd compatibility).

## Implementation Milestones & AI Agent Requirements

1. **Milestone 1: Arch Linux & ALPM Parity**:
   - Transpile AUR `PKGBUILD` metadata into native `.sigpkg` manifests.
   - Run linter checks (inspired by `namcap`) prior to package installation.
2. **Milestone 2: Linux/BSD Security Sandboxing**:
   - Enforce path validation rules rejecting path traversal (`..`), NUL bytes, and control sequences.
   - Enforce fail-closed privilege escalation barriers.
3. **Milestone 3: Kernel & MicroVM Virtualization**:
   - Validate BPF bytecode for user-space schedulers.
   - Support lightweight hypervisor guest booting with serial console diagnostic streams.

## Verification
```bash
cargo test --lib
./run_sigma_tests.sh
```

## Safety and rollback
- Transactional package rollbacks using journaled state files.
- Atomic kernel image slot swaps on boot update failure.

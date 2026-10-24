# Monthly Milestone Runbook

## Purpose
Complete one component milestone to achieve a new runtime-validated feature or recovery path.

## Monthly Schedule
- Month 1: Kernel/boot (QEMU x86_64 boot-to-shell)
- Month 2: Memory (Frame allocator, paging, leak detection)
- Month 3: Scheduler (Fair scheduling and preemption)
- Month 4: VFS (Path resolution, file descriptors, RAM-disk)
- Month 5: Processes/IPC (Pipes, signals, process lifecycle)
- Month 6: Installer/update (Clean install, atomic update, rollback)
- Month 7: Security (Capabilities and sandbox policy)
- Month 8: Drivers (virtio-blk, virtio-net, VGA)
- Month 9: Packaging (Metadata and dependency resolution)
- Month 10: Desktop (Minimal graphical session and input)
- Month 11: Networking (Basic socket and network stack)
- Month 12: Release engineering (Nightly artifact, rollback docs)

## Validation
Collect runtime evidence logs and publish monthly milestone report in `research/reports/monthly/`.

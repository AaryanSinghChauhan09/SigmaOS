# SigmaOS Roadmap

The roadmap defines our short-term and long-term milestones. Our main goals are stability, POSIX compliance, and comprehensive security auditing.

## Phase 1: Core Foundation (Current)
- [x] Basic memory management and paging
- [x] Bootloader integration
- [x] Initial VFS implementation
- [x] Sentinel capabilities prototype
- [ ] Stabilize scheduler

## Phase 2: Hardware and Networking
- [ ] Intel / AMD native network drivers
- [ ] VirtIO support for high-performance QEMU virtualization
- [ ] Full TCP/IP stack in Rust
- [ ] USB 3.0 / xHCI controller support
- [ ] NVMe storage drivers

## Phase 3: User Space and Distribution
- [ ] Mature `sigpkg` with multi-repo support
- [ ] Palette desktop environment (Wayland-compatible compositor)
- [ ] Bolt audio subsystem
- [ ] Basic web browser port
- [ ] OCI container runtime parity

## Phase 4: Self-Hosting
- [ ] Run the Rust compiler natively inside SigmaOS
- [ ] Build SigmaOS from within SigmaOS
- [ ] Implement V13 Absolute Omnipresent Self-Sufficiency subsystems

# 🔩 SigmaOS Components Table

This table lists the key subsystems and components of SigmaOS, their implementation status, source code locations, and associated wiki pages.

| Subsystem / Component | Category | Description | Status | Source File | Wiki Page |
|-----------|----------|-------------|--------|-------------|-----------|
| **Core Kernel** | Core | Microkernel/monolithic hybrid, SMP, syscall dispatch, page faults | ✅ Active / Implemented | `src/kernel/` | [[Architecture]] |
| **CachyBoreScheduler** | Core | BORE + EEVDF hybrid real-time CPU scheduler | ✅ Active / Implemented | `src/kernel/scheduler.rs` | [[CachyOS-BORE-Scheduler-Architecture]] |
| **SigmaMemoryManager** | Core | Buddy allocator + 4-level paging + slab allocator | ✅ Active / Implemented | `src/kernel/memory.rs` | [[Memory-Management]] |
| **SigmaShell** | Userland | Advanced shell and job control lifecycle engine | ✅ Active / Implemented | `src/shell/` | [[Architecture]] |
| **SigmaPkg (Package Manager)** | System | Universal package manager with transaction logs and Nix/Portage compatibility | ✅ Active / Implemented | `src/sigpkg/` | [[Package-Manager]] |
| **SigmaFS & VFS** | Core | CoW filesystem with ZFS compatibility layer, virtual file system switch | ✅ Active / Implemented | `src/fs/` | [[Architecture]] |
| **SigmaNet & Firewall** | Core | Zero-dependency network stack, zone-based firewall (pf/iptables style) | 🔄 In Progress | `src/net/` | [[Networking]] |
| **SigmaMAC & SELinux** | Security | SELinux AVC caching, RBAC, Bell-LaPadula MAC | ✅ Active / Implemented | `src/security/` | [[SECURITY-MODEL]] |
| **OpenBSD Pledge/Unveil** | Security | Sandbox restrictions for syscalls and file paths | ✅ Active / Implemented | `src/security/pledge.rs` | [[OpenBSD-Pledge-Unveil-Deep-Dive]] |
| **Post-Quantum Cryptography** | Security | Kyber-1024 + Dilithium-5 TLS 1.3 stack | ✅ Active / Implemented | `src/crypto/` | [[Post-Quantum-Cryptography]] |
| **AI Subsystem (S-AI)** | System | S-AI multi-agent orchestrator, LLM Router, Copilot | ✅ Active / Implemented | `src/ai/` | [[AI-Subsystem]] |
| **Device Drivers** | Core | KMS/DRM compositor drivers, PCI scanner, block/USB drivers | 🔄 In Progress | `src/driver/` | [[Architecture]] |
| **sigmainit** | System | Init system, service manager, systemd parity unit management | ✅ Active / Implemented | `src/init/` | [[Architecture]] |
| **Cgroup Governor** | Core | Sovereign resource control and limits | ✅ Active / Implemented | `src/kernel/` | [[Architecture]] |
| **Bootloader Integration** | Core | UEFI vector, secure boot, measured boot chain | 🔄 In Progress | `src/boot/` | [[Boot-Process]] |

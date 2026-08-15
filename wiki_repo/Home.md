# SigmaOS Wiki

Welcome to the SigmaOS wiki! SigmaOS is a from-scratch operating system written in pure Rust, with zero dependency on the Linux kernel, GNU userland, or C standard libraries.

## Quick Navigation

### Architecture
- [Kernel Architecture](Kernel-Architecture)
- [Security Architecture](Security-Architecture)
- [Virtual Memory Hardening](Virtual-Memory-Hardening)
- [Syscall Table](SYSCALL_TABLE)

### Security
- [Security Policy](SECURITY_POLICY)
- [Boot Security](Boot-Security)
- [PQC Enclave](AntiX-Zorin-Parity#pqc-enclave-post-quantum-cryptography)
- [Security Code Scanning](Security-Code-Scanning-Fixes)
- [Zero Trust Network](zero_trust_network)

### Developer Platform
- [AI Developer Platform](AI-Developer-Platform)
- [Tools System Suite](Tools-System-Suite)
- [Developer Guide](DEVELOPER_GUIDE)
- [Contributing](CONTRIBUTING)

### Compatibility
- [antiX Linux & Zorin OS Parity](AntiX-Zorin-Parity)
- [Arch Linux Parity Roadmap](ARCH_LINUX_PARITY_ROADMAP)
- [Linux Distro Integration](LINUX_DISTRO_INTEGRATION)
- [Win32 Compatibility](WIN32_COMPATIBILITY_PLANS)

### Roadmaps
- [100-Item Roadmap](100-Item-Roadmap)
- [3-Year Strategic Vision](3-Year-Strategic-Vision)
- [Future Development Roadmap](FUTURE-DEVELOPMENT-ROADMAP)
- [Gap Matrix vs Competitors](Gap-Matrix-SigmaOS-vs-Competitors)

### Package Management
- [SigPkg Reference](sigpkg_reference)
- [Zero Dependency Architecture](Zero-Dependency-Architecture)
- [Dependency Reduction Guide](DEPENDENCY_REDUCTION_GUIDE)

### Performance
- [Kernel Performance Plan](KERNEL_PERFORMANCE_PLAN)
- [Realtime HPC Scheduling](REALTIME_HPC_SCHEDULING_ROADMAP)

## Latest Changes (August 2026)

### Session 3 - August 15, 2026
**Branches merged** (5 total):
1. `jules-18086519973691592816`: AI Developer Platform suite (Roadmap 81-100), SerenityOS terminal tabs
2. `jules-3220898152855664802`: Boot refactor - raw pointer elimination, TPM 2.0, Secure Boot
3. `jules-514337451030587058`: Tools system suite registration, core utility types
4. `jules-8362645389262009630`: antiX Linux parity, Zorin OS compatibility, PQC enclave
5. `jules-880081283500171861`: Virtual MM paging with OpenBSD W^X, FreeBSD wired pages

**Security fixes:**
- Eliminated `unsafe mem::transmute` in ML inference, training, and print driver
- Removed duplicate enum definitions causing potential UB
- Resolved 26 source files with lingering conflict markers
- Added SAFETY documentation to all unsafe blocks

**New wiki pages:**
- [AI Developer Platform](AI-Developer-Platform)
- [Virtual Memory Hardening](Virtual-Memory-Hardening)
- [Boot Security](Boot-Security)
- [Tools System Suite](Tools-System-Suite)
- [antiX & Zorin Parity](AntiX-Zorin-Parity)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING) for how to contribute to SigmaOS.

All contributions must:
1. Be in pure Rust (no unsafe without SAFETY comment)
2. Pass `cargo clippy -- -D warnings`
3. Pass `cargo audit`
4. Include tests

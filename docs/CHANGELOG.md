# Changelog

All notable changes to SigmaOS will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive SigmaOS Development Roadmap documenting architectural improvements, OOP principles, and modern language migration strategy
- Modern language implementations using Rust, Nim, and Zig with OOP design patterns
- Enhanced sigpkg core with RepositoryFetcher, DependencyResolver, AIDependencyResolver, PackageInstaller, PackageRemover, PackageUpgrader, TransactionRollback, VersionComparator, and StringMatcher classes
- Manual SemVer comparison implementation without external libraries
- Manual substring search implementation without string libraries
- Dilithium-5 post-quantum signature verification (simplified implementation)
- SHA-3-512 hash function for signature verification
- Keccak permutation for cryptographic operations

### Changed
- Reduced predefined function dependencies in sigpkg by implementing custom algorithms
- Enhanced cloud orchestration daemon (sigma_nebula.nim) with OOP ContainerRuntime and SovereignContainer classes
- Enhanced desktop UX control center (sigma_control_center.nim) with OOP DesktopWidget hierarchy
- Enhanced sigma-agent (sigma_agent.rs) with trait-based Tool system

### Fixed
- KABI Symbol Checker: O(n) → O(1) hash table lookup for symbol approval
- Natural Language CLI: O(n) → O(1) hash table for intent matching, buffer overflow prevention
- Markdown Fixer: Precompiled regex patterns, optimized line iteration
- Rust Singletons: Thread-safe atomic operations replacing static mut

### Security
- Post-Quantum Cryptography: Kyber-1024 KEM and Dilithium-5 signatures
- Capability-based security model documentation
- Kernel hardening guidelines (W^X, ASLR, sigma_pledge, sigma_unveil)

## [0.2.0] - 2026-07-13

### Added
- Initial microkernel architecture with capability-based security
- Shard architecture for modular kernel components
- Core shards: S-MM (Memory Manager), S-SCHED (Scheduler), S-NET (Network Stack), S-FS (Filesystem), S-IPC (IPC), S-SEC (Security Manager), S-SYS (Syscall Interface)
- Essential shards: GPU Driver, Storage Driver, Audio Driver, Network Driver, Input Driver
- Optional shards: Zenith Compositor, Desktop Shell, LLM Integration, Package Manager
- EEVDF scheduler implementation
- Buddy allocator for physical memory management
- VFS layer with ext4, FAT32, NTFS support
- Zero-trust network stack with TCP/UDP protocols
- Post-quantum cryptography support (Kyber-1024, Dilithium-5)
- Zero-copy IPC operations
- Capability-based security system
- Multi-format deployment profiles (Standalone, Microkernel, Cloud)

### Security
- 64-bit hardware-enforced capability-based security
- Post-quantum cryptography (NIST FIPS 203/204)
- Default-deny security model
- Audit trail with BLAKE2b hash chains

## [0.1.0] - 2026-07-01

### Added
- Initial SigmaOS repository structure
- Basic microkernel design
- Rust-based kernel components
- Nim-based userland components
- Zig-based low-level runtime
- Initial documentation (README, ARCHITECTURE, INSTALL, CONTRIBUTING, SECURITY_POLICY)
- GitHub Wiki integration

---

## Versioning Scheme

SigmaOS follows Semantic Versioning (MAJOR.MINOR.PATCH):

- **MAJOR**: Incompatible API changes
- **MINOR**: Backwards-compatible functionality additions
- **PATCH**: Backwards-compatible bug fixes

### Release Types

- **Stable Releases**: Even-numbered minor versions (0.2.0, 0.4.0)
- **Development Releases**: Odd-numbered minor versions (0.3.0, 0.5.0)
- **LTS Releases**: Major versions with long-term support (1.0.0, 2.0.0)

---

## Migration Guide

### Upgrading from 0.1.0 to 0.2.0

1. **Backup your system**: Create a full system backup before upgrading
2. **Update dependencies**: Ensure all custom shards are compatible with new shard API
3. **Rebuild kernel**: The microkernel interface has changed; rebuild is required
4. **Update configuration**: Capability system configuration format has changed
5. **Test applications**: Verify all applications work with new security model

See [INSTALL.md](INSTALL.md) for detailed upgrade instructions.

---

## Security Advisories

Security advisories are published separately at:
- GitHub Security Advisories: https://github.com/AaryanSinghChauhan09/SigmaOS/security/advisories
- Security mailing list: security@sigmaos.dev

For reporting vulnerabilities, see [SECURITY_POLICY.md](SECURITY_POLICY.md).

---

*Last Updated: 2026-07-14*

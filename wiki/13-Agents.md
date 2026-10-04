# AI Agent Guidelines

SigmaOS provides comprehensive guidelines for AI agents working on various components to enable autonomous development and continuous improvement.

## Tri-Agent Framework

SigmaOS employs a three-agent autonomous continuous development framework:

### ⚡ Bolt: Performance Agent
Identifies and implements focused, measurable performance improvements to make SigmaOS faster, lighter, and more memory-efficient.

### 🎨 Palette: UX & Accessibility Agent
Enhances Zenith Desktop, Web UI, and CLI user interfaces with accessible, intuitive, and delightful user interactions.

### 🛡️ Sentinel: Security Agent
Protects SigmaOS kernel and userland from security vulnerabilities, privilege escalation, memory unsafety, and data leaks.

## Component-Specific Agent Guidelines

Located in the `Agents/` folder, each major subsystem has dedicated agent documentation:

- **KERNEL_AGENTS.md** - Core kernel subsystems (scheduling, interrupts, syscalls)
- **MEMORY_AGENTS.md** - Memory management (buddy allocator, slab allocator, paging)
- **FILESYSTEM_AGENTS.md** - Virtual filesystem layer and implementations
- **NETWORK_AGENTS.md** - Networking stack (TCP/IP, WireGuard, packet filtering)
- **SECURITY_AGENTS.md** - Security framework (capabilities, seccomp, sandboxing)
- **DESKTOP_AGENTS.md** - Zenith desktop environment and window management
- **PACKAGE_AGENTS.md** - Universal package manager SigmaPkg
- **DISTRO_AGENTS.md** - Linux/BSD distro compatibility and gateway
- **AUDIO_AGENTS.md** - Audio subsystem and sound management
- **BLUETOOTH_AGENTS.md** - Bluetooth stack and GATT client
- **DRIVERS_AGENTS.md** - Hardware drivers (PCIe, NVMe, USB, WiFi)
- **CRYPTO_AGENTS.md** - Cryptographic operations and encryption
- **IPC_AGENTS.md** - Inter-process communication mechanisms
- **ARCH_AGENTS.md** - Architecture portability (x86_64, ARM64, RISC-V)

## Open Source Inspiration

Each component agent document includes:
- Linux kernel references and implementation patterns
- FreeBSD and OpenBSD design inspirations
- Specific improvement opportunities
- Performance optimization strategies
- Security hardening guidelines

## Continuous Improvement

These agent guidelines enable SigmaOS to continuously improve by:
- Learning from open source competitors (Linux, FreeBSD, OpenBSD)
- Implementing best practices from mature operating systems
- Maintaining zero-dependency philosophy
- Ensuring security and performance excellence

For detailed agent guidelines, see the component-specific files in the [Agents/](../Agents/) folder.

## AI Agent Maintenance Instructions

### 🚫 CRITICAL RULE: No Session-Specific Wiki Pages

**AI agents MUST NOT create session-specific documentation pages** such as:
- ❌ "Merge Summary Oct 2026"
- ❌ "New Components Oct 2026"  
- ❌ "Session Summary [Date]"
- ❌ "Components Added [Date]"
- ❌ Any date-stamped session logs

**Instead, AI agents MUST:**
- ✅ **Update the strategic roadmap** (`11-Roadmap.md`) with competitive analysis
- ✅ **Update component status** in permanent pages (04-Kernel.md, 07-Security.md, etc.)
- ✅ **Maintain the future development plan** with Linux/BSD/competitor inspiration
- ✅ **Document architecture decisions** in `15-Architecture-Decisions.md`
- ✅ **Keep one wiki page per topic** - merge redundant information

### Why This Rule Exists
Session-specific pages create noise, become outdated immediately, and don't provide strategic value. Users need **current capability documentation** and **forward-looking roadmaps**, not historical session logs.

### Correct Documentation Pattern
```
❌ BAD:  Create "23-October-2026-Repository-Sync.md"
✅ GOOD: Update "11-Roadmap.md" with newly implemented features and next priorities

❌ BAD:  Create "24-Component-Improvements-Oct-2026.md"  
✅ GOOD: Update "07-Security.md" with new security features and their status

❌ BAD:  Document every PR merge in a new wiki page
✅ GOOD: Update the relevant component page when features are completed
```

### Strategic Documentation Priorities
1. **Current Capabilities** - What works now (component pages)
2. **Architecture Rationale** - Why design decisions were made
3. **Competitive Analysis** - How SigmaOS compares to Linux/BSD/*OS
4. **Future Roadmap** - What's planned based on competitor analysis
5. **Development Guidelines** - How to contribute effectively

### Maintenance Responsibilities by Agent

- **Bolt ⚡**: Ensure documentation of any new zero-allocation optimizations or performance improvements are added concisely without marketing fluff. Update performance metrics in component pages, not session summaries.
- **Palette 🎨**: Maintain Arch Linux wiki style: clear, factual, one page per topic, using appropriate markdown formatting and tables where necessary. Update UX/accessibility status in desktop pages.
- **Sentinel 🛡️**: Verify that no hardcoded credentials or unvetted cryptographic algorithms are documented as production-ready. Ensure security limitations are accurately stated. Update security feature status in `07-Security.md`.
- **General**: Keep pages up-to-date with current repository capabilities. Remove redundant files when consolidating information. **Delete any session-specific pages created before this rule.**

---

## Oct 2026 Agent Guideline Updates

### New Build Rules (mandatory)
- `cargo check 2>&1 | grep '^error' | wc -l` → must be **0** before every commit
- No `std::` imports in `no_std` modules — use `alloc::` instead
- No external crate dependencies — implement everything from scratch
- Only ONE branch: **main**. Delete feature branches immediately after merging.

### Architecture Targets (all must compile)
- x86_64 bare-metal (primary)
- AArch64 / ARM64 (secondary)
- RISC-V 64-bit (tertiary)

### New Module References
| Module | Purpose |
|--------|---------|
| `src/crypto/entropy.rs` | XorShift64 entropy pool (RDRAND-ready) |
| `src/syscall/posix_compat.rs` | POSIX compatibility stubs |
| `src/kernel/wx_pte_hardening.rs` | W^X memory enforcement |
| `src/security/pledge_unveil.rs` | OpenBSD pledge/unveil |
| `src/security/capsicum.rs` | FreeBSD Capsicum capabilities |

### Distro Inspiration Matrix
| Source OS | Concept | SigmaOS Implementation |
|-----------|---------|----------------------|
| OpenBSD | W^X, pledge, unveil | `src/kernel/wx_pte_hardening.rs`, `src/security/pledge_unveil.rs` |
| FreeBSD | Capsicum, VIMAGE, Netmap | `src/security/capsicum.rs`, `src/networking/sovereign_net.rs` |
| Linux | EEVDF, CFI, cgroup v2 | `src/kernel/scheduler.rs`, `src/kernel/cfi.rs`, `src/resource/cgroup_v2.rs` |
| NetBSD | pkgsrc | `src/package/` |
| NixOS | Declarative config | `src/config/` |

---

## Maintenance Instructions for AI Agents

1. After adding new modules, update this table
2. Keep one wiki page per topic — merge duplicates
3. When `.md` roadmap files are fully implemented, move them here and delete originals

# AI Agent Chroot Management Specification for SigmaOS (`docs/AGENTS_CHROOT_MANAGEMENT.md`)

This document provides technical guidelines and reference specifications for AI agents developing, configuring, and managing **chroot environments, build sandboxes, and rootfs isolations** in **SigmaOS**.

---

## 1. Overview & Chroot Subsystem Architecture

In SigmaOS, chroot management is handled by native `#![no_std]` Rust modules that combine Linux build sandboxing with BSD security primitives:

- **Alpine / Void Chroot Engine (`ApkChrootBuildSandboxEngine` in `src/distro/linux_bsd_inspirations.rs`)**: Light, fast chroot build sandbox supporting bind mounts, environment variable isolation, network suppression, and chroot lifecycle hooks.
- **Debian / Ubuntu Sbuild Engine (`SbuildChrootSandboxEngine` in `src/distro/developer.rs`)**: Handles Debian `sbuild` chroot stages and dependency isolation.
- **FreeBSD Jails Integration (`FreeBSDJail` in `src/distro/linux_bsd_inspirations.rs`)**: Integrates directory containment with PID, memory, and VNET stack isolation.
- **OpenBSD Path & Descriptor Restriction (`OpenBSDUnveil` & `OpenBsdFdPledgeGate`)**: Restricts file system visibility and descriptor rights within chroot boundaries.

---

## 2. Core OOP Design Patterns for Chroot Management

When interacting with or extending chroot components, AI agents must adhere to the following design patterns:

### A. Sandbox Lifecycle Management (State & Strategy Patterns)
- Chroot sandboxes transition through defined states: `Created -> BindMounted -> ActiveChroot -> Executing -> Exited -> Cleaned`.
- AI agents must enforce `is_active` state checks before permitting package compilation or bind mount additions:

```rust
// Example: Correct usage of ApkChrootBuildSandboxEngine
let mut sandbox = ApkChrootBuildSandboxEngine::new("sbx_alpine_01", "/var/chroot/build", true);

// Configure mounts and environment variables BEFORE entering chroot
sandbox.add_bind_mount("/usr/include")?;
sandbox.set_env("CC", "gcc");

// Enter isolated chroot environment
sandbox.enter_chroot()?;

// Execute hermetic package compilation
let result = sandbox.compile_package("curl", "make")?;

// Safely exit chroot environment
sandbox.exit_chroot()?;
```

### B. Decorator Pattern for Security & Audit Capabilities
- Enhance raw chroot sandboxes with OpenBSD unveil rules and post-quantum attestation:

```rust
// Apply OpenBSD unveil path restriction to chroot rootfs
let mut unveil = OpenBSDUnveil::new();
unveil.unveil("/var/chroot/build/usr", "rx")?;
unveil.lock();
```

---

## 3. Best Practices for AI Agents Working with Chroots

1. **Zero External Dependencies**:
   - Maintain `#![no_std]` compatibility. Use `alloc::string::String`, `alloc::vec::Vec`, and `alloc::format!`.
2. **Hermeticity & Network Isolation**:
   - Always initialize chroot build sandboxes with `isolate_network: true` unless network access is explicitly requested for mirror downloads.
3. **Explicit State Cleanup**:
   - Ensure `exit_chroot()` is called in both success and error handling paths to prevent leaking active chroot mount states.
4. **Audit & Provenance Recording**:
   - Log all chroot compilation steps and compute SHA-256 / Dilithium-5 build attestations via `SovereignPackageBuildProvenanceEngine`.

---

## 4. Verification & Testing

AI agents must verify chroot changes using the standalone rustc runner and full test suite:

```bash
# 1. Compile & run linux_bsd_inspirations standalone tests
rustc --edition=2021 --test src/distro/linux_bsd_inspirations.rs -o build/test_inspirations && ./build/test_inspirations

# 2. Run global test suite
./run_sigma_tests.sh
```

---
*End of docs/AGENTS_CHROOT_MANAGEMENT.md*

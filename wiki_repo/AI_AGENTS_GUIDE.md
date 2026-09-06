# SigmaOS AI Agents Master Developer & Architecture Guide

Welcome to the **SigmaOS AI Agents Master Developer & Architecture Guide**. This document serves as the authoritative, self-contained reference manual for AI coding agents, autonomous agents, subagent frameworks, and human maintainers working on the SigmaOS codebase.

---

## 1. Core Directives for AI Agents

When modifying, extending, or debugging the SigmaOS repository, AI agents MUST adhere to the following core tenets:

1. **Zero External Dependency Rule**:
   - SigmaOS is built on a pure, self-sufficient, `#![no_std]` capable Rust architecture.
   - Core data structures, collections, parsers, and primitives are implemented in the `klib` module (`src/klib/`). Do NOT introduce third-party external crates in `Cargo.toml`.
2. **Linux & BSD Subsystem Interoperability**:
   - All package managers, security mechanisms, and service init systems must support cross-subsystem dispatching via `SovereignUniversalDistroBridge` (`src/distro/linux_bsd_inspirations.rs`).
   - Support all 21 `DistroSubsystemMode` variants (Arch, Debian, Alpine, NixOS, Gentoo, Fedora, Void, FreeBSD, OpenBSD, NetBSD, DragonFly BSD, Solaris/Illumos, etc.).
3. **UX & UI Interface Management Guidelines**:
   - Adhere to `AI_AGENTS_UX_MANAGEMENT_GUIDE.md` for visual-first, GTK/Libadwaita toolkit bindings, WCAG AA contrast compliance, and Zenith Desktop layout presets (`KdePlasma`, `GnomeShell`, `XfceModular`, `CinnamonMint`, `LuminaBsd`, `CosmicRust`).
4. **Mandatory Test Verification**:
   - Always run `./run_sigma_tests.sh` to verify changes.
   - Do NOT run `cargo test --lib` directly due to `#![no_std]` bare-metal test harness constraints; invoke `./run_sigma_tests.sh` which handles the 13-stage test pipeline correctly.
5. **Memory Recording & Knowledge Persistence**:
   - Call `initiate_memory_recording` upon completing code reviews and verifying test pass rates to record key learnings, bug fixes, and architectural advancements.

---

## 2. Twelve Sovereign System Shards (`S-SHARDS`)

SigmaOS organizes all system services, native drivers, applications, AI/ML models, and security subsystems into **Twelve Native Sovereign System Shards**:

1. **Shard 1: Core Kernel & Microkernel Subsystem** (`src/kernel/`, `src/klib/`)
2. **Shard 2: Universal Package Management (`sigpkg`)** (`src/sigpkg/`, `src/package/`)
3. **Shard 3: Zenith Desktop Environment & Display Compositor** (`src/desktop/`, `zenith_desktop/`)
4. **Shard 4: Security, Isolation & Cryptography** (`src/security/`, `src/crypto/`)
5. **Shard 5: Network Stack, eBPF & Sovereign Browser** (`src/net/`, `src/kernel/ebpf.rs`)
6. **Shard 6: Driver Framework & HAL** (`src/driver/`, `src/drivers/`, `src/hal/`)
7. **Shard 7: Virtualization, Containers & MicroVMs** (`src/virtualization/`, `src/container/`)
8. **Shard 8: AI/ML Inference, Orchestration & Agents** (`src/ai/`)
9. **Shard 9: Unified Filesystems & Storage Engines** (`src/filesystem/`, `src/storage/`)
10. **Shard 10: Audio, Video & Creative Suite Frameworks** (`src/audio/`, `src/graphics/`)
11. **Shard 11: Productivity, Office & Education Suites** (`src/education/`, `src/pillars/`)
12. **Shard 12: Toolchain, Build System & Shell Environment** (`src/shell/`, `tools/`)

---

## 3. Subagent Workflows & Self-Created Tools

AI agents working in the sandbox can create dedicated Python tools under `/home/jules/self_created_tools/` to streamline workflows.

### Example Self-Created Test Verification Tool (`run_tests.py`)
```python
import subprocess
import sys

def main():
    res = subprocess.run(["./run_sigma_tests.sh"], capture_output=True, text=True)
    if res.returncode != 0:
        print("TEST RUNNER FAILED!")
        print("\n".join(res.stdout.splitlines()[-30:]))
        sys.exit(1)
    else:
        print("TEST RUNNER PASSED!")
        print("\n".join(res.stdout.splitlines()[-10:]))

if __name__ == "__main__":
    main()
```

---

## 4. Diagnostics & Troubleshooting Matrix

| Error / Symptom | Root Cause | Solution |
| :--- | :--- | :--- |
| `E0599: no method named ... found` | Missing trait or helper method on struct under conditional compilation | Implement missing method or check `#[cfg(test)]` guards |
| `E0004: non-exhaustive patterns` | Unhandled `DistroSubsystemMode` or `PackageFormat` enum variant | Add missing match arms across all subsystem dispatchers |
| `E0119: conflicting implementations` | Duplicate trait implementation or derive | Remove orphan or duplicate trait impl in target module |
| `./run_sigma_tests.sh: line X: No such file` | Missing `/tmp` build artifact directory | Ensure `mkdir -p build /tmp` is executed before test compilation |

---

## 5. Summary & Checklist for AI Agents

- [ ] Explored codebase and reviewed `README.md` & `ARCHITECTURE.md`.
- [ ] Formulated plan and requested plan review via `request_plan_review`.
- [ ] Executed changes in safe, zero-dependency Rust.
- [ ] Verified build with `cargo check --lib` and `./run_sigma_tests.sh`.
- [ ] Completed pre-commit checks and code review (`request_code_review`).
- [ ] Recorded learnings via `initiate_memory_recording`.
- [ ] Submitted changes via `submit`.

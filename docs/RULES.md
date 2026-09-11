# 📋 SigmaOS Contributor & AI Agent Task Rules & Execution Directives

**Scope:** Developer & AI Agent Execution Guidelines, Quality Engineering, and Subsystem Integrity

---

## 1. Contributor & AI Agent Mandatory Directives

1. **Zero External Dependencies (`#![no_std]`)**:
   - Every module added or modified in `src/` MUST maintain `#![no_std]` zero external crate dependency architecture.
   - Do NOT add third-party crates to `Cargo.toml`. Utilize native Rust or `alloc::` primitives (`alloc::vec::Vec`, `alloc::string::String`, `alloc::format!`, `alloc::collections::BTreeMap`).

2. **Mandatory Git Branch Prefix**:
   - Every git branch created by developers or AI agents MUST use the `jules-` prefix (e.g., `jules-kernel-scheduler`, `jules-pkg-adapter`).

3. **Multi-Distro Interoperability & Security Gating**:
   - Subsystem features must maintain compatibility across 25+ Linux and BSD distribution modes (`LinuxArch`, `LinuxDebian`, `LinuxFedora`, `FreeBsd`, `OpenBsd`, `NixOS`, `CachyOS`, `Omarchy`, etc.).
   - All userland utilities must declare sandboxing bounds using OpenBSD `pledge()`/`unveil()`, Linux Landlock v5, or FreeBSD Capsicum capability rights.

4. **Autonomous Testing & Self-Verification Protocol**:
   - Modifying any Rust file requires standalone test execution:
     `rustc --edition=2021 --test <file_path> -o build/test_bin && ./build/test_bin`
   - Run the master test runner `./run_sigma_tests.sh` to ensure all native test runner stages pass cleanly.

5. **AI Agent Pre-Commit Protocol**:
   - AI agents must execute `pre_commit_instructions`, perform code review verification (`request_code_review`), record learnings (`initiate_memory_recording`), and finalize changes via `submit`.

---

## 2. Distribution Engineering Execution Rules

Inspired by Linux and BSD distribution maintenance protocols:

1. **Cleanroom Chroots**: All package builds and core subsystem modifications must be verifiable in an isolated chroot / sandbox container.
2. **Zero Unverified Dependencies**: All code additions must adhere strictly to `#![no_std]` zero external dependency requirements.
3. **Reproducible Compilation**: Builds must yield byte-identical output given the same `SOURCE_DATE_EPOCH` and toolchain version.
4. **Mandatory Branch Naming**: Every git branch MUST start with the `jules-` prefix (e.g. `jules-subsystem-improvements`).
5. **Security Gating**: All new binaries and utilities must declare OpenBSD `pledge`/`unveil` privilege restrictions and FreeBSD `Capsicum` capability rights.
6. **Pre-Commit Verification**: Before submitting changes, developers must call `pre_commit_instructions` and execute `./run_sigma_tests.sh`.

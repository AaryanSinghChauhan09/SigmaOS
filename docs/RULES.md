# 📋 SigmaOS Developer Task Rules & Execution Directives

**Scope:** Developer Execution Guidelines, Quality Engineering, and Subsystem Integrity

---

## 1. Distribution Engineering Execution Rules

Inspired by Linux and BSD distribution maintenance protocols (see full rules in `docs/CONTRIBUTOR_AND_AI_AGENT_RULES.md`):

1. **Cleanroom Chroots**: All package builds and core subsystem modifications must be verifiable in an isolated chroot / sandbox container.
2. **Zero Unverified Dependencies**: All code additions must adhere strictly to `#![no_std]` zero external dependency requirements.
3. **Reproducible Compilation**: Builds must yield byte-identical output given the same `SOURCE_DATE_EPOCH` and toolchain version.
4. **Mandatory Branch Naming**: Every git branch MUST start with the `jules-` prefix (e.g. `jules-subsystem-improvements`).
5. **Security Gating**: All new binaries and utilities must declare OpenBSD `pledge`/`unveil` privilege restrictions and FreeBSD `Capsicum` capability rights.
6. **Pre-Commit Verification**: Before submitting changes, developers must call `pre_commit_instructions` and execute `./run_sigma_tests.sh`.

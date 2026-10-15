# Roadmap Specification: Sovereign Periodic Research-and-Implementation Program

## 1. Specification Overview
This specification defines the technical requirements, directory layout, governance structures, task schema, and execution rules for the SigmaOS **Periodic Research-and-Implementation Program**.

## 2. Directory Layout Requirement
The repository MUST maintain a stable research-and-implementation tree:
- `research/upstream/{xv6,linux,arch,mint,fedora,freebsd,openbsd,redox}/`
- `research/decisions/` (Architecture Decision Records)
- `research/comparisons/` (Boot, scheduler, VFS, security, packaging, hardware comparison notes)
- `research/reports/weekly/`, `research/reports/monthly/`, `research/reports/quarterly/`
- `.agents/runbooks/` (`weekly-study.md`, `monthly-capability.md`, `quarterly-review.md`)
- `.agents/tasks/open/`, `.agents/tasks/in-progress/`, `.agents/tasks/done/`, `.agents/tasks/blocked/`

## 3. Mandatory Governance Rules
1. Every study cycle MUST target a narrow OS mechanism.
2. Every study cycle MUST produce an Architecture Decision Record (ADR).
3. Implementation MUST be a small, connected vertical slice.
4. Capability status claims MUST be backed by QEMU/runtime test logs.
5. All research artifacts MUST record commit hash, toolchain version, commands, exit codes, and hashes.

## 4. Verification Gate
The program governance engine MUST pass `cargo check`, `cargo test`, and `./run_sigma_tests.sh`.

# Consolidated Package Documentation Registry (/usr/share/doc)

## Summary
In SigmaOS, offline package records traditionally located under `/usr/share/doc/<package>/` (README files, changelogs, copyright notices, license texts, and usage examples) are consolidated into a centralized, searchable GitHub Wiki registry.

---

## Package Index

### 1. `sigma-core` (v1.0.0)
- **Path:** `/usr/share/doc/sigma-core`
- **README:** SigmaOS core utilities and system runtime documentation. Provides POSIX userland commands, eBPF telemetry hooks, and system init management.
- **CHANGELOG:**
  - `v1.0.0`: Initial release of sovereign kernel, memory managers, and POSIX userland tools.
- **COPYRIGHT & LICENSE:** Copyright (c) 2026 SigmaOS Project. MIT License.

### 2. `sigma-pkg` (v1.0.0)
- **Path:** `/usr/share/doc/sigma-pkg`
- **README:** Universal package manager documentation and schema definition. Supports multi-format ingestion (.deb, .rpm, .pkg.tar.zst, .apk, .ebuild, .xbps, .pkg, etc.) and DPLL SAT dependency solving.
- **CHANGELOG:**
  - `v1.0.0`: Multi-format package ingestion, DPLL SAT solver, and PQC Dilithium-5 attestation.
- **COPYRIGHT & LICENSE:** Copyright (c) 2026 SigmaOS Project. MIT License.

### 3. `sigma-net` (v1.0.0)
- **Path:** `/usr/share/doc/sigma-net`
- **README:** High-performance zero-copy network stack documentation, including XDP Express Data Path, eBPF packet filter rules, and stateful firewall engine.
- **CHANGELOG:**
  - `v1.0.0`: XDP packet shaper, BPF filter engine, and WireGuard VPN tunnel driver.
- **COPYRIGHT & LICENSE:** Copyright (c) 2026 SigmaOS Project. MIT License.

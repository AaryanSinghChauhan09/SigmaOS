# AI Agent BSD Parity Management Specification for SigmaOS

This document provides guidelines and architectural specifications for AI agents maintaining and developing BSD compatibility components within **SigmaOS**.

---

## 1. Overview & BSD Parity Subsystem

SigmaOS implements a clean-room, zero-external-dependency BSD parity subsystem across `src/distro/bsd.rs`, `src/distro/bsd_parity.rs`, `src/distro/bsd_linux_innovations.rs`, `src/sigpkg/poudriere_xbps.rs`, and `src/net/linux_bsd_network_innovations.rs`.

Key components managed by AI agents:

1. **FreeBSD UCL Manifest Parser (`FreeBsdUclManifest`)**:
   - Parses Universal Configuration Language (UCL) FreeBSD `+MANIFEST` files (`name`, `version`, `comment`, `origin`, `deps`).
2. **OpenBSD +CONTENTS Package Parser (`OpenBsdContentsManifest`)**:
   - Parses OpenBSD package contents files (`@name`, `@comment`, `@depend`, `@pkgdep`, `@exec`, `@unexec`).
3. **NetBSD pkgsrc Package Parser (`NetBsdPkgsrcManifest`)**:
   - Parses NetBSD pkgsrc manifests (`PKGNAME`, `COMMENT`, `REQUIRES`, `DEPENDS`).
4. **OpenBSD PF Stateful Packet Filter (`OpenBsdPfCarpPfsyncStateEngine`)**:
   - Implements OpenBSD `pf.conf` rule evaluation, stateful packet inspection, CARP virtual IP failover, and PFSYNC state replication.
5. **FreeBSD Capsicum Capability Sandbox (`FreeBsdCapsicumEngine`)**:
   - Enforces `cap_enter()` capability mode sandboxing, restricting descriptor rights (`CAP_READ`, `CAP_WRITE`, `CAP_MMAP`) on system file handles.

---

## 2. Rules for AI Agents Developing BSD Parity Modules

1. **Zero External Dependencies**:
   - BSD manifest parsers, PF state tables, and Capsicum rights checkers must use safe Rust or `klib` primitives.
2. **BSD Syscall & ABI Translation**:
   - Ensure proper register alignment and errno mapping for FreeBSD, OpenBSD, and NetBSD syscall translation tables.
3. **Pledge & Unveil Security**:
   - Enforce path restrictions via OpenBSD-style `unveil` and promise restrictions via `pledge`.

---

## 3. Verification Commands

AI agents must verify BSD compatibility changes using:

```bash
rustc --edition=2021 --crate-type=lib src/lib.rs -o libsigmaos.rlib
```

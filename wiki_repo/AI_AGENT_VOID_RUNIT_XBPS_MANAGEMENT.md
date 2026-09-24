# AI Agent Void Linux Runit & XBPS Management Directives

## Executive Overview
This document outlines architecture and maintenance procedures for AI agents maintaining Void Linux `runit` init supervision and `XBPS` package management in SigmaOS.

---

## Core Infrastructure
1. **Runit Service Supervisor**: `VoidRunitManager` and `RunitService` in `src/distro/bsd_linux_innovations.rs`.
2. **XBPS Soname & Orphan Resolver**: `XbpsSonameAndOrphanEngine` in `src/package/bsd_linux_package_innovations.rs`.
3. **XBPS Signature Verification**: `XbpsSignatureVerifier` in `src/compatibility/distro_parity_ultimate.rs`.

---

## AI Agent Verification Protocol
Agents modifying runit or XBPS modules must execute:
```bash
rustc --test src/distro/bsd_linux_innovations.rs --edition=2021 -o build/bsd_linux_innovations_test && ./build/bsd_linux_innovations_test
./run_sigma_tests.sh
```

# AI Agent FreeBSD Ports & Pkg Management Directives

## Executive Overview
This document specifies operational directives, architecture guidelines, and verification procedures for AI agents maintaining FreeBSD Ports, `pkg`, VuXML vulnerability auditing, and Capsicum sandboxing compatibility in SigmaOS.

---

## Architectural Principles
1. **Zero External Dependencies**: All FreeBSD ports parsing, `pkg` UCL database queries, and VuXML audit logic must be implemented in native `#![no_std]` Rust.
2. **VuXML Vulnerability Auditing**: The `FreeBsdPortsFlavoursAndVuxmlEngine` in `src/package/bsd_linux_package_innovations.rs` validates installed packages against active CVE vulnerability ranges.
3. **Capsicum Capability Sandboxing**: Use `BsdCapsicumRights` and capability file descriptors to isolate untrusted package compilation tasks.

---

## AI Agent Verification Protocol
Agents modifying FreeBSD ports or `pkg` components must run:
```bash
rustc --test src/package/bsd_linux_package_innovations.rs --edition=2021 -o build/package_innovations_test && ./build/package_innovations_test
./run_sigma_tests.sh
```

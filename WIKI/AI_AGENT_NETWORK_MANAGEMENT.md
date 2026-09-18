# AI Agent Network Management Architecture in SigmaOS

This document specifies network management architectures, security validation rules, and cross-platform firewall/IPC abstractions for AI agents working on networking modules in SigmaOS.

---

## 🌐 1. Network Subsystem Architecture

SigmaOS implements a multi-paradigm network stack located in `src/network/`:

```
+-----------------------------------------------------------------------+
| Universal Command & Control (`src/network/commands.rs`)               |
| Unified CLI shims for ifconfig, ip, pfctl, nftables, npfctl          |
+-----------------------------------------------------------------------+
| Multi-Distro Firewall Translation Engine                               |
| OpenBSD pf (`pf_firewall.rs`), Linux nftables (`nftables.rs`),        |
| NetBSD NPF (`npf_firewall.rs`)                                         |
+-----------------------------------------------------------------------+
| Core Protocols & Acceleration (`src/network/tcp_udp.rs`)              |
| BBR & Reno Congestion Control, TCP Option Parsing, UDP Checksums     |
| FreeBSD VNET Per-Jail Virtualized Stacks (`distro_net.rs`)            |
+-----------------------------------------------------------------------+
```

---

## 🔒 2. Input Validation & Security Rules

1. **IPv4 Validation (`validate_ipv4` in `src/security/input_validation.rs`)**
   - Reject multi-digit octets with leading zeros (e.g. `010.0.0.1` or `192.168.01.1`) to prevent octal parser differential and SSRF vulnerabilities.

2. **IPv6 Validation (`validate_ipv6` in `src/security/input_validation.rs`)**
   - Explicit block count must be tracked alongside `double_colon`.
   - Compressed IPv6 addresses with 8 or more explicit blocks (`blocks >= 8`) **must** be rejected to prevent over-length validation bypasses.

3. **Port & URI Boundaries**
   - Enforce colon (`:`) boundaries when parsing network paths or URI schemes to prevent path traversal bypasses (`file:../`).

---

## ⚙️ 3. Verification Commands for Network Agents

- **Input Validation & Security Test Suite:**
  `rustc --test src/security/input_validation.rs --edition=2021 -o build/input_val_test && ./build/input_val_test`
- **Network Ecosystem & IPC Bridge Test:**
  `rustc --test src/compatibility/linux_bsd_ecosystem_bridge.rs --edition=2021 -o build/ecosystem_bridge_test && ./build/ecosystem_bridge_test`
- **Core Network Protocol Tests:**
  `cargo test --lib -- network::tcp_udp`

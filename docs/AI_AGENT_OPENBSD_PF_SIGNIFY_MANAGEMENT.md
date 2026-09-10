# AI Agent OpenBSD PF, Signify & Pledge Management Directives

## Executive Overview
This document specifies operational directives and verification rules for AI agents maintaining OpenBSD Packet Filter (PF) stateful firewalls, `signify` Ed25519 signature verification, and `pledge`/`unveil` sandboxing in SigmaOS.

---

## Key Modules
1. **PF Firewall**: `BsdStatefulPacketFilter` and `PfStateSynchronizationEngine` in `src/distro/bsd_linux_innovations.rs`.
2. **Signify Verification**: `OpenBsdSignifyVerifierEngine` in `src/distro/linux_bsd_inspirations.rs`.
3. **Pledge & Unveil**: `OpenBsdMonotonicSandbox` in `src/compatibility/distro_parity_ultimate.rs`.

---

## AI Agent Verification Protocol
Agents modifying OpenBSD security or firewall components must run:
```bash
rustc --test src/distro/bsd_linux_innovations.rs --edition=2021 -o build/bsd_linux_innovations_test && ./build/bsd_linux_innovations_test
./run_sigma_tests.sh
```

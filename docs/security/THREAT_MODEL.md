# SigmaOS Kernel Threat Model

## Overview

This document details the threat model for the SigmaOS kernel architecture, outlining trust boundaries, threat actors, attack vectors, and kernel mitigations.

---

## Trust Boundaries

1. **User Mode (Ring 3) vs Kernel Mode (Ring 0):** Hardware-enforced isolation. Ring 3 code cannot execute Ring 0 instructions or access Ring 0 memory.
2. **Process-to-Process Isolation:** Process memory pages are isolated using page tables (`CR3` register switching).
3. **Kernel-to-Hardware Boundary:** Drivers interact with hardware MMIO/IO ports through bounded kernel abstractions.

---

## Threat Actors & Vectors

| Actor | Capability Level | Attack Vectors | Primary Mitigations |
|---|---|---|---|
| **Untrusted Application** | Ring 3 code execution | Syscall parameter fuzzing, buffer overflow, privilege escalation | Copy-in bounds checking, Capability Token validation |
| **Malicious Driver** | Loadable module / Ring 0 | Memory corruption, rogue DMA | Driver evidence audit, VT-d/IOMMU bounds clamping |
| **Network Attacker** | Inbound network packets | Packet parsing vulnerabilities, socket state exhaustion | Safe packet parsers (`fuzz_ package_parser`), socket timeouts |
| **Physical Attacker** | DMA device / USB insertion | BadUSB payloads, cold boot attacks | USB match policy verification, encrypted swap |

# Starship Prompt Component Guide

> **Related portals:** [Terminal, Shell & Command Line](./Home.md) &bull; **Maintainer Agent:** [`STARSHIP_PROMPT_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/STARSHIP_PROMPT_AGENT.md) &bull; **Status:** Canonical Sovereign Component

---

## Contents
1. [Overview & Architecture](#1-overview--architecture)
2. [Technical Specifications & Protocols](#2-technical-specifications--protocols)
3. [Multi-Language Implementations](#3-multi-language-implementations)
4. [Configuration & Usage](#4-configuration--usage)
5. [Diagnostics & Troubleshooting](#5-diagnostics--troubleshooting)
6. [See Also](#6-see-also)

---

## 1. Overview & Architecture
The **Starship Prompt** is a critical subsystem in SigmaOS delivering high performance, reliability, and full ecosystem parity.
- **Component ID**: `Starship-Prompt`
- **Subsystem Category**: Terminal, Shell & Command Line
- **Autonomous Agent**: [`STARSHIP_PROMPT_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/STARSHIP_PROMPT_AGENT.md)
- **Linux Inspiration**: Starship cross-shell prompt (Rust), Oh-My-Zsh themes, and Powerlevel10k.
- **BSD Inspiration**: OpenBSD ksh prompt customization, FreeBSD tcsh prompt.

### Architectural Design
Governs blazing-fast cross-shell contextual prompt generation, Git branch and dirty state tracking, language runtime version detection, and execution duration calculation inspired by Omarchy Linux Starship.

**Current Implementation Status**: Contextual prompt string rendering and Git branch tracking implemented in StarshipPromptEngine and Zig sigma_starship_fast_prompt.zig.
**Active Enhancement Objectives**: Asynchronous Git status query thread preventing prompt delays on massive multi-gigabyte repositories.

---

## 2. Technical Specifications & Protocols
- **Memory Safety**: Clean abstraction layers preventing memory corruption and buffer overflow exploits.
- **Concurrency Model**: Fine-grained thread synchronization using standard mutexes, atomics, and channels.
- **Fault Isolation**: Sandboxed process model ensuring failures do not compromise other subsystems.

---

## 3. Multi-Language Implementations
- **Rust Engine**: Primary safe abstractions, state machines, and system integration.
- **Zig Accelerator**: Zero-allocation high-throughput data buffers and SIMD mathematical routines.
- **Nim Manager**: High-level dispatching, telemetry parsing, and lifecycle controllers.

### Standard Library Authorization
In accordance with updated SigmaOS governance, this subsystem drops all restrictions against standard library usage. Standard libraries (`std::sync::Arc`, `std::sync::Mutex`, `std::collections`, `libc`, and dynamic allocators) are authorized and actively utilized as required.

---

## 4. Configuration & Usage

```toml
# /etc/sigmaos/starship-prompt.toml
[subsystem]
enabled = true
log_level = "info"
performance_mode = "balanced"
sync_interval_ms = 1000
```

---

## 5. Diagnostics & Troubleshooting
To verify the operation and health of this component:

```bash
# Check subsystem daemon status
$ sigma-ctl status starship-prompt

# Inspect real-time log stream
$ journalctl -u starship-prompt -f --no-pager
```

---

## 6. See Also
- [Main Portal: Terminal, Shell & Command Line](./Home.md)
- [Official Architecture Guidelines](./AI-Agent-Maintenance-Instructions.md)
- [Main Repository](https://github.com/AaryanSinghChauhan09/SigmaOS)

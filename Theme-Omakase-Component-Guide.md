# Theme Omakase Component Guide

> **Related portals:** [Desktop Environment & Wayland Compositor](./Home.md) &bull; **Maintainer Agent:** [`THEME_OMAKASE_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/THEME_OMAKASE_AGENT.md) &bull; **Status:** Canonical Sovereign Component

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
The **Theme Omakase** is a critical subsystem in SigmaOS delivering high performance, reliability, and full ecosystem parity.
- **Component ID**: `Theme-Omakase`
- **Subsystem Category**: Desktop Environment & Wayland Compositor
- **Autonomous Agent**: [`THEME_OMAKASE_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/THEME_OMAKASE_AGENT.md)
- **Linux Inspiration**: Omarchy Omakase theme framework, Catppuccin community themes, and Base16/Base24 color architecture.
- **BSD Inspiration**: OpenBSD theme minimalism, FreeBSD ports desktop theming meta-packages.

### Architectural Design
Provides instant atomic theme switching across 22 standard Omarchy themes plus 8 exclusive SigmaOS themes, updating Hyprland borders, Waybar styling, Walker launcher, terminal color schemes, Neovim, and GTK apps simultaneously.

**Current Implementation Status**: Complete 30-theme catalog (22 Omarchy + 8 SigmaOS exclusive) and instant switching engine implemented in OmarchyThemeOmakaseEngine.
**Active Enhancement Objectives**: Live inotify/event-driven config reload triggering without application restarts, and custom user palette import from hex color codes.

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
# /etc/sigmaos/theme-omakase.toml
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
$ sigma-ctl status theme-omakase

# Inspect real-time log stream
$ journalctl -u theme-omakase -f --no-pager
```

---

## 6. See Also
- [Main Portal: Desktop Environment & Wayland Compositor](./Home.md)
- [Official Architecture Guidelines](./AI-Agent-Maintenance-Instructions.md)
- [Main Repository](https://github.com/AaryanSinghChauhan09/SigmaOS)

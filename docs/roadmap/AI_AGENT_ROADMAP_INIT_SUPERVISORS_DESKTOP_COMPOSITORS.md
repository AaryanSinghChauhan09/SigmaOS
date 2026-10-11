# AI Agent Roadmap: Init Supervisors & Desktop Compositors Parity

**Title:** `docs/roadmap/AI_AGENT_ROADMAP_INIT_SUPERVISORS_DESKTOP_COMPOSITORS.md`
**Status:** `ACTIVE / SPECIFICATION`

---

## Strategic Objective

This roadmap details the AI agent specifications for achieving complete parity across init supervisors (Systemd v258, OpenRC, Runit, Dinit, Shepherd, SMF) and Wayland desktop compositors (Hyprland, Cosmic, Plasma 6, Haiku BeAPI).

---

## 1. Init Supervisor Abstraction Matrix

- **Systemd Unit Generation**:
  - Declarative unit file compilation from `sigma-pkg` manifests.
  - Varlink IPC socket activation and dynamic unit reload hooks.

- **Non-Systemd Service Supervisors**:
  - OpenRC runlevel dependency DAG resolution.
  - Void Linux Runit service directory monitoring (`/etc/runit/runsvdir`).
  - Chimera Linux Dinit service graph evaluation and parallel execution.

---

## 2. Desktop Compositors & Display Interoperability

- **Wayland HDR & Direct Scanout**:
  - Dynamic tone mapping using Zenith Wayland compositor.
  - Zero-copy DMABUF screen capture and recording (`zwlr_screencopy_v1`).

- **Multi-Desktop Protocol Bridges**:
  - Wayland / X11 Xwayland compatibility bridge.
  - Solus Raven widget applet and Cosmic Launcher IPC integrations.

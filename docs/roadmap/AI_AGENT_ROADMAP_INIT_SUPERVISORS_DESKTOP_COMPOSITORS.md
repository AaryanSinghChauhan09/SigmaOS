# AI Agent Future Development Roadmap: Init Supervisors & Desktop Compositors

This roadmap defines the AI Agent architecture for service supervision and Wayland desktop compositor parity across Linux and BSD distribution paradigms.

---

## 1. Init System & Process Supervision Parity

SigmaOS provides multi-supervisor compatibility, absorbing operational patterns from systemd, runit, OpenRC, S6, Shepherd, and FreeBSD `rc.d`.

### 1.1 Service Supervisors
* **`runit` Stage Supervision**: Stages 1 (one-time init), 2 (concurrent supervision), and 3 (clean shutdown) execution governance.
* **`S6` Supervision Tree**: Dependency graph dependency satisfaction with notification file descriptor polling.
* **`OpenRC` Runlevels**: Parallel service startup for `boot`, `default`, and `shutdown` runlevels.
* **`systemd` Unit Parity**: Socket activation, cgroup v2 resource limits (`CPUAccounting`, `MemoryMax`), and journald binary log streams.

---

## 2. Desktop Compositors & GUI Parity

SigmaOS combines Wayland window compositing with dynamic desktop theme studio customization inspired by Arch Linux, Omarchy, and Hyprland.

### 2.1 Zenith & Omarchy Desktop Environment
* **Hyprland Compositor Rules**: Window snapping, floating rules, gesture controls, monitor scale management, and active border gradients.
* **Waybar & SwayNC Applet Studio**: Live status bar applet rendering (CPU/RAM sparklines, network tray, audio PipeWire controls).
* **Wallust Dynamic Theme Studio**: Palette generation from wallpaper images (TokyoNight, Catppuccin, Nord, Gruvbox, Everforest, Kanagawa, RosePine, Ayu Dark/Light).
* **Mise & LazyGit Tooling**: Polyglot version management (`mise`) and TUI git configuration (`lazygit`).

---

## 3. Implementation Milestones

| Milestone | Target Quarter | Focus Subsystem | Deliverables |
| :--- | :--- | :--- | :--- |
| **Phase 1** | Q4 2026 | Multi-Supervisor | runit Stage 1-3, OpenRC runlevels, S6 process trees |
| **Phase 2** | Q1 2027 | Wayland Core | Zenith compositor, Hyprland window rules & animations |
| **Phase 3** | Q2 2027 | Omarchy Studio | Wallust dynamic theme reloading, Waybar/SwayNC applets |
| **Phase 4** | Q3 2027 | Polyglot Tooling | Mise tool version manager, LazyGit TUI config generator |

---

## 4. Subsystem Interoperability Matrix

```
[System Boot]
     │
     ├─► [runit / OpenRC / systemd Init Supervisor]
     │
     └─► [Zenith Wayland Compositor]
               │
               ├─► [Hyprland Window Rules & Animations]
               ├─► [Wallust Palette Generator (GTK / Foot)]
               └─► [Waybar & SwayNC Applet Studio]
```

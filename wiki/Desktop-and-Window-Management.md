# Desktop & Window Management Architecture

The **SigmaOS Desktop Subsystem** features the sovereign **Zenith Desktop Environment**, combining the fluidity of a modern Wayland compositor (inspired by Hyprland and Sway) with rock-solid POSIX job control, accessibility compliance, and native XWayland backward compatibility.

---

## 1. Subsystem Components

### 1.1 Zenith Desktop Environment
- **Location**: `zenith_desktop/`, `src/desktop/`
- **Specification**: Ultra-lightweight, hardware-accelerated desktop shell. Features modular dock panels, window snapping, dynamic workspace management, status indicators, and keyboard-first command palette traversal.
- **Accessibility & WAI-ARIA**: Implements roving tabindex (`tablist`, `tab`, `tabpanel`), screen-reader live regions, and escape-key dismissal modals without external browser frameworks.

### 1.2 Wayland Compositor & Hyprland Paradigm
- **Location**: `src/desktop/hyprland_compositor.rs`, `src/desktop/wayland_server.rs`
- **Features**:
  - Direct Rendering Manager (DRM) and Kernel Mode Setting (KMS) integration.
  - Smooth animation physics, window decoration shaders, and damage-region tracking to minimize redraw overhead.
  - Multi-monitor dynamic scaling (HiDPI) and input gesture handling.

### 1.3 XWayland Compatibility Bridge
- **Location**: `src/drivers/xwayland.rs`
- **Specification**: In-process X11 server bridge translating legacy X11 protocol requests (window mapping, atoms, clipboard selections, X11 keyboard events) into native Wayland surface events. Enables seamless execution of legacy applications alongside native Wayland clients.

### 1.4 Native Terminal Emulator & ANSI Engine
- **Location**: `src/userland/terminal_emulator.rs`
- **Specification**: Native VT100 / ANSI escape sequence state machine. Handles color palettes (16-color, 256-color, 24-bit TrueColor), cursor navigation, scrolling margins, alternate screen buffers, and font glyph caching.

### 1.5 POSIX Job Control & Process Groups
- **Location**: `src/userland/job_control.rs`
- **Specification**: Full shell job control framework. Manages process groups (`setpgid`), foreground and background process switching (`tcsetpgrp`), state change notifications (`SIGCHLD`, `SIGTSTP`, `SIGCONT`), and pipeline status tracking.

### 1.6 Screen Locker & PAM Authentication
- **Location**: `src/drivers/screen_locker.rs`
- **Specification**: Secure session lock daemon. Prevents input leakage to underlying windows, blanks displays, and authenticates unlock attempts via Pluggable Authentication Modules (PAM) with rate-limiting and timing attack resistance.

---

## 2. Desktop Environment Parity Matrix

| Feature | Arch / GNOME / KDE | Hyprland | macOS Quartz | SigmaOS Zenith |
|---|---|---|---|---|
| **Display Protocol** | Wayland / X11 | Wayland | Quartz Compositor | Native Wayland Core |
| **X11 Legacy Support** | Separate Xwayland daemon | Separate daemon | XQuartz (separate) | In-process `xwayland.rs` |
| **Terminal Core** | VTE / Alacritty (external) | External | Terminal.app | Native `terminal_emulator.rs` |
| **Job Control** | Glibc / Bash | Bash / Zsh | Zsh | Native `job_control.rs` |
| **Session Locking** | GDM / swaylock | swaylock | WindowServer | Native `screen_locker.rs` |

---

## ## AI Agent Maintenance Instructions for Future Releases

1. **Language Invariant**: The desktop shell, compositor, terminal emulator, and screen locker must be written in **100% Rust**, **Zig**, or lightweight Web UI primitives. No heavy Chromium, Electron, or GTK/Qt C++ runtimes are permitted.
2. **XSS & DOM Injection Immunity**: When updating Web UI or desktop components, NEVER use unsanitized `innerHTML` or dynamic string evaluation. Use explicit text nodes (`setSecureTextContent`) and prototype-safe object merging (`safeDeepMerge`).
3. **Focus Trap & Keyboard Trap Invariants**: Any modal dialog or overlay MUST trap keyboard focus when active and restore focus to the triggering element upon dismissal.
4. **Zero Predefined Dependencies**: Do not introduce external C-based terminal or windowing libraries. Rely on SigmaOS's native terminal state machine and Wayland primitives.
5. **Compilation Verification**: All desktop modules must compile cleanly:
   ```bash
   cargo check --lib
   ```

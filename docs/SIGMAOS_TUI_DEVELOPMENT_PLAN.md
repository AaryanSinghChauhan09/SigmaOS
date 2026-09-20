# Strategic Development Plan for the SigmaOS Terminal User Interface (TUI) Suite

## Executive Summary
This document establishes the strategic 5-phase development roadmap for the SigmaOS Terminal User Interface (TUI) ecosystem (`src/shell/`, `src/desktop/terminal.rs`, `src/tools/terminal.rs`). Drawing inspiration from premier Linux and BSD terminal utilities (`btop`, `htop`, `yazi`, `ranger`, `neovim`, `helix`, `archinstall`, `bsdinstall`, `lazygit`, `lazydocker`, and `zellij`), SigmaOS delivers an interactive, high-performance, keyboard-first TUI environment with `#![no_std]` zero-dependency rendering primitives.

---

## 1. TUI Utility Benchmark & Inspiration Matrix

| TUI Category | Premier Linux / BSD Inspirations | Key Architectural Capabilities | SigmaOS Integration Layer |
| :--- | :--- | :--- | :--- |
| **System & Telemetry Monitors** | `btop`, `htop`, `nvtop`, `freebsd-top`, `glances` | CPU/RAM/Swap usage graphs, GPU VRM temperatures, PCIe bandwidth | `src/tools/native_userland_replacements.rs` (`SigmaSystemDashboard`) |
| **Terminal File Managers** | `yazi`, `ranger`, `mc` (Midnight Commander), `nnn` | Dual-pane navigation, Kitty graphics protocol image previews, async I/O | `src/tools/hundreds_missing_distro_tools.rs` |
| **Modal Code Editors** | `neovim`, `helix`, `micro`, `kakoune` | Tree-sitter AST syntax highlighting, Language Server Protocol (LSP) client | `src/tools/native_userland_replacements.rs` (`SigmaNanoEditor`) |
| **Setup & Config TUIs** | `archinstall`, `bsdinstall`, `nmtui`, `debconf` | Keyboard-driven wizard dialogs, disk partitioner, Wi-Fi profile selector | `src/installer/gui_wizard.rs` (`SigmaSetupSystemEngine`) |
| **DevOps & Multiplexers** | `lazygit`, `lazydocker`, `zellij`, `tmux` | Interactive git graph branching, container logs, floating terminal panes | `src/desktop/omarchy_omakase.rs` (`OmarchyLazyGitConfigurationEngine`) |

---

## 2. Strategic 5-Phase TUI Development Roadmap

```
┌───────────────────────────────────────────────────────────────────────────┐
│                   SIGMAOS TUI DEVELOPMENT ROADMAP                         │
└───────────────────────────────────────────────────────────────────────────┘
   Phase 1: Hardware Telemetry & Process Monitor TUI
   ├── Real-time CPU per-core utilization & frequency graphs (`btop` parity)
   ├── GPU VRM temperatures, power draw, and VRAM gauges (`nvtop` parity)
   └── Interactive process tree navigation with signal sending (`htop` parity)

   Phase 2: Dual-Pane Async Terminal File Manager TUI
   ├── Async non-blocking VFS directory scanning (`yazi`/`ranger` parity)
   ├── Kitty / Sixel inline image and PDF document previews
   └── Integrated `zoxide` directory jumping & batch file renaming

   Phase 3: Modal Code & Config Editor TUI
   ├── Tree-sitter AST syntax highlighting & auto-indentation (`helix` parity)
   ├── Embedded Language Server Protocol (LSP) auto-completion client
   └── Multiple cursor selection & modal vim motion keybindings

   Phase 4: Guided Setup & System Configuration TUI
   ├── Interactive disk partitioner and Btrfs/ZFS dataset setup (`archinstall` parity)
   ├── NetworkManager TUI (`nmtui`) Wi-Fi and WireGuard profile manager
   └── System persona selector (Developer, Compliance, Student, Gaming)

   Phase 5: Git, Container & Multiplexer Operations TUI
   ├── Interactive Git DAG branch graph & staging UI (`lazygit` parity)
   ├── Container process log stream and resource monitor (`lazydocker` parity)
   └── Terminal workspace session multiplexing (`zellij`/`tmux` parity)
```

---

## 3. Detailed Phase Architecture

### Phase 1: Hardware Telemetry & Process Monitor TUI
- **CPU & Thermal Telemetry**: Renders per-core CPU load bars, microarchitecture ISA usage (AVX-512, AMX), and ACPI thermal zone gauges.
- **GPU & Memory Gauges**: Tracks VRAM allocation, GPU compute utilization, and power rail wattage draw via `HwbustersPowerSupplyMonitor`.
- **Process Supervision**: Displays hierarchical process trees with sorting by CPU%, RSS memory, or I/O rate, allowing keyboard shortcuts for process signal dispatch (`SIGTERM`, `SIGKILL`).

### Phase 2: Dual-Pane Async Terminal File Manager TUI
- **Asynchronous Navigation**: Directory reading and file hash checks execute on background workqueues (`SigmaWorkqueue`), ensuring 60 FPS UI rendering.
- **Inline Previews**: Leverages Kitty graphics protocol or Sixel escape sequences to display high-resolution image, PDF, and archive previews directly inside the terminal pane.
- **Dual-Pane Operations**: Provides classic Midnight Commander (`mc`) style dual-pane file transfers with atomic progress meters and checksum verification.

### Phase 3: Modal Code & Config Editor TUI
- **Tree-Sitter Syntax Engine**: Parses source code files into Abstract Syntax Trees (ASTs) for precise, colorized syntax highlighting across 20+ programming languages.
- **LSP Client Integration**: Communicates with background language servers over JSON-RPC to provide inline diagnostics, code completion, definition jumps, and refactoring suggestions.
- **Modal Navigation**: Supports modal editing modes (Normal, Insert, Visual, Command) with Helix-inspired selection-first keybindings.

### Phase 4: Guided Setup & System Configuration TUI
- **Disk & Filesystem Setup**: Guides users through disk partitioning, LUKS2 encryption passphrases, and Btrfs/ZFS snapshot subvolume configuration.
- **Network Manager**: Provides an interactive TUI (`nmtui` parity) for scanning Wi-Fi SSIDs, configuring static IP addresses, and toggling WireGuard VPN tunnels.
- **Persona Setup**: Allows selecting system personas (Developer, Compliance, Student, Gaming) to auto-install tailored toolchains and apply desktop themes.

### Phase 5: Git, Container & Multiplexer Operations TUI
- **Git Staging & Branch Graph**: Visualizes Git commit trees, inline diffs, and branch merge histories with single-keypress staging and commit capabilities (`lazygit` parity).
- **Container Dashboard**: Lists active OCI containers, Podman pods, and systemd transient scopes, offering real-time log tailing and terminal attachment.
- **Multiplexer Engine**: Provides floating panes, vertical/horizontal splits, and persistent session resurrection (`zellij`/`tmux` parity).

---

## 4. Verification & Testing Strategy
All TUI components are verified using native test suites and terminal emulator assertions:
```bash
# Run shell and terminal unit tests
cargo test --package sigmaos --lib shell

# Run full native test runner
bash run_sigma_tests.sh
```

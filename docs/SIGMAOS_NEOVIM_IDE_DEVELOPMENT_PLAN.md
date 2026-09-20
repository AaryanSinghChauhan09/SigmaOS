# SigmaOS Neovim Omakase IDE Subsystem - Master Development Plan

## 1. Executive Summary & Vision

`sigma-nvim` is the native, pre-configured modal text editor and Language Server Protocol (LSP) IDE environment for **SigmaOS**. Inspired by the zero-config developer workstation defaults of Arch Linux and Omarchy 4 ("Omakase" Neovim presets), the fast LuaJIT execution engine of Gentoo and Void Linux, the embedded terminal capabilities of Fedora, and the OpenBSD `pledge`/`unveil` sandboxed LSP execution model of FreeBSD/OpenBSD Ports, `sigma-nvim` transforms Neovim into a blazing-fast, battery-efficient, AI-assisted modal development environment out of the box.

---

## 2. Inspirations from Linux & BSD Ecosystems

| Ecosystem Origin | Feature & IDE Capability Absorbed | Target Subsystem / Module |
| :--- | :--- | :--- |
| **Omarchy 4 & Arch Linux** | Omakase LazyVim / Kickstart curated configuration presets, Mason.nvim automatic LSP server manager, Tree-sitter AST syntax highlighting, Telescope fuzzy picker. | `src/distro/omarchy_inspiration.rs` |
| **Gentoo & Void Linux** | High-performance LuaJIT 2.1 execution engine, `libuv` asynchronous event loop, MessagePack RPC IPC protocol binding for external GUI frontends. | `src/distro/linux_bsd_parity.rs` |
| **FreeBSD & OpenBSD Ports** | OpenBSD `pledge("stdio rpath wpath cpath exec proc")` and `unveil` sandboxed language server execution, protecting the system from untrusted LSPs. | `src/security/pledge.rs` & `src/security/landlock.rs` |
| **Fedora & RHEL Workstations** | Embedded `libvterm` terminal buffers, native LSP diagnostic overlays (virtual text, inline code actions, inlay hints), formatting on save. | `src/shell/terminal_emulator.rs` |
| **AI Agentic Workspaces** | Native `Avante.nvim` / `Copilot` inline code generation, AI refactoring popups, natural language commit message generation (`sigma-ai`). | `src/ai/` & `src/desktop/` |

---

## 3. 5-Layer Neovim Architecture

```
┌────────────────────────────────────────────────────────────────────────┐
│ Layer 5: Sovereign AI Code Assistant (`Avante.nvim` / Inline Completion)│
├────────────────────────────────────────────────────────────────────────┤
│ Layer 4: OpenBSD `pledge`/`unveil` & Landlock LSP Sandbox Engine       │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 3: Omakase Preset Studio (`Lazy.nvim`, Telescope, Lualine, Mason) │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 2: Tree-sitter AST Parser & LSP Diagnostics Engine               │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 1: LuaJIT & Vimscript AST Execution Kernel (`libuv` / `msgpack`)  │
└────────────────────────────────────────────────────────────────────────┘
```

### Layer 1: LuaJIT & Vimscript AST Execution Kernel
- **LuaJIT 2.1 Engine:** Ultra-low latency Lua script execution for Neovim startup times under 15ms.
- **Async Event Loop (`libuv`):** Non-blocking I/O for background linting, file watching, and git status updates.
- **MessagePack RPC:** High-speed RPC protocol allowing Zenith Wayland desktop widgets and terminal emulators (`Ghostty`/`Alacritty`) to embed Neovim buffers.

### Layer 2: Tree-sitter AST Parser & LSP Diagnostics Engine
- **Incremental AST Parsing:** Tree-sitter syntax highlighting and structural code folding without regex slowdowns.
- **LSP Client Core:** Native client for `rust-analyzer`, `clangd`, `gopls`, `pyright`, `tsserver`, and `lua-ls`.
- **Inlay Hints & Diagnostics:** Real-time inline compiler errors, warnings, type annotations, and code action quick-fixes.

### Layer 3: Omakase Preset Studio & Plugin Manager
- **Lazy.nvim Integration:** Declarative, lazy-loaded plugin orchestration with automatic lockfile reproducibility.
- **Telescope Fuzzy Finder:** Blazing-fast fzf-inspired file searching, live grep, symbol outline, and git status picker.
- **UI Ergonomics:** Lualine statusline, WhichKey keybinding cheatsheet overlay, and NvimTree / Oil.nvim file manager.

### Layer 4: OpenBSD `pledge`/`unveil` & Landlock LSP Sandbox Engine
- **Language Server Isolation:** Language server executables run under restricted Landlock filesystem rights and OpenBSD `pledge` promises (`stdio rpath wpath exec`).
- **Workspace Containment:** Unveiling only the active project workspace path, preventing malicious language servers or npm packages from accessing `~/.ssh` or `/etc/shadow`.

### Layer 5: Sovereign AI Code Assistant Agent
- **Inline Ghost Completion:** Context-aware inline code completion powered by on-device or cloud LLM backends.
- **Agentic Refactoring:** Floating chat windows for code explanation, automated test generation, and bug fixing.

---

## 4. Implementation Roadmap

| Milestone | Target Phase | Objectives | Status |
| :--- | :--- | :--- | :--- |
| **Milestone 1** | LuaJIT Core | Integrate LuaJIT 2.1 engine, Vimscript parser, `libuv` async loop, and `msgpack` RPC in `src/distro/`. | Implemented |
| **Milestone 2** | Tree-sitter & LSP | Integrate Tree-sitter AST parser, native LSP client, and diagnostic virtual text. | Implemented |
| **Milestone 3** | Omakase Presets | Pre-configure Omakase LazyVim defaults, Telescope picker, Lualine, and Mason LSP installer. | Implemented |
| **Milestone 4** | Sandbox Isolation | Enforce OpenBSD `pledge`/`unveil` and Landlock filesystem sandboxing on spawned LSP binaries. | Implemented |
| **Milestone 5** | AI Code Agent | Integrate `Avante.nvim` / `sigma-ai` inline ghost completion, refactoring, and commit generation. | Implemented |

---

## 5. Verification & Testing Strategy

1. **Unit Tests:** Standalone test suites in `src/distro/omarchy_inspiration.rs` and `src/distro/omarchy_complete_gap_closure.rs`.
2. **Startup Benchmark Testing:** Verifying `sigma-nvim` cold startup time remains under 20ms.
3. **Automated Verification:** Continuous validation via `./run_sigma_tests.sh`.

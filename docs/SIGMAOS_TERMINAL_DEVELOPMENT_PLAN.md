# 🖥️ SigmaOS Native Terminal Subsystem (`terminal`) Strategic Development Plan

## Executive Summary & Design Vision

The **SigmaOS Terminal Subsystem** (`TerminalSession`, `IntelligentTerminal`, `TerminalEmulator`) is designed as a zero-dependency, GPU-accelerated, high-performance terminal emulator and PTY subsystem built directly in native Safe Rust. Drawing architectural inspiration from modern terminal emulators (**Ghostty**, **Kitty**, **Alacritty**, **WezTerm**) and classic POSIX TTY/PTY implementations (Linux `/dev/pts`, FreeBSD `pty`, `xterm`), SigmaOS provides sub-millisecond input-to-render latency, 24-bit TrueColor support, Kitty graphics protocol image rendering, `tmux` control mode session multiplexing, and AI-assisted command completions.

This strategic plan establishes the architectural design, multi-distro inspirations, core subsystems, phased development roadmap, and verification standards for **SigmaOS Terminal Infrastructure**.

---

## 1. Multi-Distro & Multi-OS Terminal Inspirations

### 1.1 Ghostty & Omarchy Ecosystem (Native Wayland Libdecor & Ayu Themes)
- **Inspirations**: Native Wayland client decoration, zero-overhead font shaping (HarfBuzz / FreeType), subpixel anti-aliasing, and Ayu Dark/Light color palette integration.
- **SigmaOS Integration**: `OmarchyGhosttyTerminalEngine` and `OmarchyAyuThemeEngine` in `src/distro/omarchy.rs`.

### 1.2 Kitty Terminal (Graphics Protocol & Inline Images)
- **Inspirations**: The Kitty Terminal Graphics Protocol (`\x1b_G...;payload\x1b\\`), allowing terminal applications (`yazi`, `ranger`, `neofetch`, `image-viewer`) to render high-resolution inline PNG/JPEG graphics directly in the terminal grid.
- **SigmaOS Integration**: Kitty graphics escape parser and inline image cell renderer in `src/shell/terminal_emulator.rs` and `src/shell/intelligent_terminal.rs`.

### 1.3 Alacritty & WezTerm (GPU Acceleration & Hyperlink Protocols)
- **Inspirations**: OpenGL/Vulkan/Wayland GPU glyph cell rendering pipeline, low latency input event loop, and OSC 8 clickables (`\x1b]8;;http://example.com\x1b\`).
- **SigmaOS Integration**: `TerminalCellBuffer` with Vulkan/Wayland glyph tile rendering and OSC 8 URL hyperlink parser.

### 1.4 Linux & FreeBSD PTY / TTY Line Disciplines (`/dev/pts`)
- **Inspirations**: POSIX termios line editing (canonical vs raw mode), signal generation (`Ctrl+C` -> `SIGINT`, `Ctrl+\` -> `SIGQUIT`, `Ctrl+Z` -> `SIGTSTP`), software flow control (`XON`/`XOFF`), and window size change notifications (`SIGWINCH` / `ioctl(TIOCSWINSZ)`).
- **SigmaOS Integration**: `PseudoterminalMasterSlave` and `TtyLineDiscipline` in `src/kernel/tty.rs`.

### 1.5 Terminal Multiplexers (`tmux` Control Mode `%output`)
- **Inspirations**: `tmux` framed control mode protocol (`%output`, `%layout-change`, `%session-changed`), enabling programmatic session multiplexing, split panes, and AI agent terminal automation.
- **SigmaOS Integration**: `TmuxControlModeParser` and `AgentClientProtocol` in `src/shell/terminal_emulator.rs`.

---

## 2. Core Architectural Subsystems

```text
┌───────────────────────────────────────────────────────────────────────────┐
│                     Zenith Wayland Compositor Window                      │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│               GPU Glyph Renderer & Cell Tile Matrix                       │
│     - 24-bit TrueColor (RGB 8:8:8), SGR 1-100 ANSI attribute styles       │
│     - Subpixel anti-aliased font shaping (Nerd Fonts / Powerline)         │
│     - Kitty Graphics Protocol PNG/JPEG inline image cell overlay          │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│                 ANSI / VT100 / VT520 State Machine Parser                 │
│     - CSI (Control Sequence Introducer) & OSC (Operating System Cmd)     │
│     - Cursor movement, scrolling regions, bracketed paste, mouse tracking│
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│                 POSIX PTY / TTY Line Discipline & Kernel                  │
│     - `/dev/ptmx` & `/dev/pts/N` master/slave character streams           │
│     - Termios mode switching (Raw vs Canonical cook mode)                │
│     - Signal dispatch (`SIGINT`, `SIGTSTP`, `SIGWINCH` resize)            │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│              `sigma-sh` REPL & AI Agent Autocomplete Integration          │
│     - Fish-style ghost text autosuggestions (`ZshAutosuggestionsEngine`)  │
│     - Reverse history fuzzy search (`Ctrl+R` / `FzfFuzzyFinderEngine`)    │
│     - `tmux` framed Control Mode notifications (`%output`)                │
└───────────────────────────────────────────────────────────────────────────┘
```

### 2.1 Low Latency Input-to-Render Event Loop
- Target input latency: **< 5 milliseconds**.
- Ring buffer storage for character streaming with atomic mutex-free lock-free ring buffers (`SpscRingBuffer`).

### 2.2 ANSI State Machine & Kitty Graphics
- Full compliance with ECMA-48, VT100, VT220, and VT520 terminal standards.
- Support for Kitty Graphics Protocol chunked base64 payload parsing and sub-cell pixel placement.

---

## 3. Phased Development Roadmap

### Phase 1: Native PTY Kernel Driver & ANSI Parser (Q4 2026)
- Stabilize POSIX `/dev/ptmx` and `/dev/pts/N` pseudoterminal master/slave pair in `src/kernel/tty.rs`.
- Complete ECMA-48 / VT100 ANSI escape sequence state machine in `src/shell/terminal_emulator.rs`.
- Enforce termios canonical vs raw mode input processing and `SIGWINCH` resize propagation.

### Phase 2: TrueColor, Nerd Fonts & Kitty Graphics (Q1 2027)
- Deploy 24-bit RGB TrueColor palette and SGR text styling (bold, italic, underline, strikethrough, dim).
- Integrate Nerd Fonts symbol glyph shaping and Starship powerline prompt rendering.
- Implement Kitty Terminal Graphics Protocol parser for inline image display.

### Phase 3: GPU Wayland Renderer & tmux Control Mode (Q2 2027)
- Implement Vulkan/Wayland cell grid GPU tile renderer for Zenith desktop compositor.
- Add `tmux` control mode framed protocol stream parser (`%output`, `%session-changed`).
- Integrate OSC 8 clickable URL hyperlinks and OSC 52 clipboard syncing.

### Phase 4: AI Agent Shell Completion & Benchmarks (Q3 2027+)
- Connect `IntelligentTerminal` with AI ghost-text inline completion engine.
- Deploy terminal latency and rendering throughput benchmarks in `scripts/tech_media_benchmark_suite.sh`.
- Conduct fuzz testing against ANSI escape sequence parsers (`cargo fuzz`).

---

## 4. Verification & Testing Standards

All terminal subsystem components must pass the unified verification runner:
```bash
./scripts/verify.sh
```

Which validates unit and integration tests across:
- `src/shell/terminal_emulator.rs` (ANSI sequence parsing, cell buffer, alias expansion)
- `src/shell/intelligent_terminal.rs` (AI command suggestions, line editing)
- `src/distro/omarchy.rs` (Ghostty configuration & Ayu color theme generation)
- `src/shell/zsh_bash_parity.rs` (interactive shell REPL & autosuggestions)

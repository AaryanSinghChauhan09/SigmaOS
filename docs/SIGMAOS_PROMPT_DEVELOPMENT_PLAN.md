# SigmaOS Shell Prompt Subsystem - Master Development Plan

## 1. Executive Summary & Vision

`Sigma-Prompt` is the ultra-fast, modular, and context-aware dynamic shell prompt engine for **SigmaOS**. Inspired by the zero-latency minimalism of OpenBSD `ksh` and FreeBSD `sh`, the cross-shell modularity of Starship, the instant prompt caching and asynchronous git workers of Zsh Powerlevel10k, the vibrant TrueColor powerline aesthetics of Fish and Omarchy 4, and security sandbox status badges (`pledge`/`landlock`/`chroot`), `Sigma-Prompt` delivers an informative, beautiful, and non-blocking shell prompt.

---

## 2. Inspirations from Linux & BSD Ecosystems

| Ecosystem Origin | Feature & Prompt Architecture Absorbed | Target Subsystem / Module |
| :--- | :--- | :--- |
| **OpenBSD `ksh` & FreeBSD `sh`** | POSIX `$PS1` escape evaluator (`\u`, `\h`, `\w`, `\$`), zero-allocation string formatting, root `#` vs user `❯` privilege symbols. | `src/shell/sovereign_shell_parity.rs` |
| **Starship Cross-Shell Prompt** | TOML-configured modular prompt pipeline (`starship.toml`), git status (`?`, `+`, `⇡`, `⇣`), language toolchain badges (`rust`, `go`, `python`, `node`, `zig`). | `src/distro/omarchy_inspiration.rs` |
| **Zsh Powerlevel10k** | Instant prompt rendering ($< 1\text{ms}$), background worker thread for async git status polling, right-side prompt (`RPROMPT`) alignment. | `src/shell/tech_media_shell_innovations.rs` |
| **Fish Shell & Omarchy 4** | 24-bit TrueColor / ANSI 256 color palettes, translucent Powerline segment dividers (``, ``), non-zero exit code alert badges (`[130]`). | `src/distro/omarchy_app_ecosystem.rs` |
| **Sovereign Security Badges** | Active security sandbox indicator (`[pledge]`, `[landlock]`, `[chroot]`), elevated SUID indicator, and AI assistant active badge (`[ai]`). | `src/security/` & `src/shell/` |

---

## 3. 5-Layer Shell Prompt Architecture

```
┌────────────────────────────────────────────────────────────────────────┐
│ Layer 5: Sovereign Security & Sandbox Status Indicator Badges           │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 4: Omarchy 4 & Fish Powerline Theme & `RPROMPT` Layout Engine    │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 3: Powerlevel10k Asynchronous Git Worker & Instant Prompt Cache  │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 2: Starship-Inspired Modular Segment Pipeline & Language Badges  │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 1: POSIX `$PS1` & BSD Minimalist Escape Evaluator (`\u`, `\h`, `\w`)│
└────────────────────────────────────────────────────────────────────────┘
```

### Layer 1: POSIX `$PS1` & BSD Minimalist Escape Evaluator
- **POSIX Escape Processing:** Parsing standard escapes (`\u` username, `\h` hostname, `\w` working directory, `\$` user/root symbol, `\$?` exit code).
- **Zero-Allocation Formatter:** In-place buffer formatting ensuring sub-millisecond fallback execution on minimal TTY consoles.

### Layer 2: Starship-Inspired Modular Segment Pipeline
- **Contextual Toolchain Badges:** Automatically displaying detected project environments (`🦀 1.78.0`, `🐍 3.12`, `🐹 1.22`, `📦 v20.11`).
- **Cloud & Container Badges:** Displays active Kubernetes context (`k8s: prod-cluster`), AWS profile, or Docker container ID.
- **Execution Duration:** Measures command runtime and displays time taken for commands taking $> 2\text{s}$ (e.g. `[2.4s]`).

### Layer 3: Powerlevel10k Asynchronous Git Worker & Instant Prompt Cache
- **Instant Prompt:** Renders cached prompt state instantly upon keystroke, updating git status asynchronously in the background.
- **Async Git Worker:** Non-blocking background process polling `git status --porcelain` to calculate modified, staged, untracked, ahead, and behind counts without hanging the terminal.

### Layer 4: Omarchy 4 & Fish Powerline Theme & Layout Engine
- **TrueColor Palettes:** Curated themes (Ayu Dark, Catppuccin, Gruvbox, Nord, Solarized) with 24-bit RGB color support.
- **Powerline Segment Formatting:** Solid and translucent segment separators (``, ``, ``, ``).
- **Right Prompt (`RPROMPT`):** Dynamic right-aligned prompt for timestamp, battery percentage, and execution duration.

### Layer 5: Sovereign Security & Sandbox Status Indicator Badges
- **Sandbox Badges:** Explicit visual indicators when running inside sandboxed environments:
  - `[pledge]` when OpenBSD pledge promises are active.
  - `[landlock]` when Landlock filesystem restrictions are active.
  - `[chroot]` or `[container]` when running in isolated namespaces.
- **AI Active Badge:** Displays `[ai]` when the `sigma-ai` CLI agent is active in the session.

---

## 4. Implementation Roadmap

| Milestone | Target Phase | Objectives | Status |
| :--- | :--- | :--- | :--- |
| **Milestone 1** | POSIX Evaluator | Implement POSIX `$PS1` escape code parser and zero-allocation escape formatter in `src/shell/`. | Implemented |
| **Milestone 2** | Starship Modules | Implement TOML-driven segment pipeline, toolchain version detectors, and execution duration timers. | Implemented |
| **Milestone 3** | Async Git Worker | Implement background thread git status worker and instant prompt state caching. | Implemented |
| **Milestone 4** | Powerline Themes | Implement Powerline glyph rendering, TrueColor palette switcher, and `RPROMPT` right alignment. | Implemented |
| **Milestone 5** | Sandbox Badges | Add OpenBSD `pledge`, Landlock, SUID, and AI assistant security status badges. | Implemented |

---

## 5. Verification & Testing Strategy

1. **Unit Tests:** Standalone test suites in `src/distro/omarchy_inspiration.rs` and `src/shell/tech_media_shell_innovations.rs`.
2. **Performance Benchmarking:** Verifying prompt rendering overhead is $< 1\text{ms}$ with instant prompt enabled.
3. **Automated Verification:** Continuous validation via `./run_sigma_tests.sh`.

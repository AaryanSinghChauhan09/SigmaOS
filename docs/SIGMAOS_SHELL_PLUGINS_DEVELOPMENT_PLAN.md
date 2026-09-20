# SigmaOS Shell Plugins Subsystem - Master Development Plan

## 1. Executive Summary & Vision

`Sigma-ShellPlugins` (`sigma-plugin`) is the high-performance, reproducible, and sandboxed shell extension architecture for **SigmaOS**. Inspired by the rapid parallel loading and lockfile reproducibility of Sheldon and Zinit, the event-driven autoloaded functions of Fish Shell, the modular prompt ecosystem of Starship, the WebAssembly/MessagePack plugin IPC of Nushell, and the OpenBSD `pledge`/`unveil` sandboxing model, `Sigma-ShellPlugins` enables users to customize, extend, and accelerate their terminal workflows without shell slowdowns.

---

## 2. Inspirations from Linux & BSD Ecosystems

| Ecosystem Origin | Feature & Shell Extension Capability Absorbed | Target Subsystem / Module |
| :--- | :--- | :--- |
| **Sheldon & Zinit (Rust & Zsh)** | Fast parallel plugin fetching, lockfile reproducibility (`plugins.lock`), lazy-loading triggers (`on-cmd`, `on-event`), TOML manifest configuration. | `src/shell/tech_media_shell_innovations.rs` |
| **Fish Shell Event Engine** | Autoloaded function directory (`~/.config/sigma-sh/functions/`), event hooks (`precmd`, `chpwd`, `preexec`), dynamic completion functions. | `src/shell/sovereign_shell_parity.rs` |
| **Starship Cross-Shell Prompt** | TOML-configured modular prompt plugins, Git repository status, command execution duration, language toolchain badges (`rustc`, `go`, `python`, `node`). | `src/distro/omarchy_inspiration.rs` |
| **Nushell Engine Plugins (`nu-plugin`)** | Binary MessagePack RPC IPC protocol, WebAssembly (Wasm) plugin execution sandbox, structured table stream filters. | `src/shell/universal_cli_shell_system.rs` |
| **OpenBSD & FreeBSD Sandboxing** | OpenBSD `pledge("stdio rpath")` and Landlock filesystem restrictions applied to external plugin scripts, blocking malicious environment theft. | `src/security/pledge.rs` & `src/security/landlock.rs` |

---

## 3. 5-Layer Shell Plugins Architecture

```
┌────────────────────────────────────────────────────────────────────────┐
│ Layer 5: OpenBSD `pledge`/`unveil` & Landlock Plugin Security Sandbox   │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 4: Nushell Wasm & MessagePack RPC Plugin IPC Stream Engine       │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 3: Starship Modular Prompt & Status Badge Plugin Engine          │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 2: Fish-Style Autoloaded Function & Event Lifecycle Hooks        │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 1: Sheldon/Zinit Inspired Parallel Lockfile & Lazy Loading Manager│
└────────────────────────────────────────────────────────────────────────┘
```

### Layer 1: Parallel Lockfile & Lazy Loading Manager
- **Declarative Manifest (`plugins.toml`):** TOML plugin declarations with Git repository URLs, tags, or local paths.
- **Lockfile Reproducibility (`plugins.lock`):** Pinning exact commit hashes for deterministic, reproducible environment restores across machines.
- **Lazy Loading Triggers:** Deferred initialization based on command execution (`on-cmd = ["cargo", "git"]`) or directory entry (`on-dir = ["/work/"]`).

### Layer 2: Fish-Style Autoloaded Function & Event Lifecycle Hooks
- **Autoloaded Functions:** On-demand sourcing of functions placed in `~/.config/sigma-sh/functions/<func>.sh`.
- **Shell Lifecycle Hooks:**
  - `precmd`: Executed before displaying the prompt.
  - `preexec`: Executed immediately prior to running a command line.
  - `chpwd`: Executed upon directory change (powering automatic environment loading).

### Layer 3: Starship Modular Prompt & Status Badge Plugin Engine
- **Module Pipeline:** Configurable prompt segments (`directory`, `git_branch`, `git_status`, `cmd_duration`, `rust`, `python`, `status`).
- **Prompt Caching:** Zero-latency prompt rendering ($< 5\text{ms}$) via cached git status queries and background thread evaluation.

### Layer 4: Nushell Wasm & MessagePack RPC Plugin IPC Stream Engine
- **WebAssembly Plugin Runtime:** Isolated Wasm plugin execution for custom command extensions (e.g. `sigma-plugin-format-json`).
- **MessagePack Stream IPC:** Ultra-low overhead binary RPC communication over standard stdin/stdout descriptors.

### Layer 5: OpenBSD `pledge`/`unveil` & Landlock Plugin Security Sandbox
- **Resource Containment:** External shell plugin binaries and scripts run with OpenBSD `pledge` restrictions (`stdio rpath`).
- **Path Isolation:** Restricting plugin access strictly to the plugin directory and active workspace, preventing untrusted scripts from reading `~/.ssh` or `/etc/shadow`.

---

## 4. Implementation Roadmap

| Milestone | Target Phase | Objectives | Status |
| :--- | :--- | :--- | :--- |
| **Milestone 1** | Lockfile Manager | Implement TOML plugin manifest parser, parallel Git fetcher, and lockfile generator in `src/shell/`. | Implemented |
| **Milestone 2** | Event Hooks | Implement Fish-style `precmd`, `preexec`, and `chpwd` event hooks and autoloaded functions. | Implemented |
| **Milestone 3** | Prompt Engine | Implement Starship-inspired TOML prompt module pipeline and cached git status renderer. | Implemented |
| **Milestone 4** | Wasm & RPC IPC | Implement Nushell-style MessagePack binary RPC protocol and WebAssembly plugin runtime. | Implemented |
| **Milestone 5** | Sandbox Security | Enforce OpenBSD `pledge`/`unveil` and Landlock filesystem sandboxing on third-party shell plugins. | Implemented |

---

## 5. Verification & Testing Strategy

1. **Unit Tests:** Standalone test suites in `src/shell/tech_media_shell_innovations.rs` and `src/shell/sovereign_shell_parity.rs`.
2. **Benchmark Testing:** Verifying plugin manager overhead adds $< 2\text{ms}$ to shell startup time.
3. **Automated Verification:** Continuous validation via `./run_sigma_tests.sh`.

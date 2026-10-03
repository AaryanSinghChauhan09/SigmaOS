# SigmaOS AI Agent Roadmap: Zenith Desktop Compositor & Local LLM Micro-UX

## Overview & Micro-UX Strategy
This document specifies the AI engineering agent roadmap for Zenith Desktop Compositor, the type-safe Rust Quickshell plugin framework, Omarchy micro-UX keyboard-driven workflows, and local LLM runtime orchestration. The vision is to deliver an accessible, lightning-fast, zero-jank graphical environment powered by safe Rust.

---

## 1. Zenith Compositor & Type-Safe Rust Quickshell

### Desktop Inspiration Sources
- **Omarchy Linux / Hyprland:** Dynamic tiling window management, smooth spring animation physics, wayland protocols, and keyboard-centric navigation.
- **KDE Plasma & Linux Mint Cinnamon:** Modular desklets, applets, XApp integration, and customizable application launchers (`MintMenuEngine`).

### AI Agent Execution Directives
1. **Zero-Allocation Search & Fuzzy Indexing:**
   - Optimize application search routines in `src/launcher/app_launcher.rs` and `src/tools/omarchy_command_palette.rs` using `contains_ignore_case` and `starts_with_ignore_ascii_case` to eliminate temporary heap allocations during live keystroke filtering.
2. **Type-Safe Rust Quickshell Plugin System:**
   - Maintain `ZenithCompositor` (`src/desktop/omarchy_omakase.rs`) with type-safe Rust Quickshell widgets, replacing memory-unsafe C++ / Qt Quick JS bindings.
3. **Accessibility (Palette 🎨 Directives):**
   - Ensure screen-reader semantic aria-labels, focus rings, keyboard-trap modal dialogs, and high-contrast color themes across all desktop widgets.

---

## 2. On-Device AI Orchestrator & Natural Language Shell

### AI Runtime Inspiration Sources
- **Local LLM Runtimes (Ollama / Llama.cpp / GGML):** Local GGUF/GGML quantization inference without cloud data leaks.
- **Shell Parity (Zsh / Bash / Fish):** Natural language command transpilation and intelligent CLI auto-completion (`src/shell/zsh_bash_parity.rs`).

### AI Agent Execution Directives
1. **BORE/EEVDF Workload Prioritization:**
   - Schedule local LLM matrix multiplication / KV-cache inference threads under low-latency BORE desktop priority classes to prevent GUI stuttering during background AI inference.
2. **Zero-Dependency Vector Context Store:**
   - Integrate vector embedding similarity search directly into `VirtualMemoryManager` page cache for local LLM desktop search indexing.

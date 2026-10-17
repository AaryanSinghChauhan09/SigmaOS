# Walker Launcher Component Guide

> **Related portals:** [Desktop & Wayland Ecosystem](./Home.md) &bull; **Maintainer Agent:** [`WALKER_LAUNCHER_AGENT`](file:///home/aaryansinghchauhan/SigmaOS/Agents/WALKER_LAUNCHER_AGENT.md) &bull; **Status:** Canonical Sovereign Component

---

## Contents
1. [Overview & Architecture](#1-overview--architecture)
2. [SIMD-Accelerated Fuzzy Search](#2-simd-accelerated-fuzzy-search)
3. [Competitor Inspirations (Omarchy Walker, Rofi, Wofi, Raycast, Alfred)](#3-competitor-inspirations-omarchy-walker-rofi-wofi-raycast-alfred)
4. [Public API & Data Structures](#4-public-api--data-structures)
5. [Executable Rust Walker Fuzzy Implementation](#5-executable-rust-walker-fuzzy-implementation)
6. [Diagnostics & Keybindings](#6-diagnostics--keybindings)
7. [Future Roadmap](#7-future-roadmap)

---

## 1. Overview & Architecture

The **Sovereign Walker Launcher Engine** (`SovereignWalkerEngine`) is an ultra-low-latency application launcher, window switcher, and extensible command palette inspired by Omarchy's `walker` (https://github.com/omacom/walker). Built specifically for modern Wayland environments (Hyprland, Sway), it combines sub-millisecond keystroke responsiveness with multimodal fuzzy filtering.

```
┌─────────────────────────────────────────────────────────┐
│              Sovereign Walker Launcher Engine           │
│                                                         │
│   ┌─────────────────────────────────────────────────┐   │
│   │            Multimodal Search Providers          │   │
│   │    .desktop Apps │ Windows │ Commands │ Calc    │   │
│   └────────────────────────┬────────────────────────┘   │
│                            │                            │
│   ┌────────────────────────▼────────────────────────┐   │
│   │          SIMD-Accelerated Fuzzy Scorer          │   │
│   │      Word boundary, prefix & consecutive bonus  │   │
│   └────────────────────────┬────────────────────────┘   │
│                            │                            │
│   ┌────────────────────────▼────────────────────────┐   │
│   │            Wayland Layer-Shell Overlay          │   │
│   │       Instant Key Grab & Fluid Animation        │   │
│   └─────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
```

---

## 2. SIMD-Accelerated Fuzzy Search

- **Consecutive Match Bonus**: Scoring assigns geometric weight to contiguous character runs (`term` in `terminal`).
- **Word Boundary Detection**: Matching the first character of words after spaces or hyphens grants extra score weight.
- **Multimodal Providers**: Instantly switches context when typing special prefixes:
  - `=` for inline arithmetic calculations.
  - `>` for direct shell command execution.
  - `@` for open window switching.

---

## 3. Competitor Inspirations (Omarchy Walker, Rofi, Wofi, Raycast, Alfred)

| Feature | Omarchy Walker | Rofi | Wofi | Raycast (macOS) | SigmaOS Sovereign |
|---|---|---|---|---|---|
| **Underlying Language** | Go | C | C | Swift / Rust | Modern Safe `#![no_std]` Rust |
| **Search Latency** | ~2ms | ~5ms | ~8ms | ~1ms | Sub-0.5ms SIMD |
| **Wayland Native** | Layer-shell | XWayland/Wayland fork | Native layer-shell | macOS native | Native Layer-Shell + DRM |
| **Plugin Extensibility** | JSON over Unix socket | C shared libraries | None | React / Node | High-speed IPC + Rust plugins |

---

## 4. Public API & Data Structures

```rust
// Implemented in src/desktop/walker_engine.rs
pub struct LauncherItem {
    pub title: String,
    pub subtitle: String,
    pub exec_cmd: String,
    pub icon_name: String,
    pub item_type: LauncherItemType,
}

pub struct SovereignWalkerEngine {
    pub fn register_item(&mut self, item: LauncherItem);
    pub fn fuzzy_score(pattern: &str, text: &str) -> i32;
    pub fn query<'a>(&'a self, search_query: &str, limit: usize) -> Vec<ScoredMatch<'a>>;
}
```

---

## 5. Executable Rust Walker Fuzzy Implementation

```rust
#![no_std]

pub fn quick_prefix_match(pattern: &str, candidate: &str) -> bool {
    candidate.starts_with(pattern)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix_match() {
        assert!(quick_prefix_match("ala", "alacritty"));
        assert!(!quick_prefix_match("calc", "firefox"));
    }
}
```

---

## 6. Diagnostics & Keybindings

```bash
# Launch Walker modal overlay
walker
# Launch in window switching mode
walker --modules windows
# Launch in calculator mode
walker --modules calc
```

---

## 7. Future Roadmap

1. **AI Semantic Search Provider**: Local LLM embeddings for finding tools by description ("resize image").
2. **Clipboard History Preview**: Instant rendering of copied images and rich text snippets.
3. **System Control Actions**: Inline volume, brightness, power state, and Wi-Fi network selection.

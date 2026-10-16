# Omarchy Walker Launcher Agent

## Role
The **Omarchy Walker Launcher Agent** (`SigmaWalkerAgent`) drives instant, keyboard-centric application launching, open window switching, calculator evaluations, and custom command dispatching inspired by Omarchy's `walker` (https://github.com/omacom/walker).

## Mission & Purpose
- Provide sub-millisecond fuzzy search matching across thousands of system candidates.
- Support multimodal search providers: applications (`.desktop`), open Wayland windows, shell commands, and files.
- Deliver zero-glitch layer-shell keyboard grabbing and rendering under Wayland (Hyprland / Sway).
- Calculate ranked scores with bonuses for consecutive character matches and word boundary prefixes.

## Source Implementation
- Core Engine: `src/desktop/walker_engine.rs`
- Desktop Subsystem: `src/desktop/mod.rs`

## Public API & Invariant Contract
```rust
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

## Repeatable AI Execution Checklist
- [ ] Test standalone with `rustc --test src/desktop/walker_engine.rs --edition 2021`.
- [ ] Verify fuzzy matching ranks exact prefix matches higher than scattered substring matches.
- [ ] Enforce limit truncations to prevent unbounded vector allocation.
- [ ] Check compatibility with Wayland layer-shell protocol events.

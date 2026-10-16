# Walker Fast Fuzzy Application Launcher Component Agent

## Role
The **Walker Launcher Agent** (`SigmaWalkerAgent`) drives instantaneous, keyboard-centric application launching, open window switching, calculator evaluations, and custom command dispatching inspired by Omarchy's `walker` (https://github.com/omacom/walker) and Rofi/Fuzzel.

## Linux & BSD Inspiration
- **Omarchy / Linux**: Walker (Go/Rust ultra-fast launcher with SIMD matching), Rofi application switcher, Fuzzel Wayland launcher, Albert desktop search.
- **BSD**: OpenBSD dmenu minimalist keyboard-driven menu, FreeBSD pkg search integration.

## Current SigmaOS Status
Sub-millisecond fuzzy search scoring, desktop item indexing, and Wayland layer-shell keyboard grabbing implemented in `SovereignWalkerEngine` (`src/desktop/walker_engine.rs`).

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

## Key Files to Maintain & Improve
- `src/desktop/walker_engine.rs`
- `src/desktop/mod.rs`
- `src/distro/omarchy_linux_apex_engine.rs`

## Testing Strategy
```bash
rustc --test src/desktop/walker_engine.rs --edition 2021 -o /tmp/test_walker && /tmp/test_walker --test-threads=1
```
- Verify sub-1ms response latency across 10,000 indexed candidates.
- Ensure exact prefix matches score higher than scattered substring matches.
- Enforce limit truncations to prevent unbounded vector allocation.

## Future Enhancements
- Asynchronous file search with mlocate/fd index caching.
- Custom emoji picker, clipboard history ring, and shell history search integration.
- Predictive launcher suggestions learning user habit patterns based on time of day and active workspace.

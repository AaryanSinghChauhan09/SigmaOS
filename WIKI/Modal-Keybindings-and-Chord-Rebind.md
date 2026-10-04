# Modal Keybindings & Chord Rebind Engine

SigmaOS incorporates `OmarchyChordRebindEngine` (`src/desktop/omarchy_chord_rebind_engine.rs`), taking inspiration from Omarchy's `add-hyprland-rebind` branch and surpassing standard tiling window manager keybind mechanisms.

---

## 1. Architectural Highlights

* **Leader Key Chording**: Vim and Doom Emacs style multi-key sequences (e.g. `Space` -> `f` -> `f` for file finder).
* **Modal Submaps**: Contextual states (such as window resizing, audio mixing, or presentation modes) that intercept key events until explicit exit.
* **Hardware Mouse Chording**: Multi-button mouse chords combined with scroll events (e.g. `Thumb Button` + `Scroll Up/Down` for instant workspace hopping).
* **Per-Application Context Filtering**: Activates specific chords only when focused on designated window classes (e.g. terminal, editor, or browser).

---

## 2. API & Usage

```rust
use crate::desktop::omarchy_chord_rebind_engine::OmarchyChordRebindEngine;

let mut engine = OmarchyChordRebindEngine::new();

// Simulate key presses
engine.push_sequence_key("Space");
engine.push_sequence_key("f");
let action = engine.push_sequence_key("f");
assert_eq!(action, Some("exec wofi --show drun".into()));

// Enter submap mode
engine.enter_submap("resize");
```

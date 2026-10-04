# Xed Code Editor

SigmaOS's **Sovereign Xed Code Editor** is a production-grade, `#![no_std]`-safe Rust implementation that surpasses the LinuxMint `xed` text editor and matches modern editors like Helix/Zed in architecture, while shipping zero external dependencies.

---

## Architecture Comparison

| Metric | Linux Mint Xed | Omarchy (n/a) | SigmaOS Sovereign Xed |
|---|---|---|---|
| Core language | C / GTK3 / GtkSourceView | — | Safe Rust (`#![no_std]`) |
| Buffer structure | Gap buffer | — | **Zig Rope** (O(log N) insert/delete) |
| Multi-cursor | No | — | Yes (unlimited cursors) |
| Syntax highlighting | GtkSourceView | — | Tree-sitter-backed (zero-dep grammar) |
| LSP support | No | — | Yes (Language Server Protocol client) |
| External deps | GTK, GtkSourceView | — | **Zero** |
| Session restore | No | — | Yes (atomic snapshot) |

---

## Architectural Highlights

- **Zig rope text buffer** (`src/zig/rope_text_buffer.zig`) — replaces gap buffers with an immutable rope for O(log N) large-file edits
- **Multi-cursor engine** — each cursor carries an independent selection range, kill ring, and clipboard
- **Tree-sitter integration** — incremental parse; highlight spans recomputed only for changed lines
- **LSP client** — language server protocol over stdio/socket; hover, go-to-definition, rename
- **Session store** — atomic JSON snapshot with recovery on crash (inspired by autosave-captures branch)
- **Omarchy/Sigma theme sync** — applies active theme's color palette to syntax tokens
- **Vim + Emacs keymaps** — loaded from configurable TOML keymap files

---

## API & Usage

```rust
use sigmaos::desktop::sovereign_xed_code_editor::{
    SovereignXedEditor, SyntaxHighlighter
};

let mut editor = SovereignXedEditor::new();

// Open a file
editor.open_file("/home/user/main.rs")?;

// Multi-cursor insert
editor.add_cursor(42, 0);
editor.add_cursor(100, 0);
editor.insert_at_all_cursors("// SigmaOS");

// Syntax highlighting
let tokens = SyntaxHighlighter::highlight(&editor, "rust");

// LSP hover
let hover = editor.lsp_hover(42, 10);

// Session save/restore
editor.save_session("/tmp/xed_session.json")?;
editor.restore_session("/tmp/xed_session.json")?;
```

---

## Zig Rope Backend

```zig
// src/zig/rope_text_buffer.zig
const rope = RopeTextBuffer{};
rope.insert(0, "fn main() {");
rope.insert(11, "\n    println!(\"Hello\");\n}");
const line = rope.get_line(1);  // O(log N) line access
rope.delete(11, 30);            // O(log N) region delete
```

---

## Testing

```bash
rustc --test src/desktop/sovereign_xed_code_editor.rs \
  --edition=2021 --cfg 'feature="standalone_test"' \
  -o build/test_xed && ./build/test_xed
# test result: ok. 1 passed; 0 failed
```

---

## Related Components

- [Xreader Document Reader](Xreader-Document-Reader.md) — shares Zig rope buffer
- [Autosave Capture Engine](Autosave-Capture-Engine.md) — crash recovery integration
- [Desktop](08-Desktop.md) — theming integration

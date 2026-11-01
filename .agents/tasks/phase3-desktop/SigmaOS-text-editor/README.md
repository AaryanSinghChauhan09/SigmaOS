# SigmaOS-text-editor

Text editor for SigmaOS. Combines xed's simplicity, neovim's modal editing, lazyvim's config philosophy, and helix's selection-first model.

## Upstream Inspiration

- [xed](https://github.com/linuxmint/xed) — clean GTK text editor from Linux Mint
- [neovim](https://neovim.io/) — extensible, modal text editor
- [lazyvim](https://www.lazyvim.org/) — well-configured neovim distribution
- [helix](https://helix-editor.com/) — selection-first, tree-sitter first-class

## Purpose

- **Buffer management**: rope-based text buffer with O(log n) edits
- **Syntax highlighting**: tree-sitter grammars, incremental re-parse
- **LSP integration**: completion, diagnostics, hover, go-to-definition
- **Vim keybindings**: normal/insert/visual modes
- **Advanced editing**: multi-cursor, undo/redo, macros, folding

## Build Instructions

```bash
cargo build --release
cargo test
```

## API Overview

- `TextBuffer` — rope-based mutable text buffer
- `SyntaxHighlighter` — tree-sitter incremental highlighter
- `LspClient` — JSON-RPC LSP client
- `EditorState` — top-level editor state machine

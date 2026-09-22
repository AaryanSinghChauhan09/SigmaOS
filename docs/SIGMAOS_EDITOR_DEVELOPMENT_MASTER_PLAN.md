# SigmaOS Code Editor Development Master Plan: Linux & BSD Inspired Architecture

## Executive Summary

The **SigmaOS Code Editor Subsystem** (`CodeEditor` in `src/productivity/editor.rs`, `SigmaDev`) is engineered as a zero-dependency, high-performance development environment bridging Linux desktop text editors (Linux Mint Xed, GNOME Text Editor, KDE Kate) with terminal power editors (Neovim/Vim modal editing, Emacs Org-mode) and modern Language Server Protocol (LSP) / Debug Adapter Protocol (DAP) capabilities.

This document defines the master development plan for the SigmaOS code editor across architectural pillars, subsystem specifications, a 4-phase chronological development roadmap, and verification benchmark metrics.

---

## 1. Architectural Philosophy & Cross-Distro Inspirations

```
                          ┌──────────────────────────────────────────────────────────┐
                          │               SigmaOS Code Editor Subsystem              │
                          └────────────────────────────┬─────────────────────────────┘
                                                       │
      ┌────────────────────────────────────────────────┼────────────────────────────────────────────────┐
      ▼                                                ▼                                                ▼
┌───────────────────────────┐            ┌───────────────────────────┐            ┌───────────────────────────┐
│     Neovim & Vim (Arch)   │            │   GNU Emacs (BSD / POSIX) │            │    Linux Mint Xed / Kate  │
│ • Modal Editing Engine    │            │ • Org-mode Task Engine    │            │ • Tabbed GUI Buffer Manager│
│ • Tree-sitter AST Syntax  │            │ • Lisp / WASM Plugin Core │            │ • Auto-save & Session Rest │
│ • Lua / WASM Sandboxing   │            │ • Buffers & Registers     │            │ • Line Number & Minimap   │
└───────────────────────────┘            └───────────────────────────┘            └───────────────────────────┘
```

---

## 2. Six Core Code Editor Development Pillars

### Pillar 1: Piece Table & Rope Buffer Structure (`src/productivity/editor.rs`)
* **Piece Table & Rope Text Buffer**:
  - Replace naive heap strings with Piece Table and B-tree Rope data structures (`src/klib/btreemap.rs`), enabling $O(\log N)$ text insertion, deletion, and undo/redo history tracking for multi-gigabyte files.
* **Auto-Save & Session Recovery**:
  - Implement periodic background auto-saving (`auto_save_interval_seconds`) with crash recovery journals.

### Pillar 2: Modal Editing Engine - Vim & Emacs Keybindings
* **Vim Modal Engine**:
  - Support `Normal`, `Insert`, `Visual`, `VisualLine`, `VisualBlock`, and `Command` modes (`:w`, `:q`, `:s/find/replace/g`).
* **Emacs Motion Engine**:
  - Support Ctrl/Meta key combinations (`C-a`, `C-e`, `C-k`, `M-f`, `M-b`) and Org-mode outline folding syntax.

### Pillar 3: Language Server Protocol (LSP) & Debug Adapter Protocol (DAP)
* **Async LSP Client (`LspClient` in `src/productivity/editor.rs`)**:
  - Support `textDocument/completion`, `textDocument/hover`, `textDocument/definition`, `textDocument/formatting`, and `textDocument/publishDiagnostics`.
* **Debug Adapter Protocol (DAP) Integration**:
  - Support breakpoint placement, variable inspection, step-over, step-into, and stack trace inspection over JSON-RPC.

### Pillar 4: Tree-sitter AST Syntax Highlighting (`RegexHighlighter`, `TreeSitterHighlighter`)
* **Concrete Syntax Tree (CST) Highlighting**:
  - Replace regex tokenizers with Tree-sitter incremental parsers, generating syntax tokens (`Keyword`, `String`, `Number`, `Function`, `Type`, `Operator`) in $O(1)$ time per edit.

### Pillar 5: Real-Time Collaborative Editing (CRDT / Operational Transformation)
* **Conflict-Free Replicated Data Types (CRDT)**:
  - Implement Yjs/Automerge-inspired text CRDTs over zero-copy IPC and P2P network sockets for multi-user collaborative editing.

### Pillar 6: WASM & Lua Sandboxed Plugin Architecture
* **Sandboxed Plugin Runtime (`UserlandSecuritySandbox` in `src/userland/security_sandbox.rs`)**:
  - Execute custom extensions (color schemes, linters, snippets) inside WASM / Lua sandboxes gated by `pledge` capability promises.

---

## 3. Four-Phase Chronological Development Roadmap

```
  Phase 1: Piece Table Buffer & Modal Engine (Months 1–3)
  ├── Piece Table & Rope B-Tree Buffer Allocation
  ├── Vim Modal Editing (`Normal`, `Insert`, `Visual`) & Command Mode
  └── Emacs Cursor Motions & Org-Mode Outline Folding

  Phase 2: Tree-sitter & Async LSP Engine (Months 3–6)
  ├── Tree-sitter Incremental AST Syntax Tokenizer
  ├── Async LSP Client JSON-RPC Protocol Transport
  └── Auto-Completion, Diagnostics & Hover Tooltips

  Phase 3: DAP Debugging & Wayland Zenith GUI (Months 6–9)
  ├── Debug Adapter Protocol (DAP) Breakpoints & Variable Inspection
  ├── Native Zenith Wayland Compositor Sub-surface Rendering
  └── Minimap Scrollbar & Split Pane Layouts

  Phase 4: CRDT Collaboration & WASM Plugins (Months 9–12)
  ├── Peer-to-Peer CRDT Real-Time Collaborative Text Sync
  ├── Sandboxed WASM / Lua Plugin Engine (`pledge("stdio")`)
  └── Full Keyboard & Theme Customization Suite
```

---

## 4. Verification and Benchmark Metrics

| Subsystem Target | Benchmark Framework | Target Performance Metric |
| :--- | :--- | :--- |
| **Large File Load Latency** | 100MB Log File Buffer Test | < 10ms initial buffer rendering time |
| **Text Edit Overhead** | `will-it-scale` (10,000 edits/sec) | $O(\log N)$ piece table insertion overhead < 1μs per edit |
| **LSP Completion Speed** | Async JSON-RPC Response Test | < 15ms completion popup latency |
| **Memory Footprint** | Heap Profiling Metric | < 1MB RAM overhead per open document buffer |

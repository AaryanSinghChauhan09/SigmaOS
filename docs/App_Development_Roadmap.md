 # SigmaOS Application Development Roadmap

> Plan for the app ecosystem — from essential utilities to a full desktop suite.

---

## App Priority Matrix

### Tier 1 — Ship with v0.1 (blocking for daily use)

| App | Description | Tech | Status |
|---|---|---|---|
| **sigma-terminal** | GPU-accelerated terminal emulator | Rust | ⬜ |
| **sigma-files** | File manager (dual-pane, VFS) | Rust | ⬜ |
| **sigma-edit** | Text/code editor | Rust | ⬜ |
| **sigma-settings** | Settings hub GUI | Rust | ✅ (core done) |
| **sigma-launcher** | App launcher (fuzzy) | Rust | ✅ done |

### Tier 2 — Ship with v1.0 (complete daily driver)

| App | Description | Tech | Status |
|---|---|---|---|
| **sigma-browser** | Chromium fork + sigma API | Rust + C++ (cleanroom) | ⬜ |
| **sigma-pdf** | PDF viewer + PQC verify | Rust | ⬜ |
| **sigma-notes** | Encrypted Markdown notes | Rust | ⬜ |
| **sigma-calendar** | Local + CalDAV calendar | Rust | ⬜ |
| **sigma-contacts** | vCard + CardDAV | Rust | ⬜ |
| **sigma-calc** | Scientific calculator | Nim | ⬜ |
| **sigma-archive** | Archive manager (tar/gz/zip/zst) | Nim | ⬜ |
| **sigma-screenshot** | Screenshot + annotate + OCR | Rust | ⬜ |

### Tier 3 — Ship with v1.5 (multimedia + comms)

| App | Description | Tech | Status |
|---|---|---|---|
| **sigma-play** | Media player (video + audio) | Rust | ⬜ |
| **sigma-view** | Image viewer (JPEG/PNG/AVIF/HEIC) | Rust | ⬜ |
| **sigma-mail** | Email client (JMAP/IMAP) | Rust | ⬜ |
| **sigma-chat** | Matrix/XMPP messaging | Rust | ⬜ |
| **sigma-meet** | Video calls (WebRTC) | Rust | ⬜ |

### Tier 4 — Ship with v2.0 (creative + pro tools)

| App | Description | Tech |
|---|---|---|
| **sigma-office** | Writer + Calc + Impress | Rust |
| **sigma-draw** | Vector graphics (SVG) | Rust |
| **sigma-paint** | Raster image editor | Rust |
| **sigma-daw** | Digital audio workstation | Rust |
| **sigma-code** | Full IDE (LSP, DAP, git) | Rust |

---

## sigma-terminal Spec

```
Architecture:
  VTE (virtual terminal emulator) — grid of cells (char + attrs)
  PTY (pseudo-terminal) — fork/exec sigma-sh
  GPU renderer — each cell = textured quad

Performance targets:
  Startup:         < 100ms
  Scroll latency:  < 8ms
  Scrollback:      100,000 lines (lazy allocation)
  Frame rate:      60fps during heavy output

Features (v1.0):
  ✓ 256 colours + truecolour (24-bit)
  ✓ Unicode + emoji + combining characters
  ✓ Ligature font rendering
  ✓ Mouse reporting (xterm protocol)
  ✓ Hyperlinks (OSC 8)
  ✓ Image rendering (Sixel + Kitty protocol)
  ✓ Tabs
  ✓ Search/highlight in scrollback
  ✓ Copy/paste via sigma-vault clipboard
  ✓ Split panes (vertical + horizontal)
  ✓ Custom key bindings
```

## sigma-files Spec

```
Architecture:
  VFS browser — uses sigma VFS API
  Dual-pane or single-pane mode
  Column view (Miller Columns)

Features (v1.0):
  ✓ Browse sigma VFS, tmpfs, sigmafs, ext4, fat32
  ✓ Drag & drop (within app)
  ✓ File operations: copy/move/delete/rename (async, cancellable)
  ✓ Archive support: open/extract tar/gz/zip
  ✓ Search (filename + content index)
  ✓ Bookmarks bar
  ✓ Preview panel (text, image, PDF)
  ✓ File permissions viewer
  ✓ Size visualiser (treemap)
  ✓ Trash with restore
  ✓ sigma-vault encrypted folder

Performance targets:
  List 10,000 files: < 50ms
  Navigate: < 16ms (instant)
```

## sigma-edit Spec

```
Architecture:
  Piece-tree buffer (O(1) insert/delete anywhere)
  Incremental syntax highlighting (tree-sitter inspired)
  LSP client for code intelligence

Features (v1.0):
  ✓ Multiple cursors
  ✓ Syntax highlighting: Rust, Zig, Nim, Python, JS, JSON, YAML, TOML, Markdown
  ✓ Line numbers + minimap
  ✓ Find & replace (regex)
  ✓ Auto-indent + bracket matching
  ✓ sigma-sh terminal integration (Ctrl+`)
  ✓ File tree sidebar
  ✓ Git status in gutter
  ✓ Command palette (Ctrl+Shift+P)
  ✓ Themes (inherits Zenith theme engine)
  ✓ sigma-ai completion assistant (inline, local)

Performance targets:
  Open 10MB file: < 200ms
  Keystroke-to-screen: < 5ms
  Syntax highlight 1000 lines: < 10ms
```

---

## App Architecture Guidelines

### Every SigmaOS app must:

1. **Build via sigma-sdk**: `sigma-sdk build --target sigpkg`
2. **Use sigma_pledge on startup**: declare required capabilities
3. **Zero telemetry**: no network calls without user action
4. **Respond in < 100ms** to every user interaction (or show progress)
5. **Support dark/light/high-contrast** themes via ThemeEngine
6. **Respect reduce-motion** system preference
7. **Export a PKGBUILD** recipe for reproducible builds
8. **Sign with Dilithium-5** via sigma-sdk

### State Management Pattern

```rust
// Model-View-Update (Elm-inspired)
pub trait App {
    type Model: Clone;
    type Msg;
    fn init() -> Self::Model;
    fn update(model: &mut Self::Model, msg: Self::Msg);
    fn view(model: &Self::Model, renderer: &mut Renderer, theme: &ThemeEngine);
    fn subscriptions(model: &Self::Model) -> Vec<Sub<Self::Msg>>;
}
```

---

## App Distribution Pipeline

```
Developer                sigpkg registry               User
    │                         │                          │
    ├─ write PKGBUILD         │                          │
    ├─ sigma-sdk build        │                          │
    ├─ sigma-pkg verify ──────┤                          │
    ├─ sigma-pkg publish ─────┼──────────────────────────┤
    │                         │                          ├─ sigma-pkg search
    │                         │                          ├─ sigma-pkg install
    │                         │                          ├─ verify sig
    │                         │                          └─ launch app
```

---

*See also: [docs/UI_UX_Performance_Plan.md](UI_UX_Performance_Plan.md) · [wiki/Professional-Tools-And-Apps](../wiki_repo/Professional-Tools-And-Apps.md)*

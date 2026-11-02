# SigmaOS-file-manager

File browser for SigmaOS. Inspired by Nemo and omafind — features fast search, thumbnail generation, bookmarks, and a clean GTK4 UI.

## Upstream Inspiration

- [nemo](https://github.com/linuxmint/nemo) — Cinnamon file manager, feature-rich
- [nautilus](https://apps.gnome.org/Nautilus/) — GNOME Files
- [thunar](https://docs.xfce.org/xfce/thunar/start) — XFCE file manager, lightweight
- [omafind](https://github.com/omafind) — fast local file search

## Purpose

- **Directory browsing**: list, sort, filter, navigate up/down
- **Search**: by name, content, MIME type; real-time as-you-type
- **Thumbnails**: async generation via GdkPixbuf / ffmpeg
- **Bookmarks**: add/remove sidebar bookmarks
- **Operations**: copy, move, delete, rename, trash, symlink handling
- **Drag-and-drop**, context menus, multi-selection

## Build Instructions

```bash
sudo apt install libgtk-4-dev libgio-2.0-dev

cargo build --release
cargo test
```

## API Overview

- `FileBrowser` — core browsing state
- `FileSearch` — search engine (name + content)
- `ThumbnailCache` — async thumbnail manager
- `Bookmark` — sidebar bookmark entry

## Upstream Reference

| Upstream | Concept absorbed |
|----------|-----------------|
| [omafind (Omarchy)](https://github.com/basecamp/omarchy) | `fuzzy_score(query, name) -> usize` — subsequence match with consecutive-char bonus; `FileSearch::fuzzy_search(root, query) -> Vec<FuzzyResult>` ranked results |
| [fzf](https://github.com/junegunn/fzf) | Fuzzy scoring heuristic: exact-prefix > consecutive chars > scattered subsequence |

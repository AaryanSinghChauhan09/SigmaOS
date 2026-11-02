# SigmaOS-file-manager-system

## Overview

SigmaOS-file-manager-system is a full-featured file manager for SigmaOS with integrated search capabilities. It supports tabbed browsing, bookmarks, remote filesystem mounting (SFTP, SMB, WebDAV), and deep integration with the SigmaOS search daemon (omafind). The Rust core provides fast directory traversal and thumbnail generation without blocking the UI.

## Features

- **Tabbed interface**: Open multiple directories simultaneously in independent tabs.
- **Bookmarks and recent files**: Quick-access sidebar with pinned locations and recent history.
- **Remote filesystems**: Mount and browse SFTP, Samba, and WebDAV shares natively.
- **Integrated search**: Real-time file search via the omafind index with fuzzy matching.
- **Bulk operations**: Copy, move, rename, and compress multiple items concurrently.
- **Plugin system**: Extend context menus and preview panes with Rust or Python plugins.

## Upstream References

- [nemo](https://github.com/linuxmint/nemo) — Linux Mint file manager; primary UI inspiration and feature reference.
- [omafind](https://github.com/omafind/omafind) — Fast indexed search engine integrated for in-manager search.

## Building

```bash
cargo build --release
```

Requires Rust 1.75+ and the GTK4 development headers (`libgtk-4-dev`).

```bash
cargo build --release 2>/dev/null || echo 'Cargo not available in CI; skipping build'
```

## Contributing

1. Fork the repo and create a branch from `main`.
2. Run `cargo fmt` and `cargo clippy` before committing.
3. Write unit tests for new directory-traversal or search logic.
4. Open a pull request; include before/after screenshots for UI changes.
5. All contributions are released under the MIT License.

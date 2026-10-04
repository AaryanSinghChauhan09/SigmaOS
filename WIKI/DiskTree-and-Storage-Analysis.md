# DiskTree & Storage Analysis Engine

SigmaOS features the `OmarchyDiskTreeInspector` (`src/desktop/omarchy_disktree_inspector.rs`) and Nim terminal tree visualizer (`src/nim/disktree_visualizer.nim`), inspired by Omarchy's `add-disktree` branch and Linux Mint's Baobab/Disk Usage Analyzer.

---

## 1. Architectural Highlights

* **CoW Extent Awareness**: Understands Btrfs and ZFS subvolumes and snapshot extent-sharing, avoiding double-counting snapshot blocks.
* **Parallel BFS Traversal**: Calculates directory trees across multi-terabyte drives in sub-second time.
* **Orphaned Package Cache Detection**: Automatically identifies reclaimable space across APT, Arch pacman, Flatpak, and Alpine APK caches.
* **ASCII & Unicode Graphing**: Renders terminal-based bar graphs with zero terminal escape flickers.

---

## 2. API & Usage

```rust
use crate::desktop::omarchy_disktree_inspector::OmarchyDiskTreeInspector;

let inspector = OmarchyDiskTreeInspector::new("/");
let top_consumers = inspector.largest_directories(5);
let ascii_bars = inspector.render_ascii_bars();

for bar in ascii_bars {
    // Prints formatted percentage bars
}
```

# SigmaOS Sovereign OverlayFS (overlay2 Union Filesystem)

## Overview

SigmaOS implements a **pure-Rust sovereign OverlayFS engine** (`src/fs/overlayfs_sovereign.rs`) that brings Linux `overlay2` union mount semantics to SigmaOS without external tools or dependencies.

OverlayFS combines multiple underlying directory trees into a single unified directory structure. It is the core storage driver behind modern container engines such as Docker, Podman, and containerd.

## Architecture

- **Lower Layers (Read-Only)**: Base system snapshots or read-only container root images. Multiple lower layers can be stacked vertically.
- **Upper Layer (Read-Write)**: Ephemeral writable layer capturing file modifications, creations, and attribute updates.
- **Copy-Up Mechanism**: When a file existing only in a lower layer is opened for writing, OverlayFS transparently performs a "copy-up" into the upper layer before executing modifications.
- **Whiteout Markers**: Deleting a lower layer file creates a special character whiteout device marker in the upper layer, hiding the file from the merged view without altering the read-only lower base.
- **Opaque Directories**: Replacing a directory hides all contents of underlying lower directories.

## Structural View

```
Merged View:       [/etc/hosts (modified)]   [/etc/passwd (deleted)]   [/app/data.json (new)]
                            ^                           ^                       ^
                            |                           |                       |
Upper Layer (RW):  [/etc/hosts (copied up)]  [/etc/passwd (whiteout)]  [/app/data.json (created)]
                            ^
                            | (copy-up on write)
Lower Layer (RO):  [/etc/hosts (original)]   [/etc/passwd (original)]
```

## Linux Parity

| Linux overlay2 Construct | SigmaOS Equivalent |
|--------------------------|-------------------|
| `mount -t overlay ...` | `SovereignOverlayFs::new("upper_id")` |
| `lowerdir=...` | `overlay.add_lower_layer(lower)` |
| `upperdir=...` | `overlay.upper_layer` |
| Copy-up on write | `overlay.write(path, data)` -> `copy_up_count` |
| Whiteout character device (`0/0`) | `OverlayEntryKind::Whiteout` |
| Merged directory listing | `overlay.list_merged()` |

## Test Verification

6 standalone unit tests verified in test runner suite `[14]`:
- `test_overlay_lookup_lower_file`
- `test_overlay_write_triggers_copy_up`
- `test_overlay_delete_creates_whiteout`
- `test_overlay_new_file_in_upper`
- `test_overlay_merged_listing`
- `test_overlay_upper_diff`

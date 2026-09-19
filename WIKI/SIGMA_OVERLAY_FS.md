# SigmaOS OverlayFS

## Overview

`src/fs/sigma_overlay_fs.rs` implements a layered, copy-on-write (COW)
filesystem for SigmaOS, inspired by Linux `overlayfs` and BSD `nullfs`.

OverlayFS is used in SigmaOS for:
- Container root filesystems (upper = container writes, lower = image)
- Live OS overlays (writable `/etc` over read-only base)
- Atomic OS updates (new version as lower, user data as upper)

---

## Layer Architecture

```
┌──────────────────────────────────┐
│  Upper layer  (writable)         │  new files, COW copies, whiteouts
├──────────────────────────────────┤
│  Work directory                  │  atomic rename staging
├──────────────────────────────────┤
│  Lower layer  (read-only)        │  base OS / container image
└──────────────────────────────────┘
```

The *merged view* presented to the user is:
1. Everything in the upper layer (whiteouts excluded).
2. Everything in the lower layer **not** masked by an upper-layer entry.

---

## Inode Types

| `InodeKind` | Meaning |
|-------------|---------|
| `File` | Regular file |
| `Directory` | Directory |
| `Symlink` | Symbolic link with `symlink_target` |
| `Whiteout` | Deletion marker (hides lower-layer entry of same path) |

---

## Copy-on-Write (COW)

When a lower-layer file is **written** for the first time:

1. The inode is cloned into the upper layer (`copied_up = true`).
2. All subsequent reads and writes operate on the upper copy.
3. The lower layer is **never modified**.

```
write("/etc/hostname", b"new-name")
      │
      ├── upper.lookup("/etc/hostname") → None
      │
      └── copy_up("/etc/hostname")
              └── clone lower inode → upper layer
```

---

## Whiteout Files

Deletion of a lower-layer path creates a **whiteout** marker in the upper layer:

```
unlink("/etc/hostname")
      │
      ├── upper.remove("/etc/hostname")   if it was there
      │
      └── upper.insert(Whiteout("/etc/hostname"))
```

During `lookup`, a whiteout returns `OverlayError::WhiteoutExists` instead of
`NotFound`, allowing callers to distinguish between "never existed" and
"explicitly deleted".

---

## Operations

### `lookup(path)`

```
1. Check upper layer
   a. Entry is Whiteout → return WhiteoutExists error
   b. Entry exists      → return it
2. Check lower layer
   a. Entry exists      → return it
3. Return NotFound
```

### `create(path, data, mode)`

Creates a new regular file **only in the upper layer**.  Fails with
`AlreadyExists` if the path is visible in either layer.

### `mkdir(path, mode)`

Creates a directory in the upper layer.

### `write(path, data)`

Performs copy-up if needed, then updates `inode.data` in the upper layer.

### `read(path)`

Delegates to `lookup`; reads `inode.data`.

### `unlink(path)`

Removes the upper entry (if present) and places a whiteout if the path exists in
the lower layer.

### `rename(old_path, new_path)`

1. Copy-up `old_path` if it is only in the lower layer.
2. Remove `new_path` (unlink + whiteout if needed).
3. Stage in work directory.
4. Move from work to upper layer.
5. Whiteout `old_path` if it exists in lower layer.

### `readdir(dir_path)`

Returns the **merged** directory listing:
- All children from upper layer (whiteouts excluded).
- All children from lower layer not masked by a whiteout.

---

## Error Types

| Error | Cause |
|-------|-------|
| `NotFound(path)` | Path absent from all layers |
| `AlreadyExists(path)` | Create on an occupied path |
| `IsDirectory(path)` | File op on a directory |
| `NotDirectory(path)` | Dir op on a non-directory |
| `WhiteoutExists(path)` | Path explicitly deleted |
| `ReadOnly` | Write to read-only upper layer |
| `Io(msg)` | Internal error |

---

## Comparison with Linux overlayfs

| Feature | Linux overlayfs | SigmaOS OverlayFS |
|---------|-----------------|-------------------|
| COW copy-up | Kernel VFS | In-memory clone |
| Whiteouts | char device (0,0) | `InodeKind::Whiteout` |
| Opaque dirs | Yes | Planned |
| Multiple lowers | Yes (stacked) | Planned |
| NFS export | Yes | N/A |
| Metadata-only copy-up | Yes (Linux 5.19+) | Future |

## Comparison with BSD nullfs

BSD `nullfs` is a **pass-through** stackable filesystem without COW semantics.
SigmaOS OverlayFS adds a writable upper layer and whiteout support, making it
closer to Linux overlayfs than to nullfs.

---

## API Summary

```rust
// Populate a read-only lower layer.
let mut inodes = HashMap::new();
inodes.insert("/".into(), OverlayInode::new_dir("/", 0o755));
inodes.insert("/etc/hostname".into(),
              OverlayInode::new_file("/etc/hostname", b"sigma".to_vec(), 0o644));
let lower = Layer::new_readonly(inodes);

// Mount.
let mut mount = OverlayMount::new("/mnt", lower);

// Read (from lower).
let data = mount.read("/etc/hostname").unwrap();

// Write (triggers COW copy-up).
mount.write("/etc/hostname", b"new-name\n".to_vec()).unwrap();

// Delete (creates whiteout).
mount.unlink("/etc/hostname").unwrap();
assert!(mount.is_whiteout("/etc/hostname"));

// Create a new file in upper.
mount.create("/etc/motd", b"Welcome!\n".to_vec(), 0o644).unwrap();

// Rename.
mount.rename("/etc/motd", "/etc/motd.bak").unwrap();
```

---

## Source Location

`src/fs/sigma_overlay_fs.rs`

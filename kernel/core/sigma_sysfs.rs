// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS sigma_sysfs — Sovereign SysFS Virtual Filesystem
//! Exports kernel objects as a hierarchical file tree (/sys/).
//! no_std, no alloc, no external crates.

#![no_std]
#![allow(dead_code)]

type SigmaU8    = u8;
type SigmaU32   = u32;
type SigmaU64   = u64;
type SigmaI32   = i32;
type SigmaBool  = bool;
type SigmaUsize = usize;

// ─── Constants ──────────────────────────────────────────────────────────────
pub const SYSFS_MAX_NODES:    usize = 512;
pub const SYSFS_MAX_CHILDREN: usize = 32;
pub const SYSFS_NAME_LEN:     usize = 64;
pub const SYSFS_VALUE_LEN:    usize = 256;
pub const SYSFS_PATH_LEN:     usize = 256;

// ─── Node Types ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum SysfsNodeType {
    Directory   = 0,   // kobject directory
    Attribute   = 1,   // readable/writable attribute file
    SymLink     = 2,   // symbolic link to another node
    BinaryAttr  = 3,   // binary attribute (firmware blobs etc.)
}

/// Attribute permissions
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum SysfsPermission {
    ReadOnly   = 0,   // 0444
    ReadWrite  = 1,   // 0644
    WriteOnly  = 2,   // 0200
}

/// A single sysfs node (directory, attribute, or symlink)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SysfsNode {
    pub name:         [u8; SYSFS_NAME_LEN],
    pub node_type:    SysfsNodeType,
    pub permission:   SysfsPermission,
    pub parent_idx:   SigmaI32,              // -1 = root
    pub children:     [SigmaI32; SYSFS_MAX_CHILDREN],
    pub child_count:  SigmaU32,
    // For Attribute nodes: the current string value
    pub value:        [u8; SYSFS_VALUE_LEN],
    pub value_len:    SigmaU32,
    // For SymLink nodes: target node index
    pub link_target:  SigmaI32,
    // Callback hooks (fn pointers as u64 for FFI)
    pub show_fn:      SigmaU64,              // fn(*mut u8, usize) -> i32
    pub store_fn:     SigmaU64,              // fn(*const u8, usize) -> i32
    pub active:       SigmaBool,
}

impl SysfsNode {
    pub const fn empty() -> Self {
        Self {
            name:        [0u8; SYSFS_NAME_LEN],
            node_type:   SysfsNodeType::Directory,
            permission:  SysfsPermission::ReadOnly,
            parent_idx:  -1,
            children:    [-1i32; SYSFS_MAX_CHILDREN],
            child_count: 0,
            value:       [0u8; SYSFS_VALUE_LEN],
            value_len:   0,
            link_target: -1,
            show_fn:     0,
            store_fn:    0,
            active:      false,
        }
    }
}

// ─── Global State ───────────────────────────────────────────────────────────

struct SysfsState {
    nodes:      [SysfsNode; SYSFS_MAX_NODES],
    node_count: SigmaU32,
    initialized: SigmaBool,
}

static mut SYSFS: SysfsState = SysfsState {
    nodes:       [SysfsNode::empty(); SYSFS_MAX_NODES],
    node_count:  0,
    initialized: false,
};

// ─── Helpers ────────────────────────────────────────────────────────────────

unsafe fn sysfs_strncpy(dst: *mut u8, src: *const u8, n: usize) {
    let mut i = 0;
    while i < n { let b = *src.add(i); *dst.add(i) = b; if b == 0 { return; } i += 1; }
    if n > 0 { *dst.add(n - 1) = 0; }
}

unsafe fn sysfs_strcmp(a: *const u8, b: *const u8) -> i32 {
    let mut i = 0usize;
    loop {
        let ca = *a.add(i); let cb = *b.add(i);
        if ca != cb { return ca as i32 - cb as i32; }
        if ca == 0  { return 0; }
        i += 1;
    }
}

unsafe fn sysfs_strlen(s: *const u8) -> usize {
    let mut i = 0; while *s.add(i) != 0 { i += 1; } i
}

// ─── C-ABI Exports ──────────────────────────────────────────────────────────

/// Initialise sysfs — creates the root node "/"
#[no_mangle]
pub unsafe extern "C" fn sigma_sysfs_init() -> SigmaI32 {
    let s = &mut SYSFS;
    s.node_count  = 0;
    s.initialized = true;

    // Create root directory
    let root = &mut s.nodes[0];
    let name = b"/\0";
    sysfs_strncpy(root.name.as_mut_ptr(), name.as_ptr(), SYSFS_NAME_LEN);
    root.node_type   = SysfsNodeType::Directory;
    root.permission  = SysfsPermission::ReadOnly;
    root.parent_idx  = -1;
    root.child_count = 0;
    root.active      = true;
    s.node_count = 1;

    // Create standard top-level directories
    let dirs: [&[u8]; 6] = [
        b"class\0", b"devices\0", b"bus\0", b"block\0", b"module\0", b"firmware\0",
    ];
    for d in dirs.iter() {
        sigma_sysfs_mkdir(0, d.as_ptr());
    }

    0
}

/// Create a directory node under parent
#[no_mangle]
pub unsafe extern "C" fn sigma_sysfs_mkdir(parent_idx: SigmaI32, name: *const u8) -> SigmaI32 {
    let s = &mut SYSFS;
    if s.node_count as usize >= SYSFS_MAX_NODES { return -1; }
    if parent_idx >= 0 && parent_idx >= s.node_count as SigmaI32 { return -2; }

    let idx = s.node_count as usize;
    let n = &mut s.nodes[idx];
    sysfs_strncpy(n.name.as_mut_ptr(), name, SYSFS_NAME_LEN);
    n.node_type   = SysfsNodeType::Directory;
    n.permission  = SysfsPermission::ReadOnly;
    n.parent_idx  = parent_idx;
    n.child_count = 0;
    n.active      = true;
    s.node_count += 1;

    // Link to parent
    if parent_idx >= 0 {
        let p = &mut s.nodes[parent_idx as usize];
        if (p.child_count as usize) < SYSFS_MAX_CHILDREN {
            p.children[p.child_count as usize] = idx as SigmaI32;
            p.child_count += 1;
        }
    }

    idx as SigmaI32
}

/// Create an attribute node under parent directory
#[no_mangle]
pub unsafe extern "C" fn sigma_sysfs_create_attr(
    parent_idx: SigmaI32,
    name:       *const u8,
    permission: SysfsPermission,
    show_fn:    SigmaU64,
    store_fn:   SigmaU64,
) -> SigmaI32 {
    let s = &mut SYSFS;
    if s.node_count as usize >= SYSFS_MAX_NODES { return -1; }

    let idx = s.node_count as usize;
    let n = &mut s.nodes[idx];
    sysfs_strncpy(n.name.as_mut_ptr(), name, SYSFS_NAME_LEN);
    n.node_type  = SysfsNodeType::Attribute;
    n.permission = permission;
    n.parent_idx = parent_idx;
    n.show_fn    = show_fn;
    n.store_fn   = store_fn;
    n.active     = true;
    s.node_count += 1;

    if parent_idx >= 0 {
        let p = &mut s.nodes[parent_idx as usize];
        if (p.child_count as usize) < SYSFS_MAX_CHILDREN {
            p.children[p.child_count as usize] = idx as SigmaI32;
            p.child_count += 1;
        }
    }

    idx as SigmaI32
}

/// Create a symlink node
#[no_mangle]
pub unsafe extern "C" fn sigma_sysfs_create_link(
    parent_idx:  SigmaI32,
    name:        *const u8,
    target_idx:  SigmaI32,
) -> SigmaI32 {
    let s = &mut SYSFS;
    if s.node_count as usize >= SYSFS_MAX_NODES { return -1; }

    let idx = s.node_count as usize;
    let n = &mut s.nodes[idx];
    sysfs_strncpy(n.name.as_mut_ptr(), name, SYSFS_NAME_LEN);
    n.node_type   = SysfsNodeType::SymLink;
    n.permission  = SysfsPermission::ReadOnly;
    n.parent_idx  = parent_idx;
    n.link_target = target_idx;
    n.active      = true;
    s.node_count += 1;

    if parent_idx >= 0 {
        let p = &mut s.nodes[parent_idx as usize];
        if (p.child_count as usize) < SYSFS_MAX_CHILDREN {
            p.children[p.child_count as usize] = idx as SigmaI32;
            p.child_count += 1;
        }
    }

    idx as SigmaI32
}

/// Read an attribute's current value
#[no_mangle]
pub unsafe extern "C" fn sigma_sysfs_read(
    node_idx: SigmaI32,
    buf:      *mut u8,
    buf_len:  SigmaU32,
) -> SigmaI32 {
    let s = &SYSFS;
    if node_idx < 0 || node_idx >= s.node_count as SigmaI32 { return -1; }
    let n = &s.nodes[node_idx as usize];
    if !n.active { return -2; }
    if n.node_type != SysfsNodeType::Attribute { return -3; }

    // If show_fn is set, call it; otherwise return stored value
    if n.show_fn != 0 {
        // In real impl: cast to fn(*mut u8, usize) -> i32 and call
        // Stub: copy stored value
    }

    let copy_len = if n.value_len < buf_len { n.value_len } else { buf_len };
    for i in 0..copy_len as usize {
        *buf.add(i) = n.value[i];
    }
    copy_len as SigmaI32
}

/// Write to an attribute
#[no_mangle]
pub unsafe extern "C" fn sigma_sysfs_write(
    node_idx: SigmaI32,
    buf:      *const u8,
    buf_len:  SigmaU32,
) -> SigmaI32 {
    let s = &mut SYSFS;
    if node_idx < 0 || node_idx >= s.node_count as SigmaI32 { return -1; }
    let n = &mut s.nodes[node_idx as usize];
    if !n.active { return -2; }
    if n.node_type != SysfsNodeType::Attribute { return -3; }
    if n.permission == SysfsPermission::ReadOnly { return -4; }

    // If store_fn is set, call it
    if n.store_fn != 0 {
        // In real impl: cast to fn(*const u8, usize) -> i32 and call
    }

    let write_len = if (buf_len as usize) < SYSFS_VALUE_LEN { buf_len as usize } else { SYSFS_VALUE_LEN };
    for i in 0..write_len {
        n.value[i] = *buf.add(i);
    }
    n.value_len = write_len as SigmaU32;

    write_len as SigmaI32
}

/// Lookup a child by name under a parent directory
#[no_mangle]
pub unsafe extern "C" fn sigma_sysfs_lookup(
    parent_idx: SigmaI32,
    name:       *const u8,
) -> SigmaI32 {
    let s = &SYSFS;
    let pidx = if parent_idx < 0 { 0 } else { parent_idx as usize };
    if pidx >= s.node_count as usize { return -1; }
    let p = &s.nodes[pidx];

    for i in 0..p.child_count as usize {
        let ci = p.children[i];
        if ci >= 0 && (ci as usize) < s.node_count as usize {
            if sysfs_strcmp(s.nodes[ci as usize].name.as_ptr(), name) == 0 {
                return ci;
            }
        }
    }
    -1 // not found
}

/// Resolve a full path like "/sys/class/net/eth0" to a node index
#[no_mangle]
pub unsafe extern "C" fn sigma_sysfs_resolve_path(path: *const u8) -> SigmaI32 {
    let s = &SYSFS;
    let mut current: SigmaI32 = 0; // start at root

    let path_len = sysfs_strlen(path);
    let mut seg_start = 0usize;

    // Skip leading '/'
    while seg_start < path_len && *path.add(seg_start) == b'/' {
        seg_start += 1;
    }

    while seg_start < path_len {
        // Find end of segment
        let mut seg_end = seg_start;
        while seg_end < path_len && *path.add(seg_end) != b'/' {
            seg_end += 1;
        }
        if seg_end == seg_start { seg_start = seg_end + 1; continue; }

        // Build null-terminated segment name
        let seg_len = seg_end - seg_start;
        let mut seg_buf = [0u8; SYSFS_NAME_LEN];
        let copy_len = if seg_len < SYSFS_NAME_LEN - 1 { seg_len } else { SYSFS_NAME_LEN - 1 };
        for i in 0..copy_len {
            seg_buf[i] = *path.add(seg_start + i);
        }

        let found = sigma_sysfs_lookup(current, seg_buf.as_ptr());
        if found < 0 { return -1; }

        // Follow symlinks
        let node = &s.nodes[found as usize];
        if node.node_type == SysfsNodeType::SymLink && node.link_target >= 0 {
            current = node.link_target;
        } else {
            current = found;
        }

        seg_start = seg_end + 1;
    }

    current
}

/// List children of a directory node
#[no_mangle]
pub unsafe extern "C" fn sigma_sysfs_readdir(
    node_idx:  SigmaI32,
    out_names: *mut [u8; SYSFS_NAME_LEN],
    max:       SigmaU32,
) -> SigmaU32 {
    let s = &SYSFS;
    if node_idx < 0 || node_idx >= s.node_count as SigmaI32 { return 0; }
    let n = &s.nodes[node_idx as usize];
    if n.node_type != SysfsNodeType::Directory { return 0; }

    let mut count = 0u32;
    for i in 0..n.child_count as usize {
        if count >= max { break; }
        let ci = n.children[i];
        if ci >= 0 && (ci as usize) < s.node_count as usize && s.nodes[ci as usize].active {
            let dst = &mut *out_names.add(count as usize);
            sysfs_strncpy(dst.as_mut_ptr(), s.nodes[ci as usize].name.as_ptr(), SYSFS_NAME_LEN);
            count += 1;
        }
    }
    count
}

/// Remove a node (and recursively deactivate children)
#[no_mangle]
pub unsafe extern "C" fn sigma_sysfs_remove(node_idx: SigmaI32) -> SigmaI32 {
    let s = &mut SYSFS;
    if node_idx < 0 || node_idx >= s.node_count as SigmaI32 { return -1; }

    // Recursively deactivate children
    let child_count = s.nodes[node_idx as usize].child_count;
    for i in 0..child_count as usize {
        let ci = s.nodes[node_idx as usize].children[i];
        if ci >= 0 {
            sigma_sysfs_remove(ci);
        }
    }
    s.nodes[node_idx as usize].active = false;
    0
}

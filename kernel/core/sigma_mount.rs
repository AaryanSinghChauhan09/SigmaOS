// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS sigma_mount — Sovereign Mount/Umount Subsystem
//! VFS mount table, filesystem type registry, bind/overlay mounts, fstab parsing.
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
pub const MOUNT_MAX_ENTRIES:  usize = 64;
pub const MOUNT_MAX_FS_TYPES: usize = 16;
pub const MOUNT_NAME_LEN:     usize = 64;
pub const MOUNT_PATH_LEN:     usize = 256;
pub const MOUNT_OPTS_LEN:     usize = 128;
pub const FSTAB_MAX_ENTRIES:  usize = 32;

// ─── Mount Flags ────────────────────────────────────────────────────────────

pub const MS_RDONLY:     SigmaU32 = 1 << 0;   // Read-only mount
pub const MS_NOSUID:     SigmaU32 = 1 << 1;   // Disallow setuid
pub const MS_NODEV:      SigmaU32 = 1 << 2;   // Disallow device files
pub const MS_NOEXEC:     SigmaU32 = 1 << 3;   // Disallow execution
pub const MS_SYNCHRONOUS: SigmaU32 = 1 << 4;   // Synchronous writes
pub const MS_REMOUNT:    SigmaU32 = 1 << 5;   // Remount with new flags
pub const MS_BIND:       SigmaU32 = 1 << 12;  // Bind mount
pub const MS_MOVE:       SigmaU32 = 1 << 13;  // Move mount
pub const MS_NOATIME:    SigmaU32 = 1 << 10;  // Don't update access times
pub const MS_RELATIME:   SigmaU32 = 1 << 21;  // Relative atime
pub const MS_LAZYTIME:   SigmaU32 = 1 << 25;  // Lazy timestamp updates

// ─── Filesystem Type Registry ───────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum FsType {
    Ext4     = 0,
    SigmaFS  = 1,   // native SigmaOS filesystem
    TmpFS    = 2,
    ProcFS   = 3,
    SysFS    = 4,
    DevFS    = 5,   // /dev
    Overlay  = 6,
    BindFS   = 7,
    Fat32    = 8,
    Ntfs     = 9,
    Btrfs    = 10,
    Xfs      = 11,
    Zfs      = 12,
    Nfs      = 13,   // Network filesystem
    CiFS     = 14,   // SMB/CIFS
    Unknown  = 255,
}

/// Registered filesystem type
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FsTypeEntry {
    pub name:       [u8; MOUNT_NAME_LEN],
    pub fs_type:    FsType,
    pub mount_fn:   SigmaU64,    // fn(*const MountEntry) -> i32
    pub umount_fn:  SigmaU64,    // fn(*const MountEntry) -> i32
    pub requires_device: SigmaBool,
    pub active:     SigmaBool,
}

impl FsTypeEntry {
    pub const fn empty() -> Self {
        Self {
            name:       [0u8; MOUNT_NAME_LEN],
            fs_type:    FsType::Unknown,
            mount_fn:   0,
            umount_fn:  0,
            requires_device: false,
            active:     false,
        }
    }
}

/// A single mount entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MountEntry {
    pub mount_id:    SigmaU32,
    pub parent_id:   SigmaU32,     // parent mount namespace ID
    pub source:      [u8; MOUNT_PATH_LEN],  // device path or "none"
    pub target:      [u8; MOUNT_PATH_LEN],  // mount point path
    pub fs_type:     FsType,
    pub flags:       SigmaU32,     // MS_* flags
    pub options:     [u8; MOUNT_OPTS_LEN],   // fs-specific options
    pub namespace:   SigmaU32,     // mount namespace ID (for containers)
    pub active:      SigmaBool,
}

impl MountEntry {
    pub const fn empty() -> Self {
        Self {
            mount_id:  0,
            parent_id: 0,
            source:    [0u8; MOUNT_PATH_LEN],
            target:    [0u8; MOUNT_PATH_LEN],
            fs_type:   FsType::Unknown,
            flags:     0,
            options:   [0u8; MOUNT_OPTS_LEN],
            namespace: 0,
            active:    false,
        }
    }
}

/// /etc/fstab entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FstabEntry {
    pub source:   [u8; MOUNT_PATH_LEN],
    pub target:   [u8; MOUNT_PATH_LEN],
    pub fs_type:  [u8; MOUNT_NAME_LEN],
    pub options:  [u8; MOUNT_OPTS_LEN],
    pub dump:     SigmaU32,
    pub pass:     SigmaU32,      // fsck pass number
    pub active:   SigmaBool,
}

impl FstabEntry {
    pub const fn empty() -> Self {
        Self {
            source:  [0u8; MOUNT_PATH_LEN],
            target:  [0u8; MOUNT_PATH_LEN],
            fs_type: [0u8; MOUNT_NAME_LEN],
            options: [0u8; MOUNT_OPTS_LEN],
            dump: 0, pass: 0,
            active: false,
        }
    }
}

// ─── Global State ───────────────────────────────────────────────────────────

struct MountState {
    mounts:       [MountEntry; MOUNT_MAX_ENTRIES],
    mount_count:  SigmaU32,
    next_mount_id: SigmaU32,
    fs_types:     [FsTypeEntry; MOUNT_MAX_FS_TYPES],
    fs_type_count: SigmaU32,
    fstab:        [FstabEntry; FSTAB_MAX_ENTRIES],
    fstab_count:  SigmaU32,
    initialized:  SigmaBool,
}

static mut MOUNT_STATE: MountState = MountState {
    mounts:        [MountEntry::empty(); MOUNT_MAX_ENTRIES],
    mount_count:   0,
    next_mount_id: 1,
    fs_types:      [FsTypeEntry::empty(); MOUNT_MAX_FS_TYPES],
    fs_type_count: 0,
    fstab:         [FstabEntry::empty(); FSTAB_MAX_ENTRIES],
    fstab_count:   0,
    initialized:   false,
};

// ─── Helpers ────────────────────────────────────────────────────────────────

unsafe fn mount_strncpy(dst: *mut u8, src: *const u8, n: usize) {
    let mut i = 0;
    while i < n { let b = *src.add(i); *dst.add(i) = b; if b == 0 { return; } i += 1; }
    if n > 0 { *dst.add(n - 1) = 0; }
}

unsafe fn mount_strcmp(a: *const u8, b: *const u8) -> i32 {
    let mut i = 0usize;
    loop {
        let ca = *a.add(i); let cb = *b.add(i);
        if ca != cb { return ca as i32 - cb as i32; }
        if ca == 0  { return 0; }
        i += 1;
    }
}

// ─── C-ABI Exports ──────────────────────────────────────────────────────────

/// Initialize mount subsystem
#[no_mangle]
pub unsafe extern "C" fn sigma_mount_init() -> SigmaI32 {
    let m = &mut MOUNT_STATE;
    m.mount_count   = 0;
    m.next_mount_id = 1;
    m.fs_type_count = 0;
    m.fstab_count   = 0;
    m.initialized   = true;

    // Register built-in filesystem types
    let builtins: [(FsType, &[u8], bool); 6] = [
        (FsType::SigmaFS, b"sigmafs\0", true),
        (FsType::Ext4,    b"ext4\0",    true),
        (FsType::TmpFS,   b"tmpfs\0",   false),
        (FsType::ProcFS,  b"proc\0",    false),
        (FsType::SysFS,   b"sysfs\0",   false),
        (FsType::DevFS,   b"devtmpfs\0", false),
    ];

    for (fs, name, req_dev) in builtins.iter() {
        let idx = m.fs_type_count as usize;
        if idx >= MOUNT_MAX_FS_TYPES { break; }
        mount_strncpy(m.fs_types[idx].name.as_mut_ptr(), name.as_ptr(), MOUNT_NAME_LEN);
        m.fs_types[idx].fs_type = *fs;
        m.fs_types[idx].requires_device = *req_dev;
        m.fs_types[idx].active = true;
        m.fs_type_count += 1;
    }

    // Mount rootfs as first entry
    let root = &mut m.mounts[0];
    root.mount_id = m.next_mount_id;
    m.next_mount_id += 1;
    root.fs_type = FsType::SigmaFS;
    let rootsrc = b"/dev/sda1\0";
    let roottgt = b"/\0";
    mount_strncpy(root.source.as_mut_ptr(), rootsrc.as_ptr(), MOUNT_PATH_LEN);
    mount_strncpy(root.target.as_mut_ptr(), roottgt.as_ptr(), MOUNT_PATH_LEN);
    root.active = true;
    m.mount_count = 1;

    0
}

/// Register a new filesystem type
#[no_mangle]
pub unsafe extern "C" fn sigma_mount_register_fs(
    name:     *const u8,
    fs_type:  FsType,
    mount_fn: SigmaU64,
    umount_fn: SigmaU64,
    requires_device: SigmaBool,
) -> SigmaI32 {
    let m = &mut MOUNT_STATE;
    if m.fs_type_count as usize >= MOUNT_MAX_FS_TYPES { return -1; }

    let idx = m.fs_type_count as usize;
    mount_strncpy(m.fs_types[idx].name.as_mut_ptr(), name, MOUNT_NAME_LEN);
    m.fs_types[idx].fs_type         = fs_type;
    m.fs_types[idx].mount_fn        = mount_fn;
    m.fs_types[idx].umount_fn       = umount_fn;
    m.fs_types[idx].requires_device = requires_device;
    m.fs_types[idx].active          = true;
    m.fs_type_count += 1;

    idx as SigmaI32
}

/// Mount a filesystem
#[no_mangle]
pub unsafe extern "C" fn sigma_mount(
    source:  *const u8,
    target:  *const u8,
    fs_type: *const u8,
    flags:   SigmaU32,
    options: *const u8,
) -> SigmaI32 {
    let m = &mut MOUNT_STATE;
    if !m.initialized { return -1; }

    // Handle MS_REMOUNT: find existing mount and update flags
    if flags & MS_REMOUNT != 0 {
        for i in 0..m.mount_count as usize {
            if m.mounts[i].active && mount_strcmp(m.mounts[i].target.as_ptr(), target) == 0 {
                m.mounts[i].flags = flags & !MS_REMOUNT;
                if !options.is_null() {
                    mount_strncpy(m.mounts[i].options.as_mut_ptr(), options, MOUNT_OPTS_LEN);
                }
                return 0;
            }
        }
        return -2; // mount point not found
    }

    // Check for duplicate mount point
    for i in 0..m.mount_count as usize {
        if m.mounts[i].active && mount_strcmp(m.mounts[i].target.as_ptr(), target) == 0 {
            return -3; // already mounted
        }
    }

    if m.mount_count as usize >= MOUNT_MAX_ENTRIES { return -4; }

    // Resolve filesystem type
    let mut resolved_fs = FsType::Unknown;
    if !fs_type.is_null() {
        for i in 0..m.fs_type_count as usize {
            if m.fs_types[i].active && mount_strcmp(m.fs_types[i].name.as_ptr(), fs_type) == 0 {
                resolved_fs = m.fs_types[i].fs_type;
                break;
            }
        }
        if resolved_fs == FsType::Unknown { return -5; } // unknown filesystem type
    }

    // Handle bind mount
    if flags & MS_BIND != 0 {
        resolved_fs = FsType::BindFS;
    }

    let idx = m.mount_count as usize;
    let entry = &mut m.mounts[idx];
    entry.mount_id = m.next_mount_id;
    m.next_mount_id += 1;
    entry.fs_type = resolved_fs;
    entry.flags   = flags;
    entry.active  = true;
    mount_strncpy(entry.source.as_mut_ptr(), source, MOUNT_PATH_LEN);
    mount_strncpy(entry.target.as_mut_ptr(), target, MOUNT_PATH_LEN);
    if !options.is_null() {
        mount_strncpy(entry.options.as_mut_ptr(), options, MOUNT_OPTS_LEN);
    }

    m.mount_count += 1;

    // In real implementation: call the fs_type's mount_fn callback
    0
}

/// Unmount a filesystem
#[no_mangle]
pub unsafe extern "C" fn sigma_umount(target: *const u8) -> SigmaI32 {
    let m = &mut MOUNT_STATE;
    for i in 0..m.mount_count as usize {
        if m.mounts[i].active && mount_strcmp(m.mounts[i].target.as_ptr(), target) == 0 {
            // Don't allow unmounting root
            let root = b"/\0";
            if mount_strcmp(m.mounts[i].target.as_ptr(), root.as_ptr()) == 0 {
                return -2; // can't unmount root
            }
            m.mounts[i].active = false;
            return 0;
        }
    }
    -1 // not found
}

/// Get mount info by target path
#[no_mangle]
pub unsafe extern "C" fn sigma_mount_info(
    target: *const u8,
    out:    *mut MountEntry,
) -> SigmaBool {
    let m = &MOUNT_STATE;
    for i in 0..m.mount_count as usize {
        if m.mounts[i].active && mount_strcmp(m.mounts[i].target.as_ptr(), target) == 0 {
            *out = m.mounts[i];
            return true;
        }
    }
    false
}

/// List all active mounts (like /proc/mounts)
#[no_mangle]
pub unsafe extern "C" fn sigma_mount_list(
    out: *mut MountEntry,
    max: SigmaU32,
) -> SigmaU32 {
    let m = &MOUNT_STATE;
    let mut count = 0u32;
    for i in 0..m.mount_count as usize {
        if count >= max { break; }
        if m.mounts[i].active {
            *out.add(count as usize) = m.mounts[i];
            count += 1;
        }
    }
    count
}

/// Get active mount count
#[no_mangle]
pub unsafe extern "C" fn sigma_mount_count() -> SigmaU32 {
    let m = &MOUNT_STATE;
    let mut count = 0u32;
    for i in 0..m.mount_count as usize {
        if m.mounts[i].active { count += 1; }
    }
    count
}

/// Add an fstab entry
#[no_mangle]
pub unsafe extern "C" fn sigma_fstab_add(
    source:  *const u8,
    target:  *const u8,
    fs_type: *const u8,
    options: *const u8,
    dump:    SigmaU32,
    pass:    SigmaU32,
) -> SigmaI32 {
    let m = &mut MOUNT_STATE;
    if m.fstab_count as usize >= FSTAB_MAX_ENTRIES { return -1; }

    let idx = m.fstab_count as usize;
    mount_strncpy(m.fstab[idx].source.as_mut_ptr(),  source,  MOUNT_PATH_LEN);
    mount_strncpy(m.fstab[idx].target.as_mut_ptr(),  target,  MOUNT_PATH_LEN);
    mount_strncpy(m.fstab[idx].fs_type.as_mut_ptr(), fs_type, MOUNT_NAME_LEN);
    if !options.is_null() {
        mount_strncpy(m.fstab[idx].options.as_mut_ptr(), options, MOUNT_OPTS_LEN);
    }
    m.fstab[idx].dump   = dump;
    m.fstab[idx].pass   = pass;
    m.fstab[idx].active = true;
    m.fstab_count += 1;
    0
}

/// Mount all filesystems from fstab (mount -a)
#[no_mangle]
pub unsafe extern "C" fn sigma_mount_all_fstab() -> SigmaI32 {
    let m = &MOUNT_STATE;
    let mut failures = 0i32;
    for i in 0..m.fstab_count as usize {
        if !m.fstab[i].active { continue; }
        let result = sigma_mount(
            m.fstab[i].source.as_ptr(),
            m.fstab[i].target.as_ptr(),
            m.fstab[i].fs_type.as_ptr(),
            0, // default flags
            m.fstab[i].options.as_ptr(),
        );
        if result < 0 { failures += 1; }
    }
    failures
}

/// Find which filesystem a path belongs to (mount point lookup)
#[no_mangle]
pub unsafe extern "C" fn sigma_mount_findfs(
    path: *const u8,
    out:  *mut MountEntry,
) -> SigmaBool {
    let m = &MOUNT_STATE;
    let mut best_idx: SigmaI32 = -1;
    let mut best_len: usize = 0;

    for i in 0..m.mount_count as usize {
        if !m.mounts[i].active { continue; }
        // Check if target is a prefix of path
        let mut j = 0usize;
        let mut matched = true;
        loop {
            let tc = m.mounts[i].target[j];
            if tc == 0 { break; }
            let pc = {
                let mut len = 0usize;
                while *path.add(len) != 0 { len += 1; }
                if j < len { *path.add(j) } else { 0 }
            };
            if tc != pc { matched = false; break; }
            j += 1;
        }
        if matched && j > best_len {
            best_len = j;
            best_idx = i as SigmaI32;
        }
    }

    if best_idx >= 0 {
        *out = m.mounts[best_idx as usize];
        true
    } else {
        false
    }
}

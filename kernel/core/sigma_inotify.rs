// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS sigma_inotify — Sovereign File Watcher
//! File/directory event monitoring (create, delete, modify, move, attrib).
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
pub const INOTIFY_MAX_INSTANCES: usize = 16;
pub const INOTIFY_MAX_WATCHES:   usize = 128;
pub const INOTIFY_EVENT_QUEUE:   usize = 256;
pub const INOTIFY_NAME_LEN:      usize = 64;
pub const INOTIFY_PATH_LEN:      usize = 256;

// ─── Event Masks (bitflags, matching Linux inotify(7)) ──────────────────────

pub const IN_ACCESS:        SigmaU32 = 0x0000_0001;
pub const IN_MODIFY:        SigmaU32 = 0x0000_0002;
pub const IN_ATTRIB:        SigmaU32 = 0x0000_0004;
pub const IN_CLOSE_WRITE:   SigmaU32 = 0x0000_0008;
pub const IN_CLOSE_NOWRITE: SigmaU32 = 0x0000_0010;
pub const IN_OPEN:          SigmaU32 = 0x0000_0020;
pub const IN_MOVED_FROM:    SigmaU32 = 0x0000_0040;
pub const IN_MOVED_TO:      SigmaU32 = 0x0000_0080;
pub const IN_CREATE:        SigmaU32 = 0x0000_0100;
pub const IN_DELETE:        SigmaU32 = 0x0000_0200;
pub const IN_DELETE_SELF:   SigmaU32 = 0x0000_0400;
pub const IN_MOVE_SELF:     SigmaU32 = 0x0000_0800;
// Special flags
pub const IN_ONESHOT:       SigmaU32 = 0x8000_0000;
pub const IN_ONLYDIR:       SigmaU32 = 0x0100_0000;
pub const IN_ISDIR:         SigmaU32 = 0x4000_0000;
pub const IN_Q_OVERFLOW:    SigmaU32 = 0x0000_4000;
// Combined masks
pub const IN_CLOSE:         SigmaU32 = IN_CLOSE_WRITE | IN_CLOSE_NOWRITE;
pub const IN_MOVE:          SigmaU32 = IN_MOVED_FROM | IN_MOVED_TO;
pub const IN_ALL_EVENTS:    SigmaU32 = 0x0000_0FFF;

// ─── Structs ────────────────────────────────────────────────────────────────

/// An inotify event (matches struct inotify_event in Linux)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct InotifyEvent {
    pub wd:     SigmaI32,                // watch descriptor
    pub mask:   SigmaU32,                // event mask
    pub cookie: SigmaU32,                // cookie for IN_MOVED_FROM/TO pairs
    pub name:   [u8; INOTIFY_NAME_LEN],  // filename within watched directory
    pub timestamp: SigmaU64,             // kernel timestamp
}

impl InotifyEvent {
    pub const fn empty() -> Self {
        Self {
            wd: -1, mask: 0, cookie: 0,
            name: [0u8; INOTIFY_NAME_LEN],
            timestamp: 0,
        }
    }
}

/// A watch descriptor — watches a specific path for events
#[repr(C)]
#[derive(Copy, Clone)]
pub struct InotifyWatch {
    pub wd:          SigmaI32,
    pub path:        [u8; INOTIFY_PATH_LEN],
    pub mask:        SigmaU32,           // events we're interested in
    pub is_dir:      SigmaBool,
    pub oneshot:     SigmaBool,          // remove after first event
    pub active:      SigmaBool,
    pub instance_id: SigmaI32,           // which inotify instance owns this
}

impl InotifyWatch {
    pub const fn empty() -> Self {
        Self {
            wd: -1,
            path: [0u8; INOTIFY_PATH_LEN],
            mask: 0,
            is_dir: false,
            oneshot: false,
            active: false,
            instance_id: -1,
        }
    }
}

/// An inotify instance (file descriptor in Linux; here a struct index)
#[repr(C)]
pub struct InotifyInstance {
    pub id:         SigmaI32,
    pub owner_pid:  SigmaU32,
    // Event ring buffer
    pub events:     [InotifyEvent; INOTIFY_EVENT_QUEUE],
    pub ev_head:    SigmaU32,
    pub ev_tail:    SigmaU32,
    pub ev_count:   SigmaU32,
    pub overflow:   SigmaBool,
    pub active:     SigmaBool,
}

// ─── Global State ───────────────────────────────────────────────────────────

static mut WATCHES: [InotifyWatch; INOTIFY_MAX_WATCHES] = [InotifyWatch::empty(); INOTIFY_MAX_WATCHES];
static mut WATCH_COUNT: SigmaU32 = 0;
static mut NEXT_WD: SigmaI32 = 1;
static mut NEXT_COOKIE: SigmaU32 = 1;

// Instances use a simpler model — just event queues per instance ID
static mut INSTANCE_EVENTS: [[InotifyEvent; INOTIFY_EVENT_QUEUE]; INOTIFY_MAX_INSTANCES] =
    [[InotifyEvent::empty(); INOTIFY_EVENT_QUEUE]; INOTIFY_MAX_INSTANCES];
static mut INSTANCE_HEADS: [SigmaU32; INOTIFY_MAX_INSTANCES] = [0; INOTIFY_MAX_INSTANCES];
static mut INSTANCE_TAILS: [SigmaU32; INOTIFY_MAX_INSTANCES] = [0; INOTIFY_MAX_INSTANCES];
static mut INSTANCE_COUNTS: [SigmaU32; INOTIFY_MAX_INSTANCES] = [0; INOTIFY_MAX_INSTANCES];
static mut INSTANCE_ACTIVE: [SigmaBool; INOTIFY_MAX_INSTANCES] = [false; INOTIFY_MAX_INSTANCES];
static mut INSTANCE_PIDS:   [SigmaU32; INOTIFY_MAX_INSTANCES] = [0; INOTIFY_MAX_INSTANCES];
static mut NEXT_INSTANCE_ID: SigmaI32 = 0;

// ─── Helpers ────────────────────────────────────────────────────────────────

unsafe fn inotify_strncpy(dst: *mut u8, src: *const u8, n: usize) {
    let mut i = 0;
    while i < n { let b = *src.add(i); *dst.add(i) = b; if b == 0 { return; } i += 1; }
    if n > 0 { *dst.add(n - 1) = 0; }
}

unsafe fn inotify_strcmp(a: *const u8, b: *const u8) -> i32 {
    let mut i = 0usize;
    loop {
        let ca = *a.add(i); let cb = *b.add(i);
        if ca != cb { return ca as i32 - cb as i32; }
        if ca == 0  { return 0; }
        i += 1;
    }
}

/// Check if `path` starts with `prefix`
unsafe fn path_starts_with(path: *const u8, prefix: *const u8) -> bool {
    let mut i = 0usize;
    loop {
        let p = *prefix.add(i);
        if p == 0 { return true; }
        let t = *path.add(i);
        if t != p { return false; }
        i += 1;
    }
}

// ─── C-ABI Exports ──────────────────────────────────────────────────────────

/// Create a new inotify instance (returns instance ID, like inotify_init())
#[no_mangle]
pub unsafe extern "C" fn sigma_inotify_init(owner_pid: SigmaU32) -> SigmaI32 {
    if NEXT_INSTANCE_ID as usize >= INOTIFY_MAX_INSTANCES { return -1; }
    let id = NEXT_INSTANCE_ID;
    INSTANCE_ACTIVE[id as usize] = true;
    INSTANCE_PIDS[id as usize]   = owner_pid;
    INSTANCE_HEADS[id as usize]  = 0;
    INSTANCE_TAILS[id as usize]  = 0;
    INSTANCE_COUNTS[id as usize] = 0;
    NEXT_INSTANCE_ID += 1;
    id
}

/// Add a watch (returns watch descriptor, like inotify_add_watch())
#[no_mangle]
pub unsafe extern "C" fn sigma_inotify_add_watch(
    instance_id: SigmaI32,
    path:        *const u8,
    mask:        SigmaU32,
) -> SigmaI32 {
    if instance_id < 0 || instance_id >= NEXT_INSTANCE_ID { return -1; }
    if !INSTANCE_ACTIVE[instance_id as usize] { return -2; }
    if WATCH_COUNT as usize >= INOTIFY_MAX_WATCHES { return -3; }

    // Check if already watching this path on this instance — update mask
    for i in 0..WATCH_COUNT as usize {
        if WATCHES[i].active
            && WATCHES[i].instance_id == instance_id
            && inotify_strcmp(WATCHES[i].path.as_ptr(), path) == 0
        {
            WATCHES[i].mask = mask;
            WATCHES[i].oneshot = mask & IN_ONESHOT != 0;
            return WATCHES[i].wd;
        }
    }

    let idx = WATCH_COUNT as usize;
    let wd = NEXT_WD;
    NEXT_WD += 1;

    WATCHES[idx].wd          = wd;
    WATCHES[idx].mask        = mask & IN_ALL_EVENTS;
    WATCHES[idx].is_dir      = mask & IN_ONLYDIR != 0;
    WATCHES[idx].oneshot     = mask & IN_ONESHOT != 0;
    WATCHES[idx].active      = true;
    WATCHES[idx].instance_id = instance_id;
    inotify_strncpy(WATCHES[idx].path.as_mut_ptr(), path, INOTIFY_PATH_LEN);

    WATCH_COUNT += 1;
    wd
}

/// Remove a watch (like inotify_rm_watch())
#[no_mangle]
pub unsafe extern "C" fn sigma_inotify_rm_watch(instance_id: SigmaI32, wd: SigmaI32) -> SigmaI32 {
    for i in 0..WATCH_COUNT as usize {
        if WATCHES[i].active && WATCHES[i].instance_id == instance_id && WATCHES[i].wd == wd {
            WATCHES[i].active = false;
            return 0;
        }
    }
    -1 // not found
}

/// Close an inotify instance
#[no_mangle]
pub unsafe extern "C" fn sigma_inotify_close(instance_id: SigmaI32) -> SigmaI32 {
    if instance_id < 0 || instance_id >= NEXT_INSTANCE_ID { return -1; }
    if !INSTANCE_ACTIVE[instance_id as usize] { return -2; }

    // Remove all watches for this instance
    for i in 0..WATCH_COUNT as usize {
        if WATCHES[i].active && WATCHES[i].instance_id == instance_id {
            WATCHES[i].active = false;
        }
    }

    INSTANCE_ACTIVE[instance_id as usize] = false;
    0
}

/// Emit an event (called by VFS layer when file operations occur)
#[no_mangle]
pub unsafe extern "C" fn sigma_inotify_emit(
    path:      *const u8,
    filename:  *const u8,
    mask:      SigmaU32,
    is_dir:    SigmaBool,
    timestamp: SigmaU64,
) {
    let event_mask = mask | if is_dir { IN_ISDIR } else { 0 };

    for i in 0..WATCH_COUNT as usize {
        if !WATCHES[i].active { continue; }

        // Check if this watch matches the path
        let matched = inotify_strcmp(WATCHES[i].path.as_ptr(), path) == 0
            || path_starts_with(path, WATCHES[i].path.as_ptr());

        if !matched { continue; }

        // Check mask match
        if WATCHES[i].mask & mask == 0 { continue; }

        let inst = WATCHES[i].instance_id as usize;
        if inst >= INOTIFY_MAX_INSTANCES || !INSTANCE_ACTIVE[inst] { continue; }

        // Enqueue event
        let tail = INSTANCE_TAILS[inst] as usize % INOTIFY_EVENT_QUEUE;
        INSTANCE_EVENTS[inst][tail] = InotifyEvent {
            wd:    WATCHES[i].wd,
            mask:  event_mask,
            cookie: 0,
            name:  [0u8; INOTIFY_NAME_LEN],
            timestamp,
        };
        if !filename.is_null() {
            inotify_strncpy(
                INSTANCE_EVENTS[inst][tail].name.as_mut_ptr(),
                filename,
                INOTIFY_NAME_LEN,
            );
        }
        INSTANCE_TAILS[inst] = (INSTANCE_TAILS[inst] + 1) % INOTIFY_EVENT_QUEUE as u32;
        if INSTANCE_COUNTS[inst] < INOTIFY_EVENT_QUEUE as u32 {
            INSTANCE_COUNTS[inst] += 1;
        } else {
            // Overflow
            INSTANCE_HEADS[inst] = (INSTANCE_HEADS[inst] + 1) % INOTIFY_EVENT_QUEUE as u32;
        }

        // Handle oneshot
        if WATCHES[i].oneshot {
            WATCHES[i].active = false;
        }
    }
}

/// Emit a move event pair (generates cookie to pair MOVED_FROM and MOVED_TO)
#[no_mangle]
pub unsafe extern "C" fn sigma_inotify_emit_move(
    old_path:  *const u8,
    old_name:  *const u8,
    new_path:  *const u8,
    new_name:  *const u8,
    is_dir:    SigmaBool,
    timestamp: SigmaU64,
) {
    let cookie = NEXT_COOKIE;
    NEXT_COOKIE += 1;

    // Emit MOVED_FROM on old path
    sigma_inotify_emit(old_path, old_name, IN_MOVED_FROM, is_dir, timestamp);
    // Set cookie on the last enqueued event
    for inst in 0..INOTIFY_MAX_INSTANCES {
        if !INSTANCE_ACTIVE[inst] { continue; }
        if INSTANCE_COUNTS[inst] > 0 {
            let last = ((INSTANCE_TAILS[inst] as usize) + INOTIFY_EVENT_QUEUE - 1) % INOTIFY_EVENT_QUEUE;
            if INSTANCE_EVENTS[inst][last].mask & IN_MOVED_FROM != 0 {
                INSTANCE_EVENTS[inst][last].cookie = cookie;
            }
        }
    }

    // Emit MOVED_TO on new path
    sigma_inotify_emit(new_path, new_name, IN_MOVED_TO, is_dir, timestamp);
    for inst in 0..INOTIFY_MAX_INSTANCES {
        if !INSTANCE_ACTIVE[inst] { continue; }
        if INSTANCE_COUNTS[inst] > 0 {
            let last = ((INSTANCE_TAILS[inst] as usize) + INOTIFY_EVENT_QUEUE - 1) % INOTIFY_EVENT_QUEUE;
            if INSTANCE_EVENTS[inst][last].mask & IN_MOVED_TO != 0 {
                INSTANCE_EVENTS[inst][last].cookie = cookie;
            }
        }
    }
}

/// Read events from an inotify instance (like read() on inotify fd)
#[no_mangle]
pub unsafe extern "C" fn sigma_inotify_read(
    instance_id: SigmaI32,
    out:         *mut InotifyEvent,
    max:         SigmaU32,
) -> SigmaI32 {
    if instance_id < 0 || instance_id as usize >= INOTIFY_MAX_INSTANCES { return -1; }
    let inst = instance_id as usize;
    if !INSTANCE_ACTIVE[inst] { return -2; }

    let mut count = 0u32;
    while count < max && INSTANCE_COUNTS[inst] > 0 {
        let head = INSTANCE_HEADS[inst] as usize % INOTIFY_EVENT_QUEUE;
        *out.add(count as usize) = INSTANCE_EVENTS[inst][head];
        INSTANCE_HEADS[inst] = (INSTANCE_HEADS[inst] + 1) % INOTIFY_EVENT_QUEUE as u32;
        INSTANCE_COUNTS[inst] -= 1;
        count += 1;
    }

    count as SigmaI32
}

/// Get pending event count for an instance
#[no_mangle]
pub unsafe extern "C" fn sigma_inotify_pending(instance_id: SigmaI32) -> SigmaU32 {
    if instance_id < 0 || instance_id as usize >= INOTIFY_MAX_INSTANCES { return 0; }
    if !INSTANCE_ACTIVE[instance_id as usize] { return 0; }
    INSTANCE_COUNTS[instance_id as usize]
}

/// Get total active watch count across all instances
#[no_mangle]
pub unsafe extern "C" fn sigma_inotify_watch_count() -> SigmaU32 {
    let mut count = 0u32;
    unsafe {
        for i in 0..WATCH_COUNT as usize {
            if WATCHES[i].active { count += 1; }
        }
    }
    count
}

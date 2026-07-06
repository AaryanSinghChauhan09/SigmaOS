// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS sigma_procfs — Sovereign ProcFS Virtual Filesystem
//! Per-process info + system-wide stats exposed as virtual files.
//! no_std, no alloc, no external crates.

#![no_std]
#![allow(dead_code)]

type SigmaU8    = u8;
type SigmaU32   = u32;
type SigmaU64   = u64;
type SigmaI32   = i32;
type SigmaI64   = i64;
type SigmaBool  = bool;
type SigmaUsize = usize;

// ─── Constants ──────────────────────────────────────────────────────────────
pub const PROCFS_MAX_PROCS:    usize = 256;
pub const PROCFS_NAME_LEN:     usize = 64;
pub const PROCFS_CMDLINE_LEN:  usize = 256;
pub const PROCFS_MAX_FDS:      usize = 64;
pub const PROCFS_MAX_MAPS:     usize = 32;

// ─── Process State ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum ProcState {
    Running   = b'R',
    Sleeping  = b'S',
    DiskSleep = b'D',
    Zombie    = b'Z',
    Stopped   = b'T',
    TracingStop = b't',
    Dead      = b'X',
    Idle      = b'I',
}

/// Memory map entry (simplified /proc/[pid]/maps)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ProcMapEntry {
    pub start:     SigmaU64,
    pub end:       SigmaU64,
    pub perm_read:  SigmaBool,
    pub perm_write: SigmaBool,
    pub perm_exec:  SigmaBool,
    pub perm_priv:  SigmaBool,  // private (p) vs shared (s)
    pub offset:    SigmaU64,
    pub inode:     SigmaU64,
    pub pathname:  [u8; PROCFS_NAME_LEN],
    pub active:    SigmaBool,
}

impl ProcMapEntry {
    pub const fn empty() -> Self {
        Self {
            start: 0, end: 0,
            perm_read: false, perm_write: false, perm_exec: false, perm_priv: true,
            offset: 0, inode: 0,
            pathname: [0u8; PROCFS_NAME_LEN],
            active: false,
        }
    }
}

/// File descriptor entry (/proc/[pid]/fd/)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ProcFdEntry {
    pub fd:       SigmaI32,
    pub target:   [u8; PROCFS_NAME_LEN],  // symlink target (file path, socket, pipe)
    pub flags:    SigmaU32,               // O_RDONLY, O_WRONLY, etc.
    pub active:   SigmaBool,
}

impl ProcFdEntry {
    pub const fn empty() -> Self {
        Self { fd: -1, target: [0u8; PROCFS_NAME_LEN], flags: 0, active: false }
    }
}

/// Per-process /proc/[pid]/ data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ProcInfo {
    pub pid:        SigmaU32,
    pub ppid:       SigmaU32,       // parent PID
    pub uid:        SigmaU32,
    pub gid:        SigmaU32,
    pub state:      ProcState,
    pub name:       [u8; PROCFS_NAME_LEN],
    pub cmdline:    [u8; PROCFS_CMDLINE_LEN],
    pub threads:    SigmaU32,
    // /proc/[pid]/stat fields
    pub user_time:  SigmaU64,       // ticks in user mode
    pub sys_time:   SigmaU64,       // ticks in kernel mode
    pub start_time: SigmaU64,       // tick of process creation
    pub priority:   SigmaI32,
    pub nice:       SigmaI32,
    // /proc/[pid]/status memory fields
    pub vm_size:    SigmaU64,       // total virtual memory (bytes)
    pub vm_rss:     SigmaU64,       // resident set size (bytes)
    pub vm_peak:    SigmaU64,       // peak virtual memory
    pub vm_swap:    SigmaU64,       // swapped out size
    // Memory maps
    pub maps:       [ProcMapEntry; PROCFS_MAX_MAPS],
    pub map_count:  SigmaU32,
    // File descriptors
    pub fds:        [ProcFdEntry; PROCFS_MAX_FDS],
    pub fd_count:   SigmaU32,
    pub active:     SigmaBool,
}

impl ProcInfo {
    pub const fn empty() -> Self {
        Self {
            pid: 0, ppid: 0, uid: 0, gid: 0,
            state: ProcState::Dead,
            name: [0u8; PROCFS_NAME_LEN],
            cmdline: [0u8; PROCFS_CMDLINE_LEN],
            threads: 0,
            user_time: 0, sys_time: 0, start_time: 0,
            priority: 0, nice: 0,
            vm_size: 0, vm_rss: 0, vm_peak: 0, vm_swap: 0,
            maps: [ProcMapEntry::empty(); PROCFS_MAX_MAPS],
            map_count: 0,
            fds: [ProcFdEntry::empty(); PROCFS_MAX_FDS],
            fd_count: 0,
            active: false,
        }
    }
}

// ─── System-wide /proc/ entries ─────────────────────────────────────────────

/// /proc/cpuinfo
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CpuInfo {
    pub processor_count: SigmaU32,
    pub vendor:          [u8; 32],     // e.g., "SigmaOS"
    pub model_name:      [u8; 64],     // e.g., "SigmaCore v1 @ 3.5GHz"
    pub cpu_mhz:         SigmaU32,
    pub cache_size_kb:   SigmaU32,
    pub bogomips:        SigmaU32,
    pub flags:           [u8; 128],    // feature flags
}

/// /proc/meminfo
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemInfo {
    pub total_kb:       SigmaU64,
    pub free_kb:        SigmaU64,
    pub available_kb:   SigmaU64,
    pub buffers_kb:     SigmaU64,
    pub cached_kb:      SigmaU64,
    pub swap_total_kb:  SigmaU64,
    pub swap_free_kb:   SigmaU64,
    pub slab_kb:        SigmaU64,
    pub page_tables_kb: SigmaU64,
    pub dirty_kb:       SigmaU64,
    pub writeback_kb:   SigmaU64,
    pub active_kb:      SigmaU64,
    pub inactive_kb:    SigmaU64,
}

/// /proc/loadavg
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LoadAvg {
    pub load_1:        SigmaU32,   // * 100 (fixed-point: 1.23 = 123)
    pub load_5:        SigmaU32,
    pub load_15:       SigmaU32,
    pub running_tasks: SigmaU32,
    pub total_tasks:   SigmaU32,
    pub last_pid:      SigmaU32,
}

/// /proc/uptime
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Uptime {
    pub uptime_secs:  SigmaU64,
    pub idle_secs:    SigmaU64,
}

/// /proc/version
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VersionInfo {
    pub kernel_version: [u8; 64],
    pub build_date:     [u8; 32],
    pub compiler:       [u8; 32],
    pub arch:           [u8; 16],
}

// ─── Global State ───────────────────────────────────────────────────────────

struct ProcfsState {
    procs:      [ProcInfo; PROCFS_MAX_PROCS],
    proc_count: SigmaU32,
    cpuinfo:    CpuInfo,
    meminfo:    MemInfo,
    loadavg:    LoadAvg,
    uptime:     Uptime,
    version:    VersionInfo,
    initialized: SigmaBool,
}

static mut PROCFS: ProcfsState = ProcfsState {
    procs:      [ProcInfo::empty(); PROCFS_MAX_PROCS],
    proc_count: 0,
    cpuinfo:    CpuInfo {
        processor_count: 1,
        vendor:          [0u8; 32],
        model_name:      [0u8; 64],
        cpu_mhz:         3500,
        cache_size_kb:   8192,
        bogomips:        7000,
        flags:           [0u8; 128],
    },
    meminfo:    MemInfo {
        total_kb: 0, free_kb: 0, available_kb: 0, buffers_kb: 0, cached_kb: 0,
        swap_total_kb: 0, swap_free_kb: 0, slab_kb: 0, page_tables_kb: 0,
        dirty_kb: 0, writeback_kb: 0, active_kb: 0, inactive_kb: 0,
    },
    loadavg:    LoadAvg {
        load_1: 0, load_5: 0, load_15: 0,
        running_tasks: 0, total_tasks: 0, last_pid: 0,
    },
    uptime:     Uptime { uptime_secs: 0, idle_secs: 0 },
    version:    VersionInfo {
        kernel_version: [0u8; 64],
        build_date:     [0u8; 32],
        compiler:       [0u8; 32],
        arch:           [0u8; 16],
    },
    initialized: false,
};

// ─── Helpers ────────────────────────────────────────────────────────────────

unsafe fn procfs_strncpy(dst: *mut u8, src: *const u8, n: usize) {
    let mut i = 0;
    while i < n { let b = *src.add(i); *dst.add(i) = b; if b == 0 { return; } i += 1; }
    if n > 0 { *dst.add(n - 1) = 0; }
}

// ─── C-ABI Exports ──────────────────────────────────────────────────────────

/// Initialize procfs with kernel version info
#[no_mangle]
pub unsafe extern "C" fn sigma_procfs_init(
    version:    *const u8,
    build_date: *const u8,
    compiler:   *const u8,
    arch:       *const u8,
) -> SigmaI32 {
    let p = &mut PROCFS;
    p.proc_count = 0;
    p.initialized = true;

    procfs_strncpy(p.version.kernel_version.as_mut_ptr(), version,    64);
    procfs_strncpy(p.version.build_date.as_mut_ptr(),     build_date, 32);
    procfs_strncpy(p.version.compiler.as_mut_ptr(),       compiler,   32);
    procfs_strncpy(p.version.arch.as_mut_ptr(),           arch,       16);

    let vendor = b"SigmaOS\0";
    let model  = b"SigmaCore v1\0";
    procfs_strncpy(p.cpuinfo.vendor.as_mut_ptr(),     vendor.as_ptr(), 32);
    procfs_strncpy(p.cpuinfo.model_name.as_mut_ptr(), model.as_ptr(),  64);

    0
}

/// Register/update a process in procfs
#[no_mangle]
pub unsafe extern "C" fn sigma_procfs_update_proc(info: *const ProcInfo) -> SigmaI32 {
    let p = &mut PROCFS;
    let pid = (*info).pid;

    // Find existing entry by PID
    for i in 0..p.proc_count as usize {
        if p.procs[i].pid == pid && p.procs[i].active {
            p.procs[i] = *info;
            return 0;
        }
    }

    // New entry
    if p.proc_count as usize >= PROCFS_MAX_PROCS { return -1; }
    let idx = p.proc_count as usize;
    p.procs[idx] = *info;
    p.procs[idx].active = true;
    p.proc_count += 1;
    0
}

/// Remove a process from procfs
#[no_mangle]
pub unsafe extern "C" fn sigma_procfs_remove_proc(pid: SigmaU32) -> SigmaI32 {
    let p = &mut PROCFS;
    for i in 0..p.proc_count as usize {
        if p.procs[i].pid == pid {
            p.procs[i].active = false;
            p.procs[i].state  = ProcState::Dead;
            return 0;
        }
    }
    -1
}

/// Get process info by PID
#[no_mangle]
pub unsafe extern "C" fn sigma_procfs_get_proc(pid: SigmaU32, out: *mut ProcInfo) -> SigmaBool {
    let p = &PROCFS;
    for i in 0..p.proc_count as usize {
        if p.procs[i].pid == pid && p.procs[i].active {
            *out = p.procs[i];
            return true;
        }
    }
    false
}

/// List all active PIDs
#[no_mangle]
pub unsafe extern "C" fn sigma_procfs_list_pids(out: *mut SigmaU32, max: SigmaU32) -> SigmaU32 {
    let p = &PROCFS;
    let mut count = 0u32;
    for i in 0..p.proc_count as usize {
        if p.procs[i].active {
            if count < max {
                *out.add(count as usize) = p.procs[i].pid;
            }
            count += 1;
        }
    }
    count
}

/// Update /proc/meminfo
#[no_mangle]
pub unsafe extern "C" fn sigma_procfs_update_meminfo(info: *const MemInfo) {
    PROCFS.meminfo = *info;
}

/// Read /proc/meminfo
#[no_mangle]
pub unsafe extern "C" fn sigma_procfs_get_meminfo(out: *mut MemInfo) {
    *out = PROCFS.meminfo;
}

/// Update /proc/loadavg
#[no_mangle]
pub unsafe extern "C" fn sigma_procfs_update_loadavg(info: *const LoadAvg) {
    PROCFS.loadavg = *info;
}

/// Read /proc/loadavg
#[no_mangle]
pub unsafe extern "C" fn sigma_procfs_get_loadavg(out: *mut LoadAvg) {
    *out = PROCFS.loadavg;
}

/// Update /proc/uptime (called by timer tick handler)
#[no_mangle]
pub unsafe extern "C" fn sigma_procfs_update_uptime(uptime_secs: SigmaU64, idle_secs: SigmaU64) {
    PROCFS.uptime.uptime_secs = uptime_secs;
    PROCFS.uptime.idle_secs   = idle_secs;
}

/// Read /proc/uptime
#[no_mangle]
pub unsafe extern "C" fn sigma_procfs_get_uptime(out: *mut Uptime) {
    *out = PROCFS.uptime;
}

/// Read /proc/cpuinfo
#[no_mangle]
pub unsafe extern "C" fn sigma_procfs_get_cpuinfo(out: *mut CpuInfo) {
    *out = PROCFS.cpuinfo;
}

/// Read /proc/version
#[no_mangle]
pub unsafe extern "C" fn sigma_procfs_get_version(out: *mut VersionInfo) {
    *out = PROCFS.version;
}

/// Add a memory map to a process
#[no_mangle]
pub unsafe extern "C" fn sigma_procfs_add_map(
    pid:   SigmaU32,
    start: SigmaU64,
    end:   SigmaU64,
    read:  SigmaBool,
    write: SigmaBool,
    exec:  SigmaBool,
    path:  *const u8,
) -> SigmaI32 {
    let p = &mut PROCFS;
    for i in 0..p.proc_count as usize {
        if p.procs[i].pid == pid && p.procs[i].active {
            if p.procs[i].map_count as usize >= PROCFS_MAX_MAPS { return -1; }
            let mi = p.procs[i].map_count as usize;
            p.procs[i].maps[mi].start      = start;
            p.procs[i].maps[mi].end        = end;
            p.procs[i].maps[mi].perm_read  = read;
            p.procs[i].maps[mi].perm_write = write;
            p.procs[i].maps[mi].perm_exec  = exec;
            p.procs[i].maps[mi].active     = true;
            if !path.is_null() {
                procfs_strncpy(p.procs[i].maps[mi].pathname.as_mut_ptr(), path, PROCFS_NAME_LEN);
            }
            p.procs[i].map_count += 1;
            return 0;
        }
    }
    -1
}

/// Add a file descriptor entry to a process
#[no_mangle]
pub unsafe extern "C" fn sigma_procfs_add_fd(
    pid:    SigmaU32,
    fd:     SigmaI32,
    target: *const u8,
    flags:  SigmaU32,
) -> SigmaI32 {
    let p = &mut PROCFS;
    for i in 0..p.proc_count as usize {
        if p.procs[i].pid == pid && p.procs[i].active {
            if p.procs[i].fd_count as usize >= PROCFS_MAX_FDS { return -1; }
            let fi = p.procs[i].fd_count as usize;
            p.procs[i].fds[fi].fd     = fd;
            p.procs[i].fds[fi].flags  = flags;
            p.procs[i].fds[fi].active = true;
            if !target.is_null() {
                procfs_strncpy(p.procs[i].fds[fi].target.as_mut_ptr(), target, PROCFS_NAME_LEN);
            }
            p.procs[i].fd_count += 1;
            return 0;
        }
    }
    -1
}

/// Update CPU info (number of cores, frequency)
#[no_mangle]
pub unsafe extern "C" fn sigma_procfs_set_cpuinfo(
    count:    SigmaU32,
    mhz:      SigmaU32,
    cache_kb: SigmaU32,
) {
    let p = &mut PROCFS;
    p.cpuinfo.processor_count = count;
    p.cpuinfo.cpu_mhz         = mhz;
    p.cpuinfo.cache_size_kb   = cache_kb;
    p.cpuinfo.bogomips        = mhz * 2; // rough approximation
}

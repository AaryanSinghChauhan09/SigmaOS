// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS sigma_sysctl — Sovereign Sysctl Interface
//! Runtime kernel parameter tuning with hierarchical namespaces.
//! no_std, no alloc, no external crates.

#![no_std]
#![allow(dead_code)]

type SigmaU8    = u8;
type SigmaU32   = u32;
type SigmaU64   = u64;
type SigmaI32   = i32;
type SigmaI64   = i64;
type SigmaBool  = bool;

pub const SYSCTL_MAX_PARAMS:  usize = 128;
pub const SYSCTL_KEY_LEN:     usize = 64;   // e.g., "kernel.hostname"
pub const SYSCTL_VAL_LEN:     usize = 128;
pub const SYSCTL_DESC_LEN:    usize = 128;

// ─── Parameter Type ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum SysctlType {
    String  = 0,
    Int     = 1,
    UInt    = 2,
    Bool    = 3,
    Long    = 4,
    ULong   = 5,
}

/// Permission
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum SysctlPerm {
    ReadOnly   = 0,   // 0444
    ReadWrite  = 1,   // 0644
}

/// A single sysctl parameter
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SysctlParam {
    pub key:         [u8; SYSCTL_KEY_LEN],
    pub value:       [u8; SYSCTL_VAL_LEN],
    pub description: [u8; SYSCTL_DESC_LEN],
    pub param_type:  SysctlType,
    pub permission:  SysctlPerm,
    // Numeric bounds for validation
    pub min_val:     SigmaI64,
    pub max_val:     SigmaI64,
    // Callback on write (fn(*const u8, usize) -> i32)
    pub on_write:    SigmaU64,
    pub active:      SigmaBool,
}

impl SysctlParam {
    pub const fn empty() -> Self {
        Self {
            key:         [0u8; SYSCTL_KEY_LEN],
            value:       [0u8; SYSCTL_VAL_LEN],
            description: [0u8; SYSCTL_DESC_LEN],
            param_type:  SysctlType::String,
            permission:  SysctlPerm::ReadOnly,
            min_val:     0,
            max_val:     SigmaI64::MAX,
            on_write:    0,
            active:      false,
        }
    }
}

// ─── Global State ───────────────────────────────────────────────────────────

struct SysctlState {
    params:      [SysctlParam; SYSCTL_MAX_PARAMS],
    param_count: SigmaU32,
    initialized: SigmaBool,
}

static mut SYSCTL: SysctlState = SysctlState {
    params:      [SysctlParam::empty(); SYSCTL_MAX_PARAMS],
    param_count: 0,
    initialized: false,
};

// ─── Helpers ────────────────────────────────────────────────────────────────

unsafe fn sysctl_strncpy(dst: *mut u8, src: *const u8, n: usize) {
    let mut i = 0;
    while i < n { let b = *src.add(i); *dst.add(i) = b; if b == 0 { return; } i += 1; }
    if n > 0 { *dst.add(n - 1) = 0; }
}

unsafe fn sysctl_strcmp(a: *const u8, b: *const u8) -> i32 {
    let mut i = 0usize;
    loop {
        let ca = *a.add(i); let cb = *b.add(i);
        if ca != cb { return ca as i32 - cb as i32; }
        if ca == 0  { return 0; }
        i += 1;
    }
}

/// Check if `key` starts with `prefix` (for namespace listing)
unsafe fn sysctl_starts_with(key: *const u8, prefix: *const u8) -> bool {
    let mut i = 0usize;
    loop {
        let p = *prefix.add(i);
        if p == 0 { return true; }
        let k = *key.add(i);
        if k != p { return false; }
        i += 1;
    }
}

/// Parse a decimal integer from a byte string
unsafe fn sysctl_parse_i64(s: *const u8) -> SigmaI64 {
    let mut i = 0usize;
    let mut neg = false;
    let mut val: SigmaI64 = 0;

    if *s.add(0) == b'-' { neg = true; i = 1; }
    else if *s.add(0) == b'+' { i = 1; }

    loop {
        let c = *s.add(i);
        if c < b'0' || c > b'9' { break; }
        val = val * 10 + (c - b'0') as SigmaI64;
        i += 1;
    }

    if neg { -val } else { val }
}

/// Format an i64 into a byte buffer
unsafe fn sysctl_format_i64(val: SigmaI64, buf: *mut u8, buf_len: usize) -> usize {
    let mut tmp = [0u8; 21]; // max i64 digits + sign + null
    let mut v = if val < 0 { -(val as i128) as u64 } else { val as u64 };
    let mut pos = 20usize;

    if v == 0 {
        tmp[pos] = b'0';
        pos -= 1;
    } else {
        while v > 0 {
            tmp[pos] = b'0' + (v % 10) as u8;
            v /= 10;
            if pos == 0 { break; }
            pos -= 1;
        }
    }
    if val < 0 { tmp[pos] = b'-'; } else { pos += 1; }

    let len = 21 - pos;
    let copy = if len < buf_len { len } else { buf_len - 1 };
    for i in 0..copy {
        *buf.add(i) = tmp[pos + i];
    }
    *buf.add(copy) = 0;
    copy
}

// ─── C-ABI Exports ──────────────────────────────────────────────────────────

/// Initialize sysctl with default kernel parameters
#[no_mangle]
pub unsafe extern "C" fn sigma_sysctl_init() -> SigmaI32 {
    let s = &mut SYSCTL;
    s.param_count = 0;
    s.initialized = true;

    // Register default parameters
    struct DefaultParam {
        key: &'static [u8],
        val: &'static [u8],
        desc: &'static [u8],
        ptype: SysctlType,
        perm: SysctlPerm,
    }

    let defaults = [
        DefaultParam { key: b"kernel.hostname\0",     val: b"sigmaos\0",   desc: b"System hostname\0",                   ptype: SysctlType::String, perm: SysctlPerm::ReadWrite },
        DefaultParam { key: b"kernel.ostype\0",       val: b"SigmaOS\0",   desc: b"Operating system type\0",              ptype: SysctlType::String, perm: SysctlPerm::ReadOnly },
        DefaultParam { key: b"kernel.osrelease\0",    val: b"1.0.0\0",     desc: b"Kernel release version\0",             ptype: SysctlType::String, perm: SysctlPerm::ReadOnly },
        DefaultParam { key: b"kernel.pid_max\0",      val: b"32768\0",     desc: b"Maximum PID value\0",                  ptype: SysctlType::Int,    perm: SysctlPerm::ReadWrite },
        DefaultParam { key: b"kernel.threads-max\0",  val: b"4096\0",      desc: b"Maximum threads\0",                    ptype: SysctlType::Int,    perm: SysctlPerm::ReadWrite },
        DefaultParam { key: b"kernel.sched_rr_timeslice_ms\0", val: b"100\0", desc: b"Round-robin timeslice (ms)\0",       ptype: SysctlType::Int,    perm: SysctlPerm::ReadWrite },
        DefaultParam { key: b"vm.swappiness\0",       val: b"60\0",        desc: b"Swap tendency (0-100)\0",              ptype: SysctlType::Int,    perm: SysctlPerm::ReadWrite },
        DefaultParam { key: b"vm.overcommit_memory\0", val: b"0\0",        desc: b"Memory overcommit mode\0",             ptype: SysctlType::Int,    perm: SysctlPerm::ReadWrite },
        DefaultParam { key: b"vm.dirty_ratio\0",      val: b"20\0",        desc: b"Max dirty page ratio (%)\0",           ptype: SysctlType::Int,    perm: SysctlPerm::ReadWrite },
        DefaultParam { key: b"vm.dirty_background_ratio\0", val: b"10\0",  desc: b"Background dirty ratio (%)\0",         ptype: SysctlType::Int,    perm: SysctlPerm::ReadWrite },
        DefaultParam { key: b"net.core.somaxconn\0",  val: b"4096\0",      desc: b"Max socket listen backlog\0",          ptype: SysctlType::Int,    perm: SysctlPerm::ReadWrite },
        DefaultParam { key: b"net.ipv4.ip_forward\0", val: b"0\0",         desc: b"Enable IPv4 forwarding\0",             ptype: SysctlType::Bool,   perm: SysctlPerm::ReadWrite },
        DefaultParam { key: b"net.ipv4.tcp_syncookies\0", val: b"1\0",     desc: b"Enable TCP SYN cookies\0",             ptype: SysctlType::Bool,   perm: SysctlPerm::ReadWrite },
        DefaultParam { key: b"fs.file-max\0",         val: b"65536\0",     desc: b"Max open files system-wide\0",         ptype: SysctlType::Int,    perm: SysctlPerm::ReadWrite },
        DefaultParam { key: b"fs.inotify.max_user_watches\0", val: b"65536\0", desc: b"Max inotify watches per user\0",   ptype: SysctlType::Int,    perm: SysctlPerm::ReadWrite },
    ];

    for d in defaults.iter() {
        sigma_sysctl_register(
            d.key.as_ptr(), d.val.as_ptr(), d.desc.as_ptr(),
            d.ptype, d.perm, 0, SigmaI64::MAX, 0,
        );
    }

    0
}

/// Register a sysctl parameter
#[no_mangle]
pub unsafe extern "C" fn sigma_sysctl_register(
    key:        *const u8,
    value:      *const u8,
    desc:       *const u8,
    param_type: SysctlType,
    permission: SysctlPerm,
    min_val:    SigmaI64,
    max_val:    SigmaI64,
    on_write:   SigmaU64,
) -> SigmaI32 {
    let s = &mut SYSCTL;
    if s.param_count as usize >= SYSCTL_MAX_PARAMS { return -1; }

    let idx = s.param_count as usize;
    sysctl_strncpy(s.params[idx].key.as_mut_ptr(),         key,   SYSCTL_KEY_LEN);
    sysctl_strncpy(s.params[idx].value.as_mut_ptr(),       value, SYSCTL_VAL_LEN);
    sysctl_strncpy(s.params[idx].description.as_mut_ptr(), desc,  SYSCTL_DESC_LEN);
    s.params[idx].param_type = param_type;
    s.params[idx].permission = permission;
    s.params[idx].min_val    = min_val;
    s.params[idx].max_val    = max_val;
    s.params[idx].on_write   = on_write;
    s.params[idx].active     = true;

    s.param_count += 1;
    idx as SigmaI32
}

/// Read a sysctl value by key
#[no_mangle]
pub unsafe extern "C" fn sigma_sysctl_get(
    key:     *const u8,
    out:     *mut u8,
    out_len: SigmaU32,
) -> SigmaBool {
    let s = &SYSCTL;
    for i in 0..s.param_count as usize {
        if s.params[i].active && sysctl_strcmp(s.params[i].key.as_ptr(), key) == 0 {
            sysctl_strncpy(out, s.params[i].value.as_ptr(), out_len as usize);
            return true;
        }
    }
    false
}

/// Write a sysctl value by key
#[no_mangle]
pub unsafe extern "C" fn sigma_sysctl_set(
    key:   *const u8,
    value: *const u8,
) -> SigmaI32 {
    let s = &mut SYSCTL;
    for i in 0..s.param_count as usize {
        if !s.params[i].active { continue; }
        if sysctl_strcmp(s.params[i].key.as_ptr(), key) != 0 { continue; }

        if s.params[i].permission == SysctlPerm::ReadOnly { return -2; }

        // Validate numeric types
        match s.params[i].param_type {
            SysctlType::Int | SysctlType::Long => {
                let v = sysctl_parse_i64(value);
                if v < s.params[i].min_val || v > s.params[i].max_val {
                    return -3; // out of bounds
                }
            }
            SysctlType::UInt | SysctlType::ULong => {
                let v = sysctl_parse_i64(value);
                if v < 0 || v > s.params[i].max_val { return -3; }
            }
            SysctlType::Bool => {
                let c = *value;
                if c != b'0' && c != b'1' { return -3; }
            }
            SysctlType::String => {}
        }

        sysctl_strncpy(s.params[i].value.as_mut_ptr(), value, SYSCTL_VAL_LEN);

        // Fire on_write callback
        if s.params[i].on_write != 0 {
            // In real impl: cast to fn and call
        }

        return 0;
    }
    -1 // key not found
}

/// List all parameters under a namespace prefix (e.g., "kernel." or "net.")
#[no_mangle]
pub unsafe extern "C" fn sigma_sysctl_list(
    prefix:   *const u8,
    out_keys: *mut [u8; SYSCTL_KEY_LEN],
    max:      SigmaU32,
) -> SigmaU32 {
    let s = &SYSCTL;
    let mut count = 0u32;
    for i in 0..s.param_count as usize {
        if count >= max { break; }
        if !s.params[i].active { continue; }
        if prefix.is_null() || *prefix == 0 || sysctl_starts_with(s.params[i].key.as_ptr(), prefix) {
            let dst = &mut *out_keys.add(count as usize);
            sysctl_strncpy(dst.as_mut_ptr(), s.params[i].key.as_ptr(), SYSCTL_KEY_LEN);
            count += 1;
        }
    }
    count
}

/// Get parameter description
#[no_mangle]
pub unsafe extern "C" fn sigma_sysctl_describe(
    key:     *const u8,
    out:     *mut u8,
    out_len: SigmaU32,
) -> SigmaBool {
    let s = &SYSCTL;
    for i in 0..s.param_count as usize {
        if s.params[i].active && sysctl_strcmp(s.params[i].key.as_ptr(), key) == 0 {
            sysctl_strncpy(out, s.params[i].description.as_ptr(), out_len as usize);
            return true;
        }
    }
    false
}

/// Get total parameter count
#[no_mangle]
pub unsafe extern "C" fn sigma_sysctl_count() -> SigmaU32 {
    unsafe { SYSCTL.param_count }
}

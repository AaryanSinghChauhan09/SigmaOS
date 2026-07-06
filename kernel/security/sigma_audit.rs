// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS Kernel Audit Trail (auditd-style)
//! Ring buffer for security-relevant kernel events.
//! no_std, no alloc.

#![no_std]
#![allow(dead_code)]

type SigmaU32 = u32;
type SigmaU64 = u64;

pub const AUDIT_RING_SIZE: usize = 256;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum AuditEventType {
    Syscall = 1,
    MacDeny = 2,
    LoginFail = 3,
    FileAccess = 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AuditRecord {
    pub timestamp_ns: SigmaU64,
    pub pid: SigmaU32,
    pub uid: SigmaU32,
    pub event_type: AuditEventType,
    pub result: u32, // 0 = fail, 1 = success
    pub data: [u8; 32], // Additional contextual data
}

static mut AUDIT_RING: [AuditRecord; AUDIT_RING_SIZE] = [AuditRecord {
    timestamp_ns: 0, pid: 0, uid: 0, event_type: AuditEventType::Syscall, result: 0, data: [0; 32]
}; AUDIT_RING_SIZE];

static mut AUDIT_HEAD: usize = 0;
static mut AUDIT_TAIL: usize = 0;
static mut AUDIT_DROPPED: SigmaU32 = 0;

#[no_mangle]
pub unsafe extern "C" fn sigma_audit_init() {
    AUDIT_HEAD = 0;
    AUDIT_TAIL = 0;
    AUDIT_DROPPED = 0;
}

/// Log an audit event.
#[no_mangle]
pub unsafe extern "C" fn sigma_audit_log(
    timestamp_ns: SigmaU64,
    pid: SigmaU32,
    uid: SigmaU32,
    event_type: u8,
    result: u32,
    data: *const u8,
    data_len: usize
) {
    let next = (AUDIT_TAIL + 1) % AUDIT_RING_SIZE;
    if next == AUDIT_HEAD {
        // Ring full, drop record
        AUDIT_DROPPED += 1;
        return;
    }
    
    let evt = match event_type {
        1 => AuditEventType::Syscall,
        2 => AuditEventType::MacDeny,
        3 => AuditEventType::LoginFail,
        4 => AuditEventType::FileAccess,
        _ => AuditEventType::Syscall, // fallback
    };
    
    let mut record = AuditRecord {
        timestamp_ns, pid, uid, event_type: evt, result, data: [0; 32]
    };
    
    if !data.is_null() && data_len > 0 {
        let len = data_len.min(32);
        for i in 0..len {
            record.data[i] = *data.add(i);
        }
    }
    
    AUDIT_RING[AUDIT_TAIL] = record;
    AUDIT_TAIL = next;
}

/// Read an audit record. Returns 1 if successful, 0 if empty.
#[no_mangle]
pub unsafe extern "C" fn sigma_audit_read(out: *mut AuditRecord) -> SigmaU32 {
    if out.is_null() || AUDIT_HEAD == AUDIT_TAIL {
        return 0;
    }
    
    *out = AUDIT_RING[AUDIT_HEAD];
    AUDIT_HEAD = (AUDIT_HEAD + 1) % AUDIT_RING_SIZE;
    1
}

#[no_mangle]
pub unsafe extern "C" fn sigma_audit_dropped_count() -> SigmaU32 {
    AUDIT_DROPPED
}

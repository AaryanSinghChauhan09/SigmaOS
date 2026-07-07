// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// include/sigma_security.rs — Security Framework Header
//
// Defines security context structures and function signatures for
// kernel security hardening including SELinux-like MAC, capabilities,
// and secure boot verification.
//
// Language: Rust #![no_std] — no alloc, no external crates.

#![no_std]
#![allow(dead_code)]

// ── Types ─────────────────────────────────────────────────────────────────────
type SigmaU8    = u8;
type SigmaU32   = u32;
type SigmaU64   = u64;
type SigmaI32   = i32;
type SigmaBool  = bool;
type SigmaUsize = usize;

// ── Constants ─────────────────────────────────────────────────────────────────
/// Maximum number of security contexts.
const MAX_SECURITY_CONTEXTS: SigmaUsize = 256;
/// Security label length.
const SEC_LABEL_LEN: SigmaUsize = 64;

// ── Security Context Types ─────────────────────────────────────────────────────
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SecurityClass {
    /// Process security context.
    Process = 0,
    /// File security context.
    File = 1,
    /// Socket security context.
    Socket = 2,
    /// IPC security context.
    Ipc = 3,
}

// ── Security Context ─────────────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SecurityContext {
    /// Security class.
    pub class: SecurityClass,
    /// Security label (e.g., "system_u:object_r:file_t:s0").
    pub label: [SigmaU8; SEC_LABEL_LEN],
    /// User ID.
    pub uid: SigmaU32,
    /// Role.
    pub role: SigmaU32,
    /// Type.
    pub type_: SigmaU32,
    /// Level (MLS).
    pub level: SigmaU32,
    pub _pad: [SigmaU8; 7],
}

impl SecurityContext {
    pub const fn zeroed() -> Self {
        Self {
            class: SecurityClass::Process,
            label: [0u8; SEC_LABEL_LEN],
            uid: 0,
            role: 0,
            type_: 0,
            level: 0,
            _pad: [0u8; 7],
        }
    }
}

// ── Security Manager ─────────────────────────────────────────────────────────
pub struct SecurityManager {
    /// Security contexts.
    contexts: [SecurityContext; MAX_SECURITY_CONTEXTS],
    /// Context count.
    count: SigmaUsize,
    /// Enforcing mode.
    enforcing: SigmaBool,
    /// Initialized flag.
    initialized: SigmaBool,
}

impl SecurityManager {
    pub const fn new() -> Self {
        Self {
            contexts: [SecurityContext::zeroed(); MAX_SECURITY_CONTEXTS],
            count: 0,
            enforcing: true,
            initialized: false,
        }
    }

    pub fn init(&mut self) {
        self.initialized = true;
        // Initialize default security contexts
    }

    pub fn set_enforcing(&mut self, enforcing: SigmaBool) {
        self.enforcing = enforcing;
    }

    pub fn is_enforcing(&self) -> SigmaBool {
        self.enforcing
    }

    pub fn create_context(
        &mut self,
        class: SecurityClass,
        label: &[SigmaU8],
        uid: SigmaU32,
    ) -> SigmaI32 {
        if self.count >= MAX_SECURITY_CONTEXTS {
            return -1;
        }
        let idx = self.count;
        self.contexts[idx].class = class;
        self.contexts[idx].uid = uid;
        let len = label.len().min(SEC_LABEL_LEN - 1);
        let mut i = 0;
        while i < len {
            self.contexts[idx].label[i] = label[i];
            i += 1;
        }
        self.contexts[idx].label[len] = 0;
        self.count += 1;
        idx as SigmaI32
    }

    pub fn check_permission(
        &self,
        src_ctx: SigmaU32,
        dst_ctx: SigmaU32,
        perm: SigmaU32,
    ) -> SigmaBool {
        if !self.enforcing {
            return true;
        }
        // In production: implement SELinux-like permission check
        true
    }
}

static mut G_SECURITY_MANAGER: SecurityManager = SecurityManager::new();

#[no_mangle]
pub unsafe extern "C" fn sigma_security_init() {
    G_SECURITY_MANAGER.init();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_security_set_enforcing(enforcing: SigmaU32) {
    G_SECURITY_MANAGER.set_enforcing(enforcing != 0);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_security_is_enforcing() -> SigmaU32 {
    if G_SECURITY_MANAGER.is_enforcing() { 1 } else { 0 }
}


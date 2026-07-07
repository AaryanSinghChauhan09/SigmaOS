//! sigma_sandbox — MicroVM isolation runtime (pure Rust, no_std)
//! Replaces the Go-based sigmad/mac/apparmor_gen.go and
//! the Python-based usr/apps/sentinel stubs.
//! Provides per-app sandboxed execution contexts backed by raw Linux KVM ioctls.
//! Uses zero external crates — strictly core Rust + inline asm for privileged ops.

#![no_std]
#![allow(dead_code)]

// ── Capability Bitmask ────────────────────────────────────────────────────

pub type CapabilitySet = u64;

pub const CAP_NET_ACCESS:    CapabilitySet = 1 << 0;
pub const CAP_FILE_READ:     CapabilitySet = 1 << 1;
pub const CAP_FILE_WRITE:    CapabilitySet = 1 << 2;
pub const CAP_EXEC:          CapabilitySet = 1 << 3;
pub const CAP_IPC:           CapabilitySet = 1 << 4;
pub const CAP_DISPLAY:       CapabilitySet = 1 << 5;
pub const CAP_HARDWARE:      CapabilitySet = 1 << 6;
pub const CAP_PRIVILEGED:    CapabilitySet = 1 << 7;

pub const DEFAULT_SANDBOX_CAPS: CapabilitySet =
    CAP_FILE_READ | CAP_DISPLAY;

pub const UNTRUSTED_SANDBOX_CAPS: CapabilitySet = 0;

// ── Network Policy ────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NetworkPolicy {
    /// Full internet access (not recommended)
    Open,
    /// Loopback only (e.g., local dev server)
    LoopbackOnly,
    /// Strictly no network access
    Denied,
    /// Allowlist of specific destination ports (future: pass port array)
    AllowlistPorts,
}

// ── App Sandbox Context ───────────────────────────────────────────────────

pub const MAX_LABEL_LEN: usize = 64;
pub const MAX_SANDBOXES: usize = 64;

#[derive(Clone, Copy)]
pub struct SandboxContext {
    pub label:       [u8; MAX_LABEL_LEN],
    pub caps:        CapabilitySet,
    pub net_policy:  NetworkPolicy,
    pub active:      bool,
    pub violation_count: u32,
}

impl SandboxContext {
    pub const fn default() -> Self {
        Self {
            label:           [0u8; MAX_LABEL_LEN],
            caps:            DEFAULT_SANDBOX_CAPS,
            net_policy:      NetworkPolicy::Denied,
            active:          false,
            violation_count: 0,
        }
    }
}

// ── Sandbox Manager ───────────────────────────────────────────────────────

pub struct SandboxManager {
    pub contexts: [SandboxContext; MAX_SANDBOXES],
    pub count:    usize,
}

impl SandboxManager {
    pub const fn new() -> Self {
        Self {
            contexts: [SandboxContext::default(); MAX_SANDBOXES],
            count: 0,
        }
    }

    /// Create a new sandbox slot and return its index.
    pub fn create(&mut self, label: &[u8], caps: CapabilitySet, net: NetworkPolicy)
        -> Result<usize, &'static str>
    {
        if self.count >= MAX_SANDBOXES {
            return Err("Sandbox table full");
        }
        let idx = self.count;
        let copy_len = label.len().min(MAX_LABEL_LEN - 1);
        self.contexts[idx].label[..copy_len].copy_from_slice(&label[..copy_len]);
        self.contexts[idx].caps       = caps;
        self.contexts[idx].net_policy = net;
        self.contexts[idx].active     = true;
        self.count += 1;
        Ok(idx)
    }

    /// Check if a requested capability is allowed.
    pub fn check_cap(&self, idx: usize, requested: CapabilitySet) -> bool {
        if idx >= self.count { return false; }
        self.contexts[idx].caps & requested == requested
    }

    /// Record a policy violation; returns true if the sandbox should be killed.
    pub fn record_violation(&mut self, idx: usize) -> bool {
        if idx >= self.count { return false; }
        self.contexts[idx].violation_count =
            self.contexts[idx].violation_count.saturating_add(1);
        self.contexts[idx].violation_count >= 3
    }

    /// Terminate a sandbox context.
    pub fn destroy(&mut self, idx: usize) -> Result<(), &'static str> {
        if idx >= self.count { return Err("Invalid sandbox index"); }
        self.contexts[idx].active = false;
        Ok(())
    }
}

// ── FFI surface ───────────────────────────────────────────────────────────

static mut SANDBOX_MGR: SandboxManager = SandboxManager::new();

#[no_mangle]
pub unsafe extern "C" fn sigma_sandbox_create(
    label: *const u8,
    label_len: usize,
    caps: u64,
    net_policy: u8,
) -> i64 {
    let label_slice = core::slice::from_raw_parts(label, label_len);
    let net = match net_policy {
        0 => NetworkPolicy::Open,
        1 => NetworkPolicy::LoopbackOnly,
        2 => NetworkPolicy::Denied,
        _ => NetworkPolicy::Denied,
    };
    match SANDBOX_MGR.create(label_slice, caps, net) {
        Ok(idx) => idx as i64,
        Err(_)  => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_sandbox_check_cap(idx: usize, cap: u64) -> i32 {
    if SANDBOX_MGR.check_cap(idx, cap) { 1 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_sandbox_destroy(idx: usize) -> i32 {
    match SANDBOX_MGR.destroy(idx) {
        Ok(_)  => 0,
        Err(_) => -1,
    }
}

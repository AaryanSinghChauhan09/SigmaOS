// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// kernel/security/sigma_capability.rs — Capability Token System (no_std)
// Language: Rust #![no_std] — OOP via CapabilityManager + Token

#![no_std]
use core::sync::atomic::{AtomicU64, Ordering};

static NEXT_TOKEN: AtomicU64 = AtomicU64::new(1);

// ── Capability Rights ─────────────────────────────────────────────────────────
#[derive(Clone, Copy, Default)]
pub struct Rights(pub u64);

impl Rights {
    pub const READ:       u64 = 1 << 0;
    pub const WRITE:      u64 = 1 << 1;
    pub const EXEC:       u64 = 1 << 2;
    pub const SEND:       u64 = 1 << 3;
    pub const RECV:       u64 = 1 << 4;
    pub const GRANT:      u64 = 1 << 5;  // can give rights to others
    pub const REVOKE:     u64 = 1 << 6;  // can revoke derived caps
    pub const REDUCE:     u64 = 1 << 7;  // can derive with fewer rights
    pub const IO:         u64 = 1 << 8;
    pub const MMAP:       u64 = 1 << 9;
    pub const FORK:       u64 = 1 << 10;
    pub const NET:        u64 = 1 << 11;
    pub const DEVICE:     u64 = 1 << 12;
    pub const SETUID:     u64 = 1 << 13;
    pub const KILL:       u64 = 1 << 14;
    pub const ALL: u64 = u64::MAX;

    pub fn has(&self, r: u64) -> bool { self.0 & r == r }
    pub fn add(&mut self, r: u64)    { self.0 |= r; }
    pub fn remove(&mut self, r: u64) { self.0 &= !r; }
    /// Derive new rights — can only restrict, never expand
    pub fn derive(&self, subset: u64) -> Option<Rights> {
        if subset & !self.0 != 0 { return None; } // trying to expand
        Some(Rights(self.0 & subset))
    }
}

// ── Capability Token ──────────────────────────────────────────────────────────
#[derive(Clone, Copy, Debug)]
pub struct CapToken {
    pub id:         u64,
    pub owner_pid:  u32,
    pub resource:   CapResource,
    pub rights:     Rights,
    pub parent_id:  Option<u64>,
    pub revoked:    bool,
    pub expires:    Option<u64>,  // unix seconds, None = permanent
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CapResource {
    File   { inode: u64 },
    Fd     { fd: u32 },
    Ipc    { channel: u32 },
    Memory { base: u64, size: u64 },
    Device { major: u32, minor: u32 },
    Network { port: u16, proto: u8 },
    Process { pid: u32 },
    Any,
}

impl CapToken {
    pub fn new(owner: u32, resource: CapResource, rights: Rights) -> Self {
        Self {
            id: NEXT_TOKEN.fetch_add(1, Ordering::Relaxed),
            owner_pid: owner, resource, rights,
            parent_id: None, revoked: false, expires: None,
        }
    }

    pub fn is_valid(&self, now: u64) -> bool {
        !self.revoked && self.expires.map(|e| now < e).unwrap_or(true)
    }

    /// Derive a child capability with reduced rights
    pub fn derive(&self, pid: u32, subset: u64) -> Option<CapToken> {
        if self.revoked { return None; }
        let new_rights = self.rights.derive(subset)?;
        Some(CapToken {
            id: NEXT_TOKEN.fetch_add(1, Ordering::Relaxed),
            owner_pid: pid,
            resource: self.resource,
            rights: new_rights,
            parent_id: Some(self.id),
            revoked: false,
            expires: self.expires,
        })
    }
}

// ── Capability Table ──────────────────────────────────────────────────────────
pub const MAX_CAPS: usize = 1024;

pub struct CapabilityManager {
    tokens:  [Option<CapToken>; MAX_CAPS],
    n_tokens: usize,
}

impl CapabilityManager {
    pub const fn new() -> Self {
        Self { tokens: [const { None }; MAX_CAPS], n_tokens: 0 }
    }

    pub fn insert(&mut self, token: CapToken) -> bool {
        if self.n_tokens >= MAX_CAPS { return false; }
        for slot in &mut self.tokens {
            if slot.is_none() { *slot = Some(token); self.n_tokens += 1; return true; }
        }
        false
    }

    pub fn get(&self, id: u64) -> Option<&CapToken> {
        self.tokens.iter().flatten().find(|t| t.id == id)
    }

    pub fn get_mut(&mut self, id: u64) -> Option<&mut CapToken> {
        self.tokens.iter_mut().flatten().find(|t| t.id == id)
    }

    pub fn check(&self, id: u64, right: u64, now: u64) -> bool {
        match self.get(id) {
            Some(t) => t.is_valid(now) && t.rights.has(right),
            None    => false,
        }
    }

    pub fn revoke(&mut self, id: u64) {
        // Revoke the token and all derived children
        let mut to_revoke = [0u64; 64]; let mut n = 0;
        to_revoke[0] = id; n = 1;
        loop {
            let mut new_n = n;
            for slot in self.tokens.iter_mut().flatten() {
                if !slot.revoked && to_revoke[..n].contains(&slot.parent_id.unwrap_or(u64::MAX)) {
                    if new_n < 64 { to_revoke[new_n] = slot.id; new_n += 1; }
                    slot.revoked = true;
                }
            }
            if new_n == n { break; }
            n = new_n;
        }
        if let Some(t) = self.get_mut(id) { t.revoked = true; }
    }

    pub fn revoke_pid(&mut self, pid: u32) {
        for slot in self.tokens.iter_mut().flatten() {
            if slot.owner_pid == pid { slot.revoked = true; }
        }
    }

    pub fn for_pid(&self, pid: u32) -> impl Iterator<Item = &CapToken> {
        self.tokens.iter().flatten().filter(move |t| t.owner_pid == pid && !t.revoked)
    }
}

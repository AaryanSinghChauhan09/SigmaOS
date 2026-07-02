// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// kernel/security/sigma_spiffe.rs — SPIFFE Workload Identity (no_std)
// Language: Rust #![no_std] — OOP via SpiffeIdentity + SvidStore

#![no_std]

pub const MAX_TRUST_DOMAIN: usize = 128;
pub const MAX_PATH_LEN:     usize = 256;
pub const MAX_SVIDS:        usize = 64;
pub const SVID_TTL_SECS:    u64   = 3600;

// ── SPIFFE ID ─────────────────────────────────────────────────────────────────
/// spiffe://<trust-domain>/<path>
#[derive(Clone, Copy)]
pub struct SpiffeId {
    pub trust_domain: [u8; MAX_TRUST_DOMAIN],
    pub td_len:       usize,
    pub path:         [u8; MAX_PATH_LEN],
    pub path_len:     usize,
}

impl SpiffeId {
    pub fn new(trust_domain: &[u8], path: &[u8]) -> Self {
        let mut id = Self {
            trust_domain: [0u8; MAX_TRUST_DOMAIN], td_len: trust_domain.len().min(MAX_TRUST_DOMAIN),
            path:         [0u8; MAX_PATH_LEN],     path_len: path.len().min(MAX_PATH_LEN),
        };
        id.trust_domain[..id.td_len].copy_from_slice(&trust_domain[..id.td_len]);
        id.path[..id.path_len].copy_from_slice(&path[..id.path_len]);
        id
    }

    pub fn matches(&self, other: &SpiffeId) -> bool {
        self.td_len == other.td_len
            && self.path_len == other.path_len
            && self.trust_domain[..self.td_len] == other.trust_domain[..other.td_len]
            && self.path[..self.path_len] == other.path[..other.path_len]
    }

    /// Serialize as "spiffe://<td>/<path>"
    pub fn to_uri(&self, out: &mut [u8; 512]) -> usize {
        let prefix = b"spiffe://";
        let mut off = 0;
        out[off..off+prefix.len()].copy_from_slice(prefix); off += prefix.len();
        out[off..off+self.td_len].copy_from_slice(&self.trust_domain[..self.td_len]); off += self.td_len;
        out[off] = b'/'; off += 1;
        out[off..off+self.path_len].copy_from_slice(&self.path[..self.path_len]); off += self.path_len;
        off
    }
}

// ── SVID (SPIFFE Verifiable Identity Document) ────────────────────────────────
#[derive(Clone, Copy)]
pub struct Svid {
    pub id:         SpiffeId,
    pub issued_at:  u64,   // unix seconds
    pub expires_at: u64,
    pub serial:     u64,
    /// Public key fingerprint (SHA-256 of DER-encoded Dilithium-5 public key)
    pub key_fp:     [u8; 32],
    /// Dilithium-5 signature over (id_uri + issued_at + expires_at)
    pub signature:  [u8; 64], // truncated for space; full sig 4627 bytes
    pub valid:      bool,
}

impl Svid {
    pub fn new(id: SpiffeId, now: u64, key_fp: [u8;32]) -> Self {
        Self {
            id, issued_at: now, expires_at: now + SVID_TTL_SECS,
            serial: now, key_fp, signature: [0u8;64], valid: true,
        }
    }
    pub fn is_expired(&self, now: u64) -> bool { now > self.expires_at }
    pub fn is_valid(&self, now: u64) -> bool { self.valid && !self.is_expired(now) }
}

// ── Workload Entry ────────────────────────────────────────────────────────────
#[derive(Clone, Copy)]
pub struct WorkloadEntry {
    pub pid:      u32,
    pub uid:      u32,
    pub svid_idx: usize,
}

// ── SVID Store ────────────────────────────────────────────────────────────────
pub struct SvidStore {
    svids:     [Option<Svid>; MAX_SVIDS],
    n_svids:   usize,
    workloads: [Option<WorkloadEntry>; MAX_SVIDS],
    n_wl:      usize,
    trust_domain: [u8; MAX_TRUST_DOMAIN],
    td_len:    usize,
}

impl SvidStore {
    pub fn new(trust_domain: &[u8]) -> Self {
        let mut s = Self {
            svids: [const { None }; MAX_SVIDS], n_svids: 0,
            workloads: [const { None }; MAX_SVIDS], n_wl: 0,
            trust_domain: [0u8;MAX_TRUST_DOMAIN], td_len: trust_domain.len().min(MAX_TRUST_DOMAIN),
        };
        s.trust_domain[..s.td_len].copy_from_slice(&trust_domain[..s.td_len]);
        s
    }

    /// Issue an SVID for a workload path at `now`
    pub fn issue(&mut self, path: &[u8], pid: u32, uid: u32, now: u64, key_fp: [u8;32]) -> Option<usize> {
        if self.n_svids >= MAX_SVIDS { return None; }
        let id = SpiffeId::new(&self.trust_domain[..self.td_len], path);
        let svid = Svid::new(id, now, key_fp);
        for (i, slot) in self.svids.iter_mut().enumerate() {
            if slot.is_none() {
                *slot = Some(svid); self.n_svids += 1;
                for ws in &mut self.workloads {
                    if ws.is_none() { *ws = Some(WorkloadEntry { pid, uid, svid_idx: i }); self.n_wl += 1; break; }
                }
                return Some(i);
            }
        }
        None
    }

    /// Look up SVID for a PID
    pub fn lookup_pid(&self, pid: u32, now: u64) -> Option<&Svid> {
        self.workloads[..self.n_wl].iter().flatten()
            .find(|w| w.pid == pid)
            .and_then(|w| self.svids[w.svid_idx].as_ref())
            .filter(|s| s.is_valid(now))
    }

    /// Revoke all SVIDs for a PID (e.g. on process exit)
    pub fn revoke_pid(&mut self, pid: u32) {
        for ws in &mut self.workloads {
            if let Some(w) = ws {
                if w.pid == pid {
                    if let Some(ref mut s) = self.svids[w.svid_idx] { s.valid = false; }
                    *ws = None; self.n_wl -= 1;
                }
            }
        }
    }

    /// Rotate an expiring SVID
    pub fn rotate(&mut self, idx: usize, now: u64, key_fp: [u8;32]) {
        if let Some(ref mut s) = self.svids[idx] {
            s.issued_at  = now;
            s.expires_at = now + SVID_TTL_SECS;
            s.serial    += 1;
            s.key_fp     = key_fp;
        }
    }

    pub fn svid_count(&self) -> usize { self.n_svids }
}

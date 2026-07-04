// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// kernel/core/sigma_namespace.rs — Kernel Namespaces (PID/Net/Mount/UTS/IPC)
// Language: Rust #![no_std]
// Pattern: OOP via NamespaceSet + individual namespace types

#![no_std]

pub const MAX_NS: usize = 64;
pub const MAX_NS_PIDS: usize = 256;

// ── Namespace ID ──────────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NsId(pub u32);

// ── PID Namespace ─────────────────────────────────────────────────────────────
#[derive(Clone, Copy)]
pub struct PidNs {
    pub id:       NsId,
    pub parent:   Option<NsId>,
    pub pids:     [u32; MAX_NS_PIDS], // global PIDs in this namespace
    pub n_pids:   usize,
    pub init_pid: u32, // PID 1 in this namespace
}

impl PidNs {
    pub fn new(id: NsId, parent: Option<NsId>, init: u32) -> Self {
        let mut ns = Self { id, parent, pids: [0u32; MAX_NS_PIDS], n_pids: 0, init_pid: init };
        ns.add_pid(init); ns
    }
    pub fn add_pid(&mut self, pid: u32) -> bool {
        if self.n_pids >= MAX_NS_PIDS { return false; }
        self.pids[self.n_pids] = pid; self.n_pids += 1; true
    }
    pub fn remove_pid(&mut self, pid: u32) {
        if let Some(i) = self.pids[..self.n_pids].iter().position(|&p| p == pid) {
            self.n_pids -= 1; self.pids[i] = self.pids[self.n_pids];
        }
    }
    pub fn contains(&self, pid: u32) -> bool { self.pids[..self.n_pids].contains(&pid) }
}

// ── UTS Namespace (hostname / domainname) ─────────────────────────────────────
#[derive(Clone, Copy)]
pub struct UtsNs {
    pub id:        NsId,
    pub hostname:  [u8; 64],
    pub hostname_len: usize,
    pub domainname: [u8; 64],
    pub domain_len: usize,
}

impl UtsNs {
    pub fn new(id: NsId, hostname: &[u8]) -> Self {
        let mut ns = Self { id, hostname: [0u8; 64], hostname_len: hostname.len().min(64),
                            domainname: [0u8; 64], domain_len: 0 };
        ns.hostname[..ns.hostname_len].copy_from_slice(&hostname[..ns.hostname_len]);
        ns
    }
    pub fn set_hostname(&mut self, name: &[u8]) {
        let n = name.len().min(64);
        self.hostname[..n].copy_from_slice(&name[..n]);
        self.hostname_len = n;
    }
}

// ── Mount Namespace ───────────────────────────────────────────────────────────
pub const MAX_MOUNTS: usize = 32;

#[derive(Clone, Copy)]
pub struct MountEntry {
    pub mountpoint: [u8; 128],
    pub mp_len:     usize,
    pub fs_type:    [u8; 16],
    pub fs_len:     usize,
    pub flags:      u32,
}

#[derive(Clone, Copy)]
pub struct MountNs {
    pub id:      NsId,
    pub mounts:  [Option<MountEntry>; MAX_MOUNTS],
    pub n_mounts: usize,
}

impl MountNs {
    pub fn new(id: NsId) -> Self {
        Self { id, mounts: [const { None }; MAX_MOUNTS], n_mounts: 0 }
    }
    pub fn add_mount(&mut self, mp: &[u8], fs: &[u8], flags: u32) -> bool {
        if self.n_mounts >= MAX_MOUNTS { return false; }
        let mut e = MountEntry { mountpoint: [0u8; 128], mp_len: mp.len().min(128),
                                 fs_type: [0u8; 16], fs_len: fs.len().min(16), flags };
        e.mountpoint[..e.mp_len].copy_from_slice(&mp[..e.mp_len]);
        e.fs_type[..e.fs_len].copy_from_slice(&fs[..e.fs_len]);
        self.mounts[self.n_mounts] = Some(e); self.n_mounts += 1; true
    }
}

// ── Namespace Set (all namespaces for a process) ──────────────────────────────
#[derive(Clone, Copy)]
pub struct NamespaceSet {
    pub pid_ns:   NsId,
    pub uts_ns:   NsId,
    pub mnt_ns:   NsId,
    pub net_ns:   NsId,
    pub ipc_ns:   NsId,
    pub user_ns:  NsId,
}

impl NamespaceSet {
    /// All namespaces pointing to the initial (host) namespace
    pub const fn host() -> Self {
        Self { pid_ns: NsId(1), uts_ns: NsId(1), mnt_ns: NsId(1),
               net_ns: NsId(1), ipc_ns: NsId(1), user_ns: NsId(1) }
    }
}

// ── Namespace Manager ─────────────────────────────────────────────────────────
pub struct NsManager {
    pid_ns:  [Option<PidNs>;  MAX_NS],
    uts_ns:  [Option<UtsNs>;  MAX_NS],
    mnt_ns:  [Option<MountNs>;MAX_NS],
    next_id: u32,
}

impl NsManager {
    pub const fn new() -> Self {
        Self {
            pid_ns: [const { None }; MAX_NS],
            uts_ns: [const { None }; MAX_NS],
            mnt_ns: [const { None }; MAX_NS],
            next_id: 2, // 1 = host
        }
    }

    pub fn init_host(&mut self, hostname: &[u8], init_pid: u32) {
        self.pid_ns[0] = Some(PidNs::new(NsId(1), None, init_pid));
        self.uts_ns[0] = Some(UtsNs::new(NsId(1), hostname));
        self.mnt_ns[0] = Some(MountNs::new(NsId(1)));
    }

    fn alloc_id(&mut self) -> NsId {
        let id = NsId(self.next_id); self.next_id += 1; id
    }

    pub fn clone_pid_ns(&mut self, parent: NsId, init: u32) -> NsId {
        let id = self.alloc_id();
        for slot in &mut self.pid_ns {
            if slot.is_none() { *slot = Some(PidNs::new(id, Some(parent), init)); return id; }
        }
        NsId(0)
    }

    pub fn clone_uts_ns(&mut self, parent_id: NsId) -> NsId {
        let id = self.alloc_id();
        let parent_data = self.uts_ns.iter().flatten().find(|n| n.id == parent_id).copied();
        for slot in &mut self.uts_ns {
            if slot.is_none() {
                let mut ns = parent_data.unwrap_or_else(|| UtsNs::new(id, b"sigmaos"));
                ns.id = id;
                *slot = Some(ns); return id;
            }
        }
        NsId(0)
    }

    pub fn get_uts_mut(&mut self, id: NsId) -> Option<&mut UtsNs> {
        self.uts_ns.iter_mut().flatten().find(|n| n.id == id)
    }
    pub fn get_pid(&self, id: NsId) -> Option<&PidNs> {
        self.pid_ns.iter().flatten().find(|n| n.id == id)
    }
    pub fn get_pid_mut(&mut self, id: NsId) -> Option<&mut PidNs> {
        self.pid_ns.iter_mut().flatten().find(|n| n.id == id)
    }
    pub fn get_mnt_mut(&mut self, id: NsId) -> Option<&mut MountNs> {
        self.mnt_ns.iter_mut().flatten().find(|n| n.id == id)
    }
}

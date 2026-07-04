// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/sigma_namespaces.rs — Linux-compatible process namespaces
// PID, NET, MNT, IPC, UTS, USER namespaces for container isolation
// Language: Rust #![no_std]
#![no_std]
#![allow(dead_code)]

const MAX_NAMESPACES: usize = 128;
const MAX_NS_PIDS:    usize = 256;
const HOSTNAME_LEN:   usize = 64;

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum NsKind { Pid = 0, Net = 1, Mnt = 2, Ipc = 3, Uts = 4, User = 5 }

#[derive(Copy, Clone)]
pub struct Namespace {
    pub id:       u32,
    pub kind:     NsKind,
    pub parent:   u32,
    pub pids:     [u32; MAX_NS_PIDS],
    pub pid_count:usize,
    pub hostname: [u8; HOSTNAME_LEN],   // UTS ns
    pub active:   bool,
}

impl Namespace {
    pub const fn empty() -> Self {
        Self {
            id: 0, kind: NsKind::Pid, parent: 0,
            pids: [0u32; MAX_NS_PIDS], pid_count: 0,
            hostname: [0u8; HOSTNAME_LEN], active: false,
        }
    }
}

pub struct NsManager {
    ns:      [Namespace; MAX_NAMESPACES],
    next_id: u32,
}

impl NsManager {
    pub const fn new() -> Self {
        Self { ns: [const { Namespace::empty() }; MAX_NAMESPACES], next_id: 2 }
    }

    pub fn init(&mut self) {
        // Initial namespace id=1 for each kind
        for k in 0u8..6 {
            let i = k as usize;
            self.ns[i].id     = 1;
            self.ns[i].kind   = match k { 0=>NsKind::Pid, 1=>NsKind::Net, 2=>NsKind::Mnt, 3=>NsKind::Ipc, 4=>NsKind::Uts, _=>NsKind::User };
            self.ns[i].active = true;
            let h = b"sigmaos\0";
            self.ns[i].hostname[..h.len()].copy_from_slice(h);
        }
    }

    pub fn create(&mut self, kind: NsKind, parent: u32) -> u32 {
        for n in &mut self.ns {
            if !n.active {
                let id = self.next_id; self.next_id += 1;
                n.id = id; n.kind = kind; n.parent = parent; n.active = true;
                let h = b"container\0";
                n.hostname[..h.len()].copy_from_slice(h);
                return id;
            }
        }
        0
    }

    pub fn attach(&mut self, ns_id: u32, pid: u32) -> bool {
        for n in &mut self.ns {
            if n.active && n.id == ns_id && n.pid_count < MAX_NS_PIDS {
                n.pids[n.pid_count] = pid; n.pid_count += 1; return true;
            }
        }
        false
    }

    pub fn set_hostname(&mut self, ns_id: u32, name: &[u8]) -> bool {
        for n in &mut self.ns {
            if n.active && n.id == ns_id && n.kind == NsKind::Uts {
                let len = name.len().min(HOSTNAME_LEN - 1);
                n.hostname[..len].copy_from_slice(&name[..len]);
                n.hostname[len] = 0;
                return true;
            }
        }
        false
    }

    pub fn get_hostname(&self, ns_id: u32, out: *mut u8, max_len: usize) -> usize {
        for n in &self.ns {
            if n.active && n.id == ns_id && n.kind == NsKind::Uts {
                let len = n.hostname.iter().position(|&b| b == 0).unwrap_or(HOSTNAME_LEN);
                let copy = len.min(max_len);
                unsafe { core::ptr::copy_nonoverlapping(n.hostname.as_ptr(), out, copy); }
                return copy;
            }
        }
        0
    }
}

static mut G_NS: NsManager = NsManager::new();

#[no_mangle] pub unsafe extern "C" fn sigma_ns_init() { G_NS.init(); }
#[no_mangle] pub unsafe extern "C" fn sigma_ns_create(kind: u8, parent: u32) -> u32 {
    let k = match kind { 0=>NsKind::Pid, 1=>NsKind::Net, 2=>NsKind::Mnt, 3=>NsKind::Ipc, 4=>NsKind::Uts, _=>NsKind::User };
    G_NS.create(k, parent)
}
#[no_mangle] pub unsafe extern "C" fn sigma_ns_attach(ns_id: u32, pid: u32) -> i32 {
    if G_NS.attach(ns_id, pid) { 0 } else { -1 }
}
#[no_mangle] pub unsafe extern "C" fn sigma_ns_set_hostname(ns_id: u32, name: *const u8, len: usize) -> i32 {
    if name.is_null() { return -14; }
    if G_NS.set_hostname(ns_id, core::slice::from_raw_parts(name, len)) { 0 } else { -1 }
}
#[no_mangle] pub unsafe extern "C" fn sigma_ns_get_hostname(ns_id: u32, out: *mut u8, max: usize) -> usize {
    G_NS.get_hostname(ns_id, out, max)
}

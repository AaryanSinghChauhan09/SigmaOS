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

// ── Network Namespace ───────────────────────────────────────────────────────────
pub const MAX_NET_DEVICES: usize = 32;

#[derive(Clone, Copy)]
pub struct NetDevice {
    pub name: [u8; 16],
    pub ifindex: u32,
    pub mtu: u32,
    pub flags: u32,
}

#[derive(Clone, Copy)]
pub struct NetNs {
    pub id: NsId,
    pub devices: [Option<NetDevice>; MAX_NET_DEVICES],
    pub n_devices: usize,
    pub lo_enabled: bool,
}

impl NetNs {
    pub fn new(id: NsId) -> Self {
        Self { id, devices: [const { None }; MAX_NET_DEVICES], n_devices: 0, lo_enabled: true }
    }
    pub fn add_device(&mut self, name: &[u8], ifindex: u32, mtu: u32) -> bool {
        if self.n_devices >= MAX_NET_DEVICES { return false; }
        let mut dev = NetDevice { name: [0u8; 16], ifindex, mtu, flags: 0 };
        let n = name.len().min(16);
        dev.name[..n].copy_from_slice(&name[..n]);
        self.devices[self.n_devices] = Some(dev);
        self.n_devices += 1;
        true
    }
}

// ── IPC Namespace (System V IPC & POSIX message queues) ───────────────────────
pub const MAX_IPC_IDS: usize = 64;

#[derive(Clone, Copy)]
pub struct IpcId {
    pub key: i32,
    pub id: u32,
    pub perms: u16,
}

#[derive(Clone, Copy)]
pub struct IpcNs {
    pub id: NsId,
    pub ipc_ids: [Option<IpcId>; MAX_IPC_IDS],
    pub n_ipc: usize,
}

impl IpcNs {
    pub fn new(id: NsId) -> Self {
        Self { id, ipc_ids: [const { None }; MAX_IPC_IDS], n_ipc: 0 }
    }
    pub fn add_ipc(&mut self, key: i32, id: u32, perms: u16) -> bool {
        if self.n_ipc >= MAX_IPC_IDS { return false; }
        self.ipc_ids[self.n_ipc] = Some(IpcId { key, id, perms });
        self.n_ipc += 1;
        true
    }
}

// ── User Namespace (UID/GID mapping) ───────────────────────────────────────────
pub const MAX_UID_MAP: usize = 32;

#[derive(Clone, Copy)]
pub struct UidGidMap {
    pub first: u32,
    pub lower_first: u32,
    pub count: u32,
}

#[derive(Clone, Copy)]
pub struct UserNs {
    pub id: NsId,
    pub parent: Option<NsId>,
    pub uid_map: [UidGidMap; MAX_UID_MAP],
    pub gid_map: [UidGidMap; MAX_UID_MAP],
    pub n_uid_map: usize,
    pub n_gid_map: usize,
}

impl UserNs {
    pub fn new(id: NsId, parent: Option<NsId>) -> Self {
        Self { id, parent, uid_map: [UidGidMap { first: 0, lower_first: 0, count: 0 }; MAX_UID_MAP],
              gid_map: [UidGidMap { first: 0, lower_first: 0, count: 0 }; MAX_UID_MAP],
              n_uid_map: 0, n_gid_map: 0 }
    }
    pub fn add_uid_map(&mut self, first: u32, lower_first: u32, count: u32) -> bool {
        if self.n_uid_map >= MAX_UID_MAP { return false; }
        self.uid_map[self.n_uid_map] = UidGidMap { first, lower_first, count };
        self.n_uid_map += 1;
        true
    }
    pub fn add_gid_map(&mut self, first: u32, lower_first: u32, count: u32) -> bool {
        if self.n_gid_map >= MAX_UID_MAP { return false; }
        self.gid_map[self.n_gid_map] = UidGidMap { first, lower_first, count };
        self.n_gid_map += 1;
        true
    }
}

// ── Cgroup Namespace (for cgroup v2 hierarchy isolation) ─────────────────────
#[derive(Clone, Copy)]
pub struct CgroupNs {
    pub id: NsId,
    pub root_cgroup: u32, // ID of root cgroup in this namespace
}

impl CgroupNs {
    pub fn new(id: NsId, root_cgroup: u32) -> Self {
        Self { id, root_cgroup }
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
    pub cgroup_ns: NsId,
}

impl NamespaceSet {
    /// All namespaces pointing to the initial (host) namespace
    pub const fn host() -> Self {
        Self { pid_ns: NsId(1), uts_ns: NsId(1), mnt_ns: NsId(1),
               net_ns: NsId(1), ipc_ns: NsId(1), user_ns: NsId(1), cgroup_ns: NsId(1) }
    }
}

// ── Namespace Manager ─────────────────────────────────────────────────────────
pub struct NsManager {
    pid_ns:    [Option<PidNs>;    MAX_NS],
    uts_ns:    [Option<UtsNs>;    MAX_NS],
    mnt_ns:    [Option<MountNs>;  MAX_NS],
    net_ns:    [Option<NetNs>;    MAX_NS],
    ipc_ns:    [Option<IpcNs>;    MAX_NS],
    user_ns:   [Option<UserNs>;   MAX_NS],
    cgroup_ns: [Option<CgroupNs>; MAX_NS],
    next_id: u32,
}

impl NsManager {
    pub const fn new() -> Self {
        Self {
            pid_ns:    [const { None }; MAX_NS],
            uts_ns:    [const { None }; MAX_NS],
            mnt_ns:    [const { None }; MAX_NS],
            net_ns:    [const { None }; MAX_NS],
            ipc_ns:    [const { None }; MAX_NS],
            user_ns:   [const { None }; MAX_NS],
            cgroup_ns: [const { None }; MAX_NS],
            next_id: 2, // 1 = host
        }
    }

    pub fn init_host(&mut self, hostname: &[u8], init_pid: u32) {
        self.pid_ns[0] = Some(PidNs::new(NsId(1), None, init_pid));
        self.uts_ns[0] = Some(UtsNs::new(NsId(1), hostname));
        self.mnt_ns[0] = Some(MountNs::new(NsId(1)));
        self.net_ns[0] = Some(NetNs::new(NsId(1)));
        self.ipc_ns[0] = Some(IpcNs::new(NsId(1)));
        self.user_ns[0] = Some(UserNs::new(NsId(1), None));
        self.cgroup_ns[0] = Some(CgroupNs::new(NsId(1), 1));
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

    pub fn clone_net_ns(&mut self, parent_id: NsId) -> NsId {
        let id = self.alloc_id();
        for slot in &mut self.net_ns {
            if slot.is_none() { *slot = Some(NetNs::new(id)); return id; }
        }
        NsId(0)
    }

    pub fn clone_ipc_ns(&mut self, parent_id: NsId) -> NsId {
        let id = self.alloc_id();
        for slot in &mut self.ipc_ns {
            if slot.is_none() { *slot = Some(IpcNs::new(id)); return id; }
        }
        NsId(0)
    }

    pub fn clone_user_ns(&mut self, parent_id: NsId) -> NsId {
        let id = self.alloc_id();
        for slot in &mut self.user_ns {
            if slot.is_none() { *slot = Some(UserNs::new(id, Some(parent_id))); return id; }
        }
        NsId(0)
    }

    pub fn clone_cgroup_ns(&mut self, parent_id: NsId, root_cgroup: u32) -> NsId {
        let id = self.alloc_id();
        for slot in &mut self.cgroup_ns {
            if slot.is_none() { *slot = Some(CgroupNs::new(id, root_cgroup)); return id; }
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
    pub fn get_net_mut(&mut self, id: NsId) -> Option<&mut NetNs> {
        self.net_ns.iter_mut().flatten().find(|n| n.id == id)
    }
    pub fn get_ipc_mut(&mut self, id: NsId) -> Option<&mut IpcNs> {
        self.ipc_ns.iter_mut().flatten().find(|n| n.id == id)
    }
    pub fn get_user_mut(&mut self, id: NsId) -> Option<&mut UserNs> {
        self.user_ns.iter_mut().flatten().find(|n| n.id == id)
    }
    pub fn get_cgroup_mut(&mut self, id: NsId) -> Option<&mut CgroupNs> {
        self.cgroup_ns.iter_mut().flatten().find(|n| n.id == id)
    }
}

// ── C API for namespace operations (wired to unshare/clone syscalls) ─────────────
static mut NS_MANAGER: Option<NsManager> = None;

#[no_mangle]
pub unsafe extern "C" fn sigma_namespace_init(hostname: *const u8, hostname_len: usize, init_pid: u32) -> i32 {
    let mut manager = NsManager::new();
    let host_name = if !hostname.is_null() && hostname_len > 0 {
        core::slice::from_raw_parts(hostname, hostname_len.min(64))
    } else {
        b"sigmaos"
    };
    manager.init_host(host_name, init_pid);
    NS_MANAGER = Some(manager);
    0
}

#[no_mangle]
pub unsafe extern "C" fn sigma_namespace_create(ns_type: u32, pid: i32) -> u32 {
    if NS_MANAGER.is_none() {
        return 0;
    }
    let manager = NS_MANAGER.as_mut().unwrap();
    
    match ns_type {
        0 => manager.clone_pid_ns(NsId(1), pid as u32).0, // CLONE_NEWPID
        1 => manager.clone_uts_ns(NsId(1)).0,             // CLONE_NEWUTS
        2 => manager.clone_net_ns(NsId(1)).0,            // CLONE_NEWNET
        3 => manager.clone_ipc_ns(NsId(1)).0,            // CLONE_NEWIPC
        4 => manager.clone_user_ns(NsId(1)).0,           // CLONE_NEWUSER
        5 => manager.clone_cgroup_ns(NsId(1), 1).0,      // CLONE_NEWCGROUP
        _ => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_namespace_set_hostname(ns_id: u32, hostname: *const u8, len: usize) -> i32 {
    if NS_MANAGER.is_none() || hostname.is_null() {
        return -1;
    }
    let manager = NS_MANAGER.as_mut().unwrap();
    let id = NsId(ns_id);
    if let Some(uts) = manager.get_uts_mut(id) {
        let name = core::slice::from_raw_parts(hostname, len.min(64));
        uts.set_hostname(name);
        0
    } else {
        -2
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_namespace_add_net_device(ns_id: u32, name: *const u8, name_len: usize, ifindex: u32, mtu: u32) -> i32 {
    if NS_MANAGER.is_none() || name.is_null() {
        return -1;
    }
    let manager = NS_MANAGER.as_mut().unwrap();
    let id = NsId(ns_id);
    if let Some(net) = manager.get_net_mut(id) {
        let dev_name = core::slice::from_raw_parts(name, name_len.min(16));
        if net.add_device(dev_name, ifindex, mtu) { 0 } else { -3 }
    } else {
        -2
    }
}


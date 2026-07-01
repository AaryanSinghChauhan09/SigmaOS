// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: Sovereign Container Runtime (Rust, no_std)
//! Replaces: kernel/core/SovereignContainerDaemonRuntime.cpp
//! =========================================================================

#![no_std]

use core::cell::UnsafeCell;

pub const MAX_CONTAINERS: usize = 32;
pub const MAX_NAMESPACES: usize = 8;

#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum ContainerState {
    Created = 0,
    Running = 1,
    Paused = 2,
    Stopped = 3,
    Failed = 4,
}

#[derive(Copy, Clone)]
pub struct ResourceLimit {
    pub cpu_shares: u64,
    pub memory_bytes: u64,
    pub io_weight: u64,
    pub max_pids: u32,
}

#[derive(Copy, Clone)]
pub struct SovereignContainer {
    pub id: u32,
    pub name: [u8; 48],
    pub image: [u8; 64],
    pub state: ContainerState,
    pub limits: ResourceLimit,
    pub namespaces: [bool; MAX_NAMESPACES],
    pub uptime_ms: u64,
    pub restart_count: u32,
    pub auto_restart: bool,
}

impl SovereignContainer {
    pub const fn empty() -> Self {
        Self {
            id: 0,
            name: [0; 48],
            image: [0; 64],
            state: ContainerState::Stopped,
            limits: ResourceLimit {
                cpu_shares: 0,
                memory_bytes: 0,
                io_weight: 0,
                max_pids: 0,
            },
            namespaces: [false; MAX_NAMESPACES],
            uptime_ms: 0,
            restart_count: 0,
            auto_restart: false,
        }
    }
}

pub struct ContainerRuntime {
    containers: [SovereignContainer; MAX_CONTAINERS],
    container_count: u32,
}

impl ContainerRuntime {
    pub const fn new() -> Self {
        Self {
            containers: [SovereignContainer::empty(); MAX_CONTAINERS],
            container_count: 0,
        }
    }
}

struct SafeContainerRuntime {
    inner: UnsafeCell<ContainerRuntime>,
}

unsafe impl Sync for SafeContainerRuntime {}

static CONTAINER_RUNTIME: SafeContainerRuntime = SafeContainerRuntime {
    inner: UnsafeCell::new(ContainerRuntime::new()),
};

extern "C" {
    fn sigma_log(s: *const u8);
}

#[no_mangle]
pub unsafe extern "C" fn cruntime_init() {
    let r = &mut *CONTAINER_RUNTIME.inner.get();
    r.container_count = 0;
    for i in 0..MAX_CONTAINERS {
        r.containers[i] = SovereignContainer::empty();
    }

    // Load Default Daemons
    cruntime_launch(b"sigma-init\0".as_ptr(), b"sigmaos/init:latest\0".as_ptr(), 512, 64 * 1024 * 1024);
    cruntime_launch(b"sigma-logger\0".as_ptr(), b"sigmaos/logger:latest\0".as_ptr(), 256, 32 * 1024 * 1024);
    cruntime_launch(b"sigma-netd\0".as_ptr(), b"sigmaos/netd:latest\0".as_ptr(), 512, 128 * 1024 * 1024);

    sigma_log(b"[CRUNTIME] Sovereign Container Runtime engine initialized (Rust core).\n\0".as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn cruntime_launch(
    name_ptr: *const u8,
    image_ptr: *const u8,
    cpu: u64,
    mem: u64,
) -> u32 {
    let r = &mut *CONTAINER_RUNTIME.inner.get();
    if r.container_count >= MAX_CONTAINERS as u32 {
        return 0;
    }

    let id = r.container_count + 1;
    let c = &mut r.containers[r.container_count as usize];
    c.id = id;

    let mut i = 0;
    while i < 47 && *name_ptr.add(i) != 0 {
        c.name[i] = *name_ptr.add(i);
        i += 1;
    }
    c.name[i] = 0;

    let mut i = 0;
    while i < 63 && *image_ptr.add(i) != 0 {
        c.image[i] = *image_ptr.add(i);
        i += 1;
    }
    c.image[i] = 0;

    c.state = ContainerState::Running;
    c.limits = ResourceLimit {
        cpu_shares: cpu,
        memory_bytes: mem,
        io_weight: 100,
        max_pids: 64,
    };

    for j in 0..MAX_NAMESPACES {
        c.namespaces[j] = true;
    }
    c.uptime_ms = 0;
    c.restart_count = 0;
    c.auto_restart = true;

    r.container_count += 1;
    id
}

#[no_mangle]
pub unsafe extern "C" fn cruntime_stop(id: u32) -> bool {
    let r = &mut *CONTAINER_RUNTIME.inner.get();
    if id == 0 || id > r.container_count {
        return false;
    }
    r.containers[(id - 1) as usize].state = ContainerState::Stopped;
    true
}

#[no_mangle]
pub unsafe extern "C" fn cruntime_status() {
    // Audit reporting left to shell / userspace query
}

// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: Sovereign Namespace & Cgroups (Rust, no_std)
//! Replaces: kernel/core/sigma_namespace.c
//! =========================================================================

#![no_std]

use core::cell::UnsafeCell;

pub const SIGMA_CTR_NAME_LEN: usize = 32;
pub const SIGMA_CTR_MAX: usize = 32;

pub const SIGMA_CTR_ISO_PID: u32 = 0x01;
pub const SIGMA_CTR_ISO_NET: u32 = 0x02;
pub const SIGMA_CTR_ISO_MNT: u32 = 0x04;

pub const SIGMA_CTR_DEAD: u32 = 4;
pub const SIGMA_CTR_CREATED: u32 = 1;
pub const SIGMA_CTR_RUNNING: u32 = 2;
pub const SIGMA_CTR_STOPPED: u32 = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SigmaNamespace {
    pub pid_ns_id: u32,
    pub net_ns_id: u32,
    pub mnt_ns_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SigmaContainer {
    pub id: u32,
    pub name: [u8; SIGMA_CTR_NAME_LEN],
    pub state: u32,
    pub mem_limit_mb: u32,
    pub cpu_shares: u32,
    pub ns: SigmaNamespace,
}

impl SigmaContainer {
    pub const fn empty() -> Self {
        Self {
            id: 0,
            name: [0; SIGMA_CTR_NAME_LEN],
            state: 0,
            mem_limit_mb: 0,
            cpu_shares: 0,
            ns: SigmaNamespace {
                pid_ns_id: 0,
                net_ns_id: 0,
                mnt_ns_id: 0,
            },
        }
    }
}

#[repr(C)]
pub struct SigmaContainerRegistry {
    pub containers: [SigmaContainer; SIGMA_CTR_MAX],
}

struct SafeContainerRegistry {
    inner: UnsafeCell<SigmaContainerRegistry>,
    next_ns_id: UnsafeCell<u32>,
}

unsafe impl Sync for SafeContainerRegistry {}

static REGISTRY: SafeContainerRegistry = SafeContainerRegistry {
    inner: UnsafeCell::new(SigmaContainerRegistry {
        containers: [SigmaContainer::empty(); SIGMA_CTR_MAX],
    }),
    next_ns_id: UnsafeCell::new(1),
};

extern "C" {
    fn sigma_log(s: *const u8);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_sys_clone(
    _entry_point: extern "C" fn(*mut u8),
    _arg: *mut u8,
    iso_flags: u32,
) -> u32 {
    let next_id = &mut *REGISTRY.next_ns_id.get();
    let mut pid_id = 0;
    let mut net_id = 0;
    let mut mnt_id = 0;

    if (iso_flags & SIGMA_CTR_ISO_PID) != 0 {
        pid_id = *next_id;
        *next_id += 1;
    }
    if (iso_flags & SIGMA_CTR_ISO_NET) != 0 {
        net_id = *next_id;
        *next_id += 1;
    }
    if (iso_flags & SIGMA_CTR_ISO_MNT) != 0 {
        mnt_id = *next_id;
        *next_id += 1;
    }

    let _ns = SigmaNamespace {
        pid_ns_id: pid_id,
        net_ns_id: net_id,
        mnt_ns_id: mnt_id,
    };

    9999 // Mock cloned process PID
}

#[no_mangle]
pub unsafe extern "C" fn sys_container_create(
    name_ptr: *const u8,
    _iso_flags: u32,
    cpu_shares: u32,
    mem_limit_mb: u32,
) -> u32 {
    let reg = &mut *REGISTRY.inner.get();
    for i in 0..SIGMA_CTR_MAX {
        let c = &mut reg.containers[i];
        if c.state == SIGMA_CTR_DEAD || c.state == 0 {
            c.id = (i + 1) as u32;
            c.state = SIGMA_CTR_CREATED;
            c.cpu_shares = cpu_shares;
            c.mem_limit_mb = mem_limit_mb;

            let mut j = 0;
            while j < SIGMA_CTR_NAME_LEN - 1 && *name_ptr.add(j) != 0 {
                c.name[j] = *name_ptr.add(j);
                j += 1;
            }
            c.name[j] = 0;

            return c.id;
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn sys_container_start(id: u32) {
    let reg = &mut *REGISTRY.inner.get();
    if id > 0 && id <= SIGMA_CTR_MAX as u32 {
        reg.containers[(id - 1) as usize].state = SIGMA_CTR_RUNNING;
    }
}

#[no_mangle]
pub unsafe extern "C" fn sys_container_stop(id: u32) {
    let reg = &mut *REGISTRY.inner.get();
    if id > 0 && id <= SIGMA_CTR_MAX as u32 {
        reg.containers[(id - 1) as usize].state = SIGMA_CTR_STOPPED;
    }
}

#[no_mangle]
pub unsafe extern "C" fn sys_container_destroy(id: u32) {
    let reg = &mut *REGISTRY.inner.get();
    if id > 0 && id <= SIGMA_CTR_MAX as u32 {
        reg.containers[(id - 1) as usize].state = SIGMA_CTR_DEAD;
    }
}

#[no_mangle]
pub unsafe extern "C" fn sys_container_get_registry() -> *const SigmaContainerRegistry {
    REGISTRY.inner.get()
}

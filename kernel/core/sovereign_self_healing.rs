// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: Sovereign Self Healing Kernel Engine (Rust, no_std)
//! Replaces: kernel/core/SovereignSelfHealingKernel.cpp
//! =========================================================================

#![no_std]

use core::cell::UnsafeCell;

pub const MAX_WATCHERS: usize = 32;
pub const MAX_PATCHES: usize = 16;

#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum FaultSeverity {
    None = 0,
    Warn = 1,
    Recover = 2,
    Critical = 3,
}

pub type HealthProbe = Option<unsafe extern "C" fn(ctx: *mut u8) -> bool>;
pub type HealAction = Option<unsafe extern "C" fn(ctx: *mut u8)>;

#[derive(Copy, Clone)]
pub struct SubsystemWatcher {
    pub name: [u8; 48],
    pub probe: HealthProbe,
    pub heal: HealAction,
    pub ctx: *mut u8,
    pub threshold: FaultSeverity,
    pub fault_count: u32,
    pub heal_count: u32,
    pub isolated: bool,
}

impl SubsystemWatcher {
    pub const fn empty() -> Self {
        Self {
            name: [0; 48],
            probe: None,
            heal: None,
            ctx: core::ptr::null_mut(),
            threshold: FaultSeverity::None,
            fault_count: 0,
            heal_count: 0,
            isolated: false,
        }
    }
}

#[derive(Copy, Clone)]
pub struct LivePatch {
    pub patch_id: u32,
    pub description: *const u8,
    pub applied: bool,
}

impl LivePatch {
    pub const fn empty() -> Self {
        Self {
            patch_id: 0,
            description: core::ptr::null(),
            applied: false,
        }
    }
}

pub struct SelfHealingKernel {
    watchers: [SubsystemWatcher; MAX_WATCHERS],
    patches: [LivePatch; MAX_PATCHES],
    watcher_count: u32,
    patch_count: u32,
    total_heals: u32,
}

impl SelfHealingKernel {
    pub const fn new() -> Self {
        Self {
            watchers: [SubsystemWatcher::empty(); MAX_WATCHERS],
            patches: [LivePatch::empty(); MAX_PATCHES],
            watcher_count: 0,
            patch_count: 0,
            total_heals: 0,
        }
    }
}

struct SafeSelfHealingKernel {
    inner: UnsafeCell<SelfHealingKernel>,
}

unsafe impl Sync for SafeSelfHealingKernel {}

static SELF_HEAL_KERNEL: SafeSelfHealingKernel = SafeSelfHealingKernel {
    inner: UnsafeCell::new(SelfHealingKernel::new()),
};

extern "C" {
    fn sigma_log(s: *const u8);
}

#[no_mangle]
pub unsafe extern "C" fn selfheal_init() {
    let s = &mut *SELF_HEAL_KERNEL.inner.get();
    s.watcher_count = 0;
    s.patch_count = 0;
    s.total_heals = 0;
    for i in 0..MAX_WATCHERS {
        s.watchers[i] = SubsystemWatcher::empty();
    }
    for i in 0..MAX_PATCHES {
        s.patches[i] = LivePatch::empty();
    }
    sigma_log(b"[SELFHEAL] Sovereign Self-Healing Kernel engine initialised (Rust core).\n\0".as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn selfheal_register_watcher(
    name_ptr: *const u8,
    probe: HealthProbe,
    heal: HealAction,
    ctx: *mut u8,
    threshold: FaultSeverity,
) -> bool {
    let s = &mut *SELF_HEAL_KERNEL.inner.get();
    if s.watcher_count >= MAX_WATCHERS as u32 {
        return false;
    }

    let w = &mut s.watchers[s.watcher_count as usize];
    let mut i = 0;
    while i < 47 && *name_ptr.add(i) != 0 {
        w.name[i] = *name_ptr.add(i);
        i += 1;
    }
    w.name[i] = 0;

    w.probe = probe;
    w.heal = heal;
    w.ctx = ctx;
    w.threshold = threshold;
    w.fault_count = 0;
    w.heal_count = 0;
    w.isolated = false;

    s.watcher_count += 1;
    true
}

#[no_mangle]
pub unsafe extern "C" fn selfheal_register_patch(description: *const u8) -> u32 {
    let s = &mut *SELF_HEAL_KERNEL.inner.get();
    if s.patch_count >= MAX_PATCHES as u32 {
        return 0;
    }

    let p = &mut s.patches[s.patch_count as usize];
    p.patch_id = s.patch_count + 1;
    p.description = description;
    p.applied = false;

    s.patch_count += 1;
    p.patch_id
}

#[no_mangle]
pub unsafe extern "C" fn selfheal_apply_patch(patch_id: u32) -> bool {
    let s = &mut *SELF_HEAL_KERNEL.inner.get();
    for i in 0..(s.patch_count as usize) {
        if s.patches[i].patch_id == patch_id && !s.patches[i].applied {
            s.patches[i].applied = true;
            return true;
        }
    }
    false
}

#[no_mangle]
pub unsafe extern "C" fn selfheal_run_cycle() -> u32 {
    let s = &mut *SELF_HEAL_KERNEL.inner.get();
    let mut faults = 0;

    for i in 0..(s.watcher_count as usize) {
        let w = &mut s.watchers[i];
        if w.isolated {
            continue;
        }

        let healthy = match w.probe {
            Some(func) => func(w.ctx),
            None => true,
        };

        if !healthy {
            w.fault_count += 1;
            faults += 1;

            match w.threshold {
                FaultSeverity::Warn => {}
                FaultSeverity::Recover => {
                    if let Some(func) = w.heal {
                        func(w.ctx);
                        w.heal_count += 1;
                        s.total_heals += 1;
                    }
                }
                FaultSeverity::Critical => {
                    w.isolated = true;
                }
                _ => {}
            }
        }
    }

    faults
}

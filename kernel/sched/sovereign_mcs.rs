// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: Sovereign MCS Lock (Rust, no_std)
//! Replaces: kernel/sched/sigma_mcs.cpp and kernel/sched/sigma_mcs.h
//! =========================================================================

#![no_std]

use core::sync::atomic::{AtomicBool, AtomicPtr, Ordering};

pub struct McsNode {
    pub next: AtomicPtr<McsNode>,
    pub locked: AtomicBool,
}

impl McsNode {
    pub const fn new() -> Self {
        Self {
            next: AtomicPtr::new(core::ptr::null_mut()),
            locked: AtomicBool::new(false),
        }
    }
}

pub struct McsLock {
    tail: AtomicPtr<McsNode>,
}

impl McsLock {
    pub const fn new() -> Self {
        Self {
            tail: AtomicPtr::new(core::ptr::null_mut()),
        }
    }

    pub fn acquire(&self, node: &McsNode) {
        let node_ptr = node as *const McsNode as *mut McsNode;
        node.next.store(core::ptr::null_mut(), Ordering::Relaxed);
        node.locked.store(true, Ordering::Relaxed);

        let prev = self.tail.swap(node_ptr, Ordering::Acquire);
        if !prev.is_null() {
            unsafe {
                (*prev).next.store(node_ptr, Ordering::Release);
            }
            while node.locked.load(Ordering::Acquire) {
                core::hint::spin_loop();
            }
        }
    }

    pub fn release(&self, node: &McsNode) {
        let node_ptr = node as *const McsNode as *mut McsNode;
        if node.next.load(Ordering::Relaxed).is_null() {
            if self.tail.compare_exchange(
                node_ptr,
                core::ptr::null_mut(),
                Ordering::Release,
                Ordering::Relaxed,
            ).is_ok() {
                return;
            }
            while node.next.load(Ordering::Relaxed).is_null() {
                core::hint::spin_loop();
            }
        }

        let next = node.next.load(Ordering::Relaxed);
        unsafe {
            (*next).locked.store(false, Ordering::Release);
        }
    }
}

static MCS_LOCK: McsLock = McsLock::new();

#[no_mangle]
pub unsafe extern "C" fn sigma_mcs_lock_acquire(node: *mut McsNode) {
    if !node.is_null() {
        MCS_LOCK.acquire(&*node);
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_mcs_lock_release(node: *mut McsNode) {
    if !node.is_null() {
        MCS_LOCK.release(&*node);
    }
}

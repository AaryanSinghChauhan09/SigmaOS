// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: Sovereign IPC Subsystem (Rust, no_std)
//! Replaces: kernel/core/SovereignIPC.cpp and kernel/core/sigma_ipc.cpp
//! =========================================================================

#![no_std]

use core::cell::UnsafeCell;

pub const IPC_MAX_QUEUES: usize = 32;
pub const IPC_QUEUE_CAPACITY: usize = 64;
pub const IPC_MAX_SHM_SEGMENTS: usize = 64;
pub const IPC_SHM_NAME_LEN: usize = 32;
pub const SIGMA_IPC_MSG_SIZE: usize = 256;
pub const PAGE_SIZE: u64 = 4096;

pub const K_OK: i32 = 0;
pub const K_ERR_NOTFOUND: i32 = -1;
pub const K_ERR_BUSY: i32 = -2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SigmaMessage {
    pub sender_pid: u32,
    pub receiver_pid: u32,
    pub msg_type: u32,
    pub payload_len: u32,
    pub payload: [u8; SIGMA_IPC_MSG_SIZE],
}

impl SigmaMessage {
    pub const fn empty() -> Self {
        Self {
            sender_pid: 0,
            receiver_pid: 0,
            msg_type: 0,
            payload_len: 0,
            payload: [0; SIGMA_IPC_MSG_SIZE],
        }
    }
}

#[repr(C)]
pub struct SigmaIpcQueue {
    pub messages: [SigmaMessage; IPC_QUEUE_CAPACITY],
    pub head: u32,
    pub tail: u32,
    pub count: u32,
    pub owner_pid: u32,
    pub active: bool,
}

impl SigmaIpcQueue {
    pub const fn empty() -> Self {
        Self {
            messages: [SigmaMessage::empty(); IPC_QUEUE_CAPACITY],
            head: 0,
            tail: 0,
            count: 0,
            owner_pid: 0,
            active: false,
        }
    }
}

#[repr(C)]
pub struct SigmaShm {
    pub shm_id: u32,
    pub name: [u8; IPC_SHM_NAME_LEN],
    pub size: u64,
    pub phys_base: u64,
    pub ref_count: u32,
    pub owner_pid: u32,
    pub writable: bool,
}

impl SigmaShm {
    pub const fn empty() -> Self {
        Self {
            shm_id: 0,
            name: [0; IPC_SHM_NAME_LEN],
            size: 0,
            phys_base: 0,
            ref_count: 0,
            owner_pid: 0,
            writable: false,
        }
    }
}

pub trait MessagingSystem {
    fn create_queue(&mut self, owner_pid: u32) -> u32;
    fn send(&mut self, queue_id: u32, msg: &SigmaMessage) -> i32;
    fn receive(&mut self, queue_id: u32) -> Option<SigmaMessage>;
}

pub struct IpcManager {
    queues: [SigmaIpcQueue; IPC_MAX_QUEUES],
    shm: [SigmaShm; IPC_MAX_SHM_SEGMENTS],
    queue_count: u32,
    shm_count: u32,
    total_messages: u64,
}

impl IpcManager {
    pub const fn new() -> Self {
        Self {
            queues: [SigmaIpcQueue::empty(); IPC_MAX_QUEUES],
            shm: [SigmaShm::empty(); IPC_MAX_SHM_SEGMENTS],
            queue_count: 0,
            shm_count: 0,
            total_messages: 0,
        }
    }

    fn find_queue_mut(&mut self, id: u32) -> Option<&mut SigmaIpcQueue> {
        if id == 0 || id > IPC_MAX_QUEUES as u32 {
            None
        } else {
            let q = &mut self.queues[(id - 1) as usize];
            if q.active { Some(q) } else { None }
        }
    }

    fn find_shm_mut(&mut self, id: u32) -> Option<&mut SigmaShm> {
        if id == 0 || id > IPC_MAX_SHM_SEGMENTS as u32 {
            None
        } else {
            let s = &mut self.shm[(id - 1) as usize];
            if s.shm_id == id { Some(s) } else { None }
        }
    }
}

struct SafeIpcManager {
    inner: UnsafeCell<IpcManager>,
}

unsafe impl Sync for SafeIpcManager {}

static IPC_MANAGER: SafeIpcManager = SafeIpcManager {
    inner: UnsafeCell::new(IpcManager::new()),
};

extern "C" {
    fn sigma_log(s: *const u8);
    fn sigma_log_info(fmt: *const u8, val1: u32, val2: *const u8, val3: u32, val4: u32);
}

#[no_mangle]
pub unsafe extern "C" fn ipc_init() {
    let im = &mut *IPC_MANAGER.inner.get();
    im.queue_count = 0;
    im.shm_count = 0;
    im.total_messages = 0;
    for i in 0..IPC_MAX_QUEUES {
        im.queues[i] = SigmaIpcQueue::empty();
    }
    for i in 0..IPC_MAX_SHM_SEGMENTS {
        im.shm[i] = SigmaShm::empty();
    }
    sigma_log(b"[IPC] Sovereign IPC Subsystem initialized (Rust core).\n\0".as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn ipc_create_queue(owner_pid: u32) -> u32 {
    let im = &mut *IPC_MANAGER.inner.get();
    for i in 0..IPC_MAX_QUEUES {
        if !im.queues[i].active {
            let id = (i + 1) as u32;
            im.queues[i].active = true;
            im.queues[i].owner_pid = owner_pid;
            im.queues[i].head = 0;
            im.queues[i].tail = 0;
            im.queues[i].count = 0;
            im.queue_count += 1;
            return id;
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn ipc_destroy_queue(queue_id: u32) -> i32 {
    let im = &mut *IPC_MANAGER.inner.get();
    if let Some(q) = im.find_queue_mut(queue_id) {
        q.active = false;
        q.owner_pid = 0;
        q.count = 0;
        K_OK
    } else {
        K_ERR_NOTFOUND
    }
}

#[no_mangle]
pub unsafe extern "C" fn ipc_send(queue_id: u32, msg: *const SigmaMessage) -> i32 {
    let im = &mut *IPC_MANAGER.inner.get();
    if let Some(q) = im.find_queue_mut(queue_id) {
        if q.count >= IPC_QUEUE_CAPACITY as u32 {
            return K_ERR_BUSY;
        }
        q.messages[q.tail as usize] = *msg;
        q.tail = (q.tail + 1) % IPC_QUEUE_CAPACITY as u32;
        q.count += 1;
        im.total_messages += 1;
        K_OK
    } else {
        K_ERR_NOTFOUND
    }
}

#[no_mangle]
pub unsafe extern "C" fn ipc_receive(queue_id: u32, out_msg: *mut SigmaMessage) -> i32 {
    let im = &mut *IPC_MANAGER.inner.get();
    if let Some(q) = im.find_queue_mut(queue_id) {
        if q.count == 0 {
            return K_ERR_NOTFOUND;
        }
        *out_msg = q.messages[q.head as usize];
        q.head = (q.head + 1) % IPC_QUEUE_CAPACITY as u32;
        q.count -= 1;
        K_OK
    } else {
        K_ERR_NOTFOUND
    }
}

#[no_mangle]
pub unsafe extern "C" fn ipc_queue_count(queue_id: u32) -> u32 {
    let im = &*IPC_MANAGER.inner.get();
    if queue_id == 0 || queue_id > IPC_MAX_QUEUES as u32 {
        0
    } else {
        let q = &im.queues[(queue_id - 1) as usize];
        if q.active { q.count } else { 0 }
    }
}

#[no_mangle]
pub unsafe extern "C" fn shm_create(name_ptr: *const u8, size: usize, owner_pid: u32) -> u32 {
    let im = &mut *IPC_MANAGER.inner.get();
    if im.shm_count >= IPC_MAX_SHM_SEGMENTS as u32 {
        return 0;
    }

    let id = im.shm_count + 1;
    let s = &mut im.shm[im.shm_count as usize];
    s.shm_id = id;
    
    // Copy SHM name safely
    let mut i = 0;
    while i < IPC_SHM_NAME_LEN - 1 && *name_ptr.add(i) != 0 {
        s.name[i] = *name_ptr.add(i);
        i += 1;
    }
    while i < IPC_SHM_NAME_LEN {
        s.name[i] = 0;
        i += 1;
    }

    // Align size to page boundary
    let aligned_size = (size as u64 + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
    s.size = aligned_size;
    s.phys_base = 0x40000000ULL + (im.shm_count as u64) * aligned_size;
    s.ref_count = 1;
    s.owner_pid = owner_pid;
    s.writable = true;

    im.shm_count += 1;
    id
}

#[no_mangle]
pub unsafe extern "C" fn shm_attach(shm_id: u32, _pid: u32) -> i32 {
    let im = &mut *IPC_MANAGER.inner.get();
    if let Some(s) = im.find_shm_mut(shm_id) {
        s.ref_count += 1;
        K_OK
    } else {
        K_ERR_NOTFOUND
    }
}

#[no_mangle]
pub unsafe extern "C" fn shm_detach(shm_id: u32, _pid: u32) -> i32 {
    let im = &mut *IPC_MANAGER.inner.get();
    if let Some(s) = im.find_shm_mut(shm_id) {
        if s.ref_count > 0 {
            s.ref_count -= 1;
            K_OK
        } else {
            K_ERR_NOTFOUND
        }
    } else {
        K_ERR_NOTFOUND
    }
}

#[no_mangle]
pub unsafe extern "C" fn shm_destroy(shm_id: u32) -> i32 {
    let im = &mut *IPC_MANAGER.inner.get();
    if let Some(s) = im.find_shm_mut(shm_id) {
        s.shm_id = 0;
        s.ref_count = 0;
        K_OK
    } else {
        K_ERR_NOTFOUND
    }
}

#[no_mangle]
pub unsafe extern "C" fn signal_send(_target_pid: u32, _sig: u32) -> i32 {
    // Stubs for signal delivery mechanism
    K_OK
}

// ── C API for older sigma_ipc.cpp callers ──

#[no_mangle]
pub unsafe extern "C" fn sigma_ipc_create_queue(pid: u32) -> i32 {
    let qid = ipc_create_queue(pid);
    if qid == 0 { -1 } else { (qid - 1) as i32 }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_ipc_send(dest_pid: u32, msg_type: u32, payload: *const u8, len: u32) -> i32 {
    let im = &mut *IPC_MANAGER.inner.get();
    // Find queue matching owner PID
    for i in 0..IPC_MAX_QUEUES {
        let q = &mut im.queues[i];
        if q.active && q.owner_pid == dest_pid {
            let next_tail = (q.tail + 1) % IPC_QUEUE_CAPACITY as u32;
            if next_tail == q.head {
                return -1; // Full
            }
            let msg = &mut q.messages[q.tail as usize];
            msg.receiver_pid = dest_pid;
            msg.msg_type = msg_type;
            let real_len = if len < SIGMA_IPC_MSG_SIZE as u32 { len } else { SIGMA_IPC_MSG_SIZE as u32 };
            msg.payload_len = real_len;
            core::ptr::copy_nonoverlapping(payload, msg.payload.as_mut_ptr(), real_len as usize);
            q.tail = next_tail;
            q.count += 1;
            im.total_messages += 1;
            return 0;
        }
    }
    -2
}

#[no_mangle]
pub unsafe extern "C" fn sigma_ipc_recv(my_pid: u32, out: *mut SigmaMessage) -> i32 {
    let im = &mut *IPC_MANAGER.inner.get();
    for i in 0..IPC_MAX_QUEUES {
        let q = &mut im.queues[i];
        if q.active && q.owner_pid == my_pid {
            if q.head == q.tail {
                return -1; // Empty
            }
            *out = q.messages[q.head as usize];
            q.head = (q.head + 1) % IPC_QUEUE_CAPACITY as u32;
            q.count -= 1;
            return 0;
        }
    }
    -2
}

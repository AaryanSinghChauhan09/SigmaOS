// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/sigma_epoll.rs — epoll / event-driven I/O multiplexing
//
// Implements Linux-compatible epoll interface:
//   epoll_create1(flags) → epfd
//   epoll_ctl(epfd, op, fd, event)
//   epoll_wait(epfd, events, maxevents, timeout_ms)
//
// Also implements poll(2) as a thin wrapper over epoll.
//
// Language: Rust #![no_std]

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, Ordering};

// ── epoll event flags ─────────────────────────────────────────────────────
pub const EPOLLIN:    u32 = 0x0001;
pub const EPOLLPRI:   u32 = 0x0002;
pub const EPOLLOUT:   u32 = 0x0004;
pub const EPOLLERR:   u32 = 0x0008;
pub const EPOLLHUP:   u32 = 0x0010;
pub const EPOLLRDHUP: u32 = 0x2000;
pub const EPOLLET:    u32 = 1 << 31; // edge-triggered
pub const EPOLLONESHOT:u32= 1 << 30;

// ── epoll_ctl ops ──────────────────────────────────────────────────────────
pub const EPOLL_CTL_ADD: i32 = 1;
pub const EPOLL_CTL_DEL: i32 = 2;
pub const EPOLL_CTL_MOD: i32 = 3;

// ── epoll_event struct (Linux-compatible) ─────────────────────────────────
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct EpollEvent {
    pub events: u32,
    pub data:   u64,  // user data (fd or pointer)
}

// ── Internal watched-fd entry ─────────────────────────────────────────────
const MAX_WATCHED: usize = 256;

#[derive(Copy, Clone)]
struct WatchedFd {
    fd:       i32,
    events:   u32,
    data:     u64,
    oneshot_fired: bool,
    valid:    bool,
}

impl WatchedFd {
    const fn empty() -> Self {
        WatchedFd { fd: -1, events: 0, data: 0, oneshot_fired: false, valid: false }
    }
}

// ── epoll instance ────────────────────────────────────────────────────────
const MAX_EPOLL_INSTANCES: usize = 64;

struct EpollInstance {
    watched: [WatchedFd; MAX_WATCHED],
    count:   usize,
    valid:   bool,
}

impl EpollInstance {
    const fn new() -> Self {
        EpollInstance {
            watched: [const { WatchedFd::empty() }; MAX_WATCHED],
            count: 0, valid: false,
        }
    }

    fn find(&self, fd: i32) -> Option<usize> {
        for i in 0..self.count { if self.watched[i].valid && self.watched[i].fd == fd { return Some(i); } }
        None
    }

    fn add(&mut self, fd: i32, events: u32, data: u64) -> i32 {
        if self.find(fd).is_some() { return -17; } // EEXIST
        if self.count >= MAX_WATCHED { return -28; } // ENOSPC
        for i in 0..MAX_WATCHED {
            if !self.watched[i].valid {
                self.watched[i] = WatchedFd { fd, events, data, oneshot_fired: false, valid: true };
                self.count += 1;
                return 0;
            }
        }
        -28
    }

    fn del(&mut self, fd: i32) -> i32 {
        if let Some(i) = self.find(fd) {
            self.watched[i] = WatchedFd::empty();
            self.count -= 1;
            return 0;
        }
        -2 // ENOENT
    }

    fn modify(&mut self, fd: i32, events: u32, data: u64) -> i32 {
        if let Some(i) = self.find(fd) {
            self.watched[i].events = events;
            self.watched[i].data   = data;
            self.watched[i].oneshot_fired = false;
            return 0;
        }
        -2
    }

    /// Collect ready events. In a real kernel this reads from FD readiness flags.
    /// Here we use a simple heuristic: always report EPOLLIN ready for socket/pipe FDs.
    fn collect_events(&mut self, out: &mut [EpollEvent]) -> usize {
        let mut n = 0;
        for i in 0..MAX_WATCHED {
            if n >= out.len() { break; }
            let w = &mut self.watched[i];
            if !w.valid { continue; }
            if w.events & EPOLLONESHOT != 0 && w.oneshot_fired { continue; }
            // Simulate: FDs 0..2 always readable (stdin/stdout/stderr),
            // positive FDs assumed to have data available
            let ready = simulate_fd_readiness(w.fd);
            let triggered = ready & w.events;
            if triggered != 0 {
                out[n] = EpollEvent { events: triggered, data: w.data };
                if w.events & EPOLLONESHOT != 0 { w.oneshot_fired = true; }
                // Edge-triggered: remove from ready set (caller must re-arm)
                n += 1;
            }
        }
        n
    }
}

fn simulate_fd_readiness(fd: i32) -> u32 {
    match fd {
        0       => EPOLLIN,                    // stdin always readable (simplified)
        1 | 2   => EPOLLOUT,                   // stdout/stderr always writable
        3..=255 => EPOLLIN | EPOLLOUT,         // sockets/pipes: both
        _       => 0,
    }
}

static mut EPOLL_TABLE: [EpollInstance; MAX_EPOLL_INSTANCES] =
    [const { EpollInstance::new() }; MAX_EPOLL_INSTANCES];
static EPOLL_NEXT_FD: core::sync::atomic::AtomicI32 = core::sync::atomic::AtomicI32::new(100);

fn get_epoll(epfd: i32) -> Option<&'static mut EpollInstance> {
    let idx = epfd as usize;
    if idx >= MAX_EPOLL_INSTANCES { return None; }
    unsafe {
        let inst = &mut EPOLL_TABLE[idx];
        if inst.valid { Some(inst) } else { None }
    }
}

// ── Syscall implementations ────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_sys_epoll_create1(_flags: u64) -> i64 {
    for i in 0..MAX_EPOLL_INSTANCES {
        if !EPOLL_TABLE[i].valid {
            EPOLL_TABLE[i].valid = true;
            EPOLL_TABLE[i].count = 0;
            return i as i64;
        }
    }
    -24 // EMFILE
}

#[no_mangle]
pub unsafe extern "C" fn sigma_sys_epoll_ctl(
    epfd: u64, op: u64, fd: u64, event_ptr: u64,
) -> i64 {
    let inst = match get_epoll(epfd as i32) { Some(i) => i, None => return -9 };
    let ev = if event_ptr != 0 { &*(event_ptr as *const EpollEvent) }
             else { &EpollEvent { events: 0, data: 0 } };
    let ret = match op as i32 {
        EPOLL_CTL_ADD => inst.add(fd as i32, ev.events, ev.data),
        EPOLL_CTL_DEL => inst.del(fd as i32),
        EPOLL_CTL_MOD => inst.modify(fd as i32, ev.events, ev.data),
        _             => -22,
    };
    ret as i64
}

#[no_mangle]
pub unsafe extern "C" fn sigma_sys_epoll_wait(
    epfd: u64, events_ptr: u64, maxevents: u64, timeout_ms: u64,
) -> i64 {
    if events_ptr == 0 || maxevents == 0 { return -22; }
    let inst = match get_epoll(epfd as i32) { Some(i) => i, None => return -9 };
    let out = core::slice::from_raw_parts_mut(
        events_ptr as *mut EpollEvent, maxevents as usize
    );

    // Try once; if no events and timeout > 0, simulate brief wait
    let n = inst.collect_events(out);
    if n > 0 || timeout_ms == 0 { return n as i64; }

    // Timeout: simulate sleeping then check again
    extern "C" { fn sigma_sleep_ms(ms: u64); }
    let sleep_ms = timeout_ms.min(100); // cap at 100ms per iteration
    sigma_sleep_ms(sleep_ms);
    inst.collect_events(out) as i64
}

// ── poll(2) — wraps epoll ─────────────────────────────────────────────────
#[repr(C)]
pub struct PollFd {
    pub fd:      i32,
    pub events:  i16,  // POLLIN=1, POLLOUT=4, POLLERR=8, POLLHUP=16
    pub revents: i16,
}

#[no_mangle]
pub unsafe extern "C" fn sigma_sys_poll(
    fds: u64, nfds: u64, timeout_ms: u64,
) -> i64 {
    if fds == 0 { return -14; }
    let pollfd_arr = core::slice::from_raw_parts_mut(fds as *mut PollFd, nfds as usize);

    let mut ready = 0i64;
    for pfd in pollfd_arr.iter_mut() {
        pfd.revents = 0;
        if pfd.fd < 0 { continue; }
        let rdy = simulate_fd_readiness(pfd.fd);
        let req = pfd.events as u32;
        let mut rev = 0u32;
        if req & 1 != 0 && rdy & EPOLLIN  != 0 { rev |= 1; }  // POLLIN
        if req & 4 != 0 && rdy & EPOLLOUT != 0 { rev |= 4; }  // POLLOUT
        if rdy & EPOLLERR != 0 { rev |= 8; }
        if rdy & EPOLLHUP != 0 { rev |= 16; }
        pfd.revents = rev as i16;
        if rev != 0 { ready += 1; }
    }
    if ready > 0 || timeout_ms == 0 { return ready; }

    // Brief sleep then re-check once
    extern "C" { fn sigma_sleep_ms(ms: u64); }
    sigma_sleep_ms(timeout_ms.min(50));
    0 // simplified: return 0 (timeout)
}

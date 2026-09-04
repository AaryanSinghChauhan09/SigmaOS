use std::format;
use std::string::{String, ToString};
use std::vec::Vec;
// SigmaOS — kqueue Event-Notification Subsystem
//
// Inspired by BSD kqueue(2) — a unified kernel event notification interface.
// kqueue unifies monitoring of file descriptors, vnodes, processes, signals,
// and timers under a single system call, eliminating the scalability
// problems of poll(2)/select(2).
//
// References:
//   Jonathan Lemon, "Kqueue: A generic and scalable event notification
//   facility," USENIX ATC 2001.
//   OpenBSD/FreeBSD kqueue(2) man pages.
//
// This implementation is purely custom — no std, no libc, no external crates.

// ─────────────────────────────────────────────────────────────────────────────
// Filter types (EVFILT_*)
// ─────────────────────────────────────────────────────────────────────────────

/// Which kind of event a kevent monitors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KFilter {
    /// File descriptor becomes readable (EVFILT_READ).
    Read,
    /// File descriptor becomes writable (EVFILT_WRITE).
    Write,
    /// Vnode / file changes (EVFILT_VNODE).
    Vnode,
    /// Process lifecycle events — fork, exec, exit (EVFILT_PROC).
    Proc,
    /// Signal delivery (EVFILT_SIGNAL).
    Signal,
    /// Periodic or one-shot timer (EVFILT_TIMER).
    Timer,
    /// User-space triggered event (EVFILT_USER, FreeBSD 8+).
    User,
}

// ─────────────────────────────────────────────────────────────────────────────
// Flags (EV_*)
// ─────────────────────────────────────────────────────────────────────────────

/// Flags that control kevent lifecycle.
pub mod flags {
    /// Add the event to the kqueue (create if absent).
    pub const ADD: u16 = 0x0001;
    /// Remove the event from the kqueue.
    pub const DELETE: u16 = 0x0002;
    /// Enable the event (default after ADD).
    pub const ENABLE: u16 = 0x0004;
    /// Disable the event without removing it.
    pub const DISABLE: u16 = 0x0008;
    /// Auto-remove after first trigger (one-shot).
    pub const ONESHOT: u16 = 0x0010;
    /// Edge-triggered: clear the event after return.
    pub const CLEAR: u16 = 0x0020;
    /// Event error — returned in `flags` when an error occurs.
    pub const ERROR: u16 = 0x4000;
    /// Event was triggered — set when returning from kevent().
    pub const EOF: u16 = 0x8000;
}

// ─────────────────────────────────────────────────────────────────────────────
// Vnode filter flags (NOTE_*)
// ─────────────────────────────────────────────────────────────────────────────

pub mod vnote {
    /// File was deleted.
    pub const DELETE: u32 = 0x0001;
    /// File was written to.
    pub const WRITE: u32 = 0x0002;
    /// File was extended.
    pub const EXTEND: u32 = 0x0004;
    /// File attributes changed.
    pub const ATTRIB: u32 = 0x0008;
    /// File was renamed.
    pub const RENAME: u32 = 0x0020;
}

// ─────────────────────────────────────────────────────────────────────────────
// kevent structure (mirrors struct kevent in C)
// ─────────────────────────────────────────────────────────────────────────────

/// A single kqueue event registration or returned event.
#[derive(Debug, Clone)]
pub struct KEvent {
    /// Identifier: fd, pid, signal number, timer id, or user id.
    pub ident: u64,
    /// Filter type (see `KFilter`).
    pub filter: KFilter,
    /// Control flags (see `flags` module).
    pub flags: u16,
    /// Filter-specific flags (e.g. `vnote::WRITE`).
    pub fflags: u32,
    /// Filter-specific data (e.g. bytes available to read).
    pub data: i64,
    /// Opaque user-defined value, returned unchanged in ready events.
    pub udata: u64,
}

impl KEvent {
    /// Convenience constructor for a readable-fd event.
    pub fn read_fd(fd: i32, udata: u64) -> Self {
        Self {
            ident: fd as u64,
            filter: KFilter::Read,
            flags: flags::ADD | flags::ENABLE,
            fflags: 0,
            data: 0,
            udata,
        }
    }

    /// Convenience constructor for a one-shot timer (milliseconds).
    pub fn timer_ms(id: u64, ms: i64, udata: u64) -> Self {
        Self {
            ident: id,
            filter: KFilter::Timer,
            flags: flags::ADD | flags::ENABLE | flags::ONESHOT,
            fflags: 0,
            data: ms,
            udata,
        }
    }

    /// Convenience constructor for a process-exit event.
    pub fn proc_exit(pid: u64, udata: u64) -> Self {
        Self {
            ident: pid,
            filter: KFilter::Proc,
            flags: flags::ADD | flags::ENABLE | flags::ONESHOT,
            fflags: 0,
            data: 0,
            udata,
        }
    }

    fn is_enabled(&self) -> bool {
        (self.flags & flags::DISABLE) == 0
    }

    fn is_oneshot(&self) -> bool {
        (self.flags & flags::ONESHOT) != 0
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Kqueue
// ─────────────────────────────────────────────────────────────────────────────

/// A kqueue instance — an in-kernel list of monitored events and a ready list.
pub struct Kqueue {
    /// All registered events.
    interests: Vec<KEvent>,
    /// Events that became ready since last `drain_ready()`.
    ready: Vec<KEvent>,
    /// Total number of events fired since creation.
    fired_total: u64,
}

impl Kqueue {
    /// Create a new, empty kqueue.
    pub fn new() -> Self {
        Self {
            interests: Vec::new(),
            ready: Vec::new(),
            fired_total: 0,
        }
    }

    // ── Registration ──────────────────────────────────────────────────────────

    /// Add, modify, or remove an event.
    ///
    /// - `EV_ADD`: registers or updates the event.
    /// - `EV_DELETE`: removes the event matching `(ident, filter)`.
    /// - `EV_DISABLE` / `EV_ENABLE`: toggle without removing.
    pub fn kevent_ctl(&mut self, ev: KEvent) -> Result<(), KqueueError> {
        if (ev.flags & flags::DELETE) != 0 {
            return self.remove(ev.ident, ev.filter);
        }
        if (ev.flags & flags::ADD) != 0 {
            self.upsert(ev);
            return Ok(());
        }
        // EV_ENABLE or EV_DISABLE without EV_ADD — update existing.
        for i in 0..self.interests.len() {
            if let Some(existing) = self.interests.get_mut(i) {
                if existing.ident == ev.ident && existing.filter == ev.filter {
                    existing.flags = ev.flags;
                    return Ok(());
                }
            }
        }
        Err(KqueueError::NotFound)
    }

    /// Simulate firing an event (called by the kernel when a condition is met).
    ///
    /// In a real kernel this would be called from interrupt context or from the
    /// VFS/network layers when a condition changes.
    pub fn fire(&mut self, ident: u64, filter: KFilter, data: i64) {
        let mut to_remove: Option<usize> = None;
        let mut fire_ev: Option<KEvent> = None;

        for i in 0..self.interests.len() {
            if let Some(ev) = self.interests.get(i) {
                if ev.ident == ident && ev.filter == filter && ev.is_enabled() {
                    let mut ready = ev.clone();
                    ready.data = data;
                    fire_ev = Some(ready);
                    if ev.is_oneshot() {
                        to_remove = Some(i);
                    }
                    break;
                }
            }
        }

        if let Some(ev) = fire_ev {
            self.fired_total = self.fired_total.wrapping_add(1);
            self.ready.push(ev);
        }

        // Remove oneshot event after firing.
        if let Some(idx) = to_remove {
            // Swap-remove (O(1)) — order of interest list is not significant.
            let last = self.interests.len() - 1;
            if idx != last {
                if let (Some(last_ev), Some(slot)) = (
                    self.interests.get(last).cloned(),
                    self.interests.get_mut(idx),
                ) {
                    *slot = last_ev;
                }
            }
            // Remove last element — Vec::truncate via pop logic.
            // Custom Vec: use pop or remove_last if available.
            let _ = self.interests.pop();
        }
    }

    /// Drain all currently ready events.
    ///
    /// Returns the events since the last call.  In a real kqueue this would
    /// block until at least one event is ready or a timeout expires.
    pub fn drain_ready(&mut self) -> Vec<KEvent> {
        let mut out = Vec::new();
        for i in 0..self.ready.len() {
            if let Some(ev) = self.ready.get(i) {
                out.push(ev.clone());
            }
        }
        self.ready = Vec::new();
        out
    }

    // ── Query ─────────────────────────────────────────────────────────────────

    pub fn interest_count(&self) -> usize {
        self.interests.len()
    }
    pub fn ready_count(&self) -> usize {
        self.ready.len()
    }
    pub fn fired_total(&self) -> u64 {
        self.fired_total
    }

    // ── Internal helpers ──────────────────────────────────────────────────────

    fn upsert(&mut self, ev: KEvent) {
        for i in 0..self.interests.len() {
            if let Some(existing) = self.interests.get_mut(i) {
                if existing.ident == ev.ident && existing.filter == ev.filter {
                    *existing = ev;
                    return;
                }
            }
        }
        self.interests.push(ev);
    }

    fn remove(&mut self, ident: u64, filter: KFilter) -> Result<(), KqueueError> {
        for i in 0..self.interests.len() {
            if let Some(ev) = self.interests.get(i) {
                if ev.ident == ident && ev.filter == filter {
                    let last = self.interests.len() - 1;
                    if i != last {
                        if let (Some(last_ev), Some(slot)) =
                            (self.interests.get(last).cloned(), self.interests.get_mut(i))
                        {
                            *slot = last_ev;
                        }
                    }
                    let _ = self.interests.pop();
                    return Ok(());
                }
            }
        }
        Err(KqueueError::NotFound)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Error type
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KqueueError {
    /// No matching event found for DELETE / DISABLE / ENABLE.
    NotFound,
    /// Internal capacity exhausted.
    Full,
}

// ─────────────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_fire_read() {
        let mut kq = Kqueue::new();
        kq.kevent_ctl(KEvent::read_fd(3, 42)).unwrap();
        kq.fire(3, KFilter::Read, 1024);
        let ready = kq.drain_ready();
        assert_eq!(ready.len(), 1);
        assert_eq!(ready.get(0).unwrap().udata, 42);
        assert_eq!(ready.get(0).unwrap().data, 1024);
    }

    #[test]
    fn test_oneshot_auto_removes() {
        let mut kq = Kqueue::new();
        kq.kevent_ctl(KEvent::timer_ms(1, 100, 0)).unwrap();
        assert_eq!(kq.interest_count(), 1);
        kq.fire(1, KFilter::Timer, 0);
        let _ = kq.drain_ready();
        assert_eq!(
            kq.interest_count(),
            0,
            "oneshot should be removed after fire"
        );
    }

    #[test]
    fn test_delete_event() {
        let mut kq = Kqueue::new();
        kq.kevent_ctl(KEvent::read_fd(5, 0)).unwrap();
        kq.kevent_ctl(KEvent {
            ident: 5,
            filter: KFilter::Read,
            flags: flags::DELETE,
            fflags: 0,
            data: 0,
            udata: 0,
        })
        .unwrap();
        kq.fire(5, KFilter::Read, 0);
        assert_eq!(kq.drain_ready().len(), 0, "deleted event should not fire");
    }

    #[test]
    fn test_no_match_returns_empty() {
        let mut kq = Kqueue::new();
        kq.fire(99, KFilter::Read, 0);
        assert_eq!(kq.drain_ready().len(), 0);
    }
}

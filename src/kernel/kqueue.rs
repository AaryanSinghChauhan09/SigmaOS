// SigmaOS BSD-inspired kqueue(2) Kernel Event Notification Subsystem

use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum FilterFlags {
    EVFILT_READ = -1,
    EVFILT_WRITE = -2,
    EVFILT_AIO = -3,
    EVFILT_VNODE = -4,
    EVFILT_PROC = -5,
    EVFILT_SIGNAL = -6,
    EVFILT_TIMER = -7,
    EVFILT_USER = -8,
}

#[derive(Debug, Clone, Copy)]
pub struct KEvent {
    pub ident: usize,
    pub filter: FilterFlags,
    pub flags: u16,
    pub fflags: u32,
    pub data: i64,
    pub udata: usize,
}

impl KEvent {
    pub fn new(ident: usize, filter: FilterFlags, flags: u16, udata: usize) -> Self {
        Self {
            ident,
            filter,
            flags,
            fflags: 0,
            data: 0,
            udata,
        }
    }
}

pub const EV_ADD: u16 = 0x0001;
pub const EV_DELETE: u16 = 0x0002;
pub const EV_ENABLE: u16 = 0x0004;
pub const EV_DISABLE: u16 = 0x0008;
pub const EV_ONESHOT: u16 = 0x0010;
pub const EV_CLEAR: u16 = 0x0020;
pub const EV_EOF: u16 = 0x8000;
pub const EV_ERROR: u16 = 0x4000;

pub struct KQueue {
    pub kq_id: usize,
    pub registered_events: Vec<KEvent>,
    pub pending_events: Vec<KEvent>,
}

impl KQueue {
    pub fn new(kq_id: usize) -> Self {
        Self {
            kq_id,
            registered_events: Vec::new(),
            pending_events: Vec::new(),
        }
    }

    pub fn kevent_register(&mut self, event: KEvent) {
        if (event.flags & EV_DELETE) != 0 {
            self.registered_events.retain(|e| !(e.ident == event.ident && e.filter == event.filter));
            return;
        }

        if let Some(existing) = self.registered_events.iter_mut().find(|e| e.ident == event.ident && e.filter == event.filter) {
            existing.flags = event.flags;
            existing.fflags = event.fflags;
            existing.data = event.data;
            existing.udata = event.udata;
        } else if (event.flags & EV_ADD) != 0 {
            self.registered_events.push(event);
        }
        self.pending_events.push(event);
    }

    pub fn kevent_poll(&mut self) -> Vec<KEvent> {
        let events = self.pending_events.clone();
        self.pending_events.clear();

        // Handle EV_ONESHOT cleanup
        for ev in &events {
            if (ev.flags & EV_ONESHOT) != 0 {
                self.registered_events.retain(|e| !(e.ident == ev.ident && e.filter == ev.filter));
            }
        }
        events
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kqueue_kevent_flags_and_polling() {
        let mut kq = KQueue::new(1);
        let ev = KEvent::new(101, FilterFlags::EVFILT_READ, EV_ADD | EV_ONESHOT, 42);
        kq.kevent_register(ev);
        assert_eq!(kq.registered_events.len(), 1);

        let polled = kq.kevent_poll();
        assert_eq!(polled.len(), 1);
        assert_eq!(polled[0].ident, 101);
        assert_eq!(kq.registered_events.len(), 0);
    }
}

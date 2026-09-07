// SigmaOS BSD-inspired kqueue(2) Kernel Event Notification Subsystem

use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

pub struct KQueue {
    pub kq_id: usize,
    pub pending_events: Vec<KEvent>,
}

impl KQueue {
    pub fn new(kq_id: usize) -> Self {
        Self {
            kq_id,
            pending_events: Vec::new(),
        }
    }

    pub fn kevent_register(&mut self, event: KEvent) {
        self.pending_events.push(event);
    }

    pub fn kevent_poll(&mut self) -> Vec<KEvent> {
        let events = self.pending_events.clone();
        self.pending_events.clear();
        events
    }
}

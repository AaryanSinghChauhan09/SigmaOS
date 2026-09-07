#![allow(dead_code)]
// SigmaOS Kernel Subsystem - BSD Kqueue Wrapper & Interface
// Provides unified BSD kqueue event multiplexing primitives

#[cfg(not(feature = "standalone_test"))]
use crate::kernel::kqueue_event::{FilterFlags, FilterType, Kevent, Kqueue, KqueueManager};

#[cfg(feature = "standalone_test")]
#[path = "kqueue_event.rs"]
pub mod kqueue_event;

#[cfg(feature = "standalone_test")]
pub use kqueue_event::{FilterFlags, FilterType, Kevent, Kqueue, KqueueManager};

/// Wrapper around BSD kqueue descriptor
pub struct BsdKqueueHandle {
    pub manager: KqueueManager,
    pub kqueue_fd: i32,
}

impl BsdKqueueHandle {
    pub fn new() -> Result<Self, &'static str> {
        let manager = KqueueManager::new();
        let fd = manager.kqueue().map_err(|_| "Failed to create kqueue FD")?;
        Ok(Self {
            manager,
            kqueue_fd: fd,
        })
    }

    pub fn register_event(&self, ident: u64, filter: FilterType) -> Result<(), &'static str> {
        let kevent = Kevent::new(ident, filter, 0, 0);
        self.manager
            .kevent_add(self.kqueue_fd, kevent)
            .map_err(|_| "Failed to add kevent")
    }

    pub fn trigger_event(&self, ident: u64, filter: FilterType, data: i64) -> Result<(), &'static str> {
        self.manager
            .trigger_event(self.kqueue_fd, ident, filter, data)
            .map_err(|_| "Failed to trigger kevent")
    }

    pub fn poll_events(&self, max_events: usize) -> Result<std::vec::Vec<Kevent>, &'static str> {
        self.manager
            .kevent_get(self.kqueue_fd, max_events)
            .map_err(|_| "Failed to poll kevent list")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bsd_kqueue_handle() {
        let kqueue = BsdKqueueHandle::new().unwrap();
        assert!(kqueue.kqueue_fd > 0);

        assert!(kqueue.register_event(101, FilterType::Read).is_ok());
        assert!(kqueue.trigger_event(101, FilterType::Read, 4096).is_ok());

        let events = kqueue.poll_events(10).unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].ident, 101);
        assert_eq!(events[0].data, 4096);
    }
}

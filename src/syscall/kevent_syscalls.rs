// kevent System Calls for SigmaOS
// Implements BSD kevent syscall for event multiplexing

use crate::kernel::kqueue_event::{FilterType, Kevent, KqueueManager};

/// Maximum events per kevent call
pub const MAX_KEVENT_EVENTS: usize = 256;

/// kevent syscall argument structure (userland)
#[repr(C)]
#[derive(Debug, Clone)]
pub struct KeventArgs {
    /// File descriptor
    pub fd: i32,
    /// Number of change events
    pub nchanges: u32,
    /// Number of events to return
    pub nevents: u32,
    /// Timeout in milliseconds (-1 = infinite)
    pub timeout: i32,
}

/// kevent wrapper for syscalls
pub struct KeventSyscall {
    /// kqueue manager
    manager: KqueueManager,
}

impl KeventSyscall {
    /// Create new kevent syscall handler
    pub fn new(manager: KqueueManager) -> Self {
        KeventSyscall { manager }
    }

    /// sys_kqueue - create new kqueue
    pub fn sys_kqueue(&self) -> Result<i32, String> {
        self.manager.kqueue()
    }

    /// sys_kevent - wait for events
    pub fn sys_kevent(
        &self,
        fd: i32,
        changes: Vec<Kevent>,
        nevents: usize,
        _timeout_ms: i32,
    ) -> Result<Vec<Kevent>, String> {
        // Apply changes
        for change in changes {
            self.manager.kevent_add(fd, change)?;
        }

        // Get events
        let events = self.manager.kevent_get(fd, nevents.min(MAX_KEVENT_EVENTS))?;
        Ok(events)
    }

    /// sys_kevent_add - add interest (kevent with EV_ADD flag)
    pub fn sys_kevent_add(&self, fd: i32, event: Kevent) -> Result<(), String> {
        self.manager.kevent_add(fd, event)
    }

    /// sys_kevent_delete - remove interest (kevent with EV_DELETE flag)
    pub fn sys_kevent_delete(&self, fd: i32, ident: u64, filter: FilterType) -> Result<(), String> {
        self.manager.kevent_delete(fd, ident, filter)
    }

    /// sys_close on kqueue fd
    pub fn sys_close_kqueue(&self, fd: i32) -> Result<(), String> {
        self.manager.close(fd)
    }
}

impl Clone for KeventSyscall {
    fn clone(&self) -> Self {
        KeventSyscall {
            manager: self.manager.clone(),
        }
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_sys_kqueue() {
        let manager = KqueueManager::new();
        let syscall = KeventSyscall::new(manager);
        let fd = syscall.sys_kqueue().unwrap();
        assert_eq!(fd, 1);
    }

    #[test]
    fn test_sys_kevent_add() {
        let manager = KqueueManager::new();
        let syscall = KeventSyscall::new(manager);
        let fd = syscall.sys_kqueue().unwrap();

        let event = Kevent::new(1, FilterType::Read, 0, 0);
        syscall.sys_kevent_add(fd, event).unwrap();
    }

    #[test]
    fn test_sys_kevent_delete() {
        let manager = KqueueManager::new();
        let syscall = KeventSyscall::new(manager);
        let fd = syscall.sys_kqueue().unwrap();

        let event = Kevent::new(1, FilterType::Read, 0, 0);
        syscall.sys_kevent_add(fd, event).unwrap();
        syscall.sys_kevent_delete(fd, 1, FilterType::Read).unwrap();
    }

    #[test]
    fn test_sys_kevent() {
        let manager = KqueueManager::new();
        let syscall = KeventSyscall::new(manager);
        let fd = syscall.sys_kqueue().unwrap();

        let event = Kevent::new(1, FilterType::Read, 0, 0);
        let changes = vec![event];
        let events = syscall.sys_kevent(fd, changes, 10, -1).unwrap();
        assert_eq!(events.len(), 0); // No events triggered yet
    }

    #[test]
    fn test_sys_close_kqueue() {
        let manager = KqueueManager::new();
        let syscall = KeventSyscall::new(manager.clone());
        let fd = syscall.sys_kqueue().unwrap();
        syscall.sys_close_kqueue(fd).unwrap();
        assert!(manager.kqueue_count().unwrap() == 0);
    }
}

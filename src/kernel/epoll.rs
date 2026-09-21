// Linux-inspired epoll I/O event notification
// Efficient I/O multiplexing for SigmaOS

use std::collections::HashMap;

/// epoll operation types (Linux epoll.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EpollCtlOp {
    Add = 1,
    Del = 2,
    Mod = 3,
}

/// epoll event flags (Linux epoll.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EpollEventFlags {
    pub read: bool,
    pub write: bool,
    pub edge_triggered: bool,
    pub urgent: bool,
    pub error: bool,
    pub hangup: bool,
}

impl EpollEventFlags {
    pub fn new() -> Self {
        EpollEventFlags {
            read: false,
            write: false,
            edge_triggered: false,
            urgent: false,
            error: false,
            hangup: false,
        }
    }

    pub fn from_bits(bits: u32) -> Self {
        EpollEventFlags {
            read: bits & 0x001 != 0,
            write: bits & 0x004 != 0,
            edge_triggered: bits & 0x800 != 0,
            urgent: bits & 0x002 != 0,
            error: bits & 0x008 != 0,
            hangup: bits & 0x010 != 0,
        }
    }

    pub fn to_bits(&self) -> u32 {
        let mut bits = 0u32;
        if self.read { bits |= 0x001; }
        if self.write { bits |= 0x004; }
        if self.edge_triggered { bits |= 0x800; }
        if self.urgent { bits |= 0x002; }
        if self.error { bits |= 0x008; }
        if self.hangup { bits |= 0x010; }
        bits
    }
}

impl Default for EpollEventFlags {
    fn default() -> Self {
        Self::new()
    }
}

/// epoll event
#[derive(Debug, Clone)]
pub struct EpollEvent {
    pub fd: i32,
    pub flags: EpollEventFlags,
    pub data: u64,
}

impl EpollEvent {
    pub fn new(fd: i32, flags: EpollEventFlags, data: u64) -> Self {
        EpollEvent { fd, flags, data }
    }
}

/// epoll instance
pub struct Epoll {
    events: HashMap<i32, EpollEvent>,
    ready_events: Vec<EpollEvent>,
}

impl Epoll {
    pub fn new() -> Self {
        Epoll {
            events: HashMap::new(),
            ready_events: Vec::new(),
        }
    }

    /// Add, modify, or remove an epoll interest
    pub fn ctl(&mut self, op: EpollCtlOp, fd: i32, event: EpollEvent) -> Result<(), String> {
        match op {
            EpollCtlOp::Add => {
                if self.events.contains_key(&fd) {
                    return Err(format!("File descriptor already exists: {}", fd));
                }
                self.events.insert(fd, event);
            }
            EpollCtlOp::Del => {
                self.events.remove(&fd)
                    .ok_or_else(|| format!("File descriptor not found: {}", fd))?;
            }
            EpollCtlOp::Mod => {
                self.events.get_mut(&fd)
                    .ok_or_else(|| format!("File descriptor not found: {}", fd))?
                    .flags = event.flags;
            }
        }
        Ok(())
    }

    /// Wait for events
    pub fn wait(&mut self, max_events: usize, _timeout_ms: i32) -> Vec<EpollEvent> {
        // Simulate event readiness (in real implementation, this would block)
        let mut ready = Vec::new();
        
        for event in self.events.values() {
            // Simulate random readiness for testing
            if event.flags.read || event.flags.write {
                ready.push(event.clone());
                if ready.len() >= max_events {
                    break;
                }
            }
        }
        
        ready
    }

    /// Get event count
    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    /// Check if fd is being monitored
    pub fn is_monitored(&self, fd: i32) -> bool {
        self.events.contains_key(&fd)
    }
}

impl Default for Epoll {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epoll_event_flags() {
        let flags = EpollEventFlags::new();
        assert!(!flags.read);
        assert!(!flags.write);
    }

    #[test]
    fn test_epoll_event_flags_from_bits() {
        let bits = 0x001 | 0x004;
        let flags = EpollEventFlags::from_bits(bits);
        
        assert!(flags.read);
        assert!(flags.write);
        assert!(!flags.edge_triggered);
    }

    #[test]
    fn test_epoll_event_flags_to_bits() {
        let mut flags = EpollEventFlags::new();
        flags.read = true;
        flags.write = true;
        
        let bits = flags.to_bits();
        assert_eq!(bits, 0x001 | 0x004);
    }

    #[test]
    fn test_epoll_ctl_add() {
        let mut epoll = Epoll::new();
        let event = EpollEvent::new(1, EpollEventFlags::new(), 0);
        
        epoll.ctl(EpollCtlOp::Add, 1, event).unwrap();
        assert_eq!(epoll.event_count(), 1);
        assert!(epoll.is_monitored(1));
    }

    #[test]
    fn test_epoll_ctl_add_duplicate() {
        let mut epoll = Epoll::new();
        let event = EpollEvent::new(1, EpollEventFlags::new(), 0);
        
        epoll.ctl(EpollCtlOp::Add, 1, event.clone()).unwrap();
        let result = epoll.ctl(EpollCtlOp::Add, 1, event);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_epoll_ctl_del() {
        let mut epoll = Epoll::new();
        let event = EpollEvent::new(1, EpollEventFlags::new(), 0);
        
        epoll.ctl(EpollCtlOp::Add, 1, event).unwrap();
        epoll.ctl(EpollCtlOp::Del, 1, EpollEvent::new(1, EpollEventFlags::new(), 0)).unwrap();
        
        assert_eq!(epoll.event_count(), 0);
        assert!(!epoll.is_monitored(1));
    }

    #[test]
    fn test_epoll_ctl_del_nonexistent() {
        let mut epoll = Epoll::new();
        let event = EpollEvent::new(1, EpollEventFlags::new(), 0);
        
        let result = epoll.ctl(EpollCtlOp::Del, 1, event);
        assert!(result.is_err());
    }

    #[test]
    fn test_epoll_ctl_mod() {
        let mut epoll = Epoll::new();
        let mut event = EpollEvent::new(1, EpollEventFlags::new(), 0);
        event.flags.read = true;
        
        epoll.ctl(EpollCtlOp::Add, 1, event.clone()).unwrap();
        
        let mut mod_event = EpollEvent::new(1, EpollEventFlags::new(), 0);
        mod_event.flags.write = true;
        epoll.ctl(EpollCtlOp::Mod, 1, mod_event).unwrap();
        
        let monitored = epoll.events.get(&1).unwrap();
        assert!(!monitored.flags.read);
        assert!(monitored.flags.write);
    }

    #[test]
    fn test_epoll_wait() {
        let mut epoll = Epoll::new();
        
        let mut event1 = EpollEvent::new(1, EpollEventFlags::new(), 0);
        event1.flags.read = true;
        epoll.ctl(EpollCtlOp::Add, 1, event1).unwrap();
        
        let mut event2 = EpollEvent::new(2, EpollEventFlags::new(), 0);
        event2.flags.write = true;
        epoll.ctl(EpollCtlOp::Add, 2, event2).unwrap();
        
        let ready = epoll.wait(10, 100);
        assert!(ready.len() >= 1);
    }
}

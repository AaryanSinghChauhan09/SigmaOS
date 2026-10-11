// SPDX-License-Identifier: MIT
// SigmaOS - epoll/kqueue Event Notification Subsystem
// Inspired by Linux epoll (fs/eventpoll.c) and FreeBSD kqueue (sys/kern/kqueue.c)
// Provides scalable event notification for file descriptors

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

// ============================================================================
// epoll (Linux-inspired)
// ============================================================================

/// epoll operation types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EpollOp {
    Add = 1,
    Del = 2,
    Mod = 3,
}

/// epoll event flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EpollEventFlags(pub u32);

impl EpollEventFlags {
    pub const IN: u32 = 0x001;
    pub const PRI: u32 = 0x002;
    pub const OUT: u32 = 0x004;
    pub const ERR: u32 = 0x008;
    pub const HUP: u32 = 0x010;
    pub const RDHUP: u32 = 0x2000;
    pub const EDGE: u32 = 1u32 << 31;
    pub const ONESHOT: u32 = 1u32 << 30;
    pub const WAKEUP: u32 = 1u32 << 29;
}

impl EpollEventFlags {
    pub fn readable(&self) -> bool {
        self.0 & Self::IN != 0
    }
    pub fn writable(&self) -> bool {
        self.0 & Self::OUT != 0
    }
    pub fn edge_triggered(&self) -> bool {
        self.0 & Self::EDGE != 0
    }
}

/// epoll event structure
#[derive(Debug, Clone, Copy)]
pub struct EpollEvent {
    pub events: EpollEventFlags,
    pub data: u64,
}

impl EpollEvent {
    pub fn new(events: EpollEventFlags, data: u64) -> Self {
        Self { events, data }
    }
}

/// epoll instance
pub struct EpollInstance {
    pub ep_id: usize,
    pub monitored_fds: BTreeMap<i32, EpollEvent>,
    pub ready_events: Vec<EpollEvent>,
}

impl EpollInstance {
    pub fn new(ep_id: usize) -> Self {
        Self {
            ep_id,
            monitored_fds: BTreeMap::new(),
            ready_events: Vec::new(),
        }
    }
    
    /// Add/modify/delete file descriptor monitoring
    pub fn ctl(&mut self, op: EpollOp, fd: i32, event: EpollEvent) -> Result<(), &'static str> {
        match op {
            EpollOp::Add => {
                if self.monitored_fds.contains_key(&fd) {
                    return Err("File descriptor already monitored");
                }
                self.monitored_fds.insert(fd, event);
            }
            EpollOp::Del => {
                self.monitored_fds.remove(&fd).ok_or("File descriptor not monitored")?;
            }
            EpollOp::Mod => {
                self.monitored_fds.insert(fd, event);
            }
        }
        Ok(())
    }
    
    /// Wait for events (blocking)
    pub fn wait(&mut self, max_events: usize, timeout_ms: i32) -> Result<usize, &'static str> {
        // In real implementation, this would block until events arrive
        // For now, simulate by checking ready events
        let count = self.ready_events.len().min(max_events);
        Ok(count)
    }
    
    /// Get ready events
    pub fn get_events(&mut self) -> Vec<EpollEvent> {
        let events = self.ready_events.clone();
        self.ready_events.clear();
        events
    }
    
    /// Signal that a file descriptor is ready (called by other subsystems)
    pub fn signal_ready(&mut self, fd: i32, event_flags: EpollEventFlags) {
        if let Some(event) = self.monitored_fds.get(&fd) {
            let mut ready_event = *event;
            ready_event.events = event_flags;
            self.ready_events.push(ready_event);
        }
    }
}

// ============================================================================
// kqueue (FreeBSD-inspired)
// ============================================================================

/// kqueue filter types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum KqueueFilter {
    Read = -1,
    Write = -2,
    Aio = -3,
    Vnode = -4,
    Proc = -5,
    Signal = -6,
    Timer = -7,
    User = -8,
}

/// kqueue event flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KqueueEventFlags(pub u16);

impl KqueueEventFlags {
    pub const ADD: u16 = 0x0001;
    pub const DELETE: u16 = 0x0002;
    pub const ENABLE: u16 = 0x0004;
    pub const DISABLE: u16 = 0x0008;
    pub const ONESHOT: u16 = 0x0010;
    pub const CLEAR: u16 = 0x0020;
    pub const RECEIPT: u16 = 0x0040;
    pub const DISPATCH: u16 = 0x0080;
    pub const EOF: u16 = 0x8000;
    pub const ERROR: u16 = 0x4000;
}

/// kqueue event structure
#[derive(Debug, Clone, Copy)]
pub struct KqueueEvent {
    pub ident: usize,
    pub filter: KqueueFilter,
    pub flags: KqueueEventFlags,
    pub fflags: u32,
    pub data: i64,
    pub udata: usize,
}

impl KqueueEvent {
    pub fn new(ident: usize, filter: KqueueFilter, flags: KqueueEventFlags, udata: usize) -> Self {
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

/// kqueue instance
pub struct KqueueInstance {
    pub kq_id: usize,
    pub monitored: BTreeMap<usize, KqueueEvent>,
    pub ready_events: Vec<KqueueEvent>,
}

impl KqueueInstance {
    pub fn new(kq_id: usize) -> Self {
        Self {
            kq_id,
            monitored: BTreeMap::new(),
            ready_events: Vec::new(),
        }
    }
    
    /// Register event interest
    pub fn kevent(&mut self, changelist: &[KqueueEvent]) -> Result<usize, &'static str> {
        let mut nchanges = 0;
        
        for event in changelist {
            let flags = event.flags;
            
            if flags.0 & KqueueEventFlags::ADD != 0 || flags.0 & KqueueEventFlags::ENABLE != 0 {
                self.monitored.insert(event.ident, *event);
                nchanges += 1;
            } else if flags.0 & KqueueEventFlags::DELETE != 0 || flags.0 & KqueueEventFlags::DISABLE != 0 {
                self.monitored.remove(&event.ident);
                nchanges += 1;
            }
        }
        
        Ok(nchanges)
    }
    
    /// Wait for events
    pub fn kevent_wait(&mut self, max_events: usize, timeout_ms: i32) -> Result<usize, &'static str> {
        let count = self.ready_events.len().min(max_events);
        Ok(count)
    }
    
    /// Get ready events
    pub fn get_events(&mut self) -> Vec<KqueueEvent> {
        let events = self.ready_events.clone();
        self.ready_events.clear();
        events
    }
    
    /// Signal event (called by other subsystems)
    pub fn signal_event(&mut self, ident: usize, filter: KqueueFilter, fflags: u32) {
        if let Some(event) = self.monitored.get(&ident) {
            let mut ready_event = *event;
            ready_event.fflags = fflags;
            ready_event.flags = KqueueEventFlags(ready_event.flags.0 | KqueueEventFlags::DISPATCH);
            self.ready_events.push(ready_event);
        }
    }
}

// ============================================================================
// Unified Event Notification Manager
// ============================================================================

/// Event notification manager supporting both epoll and kqueue
pub struct EventNotificationManager {
    pub epoll_instances: BTreeMap<usize, EpollInstance>,
    pub kqueue_instances: BTreeMap<usize, KqueueInstance>,
    pub next_ep_id: usize,
    pub next_kq_id: usize,
}

impl EventNotificationManager {
    pub fn new() -> Self {
        Self {
            epoll_instances: BTreeMap::new(),
            kqueue_instances: BTreeMap::new(),
            next_ep_id: 1,
            next_kq_id: 1,
        }
    }
    
    /// Create epoll instance
    pub fn epoll_create(&mut self) -> Result<i32, &'static str> {
        let ep_id = self.next_ep_id;
        self.next_ep_id += 1;
        
        let instance = EpollInstance::new(ep_id);
        self.epoll_instances.insert(ep_id, instance);
        
        Ok(ep_id as i32)
    }
    
    /// epoll_ctl operation
    pub fn epoll_ctl(&mut self, epfd: i32, op: EpollOp, fd: i32, event: EpollEvent) -> Result<(), &'static str> {
        let ep_id = epfd as usize;
        let instance = self.epoll_instances.get_mut(&ep_id)
            .ok_or("Invalid epoll file descriptor")?;
        
        instance.ctl(op, fd, event)
    }
    
    /// epoll_wait
    pub fn epoll_wait(&mut self, epfd: i32, max_events: usize, timeout_ms: i32) -> Result<Vec<EpollEvent>, &'static str> {
        let ep_id = epfd as usize;
        let instance = self.epoll_instances.get_mut(&ep_id)
            .ok_or("Invalid epoll file descriptor")?;
        
        instance.wait(max_events, timeout_ms)?;
        Ok(instance.get_events())
    }
    
    /// Create kqueue instance
    pub fn kqueue_create(&mut self) -> Result<i32, &'static str> {
        let kq_id = self.next_kq_id;
        self.next_kq_id += 1;
        
        let instance = KqueueInstance::new(kq_id);
        self.kqueue_instances.insert(kq_id, instance);
        
        Ok(kq_id as i32)
    }
    
    /// kevent syscall
    pub fn kevent(&mut self, kq: i32, changelist: &[KqueueEvent], max_events: usize, timeout_ms: i32) -> Result<Vec<KqueueEvent>, &'static str> {
        let kq_id = kq as usize;
        let instance = self.kqueue_instances.get_mut(&kq_id)
            .ok_or("Invalid kqueue file descriptor")?;
        
        instance.kevent(changelist)?;
        instance.kevent_wait(max_events, timeout_ms)?;
        Ok(instance.get_events())
    }
    
    /// Signal file descriptor ready (for integration with VFS, sockets, etc.)
    pub fn signal_fd_ready(&mut self, fd: i32, event_flags: EpollEventFlags) {
        for instance in self.epoll_instances.values_mut() {
            instance.signal_ready(fd, event_flags);
        }
    }
    
    /// Signal kqueue event
    pub fn signal_kqueue_event(&mut self, ident: usize, filter: KqueueFilter, fflags: u32) {
        for instance in self.kqueue_instances.values_mut() {
            instance.signal_event(ident, filter, fflags);
        }
    }
}

impl Default for EventNotificationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global event notification manager
static mut EVENT_MANAGER: Option<EventNotificationManager> = None;

/// Initialize the event notification manager
pub fn init_event_manager() {
    unsafe {
        EVENT_MANAGER = Some(EventNotificationManager::new());
    }
}

/// Get the global event notification manager
pub fn get_event_manager() -> Option<&'static mut EventNotificationManager> {
    unsafe {
        EVENT_MANAGER.as_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_epoll_create() {
        let mut manager = EventNotificationManager::new();
        let epfd = manager.epoll_create().unwrap();
        assert!(epfd > 0);
    }
    
    #[test]
    fn test_epoll_ctl_add() {
        let mut manager = EventNotificationManager::new();
        let epfd = manager.epoll_create().unwrap();
        
        let event = EpollEvent::new(EpollEventFlags(EpollEventFlags::IN), 42);
        manager.epoll_ctl(epfd, EpollOp::Add, 3, event).unwrap();
    }
    
    #[test]
    fn test_epoll_ctl_mod() {
        let mut manager = EventNotificationManager::new();
        let epfd = manager.epoll_create().unwrap();
        
        let event = EpollEvent::new(EpollEventFlags(EpollEventFlags::IN), 42);
        manager.epoll_ctl(epfd, EpollOp::Add, 3, event).unwrap();
        
        let event2 = EpollEvent::new(EpollEventFlags(EpollEventFlags::OUT), 43);
        manager.epoll_ctl(epfd, EpollOp::Mod, 3, event2).unwrap();
    }
    
    #[test]
    fn test_epoll_ctl_del() {
        let mut manager = EventNotificationManager::new();
        let epfd = manager.epoll_create().unwrap();
        
        let event = EpollEvent::new(EpollEventFlags(EpollEventFlags::IN), 42);
        manager.epoll_ctl(epfd, EpollOp::Add, 3, event).unwrap();
        manager.epoll_ctl(epfd, EpollOp::Del, 3, event).unwrap();
    }
    
    #[test]
    fn test_epoll_wait() {
        let mut manager = EventNotificationManager::new();
        let epfd = manager.epoll_create().unwrap();
        
        let event = EpollEvent::new(EpollEventFlags(EpollEventFlags::IN), 42);
        manager.epoll_ctl(epfd, EpollOp::Add, 3, event).unwrap();
        
        let events = manager.epoll_wait(epfd, 10, 100).unwrap();
        assert_eq!(events.len(), 0);
    }
    
    #[test]
    fn test_signal_fd_ready() {
        let mut manager = EventNotificationManager::new();
        let epfd = manager.epoll_create().unwrap();
        
        let event = EpollEvent::new(EpollEventFlags(EpollEventFlags::IN), 42);
        manager.epoll_ctl(epfd, EpollOp::Add, 3, event).unwrap();
        
        manager.signal_fd_ready(3, EpollEventFlags(EpollEventFlags::IN));
        
        let events = manager.epoll_wait(epfd, 10, 100).unwrap();
        assert_eq!(events.len(), 1);
    }
    
    #[test]
    fn test_kqueue_create() {
        let mut manager = EventNotificationManager::new();
        let kq = manager.kqueue_create().unwrap();
        assert!(kq > 0);
    }
    
    #[test]
    fn test_kevent_add() {
        let mut manager = EventNotificationManager::new();
        let kq = manager.kqueue_create().unwrap();
        
        let event = KqueueEvent::new(
            3,
            KqueueFilter::Read,
            KqueueEventFlags(KqueueEventFlags::ADD),
            42,
        );
        
        let nchanges = manager.kevent(kq, &[event], 10, 100).unwrap();
        assert_eq!(nchanges, 0);
    }
    
    #[test]
    fn test_kevent_delete() {
        let mut manager = EventNotificationManager::new();
        let kq = manager.kqueue_create().unwrap();
        
        let event = KqueueEvent::new(
            3,
            KqueueFilter::Read,
            KqueueEventFlags(KqueueEventFlags::ADD),
            42,
        );
        
        manager.kevent(kq, &[event], 10, 100).unwrap();
        
        let event2 = KqueueEvent::new(
            3,
            KqueueFilter::Read,
            KqueueEventFlags(KqueueEventFlags::DELETE),
            42,
        );
        
        let nchanges = manager.kevent(kq, &[event2], 10, 100).unwrap();
        assert_eq!(nchanges, 0);
    }
    
    #[test]
    fn test_signal_kqueue_event() {
        let mut manager = EventNotificationManager::new();
        let kq = manager.kqueue_create().unwrap();
        
        let event = KqueueEvent::new(
            3,
            KqueueFilter::Read,
            KqueueEventFlags(KqueueEventFlags::ADD),
            42,
        );
        
        manager.kevent(kq, &[event], 10, 100).unwrap();
        
        manager.signal_kqueue_event(3, KqueueFilter::Read, 0x100);
        
        let events = manager.kevent(kq, &[], 10, 100).unwrap();
        assert_eq!(events.len(), 1);
    }
}

// Linux-inspired epoll for I/O event notification
// Provides scalable I/O event monitoring

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Epoll event types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EpollEvents {
    pub in_events: bool,
    pub out_events: bool,
    pub rdhup: bool,
    pub pri: bool,
    pub err: bool,
    pub hup: bool,
}

impl EpollEvents {
    pub fn new() -> Self {
        Self {
            in_events: false,
            out_events: false,
            rdhup: false,
            pri: false,
            err: false,
            hup: false,
        }
    }

    pub fn with_in(mut self) -> Self {
        self.in_events = true;
        self
    }

    pub fn with_out(mut self) -> Self {
        self.out_events = true;
        self
    }

    pub fn with_rdhup(mut self) -> Self {
        self.rdhup = true;
        self
    }

    pub fn with_pri(mut self) -> Self {
        self.pri = true;
        self
    }

    pub fn with_err(mut self) -> Self {
        self.err = true;
        self
    }

    pub fn with_hup(mut self) -> Self {
        self.hup = true;
        self
    }

    pub fn to_u32(&self) -> u32 {
        let mut bits = 0u32;
        if self.in_events { bits |= 0x001; }
        if self.out_events { bits |= 0x004; }
        if self.rdhup { bits |= 0x2000; }
        if self.pri { bits |= 0x002; }
        if self.err { bits |= 0x008; }
        if self.hup { bits |= 0x010; }
        bits
    }
}

impl Default for EpollEvents {
    fn default() -> Self {
        Self::new()
    }
}

/// Epoll operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EpollOp {
    Add,
    Del,
    Mod,
}

/// Epoll event
#[derive(Debug, Clone)]
pub struct EpollEvent {
    pub events: EpollEvents,
    pub data: u64,
}

impl EpollEvent {
    pub fn new(events: EpollEvents, data: u64) -> Self {
        Self {
            events,
            data,
        }
    }

    /// Check if event is ready for read
    pub fn is_in(&self) -> bool {
        self.events.in_events
    }

    /// Check if event is ready for write
    pub fn is_out(&self) -> bool {
        self.events.out_events
    }

    /// Check if event has error
    pub fn is_err(&self) -> bool {
        self.events.err
    }

    /// Check if event has hangup
    pub fn is_hup(&self) -> bool {
        self.events.hup
    }
}

/// Epoll instance
#[derive(Debug, Clone)]
pub struct EpollInstance {
    pub id: u64,
    pub interests: HashMap<i32, EpollEvent>,
    pub ready_events: Vec<EpollEvent>,
}

impl EpollInstance {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            interests: HashMap::new(),
            ready_events: Vec::new(),
        }
    }

    /// Add/modify/delete a file descriptor
    pub fn ctl(&mut self, op: EpollOp, fd: i32, event: EpollEvent) -> Result<(), String> {
        match op {
            EpollOp::Add => {
                if self.interests.contains_key(&fd) {
                    return Err(format!("File descriptor {} already exists", fd));
                }
                self.interests.insert(fd, event);
                Ok(())
            }
            EpollOp::Del => {
                match self.interests.remove(&fd) {
                    Some(_) => Ok(()),
                    None => Err(format!("File descriptor {} not found", fd)),
                }
            }
            EpollOp::Mod => {
                match self.interests.get_mut(&fd) {
                    Some(e) => {
                        *e = event;
                        Ok(())
                    }
                    None => Err(format!("File descriptor {} not found", fd)),
                }
            }
        }
    }

    /// Wait for events (simulated)
    pub fn wait(&mut self, max_events: usize) -> Vec<EpollEvent> {
        let events = self.ready_events.clone();
        self.ready_events.clear();
        events.into_iter().take(max_events).collect()
    }

    /// Add a ready event (simulates I/O readiness)
    pub fn add_ready_event(&mut self, event: EpollEvent) {
        self.ready_events.push(event);
    }

    /// Get interest count
    pub fn interest_count(&self) -> usize {
        self.interests.len()
    }

    /// Get ready event count
    pub fn ready_count(&self) -> usize {
        self.ready_events.len()
    }
}

/// Epoll manager for system-wide epoll management
pub struct EpollManager {
    instances: Arc<Mutex<HashMap<u64, EpollInstance>>>,
    next_instance_id: Arc<Mutex<u64>>,
}

impl EpollManager {
    pub fn new() -> Self {
        Self {
            instances: Arc::new(Mutex::new(HashMap::new())),
            next_instance_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Create a new epoll instance
    pub fn create_instance(&self) -> u64 {
        let mut next_id = self.next_instance_id.lock().unwrap();
        let instance_id = *next_id;
        *next_id += 1;
        drop(next_id);

        let instance = EpollInstance::new(instance_id);
        let mut instances = self.instances.lock().unwrap();
        instances.insert(instance_id, instance);

        instance_id
    }

    /// Get an instance by ID
    pub fn get_instance(&self, instance_id: u64) -> Option<EpollInstance> {
        let instances = self.instances.lock().unwrap();
        instances.get(&instance_id).cloned()
    }

    /// Remove an instance
    pub fn remove_instance(&self, instance_id: u64) -> Result<(), String> {
        let mut instances = self.instances.lock().unwrap();
        match instances.remove(&instance_id) {
            Some(_) => Ok(()),
            None => Err(format!("Instance {} not found", instance_id)),
        }
    }

    /// Add/modify/delete a file descriptor
    pub fn ctl(&self, instance_id: u64, op: EpollOp, fd: i32, event: EpollEvent) -> Result<(), String> {
        let mut instances = self.instances.lock().unwrap();
        match instances.get_mut(&instance_id) {
            Some(inst) => inst.ctl(op, fd, event),
            None => Err(format!("Instance {} not found", instance_id)),
        }
    }

    /// Wait for events
    pub fn wait(&self, instance_id: u64, max_events: usize) -> Result<Vec<EpollEvent>, String> {
        let mut instances = self.instances.lock().unwrap();
        match instances.get_mut(&instance_id) {
            Some(inst) => Ok(inst.wait(max_events)),
            None => Err(format!("Instance {} not found", instance_id)),
        }
    }

    /// Add a ready event (simulates I/O readiness)
    pub fn add_ready_event(&self, instance_id: u64, event: EpollEvent) -> Result<(), String> {
        let mut instances = self.instances.lock().unwrap();
        match instances.get_mut(&instance_id) {
            Some(inst) => {
                inst.add_ready_event(event);
                Ok(())
            }
            None => Err(format!("Instance {} not found", instance_id)),
        }
    }

    /// Get instance count
    pub fn instance_count(&self) -> usize {
        let instances = self.instances.lock().unwrap();
        instances.len()
    }
}

impl Default for EpollManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epoll_events() {
        let events = EpollEvents::new().with_in().with_out();
        assert!(events.in_events);
        assert!(events.out_events);
        assert!(!events.err);
    }

    #[test]
    fn test_epoll_events_all() {
        let events = EpollEvents::new()
            .with_in()
            .with_out()
            .with_rdhup()
            .with_pri()
            .with_err()
            .with_hup();

        assert!(events.in_events);
        assert!(events.out_events);
        assert!(events.rdhup);
        assert!(events.pri);
        assert!(events.err);
        assert!(events.hup);
    }

    #[test]
    fn test_epoll_event() {
        let events = EpollEvents::new().with_in();
        let event = EpollEvent::new(events, 42);

        assert!(event.is_in());
        assert!(!event.is_out());
        assert_eq!(event.data, 42);
    }

    #[test]
    fn test_epoll_instance() {
        let instance = EpollInstance::new(1);
        assert_eq!(instance.id, 1);
        assert_eq!(instance.interest_count(), 0);
        assert_eq!(instance.ready_count(), 0);
    }

    #[test]
    fn test_epoll_instance_ctl_add() {
        let mut instance = EpollInstance::new(1);

        let events = EpollEvents::new().with_in();
        let event = EpollEvent::new(events, 42);
        instance.ctl(EpollOp::Add, 5, event).unwrap();

        assert_eq!(instance.interest_count(), 1);
    }

    #[test]
    fn test_epoll_instance_ctl_add_duplicate() {
        let mut instance = EpollInstance::new(1);

        let events = EpollEvents::new().with_in();
        let event = EpollEvent::new(events, 42);
        instance.ctl(EpollOp::Add, 5, event.clone()).unwrap();
        assert!(instance.ctl(EpollOp::Add, 5, event).is_err());
    }

    #[test]
    fn test_epoll_instance_ctl_del() {
        let mut instance = EpollInstance::new(1);

        let events = EpollEvents::new().with_in();
        let event = EpollEvent::new(events, 42);
        instance.ctl(EpollOp::Add, 5, event.clone()).unwrap();
        instance.ctl(EpollOp::Del, 5, event).unwrap();

        assert_eq!(instance.interest_count(), 0);
    }

    #[test]
    fn test_epoll_instance_ctl_mod() {
        let mut instance = EpollInstance::new(1);

        let events1 = EpollEvents::new().with_in();
        let event1 = EpollEvent::new(events1, 42);
        instance.ctl(EpollOp::Add, 5, event1).unwrap();

        let events2 = EpollEvents::new().with_out();
        let event2 = EpollEvent::new(events2, 43);
        instance.ctl(EpollOp::Mod, 5, event2).unwrap();

        assert_eq!(instance.interest_count(), 1);
    }

    #[test]
    fn test_epoll_instance_wait() {
        let mut instance = EpollInstance::new(1);

        let events = EpollEvents::new().with_in();
        let event = EpollEvent::new(events, 42);
        instance.add_ready_event(event.clone());
        instance.add_ready_event(event);

        let ready = instance.wait(10);
        assert_eq!(ready.len(), 2);
        assert_eq!(instance.ready_count(), 0);
    }

    #[test]
    fn test_epoll_instance_wait_max() {
        let mut instance = EpollInstance::new(1);

        let events = EpollEvents::new().with_in();
        let event = EpollEvent::new(events, 42);
        for _ in 0..5 {
            instance.add_ready_event(event.clone());
        }

        let ready = instance.wait(3);
        assert_eq!(ready.len(), 3);
    }

    #[test]
    fn test_epoll_manager() {
        let manager = EpollManager::new();

        let instance_id = manager.create_instance();
        assert_eq!(instance_id, 1);

        let events = EpollEvents::new().with_in();
        let event = EpollEvent::new(events, 42);
        manager.ctl(instance_id, EpollOp::Add, 5, event).unwrap();

        assert_eq!(manager.instance_count(), 1);
    }

    #[test]
    fn test_epoll_manager_add_ready() {
        let manager = EpollManager::new();

        let instance_id = manager.create_instance();
        let events = EpollEvents::new().with_in();
        let event = EpollEvent::new(events, 42);
        manager.add_ready_event(instance_id, event).unwrap();

        assert_eq!(manager.wait(instance_id, 10).unwrap().len(), 1);
    }

    #[test]
    fn test_epoll_manager_remove() {
        let manager = EpollManager::new();

        let instance_id = manager.create_instance();
        manager.remove_instance(instance_id).unwrap();

        assert_eq!(manager.instance_count(), 0);
    }

    #[test]
    fn test_epoll_manager_invalid() {
        let manager = EpollManager::new();
        let events = EpollEvents::new().with_in();
        let event = EpollEvent::new(events, 42);
        assert!(manager.ctl(999, EpollOp::Add, 5, event).is_err());
        assert!(manager.wait(999, 10).is_err());
    }
}

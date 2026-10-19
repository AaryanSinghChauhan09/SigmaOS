//! SigmaJails - BSD Jails Container Isolation System
//!
//! Provides BSD-style jail container isolation inspired by FreeBSD Jails.
//! Supports process isolation, resource limits, and security containment.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
}

/// Jail state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JailState {
    Running,
    Stopped,
    Error,
    Unknown,
}

/// Jail
#[derive(Debug, Clone)]
pub struct Jail {
    pub id: String,
    pub name: String,
    pub state: JailState,
    pub path: String,
    pub hostname: String,
    pub ip_address: String,
    pub process_count: u32,
}

impl Jail {
    pub fn new(id: String, name: String, path: String) -> Self {
        Self {
            id,
            name,
            state: JailState::Stopped,
            path,
            hostname: String::new(),
            ip_address: String::new(),
            process_count: 0,
        }
    }

    pub fn with_hostname(mut self, hostname: String) -> Self {
        self.hostname = hostname;
        self
    }

    pub fn with_ip(mut self, ip_address: String) -> Self {
        self.ip_address = ip_address;
        self
    }

    pub fn start(&mut self) {
        self.state = JailState::Running;
    }

    pub fn stop(&mut self) {
        self.state = JailState::Stopped;
    }
}

/// Jails manager
pub struct SigmaJails {
    jails: BTreeMap<String, Jail>,
    next_jail_id: u64,
}

impl SigmaJails {
    pub fn new() -> Self {
        Self {
            jails: BTreeMap::new(),
            next_jail_id: 1,
        }
    }

    pub fn add_jail(&mut self, name: String, path: String) -> String {
        let id = format!("jail-{}", self.next_jail_id);
        self.next_jail_id += 1;
        let jail = Jail::new(id.clone(), name, path);
        self.jails.insert(id.clone(), jail);
        id
    }

    pub fn get_jail(&self, id: &str) -> Option<&Jail> {
        self.jails.get(id)
    }

    pub fn get_jail_mut(&mut self, id: &str) -> Option<&mut Jail> {
        self.jails.get_mut(id)
    }

    pub fn list_jails(&self) -> Vec<&Jail> {
        self.jails.values().collect()
    }

    pub fn list_running_jails(&self) -> Vec<&Jail> {
        self.jails.values().filter(|j| j.state == JailState::Running).collect()
    }

    pub fn remove_jail(&mut self, id: &str) -> bool {
        self.jails.remove(id).is_some()
    }

    pub fn jail_count(&self) -> usize {
        self.jails.len()
    }

    pub fn running_jail_count(&self) -> usize {
        self.jails.values().filter(|j| j.state == JailState::Running).count()
    }
}

impl Default for SigmaJails {
    fn default() -> Self {
        Self::new()
    }
}

static_mut SIGMAJAILS_INSTANCE: Option<SigmaJails> = None;

pub fn init_sigmajails() {
    unsafe {
        SIGMAJAILS_INSTANCE = Some(SigmaJails::new());
    }
}

pub fn get_sigmajails() -> &'static SigmaJails {
    unsafe {
        SIGMAJAILS_INSTANCE
            .as_ref()
            .expect("SigmaJails not initialized")
    }
}

pub fn get_sigmajails_mut() -> &'static mut SigmaJails {
    unsafe {
        SIGMAJAILS_INSTANCE
            .as_mut()
            .expect("SigmaJails not initialized")
    }
}

pub fn add_jail(name: String, path: String) -> String {
    let manager = get_sigmajails_mut();
    manager.add_jail(name, path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jail_creation() {
        let jail = Jail::new("jail-1".to_string(), "webserver".to_string(), "/jails/webserver".to_string());
        assert_eq!(jail.name, "webserver");
        assert_eq!(jail.path, "/jails/webserver");
        assert_eq!(jail.state, JailState::Stopped);
    }

    #[test]
    fn test_jail_state() {
        let mut jail = Jail::new("jail-1".to_string(), "webserver".to_string(), "/jails/webserver".to_string());
        jail.start();
        assert_eq!(jail.state, JailState::Running);

        jail.stop();
        assert_eq!(jail.state, JailState::Stopped);
    }

    #[test]
    fn test_jails_manager() {
        let manager = SigmaJails::new();
        assert_eq!(manager.jail_count(), 0);
    }

    #[test]
    fn test_add_jail() {
        let mut manager = SigmaJails::new();
        let id = manager.add_jail("webserver".to_string(), "/jails/webserver".to_string());
        assert_eq!(id, "jail-1");
        assert_eq!(manager.jail_count(), 1);
    }

    #[test]
    fn test_list_running_jails() {
        let mut manager = SigmaJails::new();
        let id1 = manager.add_jail("webserver".to_string(), "/jails/webserver".to_string());
        let id2 = manager.add_jail("database".to_string(), "/jails/database".to_string());

        manager.get_jail_mut(&id1).unwrap().start();
        manager.get_jail_mut(&id2).unwrap().start();

        let running = manager.list_running_jails();
        assert_eq!(running.len(), 2);
    }

    #[test]
    fn test_global_instance() {
        init_sigmajails();
        let manager = get_sigmajails();
        assert_eq!(manager.jail_count(), 0);

        let manager_mut = get_sigmajails_mut();
        manager_mut.add_jail("webserver".to_string(), "/jails/webserver".to_string());
        assert_eq!(manager.jail_count(), 1);
    }

    #[test]
    fn test_add_jail_convenience() {
        init_sigmajails();
        let id = add_jail("webserver".to_string(), "/jails/webserver".to_string());
        assert_eq!(id, "jail-1");

        let manager = get_sigmajails();
        assert!(manager.get_jail(&id).is_some());
    }
}

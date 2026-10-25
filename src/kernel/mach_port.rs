// SPDX-License-Identifier: GPL-2.0-or-later
// Principle: CAPABILITY-BASED IPC, INFORMATION HIDING, ENCAPSULATION, SAFE CONCURRENCY

use alloc::{collections::BTreeMap, string::String, vec::Vec};
use core::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

pub type PortName = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortRight {
    Send,
    Receive,
    SendOnce,
    DeadName,
    PortSet,
}

#[derive(Debug, Clone)]
pub struct MachMsgHeader {
    pub bits: u32,
    pub size: u32,
    pub remote_port: PortName,
    pub local_port: PortName,
    pub voucher_port: PortName,
    pub id: i32,
}

#[derive(Debug, Clone)]
pub struct MachMessage {
    pub header: MachMsgHeader,
    pub body: Vec<u8>,
}

#[derive(Debug)]
pub struct MachPort {
    pub name: PortName,
    pub right: PortRight,
    pub queue: Vec<MachMessage>,
    pub ref_count: usize,
}

impl MachPort {
    pub fn new(name: PortName, right: PortRight) -> Self {
        Self {
            name,
            right,
            queue: Vec::new(),
            ref_count: 1,
        }
    }

    pub fn send(&mut self, msg: MachMessage) -> Result<(), &'static str> {
        match self.right {
            PortRight::Send | PortRight::SendOnce => {
                self.queue.push(msg);
                Ok(())
            }
            _ => Err("no send right"),
        }
    }

    pub fn receive(&mut self) -> Option<MachMessage> {
        if self.right == PortRight::Receive {
            self.queue.pop()
        } else {
            None
        }
    }

    pub fn retain(&mut self) {
        self.ref_count += 1;
    }

    pub fn release(&mut self) -> bool {
        self.ref_count = self.ref_count.saturating_sub(1);
        self.ref_count == 0
    }
}

pub struct PortSet {
    pub members: Vec<PortName>,
}

impl PortSet {
    pub fn new() -> Self {
        Self {
            members: Vec::new(),
        }
    }

    pub fn insert(&mut self, name: PortName) {
        if !self.members.contains(&name) {
            self.members.push(name);
        }
    }

    pub fn remove(&mut self, name: PortName) {
        self.members.retain(|&n| n != name);
    }

    pub fn members(&self) -> &[PortName] {
        &self.members
    }
}

pub struct MachNamespace {
    next_name: u32,
    ports: BTreeMap<PortName, MachPort>,
}

impl MachNamespace {
    pub fn new() -> Self {
        Self {
            next_name: 1,
            ports: BTreeMap::new(),
        }
    }

    pub fn allocate(&mut self, right: PortRight) -> PortName {
        let name = self.next_name;
        self.next_name += 1;
        self.ports.insert(name, MachPort::new(name, right));
        name
    }

    pub fn get(&self, name: PortName) -> Option<&MachPort> {
        self.ports.get(&name)
    }

    pub fn get_mut(&mut self, name: PortName) -> Option<&mut MachPort> {
        self.ports.get_mut(&name)
    }

    pub fn deallocate(&mut self, name: PortName) {
        self.ports.remove(&name);
    }

    pub fn port_count(&self) -> usize {
        self.ports.len()
    }
}

/// Voucher-based IPC attribute propagation (XNU-style)
#[derive(Debug, Clone)]
pub struct IpcVoucher {
    pub id: u32,
    pub attributes: BTreeMap<u32, Vec<u8>>,
}

impl IpcVoucher {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            attributes: BTreeMap::new(),
        }
    }

    pub fn set_attribute(&mut self, key: u32, value: Vec<u8>) {
        self.attributes.insert(key, value);
    }

    pub fn get_attribute(&self, key: u32) -> Option<&Vec<u8>> {
        self.attributes.get(&key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_receive() {
        let mut ns = MachNamespace::new();
        let recv = ns.allocate(PortRight::Receive);
        let send = ns.allocate(PortRight::Send);
        let msg = MachMessage {
            header: MachMsgHeader {
                bits: 0,
                size: 4,
                remote_port: recv,
                local_port: send,
                voucher_port: 0,
                id: 42,
            },
            body: alloc::vec![1, 2, 3, 4],
        };
        ns.get_mut(send).unwrap().send(msg).unwrap();
        assert_eq!(ns.get_mut(send).unwrap().queue.len(), 1);
    }

    #[test]
    fn test_no_send_right_on_receive_port() {
        let mut ns = MachNamespace::new();
        let recv = ns.allocate(PortRight::Receive);
        let msg = MachMessage {
            header: MachMsgHeader {
                bits: 0,
                size: 0,
                remote_port: recv,
                local_port: 0,
                voucher_port: 0,
                id: 0,
            },
            body: alloc::vec![],
        };
        let result = ns.get_mut(recv).unwrap().send(msg);
        assert!(result.is_err());
    }

    #[test]
    fn test_port_set() {
        let mut ps = PortSet::new();
        ps.insert(1);
        ps.insert(2);
        ps.insert(1); // duplicate
        assert_eq!(ps.members().len(), 2);
        ps.remove(1);
        assert_eq!(ps.members().len(), 1);
    }

    #[test]
    fn test_ref_count() {
        let mut port = MachPort::new(1, PortRight::Send);
        assert_eq!(port.ref_count, 1);
        port.retain();
        assert_eq!(port.ref_count, 2);
        assert!(!port.release());
        assert!(port.release());
    }

    #[test]
    fn test_voucher_attributes() {
        let mut v = IpcVoucher::new(42);
        v.set_attribute(1, alloc::vec![0xDE, 0xAD]);
        assert_eq!(v.get_attribute(1).unwrap(), &alloc::vec![0xDE, 0xAD]);
        assert!(v.get_attribute(99).is_none());
    }
}

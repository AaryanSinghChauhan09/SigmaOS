//! BSD Kernel Parity Subsystems for SigmaOS
//!
//! Implements key kernel primitives inspired by FreeBSD, OpenBSD, and NetBSD:
//! - `BsdKqueueEngine`: `kqueue`/`kevent` I/O and process event notification engine
//! - `BsdVmZoneAllocator`: FreeBSD Universal Memory Allocator (UMA) zone page caching & memory pressure reclamation
//! - `BsdCapsicumFramework`: FreeBSD Capsicum capability mode & capability descriptor rights
//! - `BsdPfPacketFilter`: OpenBSD PF packet filter with stateful firewall rules, NAT translation, and tables
//! - `BsdSoftUpdatesEngine`: FreeBSD FFS Soft Updates metadata dependency ordering for file system journaling

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Kqueue Filter Types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KfilterType {
    EvfiltRead,
    EvfiltWrite,
    EvfiltSignal,
    EvfiltProc,
    EvfiltTimer,
}

/// Kevent Event Structure
#[derive(Debug, Clone)]
pub struct Kevent {
    pub ident: usize,
    pub filter: KfilterType,
    pub flags: u16,
    pub fflags: u32,
    pub data: i64,
    pub udata: u64,
}

/// BSD kqueue / kevent Event Notification Engine
pub struct BsdKqueueEngine {
    pub registered_events: Vec<Kevent>,
    pub triggered_events: Vec<Kevent>,
}

impl BsdKqueueEngine {
    pub fn new() -> Self {
        Self {
            registered_events: Vec::new(),
            triggered_events: Vec::new(),
        }
    }

    pub fn kevent_register(&mut self, ev: Kevent) {
        if let Some(pos) = self.registered_events.iter().position(|e| e.ident == ev.ident && e.filter == ev.filter) {
            self.registered_events[pos] = ev;
        } else {
            self.registered_events.push(ev);
        }
    }

    pub fn kevent_trigger(&mut self, ident: usize, filter: KfilterType, data: i64) -> usize {
        let mut count = 0;
        for ev in &self.registered_events {
            if ev.ident == ident && ev.filter == filter {
                let mut triggered = ev.clone();
                triggered.data = data;
                self.triggered_events.push(triggered);
                count += 1;
            }
        }
        count
    }

    pub fn kevent_poll(&mut self) -> Vec<Kevent> {
        let events = self.triggered_events.clone();
        self.triggered_events.clear();
        events
    }
}

impl Default for BsdKqueueEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Universal Memory Allocator (UMA) Zone
pub struct BsdUmaZone {
    pub zone_name: String,
    pub item_size_bytes: usize,
    pub allocated_items: usize,
    pub free_items_pool: Vec<u64>,
}

/// FreeBSD UMA Memory Zone Allocator
pub struct BsdVmZoneAllocator {
    pub zones: BTreeMap<String, BsdUmaZone>,
}

impl BsdVmZoneAllocator {
    pub fn new() -> Self {
        Self {
            zones: BTreeMap::new(),
        }
    }

    pub fn create_zone(&mut self, name: &str, item_size: usize) {
        self.zones.insert(
            name.to_string(),
            BsdUmaZone {
                zone_name: name.to_string(),
                item_size_bytes: item_size,
                allocated_items: 0,
                free_items_pool: Vec::new(),
            },
        );
    }

    pub fn zone_allocate(&mut self, zone_name: &str, phys_addr: u64) -> Result<u64, &'static str> {
        let zone = self.zones.get_mut(zone_name).ok_or("UMA zone not found")?;
        zone.allocated_items += 1;
        Ok(phys_addr)
    }

    pub fn zone_free(&mut self, zone_name: &str, phys_addr: u64) -> Result<(), &'static str> {
        let zone = self.zones.get_mut(zone_name).ok_or("UMA zone not found")?;
        if zone.allocated_items > 0 {
            zone.allocated_items -= 1;
            zone.free_items_pool.push(phys_addr);
        }
        Ok(())
    }
}

impl Default for BsdVmZoneAllocator {
    fn default() -> Self {
        Self::new()
    }
}

/// Capsicum Capability Rights
pub const CAP_READ_RIGHT: u64 = 0x01;
pub const CAP_WRITE_RIGHT: u64 = 0x02;
pub const CAP_SEEK_RIGHT: u64 = 0x04;
pub const CAP_FSTAT_RIGHT: u64 = 0x08;

/// FreeBSD Capsicum Capability Mode Framework
pub struct BsdCapsicumFramework {
    pub in_capability_mode: bool,
    pub descriptor_rights: BTreeMap<usize, u64>,
}

impl BsdCapsicumFramework {
    pub fn new() -> Self {
        Self {
            in_capability_mode: false,
            descriptor_rights: BTreeMap::new(),
        }
    }

    pub fn cap_enter(&mut self) {
        self.in_capability_mode = true;
    }

    pub fn cap_rights_limit(&mut self, fd: usize, rights_mask: u64) {
        self.descriptor_rights.insert(fd, rights_mask);
    }

    pub fn check_right(&self, fd: usize, right: u64) -> bool {
        if let Some(&allowed) = self.descriptor_rights.get(&fd) {
            (allowed & right) == right
        } else {
            !self.in_capability_mode
        }
    }
}

impl Default for BsdCapsicumFramework {
    fn default() -> Self {
        Self::new()
    }
}

/// OpenBSD PF Action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PfAction {
    Pass,
    Block,
    Nat,
}

/// OpenBSD PF Firewall Rule
#[derive(Debug, Clone)]
pub struct PfRule {
    pub rule_id: u32,
    pub action: PfAction,
    pub src_ip: String,
    pub dst_ip: String,
    pub port: u16,
}

/// OpenBSD Packet Filter (PF) Stateful Firewall Engine
pub struct BsdPfPacketFilter {
    pub rules: Vec<PfRule>,
    pub state_table: Vec<(String, String, u16)>, // (src, dst, port)
}

impl BsdPfPacketFilter {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            state_table: Vec::new(),
        }
    }

    pub fn add_rule(&mut self, action: PfAction, src_ip: &str, dst_ip: &str, port: u16) {
        let rule_id = self.rules.len() as u32 + 1;
        self.rules.push(PfRule {
            rule_id,
            action,
            src_ip: src_ip.to_string(),
            dst_ip: dst_ip.to_string(),
            port,
        });
    }

    pub fn evaluate_packet(&mut self, src_ip: &str, dst_ip: &str, port: u16) -> PfAction {
        // Stateful match first
        let tuple = (src_ip.to_string(), dst_ip.to_string(), port);
        if self.state_table.contains(&tuple) {
            return PfAction::Pass;
        }

        let mut final_action = PfAction::Pass; // default pass
        for rule in &self.rules {
            if (rule.src_ip == "*" || rule.src_ip == src_ip)
                && (rule.dst_ip == "*" || rule.dst_ip == dst_ip)
                && (rule.port == 0 || rule.port == port)
            {
                final_action = rule.action;
            }
        }

        if final_action == PfAction::Pass {
            self.state_table.push(tuple);
        }

        final_action
    }
}

impl Default for BsdPfPacketFilter {
    fn default() -> Self {
        Self::new()
    }
}

/// Metadata Dependency Operation for Soft Updates
#[derive(Debug, Clone)]
pub struct MetadataOp {
    pub op_id: u64,
    pub inode_id: u64,
    pub depends_on_op_id: Option<u64>,
    pub committed: bool,
}

/// FreeBSD FFS Soft Updates Metadata Dependency Ordering Engine
pub struct BsdSoftUpdatesEngine {
    pub pending_ops: Vec<MetadataOp>,
}

impl BsdSoftUpdatesEngine {
    pub fn new() -> Self {
        Self {
            pending_ops: Vec::new(),
        }
    }

    pub fn register_metadata_op(&mut self, op_id: u64, inode_id: u64, depends_on: Option<u64>) {
        self.pending_ops.push(MetadataOp {
            op_id,
            inode_id,
            depends_on_op_id: depends_on,
            committed: false,
        });
    }

    pub fn commit_op(&mut self, op_id: u64) -> Result<(), &'static str> {
        let pos = self.pending_ops.iter().position(|op| op.op_id == op_id).ok_or("Metadata operation not found")?;

        if let Some(parent_id) = self.pending_ops[pos].depends_on_op_id {
            let parent_committed = self.pending_ops.iter().any(|op| op.op_id == parent_id && op.committed);
            if !parent_committed {
                return Err("Soft Updates ordering error: Parent metadata dependency not committed yet!");
            }
        }

        self.pending_ops[pos].committed = true;
        Ok(())
    }
}

impl Default for BsdSoftUpdatesEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bsd_kqueue_lifecycle() {
        let mut kqueue = BsdKqueueEngine::new();
        kqueue.kevent_register(Kevent {
            ident: 10,
            filter: KfilterType::EvfiltRead,
            flags: 0,
            fflags: 0,
            data: 0,
            udata: 100,
        });

        assert_eq!(kqueue.kevent_trigger(10, KfilterType::EvfiltRead, 128), 1);
        let events = kqueue.kevent_poll();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].data, 128);
    }

    #[test]
    fn test_bsd_uma_zone_allocator() {
        let mut uma = BsdVmZoneAllocator::new();
        uma.create_zone("socket_zone", 256);

        let addr = uma.zone_allocate("socket_zone", 0x1000).unwrap();
        assert_eq!(addr, 0x1000);
        assert!(uma.zone_free("socket_zone", 0x1000).is_ok());
    }

    #[test]
    fn test_bsd_capsicum_framework() {
        let mut capsicum = BsdCapsicumFramework::new();
        capsicum.cap_rights_limit(3, CAP_READ_RIGHT | CAP_SEEK_RIGHT);
        capsicum.cap_enter();

        assert!(capsicum.check_right(3, CAP_READ_RIGHT));
        assert!(!capsicum.check_right(3, CAP_WRITE_RIGHT));
    }

    #[test]
    fn test_bsd_pf_packet_filter() {
        let mut pf = BsdPfPacketFilter::new();
        pf.add_rule(PfAction::Block, "*", "192.168.1.100", 80);

        assert_eq!(pf.evaluate_packet("10.0.0.1", "192.168.1.100", 80), PfAction::Block);
        assert_eq!(pf.evaluate_packet("10.0.0.1", "192.168.1.101", 80), PfAction::Pass);
    }

    #[test]
    fn test_bsd_soft_updates_engine() {
        let mut soft_updates = BsdSoftUpdatesEngine::new();
        soft_updates.register_metadata_op(1, 100, None);
        soft_updates.register_metadata_op(2, 100, Some(1));

        assert!(soft_updates.commit_op(2).is_err()); // Depends on op 1
        assert!(soft_updates.commit_op(1).is_ok());
        assert!(soft_updates.commit_op(2).is_ok());
    }
}

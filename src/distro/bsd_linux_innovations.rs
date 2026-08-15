// SPDX-License-Identifier: MIT
// SigmaOS BSD & Linux Innovations Subsystem
// Inspired by OpenBSD/FreeBSD PF, DragonFly BSD HAMMER2, Void Linux runit, and Parrot OS AnonSurf

#[cfg(not(target_os = "none"))]
use std::vec::Vec;

#[cfg(target_os = "none")]
extern crate alloc;

#[cfg(target_os = "none")]
use alloc::vec::Vec;

// ============================================================================
// 1. OpenBSD / FreeBSD PF (Packet Filter) Stateful Firewall
// ============================================================================

/// Packet Filter Action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PfRuleAction {
    Pass,
    Block,
    Queue,
}

/// Active PF State Table Entry
#[derive(Debug, Clone)]
pub struct PfStateEntry {
    pub src_ip: [u8; 4],
    pub dst_ip: [u8; 4],
    pub src_port: u16,
    pub dst_port: u16,
    pub packets_matched: u64,
    pub bytes_matched: u64,
}

/// OpenBSD/FreeBSD PF Stateful Firewall
#[derive(Debug)]
pub struct BsdStatefulPacketFilter {
    default_action: PfRuleAction,
    state_table: Vec<PfStateEntry>,
}

impl BsdStatefulPacketFilter {
    pub fn new(default_action: PfRuleAction) -> Self {
        Self {
            default_action,
            state_table: Vec::new(),
        }
    }

    pub fn evaluate_packet(&mut self, src_ip: [u8; 4], dst_ip: [u8; 4], src_port: u16, dst_port: u16, payload_len: usize) -> PfRuleAction {
        // Check existing state table for stateful match
        if let Some(entry) = self.state_table.iter_mut().find(|e| {
            (e.src_ip == src_ip && e.dst_ip == dst_ip && e.src_port == src_port && e.dst_port == dst_port)
                || (e.src_ip == dst_ip && e.dst_ip == src_ip && e.src_port == dst_port && e.dst_port == src_port)
        }) {
            entry.packets_matched += 1;
            entry.bytes_matched += payload_len as u64;
            return PfRuleAction::Pass;
        }

        // Apply default rule action & create new state if Pass
        if self.default_action == PfRuleAction::Pass {
            self.state_table.push(PfStateEntry {
                src_ip,
                dst_ip,
                src_port,
                dst_port,
                packets_matched: 1,
                bytes_matched: payload_len as u64,
            });
        }

        self.default_action
    }

    pub fn get_active_state_count(&self) -> usize {
        self.state_table.len()
    }
}

// ============================================================================
// 2. DragonFly BSD HAMMER2 File System Snapshotter
// ============================================================================

/// HAMMER2 Snapshot Record
#[derive(Debug, Clone)]
pub struct Hammer2Snapshot {
    pub snapshot_id: u64,
    pub trans_id: u64,
    pub label: &'static str,
    pub checksum: u64,
}

/// DragonFly BSD HAMMER2 Snapshot Engine
#[derive(Debug)]
pub struct DragonFlyHammerFs {
    next_trans_id: u64,
    snapshots: Vec<Hammer2Snapshot>,
}

impl DragonFlyHammerFs {
    pub fn new() -> Self {
        Self {
            next_trans_id: 1000,
            snapshots: Vec::new(),
        }
    }

    pub fn create_snapshot(&mut self, label: &'static str, root_data: &[u8]) -> u64 {
        self.next_trans_id += 1;
        let mut checksum: u64 = 0xcbf29ce484222325;
        for &byte in root_data {
            checksum ^= u64::from(byte);
            checksum = checksum.wrapping_mul(0x100000001b3);
        }

        let snap_id = (self.next_trans_id << 16) ^ checksum;
        self.snapshots.push(Hammer2Snapshot {
            snapshot_id: snap_id,
            trans_id: self.next_trans_id,
            label,
            checksum,
        });

        snap_id
    }

    pub fn get_snapshot(&self, label: &str) -> Option<&Hammer2Snapshot> {
        self.snapshots.iter().find(|s| s.label == label)
    }

    pub fn get_snapshot_count(&self) -> usize {
        self.snapshots.len()
    }
}

impl Default for DragonFlyHammerFs {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 3. Void Linux runit Lightweight Service Supervisor
// ============================================================================

/// Service state in runit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunitServiceState {
    Down,
    Starting,
    Up,
    Stopping,
}

/// Runit Supervised Service
#[derive(Debug, Clone)]
pub struct RunitService {
    pub name: &'static str,
    pub pid: Option<u32>,
    pub state: RunitServiceState,
    pub auto_respawn: bool,
}

/// Void Linux runit Manager
#[derive(Debug)]
pub struct VoidRunitManager {
    services: Vec<RunitService>,
    respawn_triggers_count: usize,
}

impl VoidRunitManager {
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
            respawn_triggers_count: 0,
        }
    }

    pub fn register_service(&mut self, name: &'static str, auto_respawn: bool) {
        if !self.services.iter().any(|s| s.name == name) {
            self.services.push(RunitService {
                name,
                pid: None,
                state: RunitServiceState::Down,
                auto_respawn,
            });
        }
    }

    pub fn start_service(&mut self, name: &str, pid: u32) -> Result<(), &'static str> {
        if let Some(svc) = self.services.iter_mut().find(|s| s.name == name) {
            svc.pid = Some(pid);
            svc.state = RunitServiceState::Up;
            return Ok(());
        }
        Err("Service not registered in runit directory")
    }

    pub fn stop_service(&mut self, name: &str) -> Result<(), &'static str> {
        if let Some(svc) = self.services.iter_mut().find(|s| s.name == name) {
            svc.pid = None;
            svc.state = RunitServiceState::Down;
            return Ok(());
        }
        Err("Service not registered in runit directory")
    }

    pub fn supervise_all(&mut self) -> usize {
        let mut restarts = 0;
        for svc in &mut self.services {
            if svc.state == RunitServiceState::Down && svc.auto_respawn {
                svc.state = RunitServiceState::Starting;
                svc.pid = Some(2000 + restarts as u32);
                svc.state = RunitServiceState::Up;
                restarts += 1;
            }
        }
        self.respawn_triggers_count += restarts;
        restarts
    }

    pub fn get_service_state(&self, name: &str) -> RunitServiceState {
        self.services
            .iter()
            .find(|s| s.name == name)
            .map(|s| s.state)
            .unwrap_or(RunitServiceState::Down)
    }
}

impl Default for VoidRunitManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 4. Sovereign Anonymizer & Memory Scrubber (Parrot OS inspired)
// ============================================================================

/// Parrot OS inspired AnonSurf Network & Memory Scrubber
#[derive(Debug)]
pub struct SovereignAnonScrubber {
    anon_routing_enabled: bool,
    scrubbed_bytes_total: usize,
}

impl SovereignAnonScrubber {
    pub fn new() -> Self {
        Self {
            anon_routing_enabled: false,
            scrubbed_bytes_total: 0,
        }
    }

    pub fn enable_anon_routing(&mut self) {
        self.anon_routing_enabled = true;
    }

    pub fn disable_anon_routing(&mut self) {
        self.anon_routing_enabled = false;
    }

    pub fn is_anon_enabled(&self) -> bool {
        self.anon_routing_enabled
    }

    /// Zeroes out sensitive RAM buffers upon execution termination or panic
    pub fn scrub_ram_buffer(&mut self, buffer: &mut [u8]) -> usize {
        let len = buffer.len();
        for byte in buffer.iter_mut() {
            *byte = 0;
        }
        self.scrubbed_bytes_total += len;
        len
    }

    pub fn get_total_scrubbed_bytes(&self) -> usize {
        self.scrubbed_bytes_total
    }
}

impl Default for SovereignAnonScrubber {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bsd_stateful_packet_filter() {
        let mut pf = BsdStatefulPacketFilter::new(PfRuleAction::Pass);

        let src = [192, 168, 1, 10];
        let dst = [10, 0, 0, 1];

        // First packet creates state entry
        let act1 = pf.evaluate_packet(src, dst, 4433, 80, 100);
        assert_eq!(act1, PfRuleAction::Pass);
        assert_eq!(pf.get_active_state_count(), 1);

        // Reverse packet matches state table
        let act2 = pf.evaluate_packet(dst, src, 80, 4433, 500);
        assert_eq!(act2, PfRuleAction::Pass);
        assert_eq!(pf.get_active_state_count(), 1); // State re-used
    }

    #[test]
    fn test_dragonfly_hammer2_fs() {
        let mut hammer = DragonFlyHammerFs::new();
        let root_bytes = b"ROOT_DIRECTORY_INODE_DATA_TREE";

        let snap_id = hammer.create_snapshot("@snap_v1", root_bytes);
        assert!(snap_id > 0);
        assert_eq!(hammer.get_snapshot_count(), 1);

        let snap = hammer.get_snapshot("@snap_v1").unwrap();
        assert_eq!(snap.label, "@snap_v1");
        assert!(snap.checksum > 0);
    }

    #[test]
    fn test_void_runit_manager() {
        let mut runit = VoidRunitManager::new();

        runit.register_service("dhcpcd", true);
        runit.register_service("sshd", false);

        assert_eq!(runit.get_service_state("dhcpcd"), RunitServiceState::Down);

        // Start sshd manually
        assert!(runit.start_service("sshd", 1042).is_ok());
        assert_eq!(runit.get_service_state("sshd"), RunitServiceState::Up);

        // Supervise brings up auto_respawn dhcpcd
        let restarted = runit.supervise_all();
        assert_eq!(restarted, 1);
        assert_eq!(runit.get_service_state("dhcpcd"), RunitServiceState::Up);
    }

    #[test]
    fn test_sovereign_anon_scrubber() {
        let mut scrubber = SovereignAnonScrubber::new();
        assert!(!scrubber.is_anon_enabled());

        scrubber.enable_anon_routing();
        assert!(scrubber.is_anon_enabled());

        let mut secret_ram = [0xFFu8; 64];
        let scrubbed = scrubber.scrub_ram_buffer(&mut secret_ram);
        assert_eq!(scrubbed, 64);
        assert_eq!(secret_ram, [0u8; 64]);
        assert_eq!(scrubber.get_total_scrubbed_bytes(), 64);
    }
}

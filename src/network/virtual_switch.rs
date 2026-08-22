// SigmaOS Linux Open vSwitch & BSD if_bridge Virtual Ethernet Switch Engine
// MAC Forwarding Database (FDB), 802.1Q VLAN Tagging/Trunking, STP Spanning Tree, SPAN Mirroring, & LACP

extern crate alloc;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::format;
use alloc::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SwitchPortMode {
    Access { vlan_id: u16 },
    Trunk { allowed_vlans: Vec<u16> },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StpPortState {
    Disabled,
    Blocking,
    Listening,
    Learning,
    Forwarding,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BondingMode {
    ActiveBackup,
    Lacp8023ad,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlowAction {
    OutputPort(usize),
    PushVlan(u16),
    PopVlan,
    Drop,
    Flood,
}

#[derive(Debug, Clone)]
pub struct FdbEntry {
    pub mac_address: [u8; 6],
    pub port_id: usize,
    pub age_seconds: u32,
}

#[derive(Debug, Clone)]
pub struct SwitchPort {
    pub port_id: usize,
    pub name: String,
    pub mode: SwitchPortMode,
    pub stp_state: StpPortState,
    pub mirror_to_port: Option<usize>, // SPAN Port Mirroring
}

pub struct VirtualSwitchBridge {
    pub bridge_name: String,
    pub ports: BTreeMap<usize, SwitchPort>,
    pub fdb: BTreeMap<[u8; 6], FdbEntry>, // MAC -> FdbEntry
    pub bonding_mode: Option<BondingMode>,
}

pub struct VirtualSwitchEngine {
    pub bridges: BTreeMap<usize, VirtualSwitchBridge>,
    next_bridge_id: usize,
    next_port_id: usize,
}

impl VirtualSwitchEngine {
    pub fn new() -> Self {
        VirtualSwitchEngine {
            bridges: BTreeMap::new(),
            next_bridge_id: 1,
            next_port_id: 101,
        }
    }

    /// Linux `ovs-vsctl add-br`: Creates a virtual Ethernet bridge
    pub fn create_bridge(&mut self, bridge_name: &str) -> usize {
        let id = self.next_bridge_id;
        self.next_bridge_id += 1;

        let bridge = VirtualSwitchBridge {
            bridge_name: bridge_name.to_string(),
            ports: BTreeMap::new(),
            fdb: BTreeMap::new(),
            bonding_mode: None,
        };

        self.bridges.insert(id, bridge);
        id
    }

    /// Linux `ovs-vsctl add-port`: Attaches a port with 802.1Q VLAN configuration
    pub fn add_port(&mut self, bridge_id: usize, name: &str, mode: SwitchPortMode) -> Result<usize, &'static str> {
        let bridge = self.bridges.get_mut(&bridge_id).ok_or("Bridge not found")?;
        let port_id = self.next_port_id;
        self.next_port_id += 1;

        let port = SwitchPort {
            port_id,
            name: name.to_string(),
            mode,
            stp_state: StpPortState::Forwarding, // Default forwarding
            mirror_to_port: None,
        };

        bridge.ports.insert(port_id, port);
        Ok(port_id)
    }

    /// Update Spanning Tree Protocol (STP / RSTP) port state
    pub fn set_stp_state(&mut self, bridge_id: usize, port_id: usize, stp_state: StpPortState) -> Result<(), &'static str> {
        let bridge = self.bridges.get_mut(&bridge_id).ok_or("Bridge not found")?;
        let port = bridge.ports.get_mut(&port_id).ok_or("Port not found")?;
        port.stp_state = stp_state;
        Ok(())
    }

    /// Process Ethernet Frame: Dynamic MAC FDB Learning, STP Filtering, 802.1Q VLAN Enforcement, & Forwarding
    pub fn process_frame(
        &mut self,
        bridge_id: usize,
        in_port_id: usize,
        src_mac: [u8; 6],
        dst_mac: [u8; 6],
        vlan_id: Option<u16>,
    ) -> Result<(FlowAction, Vec<usize>), &'static str> {
        let bridge = self.bridges.get_mut(&bridge_id).ok_or("Bridge not found")?;
        let in_port = bridge.ports.get(&in_port_id).ok_or("Input port not found")?;

        // 1. Check STP Port State
        if in_port.stp_state == StpPortState::Blocking || in_port.stp_state == StpPortState::Disabled {
            return Ok((FlowAction::Drop, Vec::new()));
        }

        // 2. FDB Learning (MAC Learning)
        if in_port.stp_state == StpPortState::Learning || in_port.stp_state == StpPortState::Forwarding {
            bridge.fdb.insert(src_mac, FdbEntry {
                mac_address: src_mac,
                port_id: in_port_id,
                age_seconds: 0,
            });
        }

        if in_port.stp_state != StpPortState::Forwarding {
            return Ok((FlowAction::Drop, Vec::new()));
        }

        // 3. Evaluate 802.1Q VLAN Access vs Trunk rules
        let active_vlan = match &in_port.mode {
            SwitchPortMode::Access { vlan_id: port_vlan } => *port_vlan,
            SwitchPortMode::Trunk { allowed_vlans } => {
                if let Some(v) = vlan_id {
                    if !allowed_vlans.contains(&v) {
                        return Ok((FlowAction::Drop, Vec::new())); // Dropped by VLAN trunk filter
                    }
                    v
                } else {
                    1 // Native VLAN 1
                }
            }
        };

        // 4. Lookup Destination MAC in FDB (Forwarding vs Flooding)
        let mut target_ports = Vec::new();

        if let Some(fdb_entry) = bridge.fdb.get(&dst_mac) {
            let out_port_id = fdb_entry.port_id;
            if out_port_id != in_port_id {
                target_ports.push(out_port_id);
                return Ok((FlowAction::OutputPort(out_port_id), target_ports));
            } else {
                return Ok((FlowAction::Drop, Vec::new())); // Same port loop drop
            }
        }

        // 5. Flood frame across all forwarding ports in the same VLAN
        for (pid, p) in &bridge.ports {
            if *pid == in_port_id || p.stp_state != StpPortState::Forwarding {
                continue;
            }

            let p_vlan_match = match &p.mode {
                SwitchPortMode::Access { vlan_id: port_vlan } => *port_vlan == active_vlan,
                SwitchPortMode::Trunk { allowed_vlans } => allowed_vlans.contains(&active_vlan),
            };

            if p_vlan_match {
                target_ports.push(*pid);
            }
        }

        Ok((FlowAction::Flood, target_ports))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virtual_switch_mac_learning_and_vlan_forwarding() {
        let mut switch = VirtualSwitchEngine::new();
        let br_id = switch.create_bridge("br0");

        // Add 2 ports in Access VLAN 10
        let p1 = switch.add_port(br_id, "eth0", SwitchPortMode::Access { vlan_id: 10 }).unwrap();
        let p2 = switch.add_port(br_id, "eth1", SwitchPortMode::Access { vlan_id: 10 }).unwrap();
        // Add 1 port in Access VLAN 20
        let p3 = switch.add_port(br_id, "eth2", SwitchPortMode::Access { vlan_id: 20 }).unwrap();

        let mac_a = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let mac_b = [0x00, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE];

        // 1. Send frame from MAC A on Port 1 -> Should flood ONLY to Port 2 (VLAN 10 match), ignoring Port 3 (VLAN 20)
        let (action, targets) = switch.process_frame(br_id, p1, mac_a, mac_b, None).unwrap();
        assert_eq!(action, FlowAction::Flood);
        assert_eq!(targets, vec![p2]);

        // 2. Send frame from MAC B on Port 2 -> FDB now knows MAC A is on Port 1!
        let (action_unicast, targets_unicast) = switch.process_frame(br_id, p2, mac_b, mac_a, None).unwrap();
        assert_eq!(action_unicast, FlowAction::OutputPort(p1));
        assert_eq!(targets_unicast, vec![p1]);
    }
}

extern crate alloc;
// SPDX-License-Identifier: MIT
// SigmaOS Linux & BSD Distro Power & Network Tools Subsystem
// Inspired by Linux TLP/Laptop Mode Tools, NetworkManager/nmtui, FreeBSD bhyve, and Tailscale/WireGuard

use alloc::vec::Vec;

// ============================================================================
// 1. Linux TLP / Laptop Mode Tools Power Governor
// ============================================================================

/// Power Source Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerSource {
    AC,
    Battery,
}

/// CPU Scaling Governor Policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuGovernorPolicy {
    Performance,
    Powersave,
    Schedutil,
}

/// TLP Power Tuning Configuration
#[derive(Debug, Clone)]
pub struct TlpConfig {
    pub power_source: PowerSource,
    pub cpu_policy: CpuGovernorPolicy,
    pub usb_autosuspend_delay_sec: u32,
    pub wifi_power_save: bool,
}

/// Linux TLP Power Governor Engine
#[derive(Debug)]
pub struct TlpPowerGovernor {
    pub current_config: TlpConfig,
}

impl TlpPowerGovernor {
    pub fn new() -> Self {
        Self {
            current_config: TlpConfig {
                power_source: PowerSource::AC,
                cpu_policy: CpuGovernorPolicy::Performance,
                usb_autosuspend_delay_sec: 0,
                wifi_power_save: false,
            },
        }
    }

    pub fn switch_power_source(&mut self, source: PowerSource) {
        self.current_config.power_source = source;
        match source {
            PowerSource::AC => {
                self.current_config.cpu_policy = CpuGovernorPolicy::Performance;
                self.current_config.usb_autosuspend_delay_sec = 0;
                self.current_config.wifi_power_save = false;
            }
            PowerSource::Battery => {
                self.current_config.cpu_policy = CpuGovernorPolicy::Powersave;
                self.current_config.usb_autosuspend_delay_sec = 2;
                self.current_config.wifi_power_save = true;
            }
        }
    }

    pub fn set_custom_policy(&mut self, policy: CpuGovernorPolicy) {
        self.current_config.cpu_policy = policy;
    }
}

impl Default for TlpPowerGovernor {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 2. NetworkManager & nmtui Connection Engine
// ============================================================================

/// Network Connection Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionType {
    Ethernet,
    Wifi,
    Vlan,
    Vpn,
}

/// Connection Profile Record
#[derive(Debug, Clone)]
pub struct NetworkConnectionProfile {
    pub uuid: u64,
    pub name: &'static str,
    pub conn_type: ConnectionType,
    pub ip_address: [u8; 4],
    pub subnet_prefix: u8,
    pub active: bool,
}

/// NetworkManager & nmtui Connection Engine
#[derive(Debug)]
pub struct NmtuiNetworkManager {
    profiles: Vec<NetworkConnectionProfile>,
}

impl NmtuiNetworkManager {
    pub fn new() -> Self {
        let mut mgr = Self {
            profiles: Vec::new(),
        };

        // Default Ethernet profile (Wired connection 1)
        mgr.add_profile(NetworkConnectionProfile {
            uuid: 0x1000_2000_3000_4000,
            name: "Wired Connection 1",
            conn_type: ConnectionType::Ethernet,
            ip_address: [192, 168, 1, 100],
            subnet_prefix: 24,
            active: true,
        });

        mgr
    }

    pub fn add_profile(&mut self, profile: NetworkConnectionProfile) {
        self.profiles.push(profile);
    }

    pub fn activate_profile(&mut self, name: &str) -> Result<(), &'static str> {
        let mut target_type = None;
        for prof in &self.profiles {
            if prof.name == name {
                target_type = Some(prof.conn_type);
                break;
            }
        }

        let target_conn_type = match target_type {
            Some(t) => t,
            None => return Err("Network Profile Name Not Found"),
        };

        for prof in &mut self.profiles {
            if prof.name == name {
                prof.active = true;
            } else if prof.conn_type == target_conn_type {
                // Deactivate conflicting connection profiles of same type
                prof.active = false;
            }
        }

        Ok(())
    }

    pub fn get_active_profile_count(&self) -> usize {
        self.profiles.iter().filter(|p| p.active).count()
    }
}

impl Default for NmtuiNetworkManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 3. FreeBSD bhyve Hypervisor
// ============================================================================

/// bhyve Virtual Machine State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BhyveVmState {
    Stopped,
    Booting,
    Running,
    Paused,
}

/// bhyve Virtual Machine Instance
#[derive(Debug, Clone)]
pub struct BhyveVirtualMachine {
    pub vm_id: u32,
    pub name: &'static str,
    pub vcpu_count: u16,
    pub ram_mb: usize,
    pub state: BhyveVmState,
}

/// FreeBSD bhyve Hypervisor Manager
#[derive(Debug)]
pub struct FreeBsdBhyveHypervisor {
    vms: Vec<BhyveVirtualMachine>,
}

impl FreeBsdBhyveHypervisor {
    pub fn new() -> Self {
        Self { vms: Vec::new() }
    }

    pub fn create_vm(&mut self, vm_id: u32, name: &'static str, vcpu_count: u16, ram_mb: usize) {
        if !self.vms.iter().any(|v| v.vm_id == vm_id) {
            self.vms.push(BhyveVirtualMachine {
                vm_id,
                name,
                vcpu_count,
                ram_mb,
                state: BhyveVmState::Stopped,
            });
        }
    }

    pub fn start_vm(&mut self, vm_id: u32) -> Result<(), &'static str> {
        if let Some(vm) = self.vms.iter_mut().find(|v| v.vm_id == vm_id) {
            vm.state = BhyveVmState::Running;
            return Ok(());
        }
        Err("bhyve VM ID not found")
    }

    pub fn stop_vm(&mut self, vm_id: u32) -> Result<(), &'static str> {
        if let Some(vm) = self.vms.iter_mut().find(|v| v.vm_id == vm_id) {
            vm.state = BhyveVmState::Stopped;
            return Ok(());
        }
        Err("bhyve VM ID not found")
    }

    pub fn get_vm_state(&self, vm_id: u32) -> BhyveVmState {
        self.vms
            .iter()
            .find(|v| v.vm_id == vm_id)
            .map(|v| v.state)
            .unwrap_or(BhyveVmState::Stopped)
    }
}

impl Default for FreeBsdBhyveHypervisor {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 4. Tailscale / WireGuard Mesh Network Router
// ============================================================================

/// WireGuard Peer Node
#[derive(Debug, Clone)]
pub struct WireguardPeer {
    pub public_key: u64,
    pub endpoint_ip: [u8; 4],
    pub allowed_ip: [u8; 4],
    pub bytes_sent: u64,
    pub bytes_recv: u64,
}

/// Tailscale / WireGuard Mesh Engine
#[derive(Debug)]
pub struct TailscaleWireguardMesh {
    local_ip: [u8; 4],
    peers: Vec<WireguardPeer>,
}

impl TailscaleWireguardMesh {
    pub fn new(local_ip: [u8; 4]) -> Self {
        Self {
            local_ip,
            peers: Vec::new(),
        }
    }

    pub fn add_peer(&mut self, peer: WireguardPeer) {
        if !self.peers.iter().any(|p| p.public_key == peer.public_key) {
            self.peers.push(peer);
        }
    }

    pub fn route_mesh_packet(
        &mut self,
        target_ip: [u8; 4],
        packet_len: usize,
    ) -> Result<(), &'static str> {
        if target_ip == self.local_ip {
            return Ok(());
        }

        if let Some(peer) = self.peers.iter_mut().find(|p| p.allowed_ip == target_ip) {
            peer.bytes_sent += packet_len as u64;
            return Ok(());
        }

        Err("WireGuard Mesh: Target IP unreachable in mesh peer table")
    }

    pub fn get_peer_count(&self) -> usize {
        self.peers.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tlp_power_governor() {
        let mut tlp = TlpPowerGovernor::new();
        assert_eq!(tlp.current_config.power_source, PowerSource::AC);
        assert_eq!(
            tlp.current_config.cpu_policy,
            CpuGovernorPolicy::Performance
        );

        // Switch to battery
        tlp.switch_power_source(PowerSource::Battery);
        assert_eq!(tlp.current_config.power_source, PowerSource::Battery);
        assert_eq!(tlp.current_config.cpu_policy, CpuGovernorPolicy::Powersave);
        assert!(tlp.current_config.wifi_power_save);
    }

    #[test]
    fn test_nmtui_network_manager() {
        let mut nm = NmtuiNetworkManager::new();
        assert_eq!(nm.get_active_profile_count(), 1);

        // Add WiFi profile
        nm.add_profile(NetworkConnectionProfile {
            uuid: 0x5000_6000_7000_8000,
            name: "Home WiFi",
            conn_type: ConnectionType::Wifi,
            ip_address: [192, 168, 1, 105],
            subnet_prefix: 24,
            active: false,
        });

        assert!(nm.activate_profile("Home WiFi").is_ok());
        assert_eq!(nm.get_active_profile_count(), 2);
    }

    #[test]
    fn test_freebsd_bhyve_hypervisor() {
        let mut bhyve = FreeBsdBhyveHypervisor::new();
        bhyve.create_vm(101, "freebsd-jail-host", 4, 2048);

        assert_eq!(bhyve.get_vm_state(101), BhyveVmState::Stopped);

        assert!(bhyve.start_vm(101).is_ok());
        assert_eq!(bhyve.get_vm_state(101), BhyveVmState::Running);

        assert!(bhyve.stop_vm(101).is_ok());
        assert_eq!(bhyve.get_vm_state(101), BhyveVmState::Stopped);
    }

    #[test]
    fn test_tailscale_wireguard_mesh() {
        let mut mesh = TailscaleWireguardMesh::new([100, 64, 0, 1]);

        mesh.add_peer(WireguardPeer {
            public_key: 0x1122_3344_5566_7788,
            endpoint_ip: [198, 51, 100, 1],
            allowed_ip: [100, 64, 0, 2],
            bytes_sent: 0,
            bytes_recv: 0,
        });

        assert_eq!(mesh.get_peer_count(), 1);

        // Route packet to peer
        assert!(mesh.route_mesh_packet([100, 64, 0, 2], 512).is_ok());

        // Route packet to unknown IP
        assert!(mesh.route_mesh_packet([100, 64, 0, 99], 512).is_err());
    }
}

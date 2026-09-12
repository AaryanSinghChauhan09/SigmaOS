//! Whonix Dual-VM Isolated Routing & Amnesic Privacy Synthesis Engine
//!
//! Inspired by Whonix Gateway/Workstation architecture:
//! - Complete separation of Gateway (Tor routing, stream isolation, DNS leak prevention)
//!   and Workstation (untrusted apps, leak prevention, forced gateway routing).
//! - Tor stream isolation via SOCKS5 proxy ports and circuit isolation keys.
//! - Transparent TransPort firewall redirection for non-SOCKS network traffic.
//! - Amnesic RAM session memory scrubbing upon session termination or panic.

#![allow(dead_code)]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Stream Isolation Tag used by Tor to map distinct app connections to separate circuits
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StreamIsolationTag {
    pub app_id: String,
    pub user_session_id: u64,
    pub isolation_key: String,
}

impl StreamIsolationTag {
    pub fn new(app_id: &str, user_session_id: u64, isolation_key: &str) -> Self {
        Self {
            app_id: app_id.to_string(),
            user_session_id,
            isolation_key: isolation_key.to_string(),
        }
    }
}

/// Whonix Gateway Configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhonixGatewayConfig {
    pub socks5_proxy_port: u16,
    pub transparent_dns_port: u16,
    pub transparent_transport_port: u16,
    pub control_port: u16,
    pub enforce_stream_isolation: bool,
    pub block_clearnet_leaks: bool,
}

impl Default for WhonixGatewayConfig {
    fn default() -> Self {
        Self {
            socks5_proxy_port: 9050,
            transparent_dns_port: 5353,
            transparent_transport_port: 9040,
            control_port: 9051,
            enforce_stream_isolation: true,
            block_clearnet_leaks: true,
        }
    }
}

/// Whonix Workstation Isolation Profile
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhonixWorkstationIsolation {
    pub workstation_id: String,
    pub virtual_ip: String,
    pub gateway_ip: String,
    pub amnesic_ram_scrub_on_shutdown: bool,
    pub active_stream_tags: Vec<StreamIsolationTag>,
}

impl WhonixWorkstationIsolation {
    pub fn new(workstation_id: &str, virtual_ip: &str, gateway_ip: &str) -> Self {
        Self {
            workstation_id: workstation_id.to_string(),
            virtual_ip: virtual_ip.to_string(),
            gateway_ip: gateway_ip.to_string(),
            amnesic_ram_scrub_on_shutdown: true,
            active_stream_tags: Vec::new(),
        }
    }
}

/// Active Tor Circuit Mapping
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TorCircuitRecord {
    pub circuit_id: u64,
    pub isolation_tag: StreamIsolationTag,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub is_active: bool,
}

/// Whonix Gateway Routing Engine
#[derive(Debug, Clone)]
pub struct WhonixGatewayRoutingEngine {
    pub config: WhonixGatewayConfig,
    pub circuits: BTreeMap<u64, TorCircuitRecord>,
    pub workstations: BTreeMap<String, WhonixWorkstationIsolation>,
    pub leaked_packet_attempts: u64,
    pub next_circuit_id: u64,
}

impl WhonixGatewayRoutingEngine {
    pub fn new(config: WhonixGatewayConfig) -> Self {
        Self {
            config,
            circuits: BTreeMap::new(),
            workstations: BTreeMap::new(),
            leaked_packet_attempts: 0,
            next_circuit_id: 1000,
        }
    }

    pub fn register_workstation(&mut self, ws: WhonixWorkstationIsolation) {
        self.workstations.insert(ws.workstation_id.clone(), ws);
    }

    pub fn allocate_isolated_circuit(
        &mut self,
        workstation_id: &str,
        tag: StreamIsolationTag,
    ) -> Result<u64, String> {
        let ws = self
            .workstations
            .get_mut(workstation_id)
            .ok_or_else(|| "Workstation not found".to_string())?;

        let circuit_id = self.next_circuit_id;
        self.next_circuit_id += 1;

        let record = TorCircuitRecord {
            circuit_id,
            isolation_tag: tag.clone(),
            bytes_sent: 0,
            bytes_received: 0,
            is_active: true,
        };

        self.circuits.insert(circuit_id, record);
        ws.active_stream_tags.push(tag);
        Ok(circuit_id)
    }

    pub fn route_traffic(
        &mut self,
        workstation_id: &str,
        destination: &str,
        dest_port: u16,
        payload_len: usize,
        tag: Option<StreamIsolationTag>,
    ) -> Result<(u64, String), String> {
        if !self.workstations.contains_key(workstation_id) {
            return Err("Workstation not registered in Whonix isolate".to_string());
        }

        // Check if destination is clearnet IP and clearnet leak blocking is enabled
        if self.config.block_clearnet_leaks && destination.starts_with("raw_ip:") {
            self.leaked_packet_attempts += 1;
            return Err("Blocked potential clearnet leak attempt on Workstation".to_string());
        }

        let tag = tag.unwrap_or_else(|| StreamIsolationTag::new("default_app", 1, "default_circuit"));

        // Match existing circuit or allocate new stream-isolated circuit
        let circuit_id = match self
            .circuits
            .iter()
            .find(|(_, c)| c.is_active && c.isolation_tag == tag)
            .map(|(&id, _)| id)
        {
            Some(id) => id,
            None => self.allocate_isolated_circuit(workstation_id, tag)?,
        };

        if let Some(circuit) = self.circuits.get_mut(&circuit_id) {
            circuit.bytes_sent += payload_len as u64;
        }

        let route_info = if dest_port == 53 {
            format!("Routed via Whonix DNS TransPort :{}", self.config.transparent_dns_port)
        } else if dest_port == self.config.socks5_proxy_port {
            format!("Routed via Whonix SOCKS5 Proxy :{}", self.config.socks5_proxy_port)
        } else {
            format!("Routed via Whonix TransPort :{}", self.config.transparent_transport_port)
        };

        Ok((circuit_id, route_info))
    }

    pub fn wipe_workstation_amnesic_ram(&mut self, workstation_id: &str, ram_buffer: &mut [u8]) -> Result<usize, String> {
        let ws = self
            .workstations
            .get_mut(workstation_id)
            .ok_or_else(|| "Workstation not found".to_string())?;

        if ws.amnesic_ram_scrub_on_shutdown {
            for byte in ram_buffer.iter_mut() {
                *byte = 0x00;
            }
            ws.active_stream_tags.clear();
            Ok(ram_buffer.len())
        } else {
            Ok(0)
        }
    }
}

/// Sovereign Master Suite for Whonix Privacy
#[derive(Debug, Clone)]
pub struct SovereignWhonixPrivacySuite {
    pub gateway_engine: WhonixGatewayRoutingEngine,
}

impl SovereignWhonixPrivacySuite {
    pub fn new() -> Self {
        Self {
            gateway_engine: WhonixGatewayRoutingEngine::new(WhonixGatewayConfig::default()),
        }
    }

    pub fn verify_isolation_guarantee(&mut self) -> bool {
        let ws = WhonixWorkstationIsolation::new("ws-sys-1", "10.152.152.11", "10.152.152.10");
        self.gateway_engine.register_workstation(ws);

        let tag1 = StreamIsolationTag::new("browser", 101, "session_a");
        let tag2 = StreamIsolationTag::new("git", 101, "session_b");

        let res1 = self.gateway_engine.route_traffic("ws-sys-1", "onion.tor", 80, 512, Some(tag1));
        let res2 = self.gateway_engine.route_traffic("ws-sys-1", "github.tor", 443, 1024, Some(tag2));

        if res1.is_err() || res2.is_err() {
            return false;
        }

        let (c1, _) = res1.unwrap();
        let (c2, _) = res2.unwrap();

        // Must allocate distinct stream-isolated circuits
        c1 != c2 && self.gateway_engine.circuits.len() == 2
    }
}

impl Default for SovereignWhonixPrivacySuite {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whonix_gateway_routing_and_stream_isolation() {
        let mut suite = SovereignWhonixPrivacySuite::new();
        assert!(suite.verify_isolation_guarantee());

        // Test clearnet leak blocking
        let leak_res = suite.gateway_engine.route_traffic(
            "ws-sys-1",
            "raw_ip:1.1.1.1",
            80,
            64,
            None,
        );
        assert!(leak_res.is_err());
        assert_eq!(suite.gateway_engine.leaked_packet_attempts, 1);

        // Test amnesic RAM wiping
        let mut ram = [0xFFu8; 256];
        let wiped = suite
            .gateway_engine
            .wipe_workstation_amnesic_ram("ws-sys-1", &mut ram)
            .unwrap();
        assert_eq!(wiped, 256);
        assert!(ram.iter().all(|&b| b == 0));
    }
}

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;

/// Network Discovery Protocol Type
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiscoveryProtocolType {
    MdnsDnsSd = 0,
    UpnpSsdp = 1,
    Llmnr = 2,
    Nbns = 3,
    Icmpv6Ndp = 4,
}

/// Discovered Network Service Record
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscoveredNetworkService {
    pub service_name: String,
    pub service_type: String, // e.g. "_http._tcp.local", "_smb._tcp.local", "_ipp._tcp.local"
    pub domain: String,
    pub host_name: String,
    pub ip_address: String,
    pub port: u16,
    pub txt_records: BTreeMap<String, String>,
    pub protocol: DiscoveryProtocolType,
    pub ttl_seconds: u32,
}

impl DiscoveredNetworkService {
    pub fn new(
        service_name: &str,
        service_type: &str,
        host_name: &str,
        ip_address: &str,
        port: u16,
        protocol: DiscoveryProtocolType,
    ) -> Self {
        Self {
            service_name: service_name.to_string(),
            service_type: service_type.to_string(),
            domain: "local".to_string(),
            host_name: host_name.to_string(),
            ip_address: ip_address.to_string(),
            port,
            txt_records: BTreeMap::new(),
            protocol,
            ttl_seconds: 120,
        }
    }

    pub fn add_txt_record(&mut self, key: &str, value: &str) {
        self.txt_records.insert(key.to_string(), value.to_string());
    }
}

/// Discovered Network Peer Device
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkDevicePeer {
    pub device_id: String,
    pub friendly_name: String,
    pub ip_address: String,
    pub mac_address: [u8; 6],
    pub os_type: String,
    pub services: Vec<DiscoveredNetworkService>,
}

impl NetworkDevicePeer {
    pub fn new(device_id: &str, friendly_name: &str, ip_address: &str) -> Self {
        Self {
            device_id: device_id.to_string(),
            friendly_name: friendly_name.to_string(),
            ip_address: ip_address.to_string(),
            mac_address: [0u8; 6],
            os_type: "SigmaOS".to_string(),
            services: Vec::new(),
        }
    }
}

/// SSDP Header Parser / M-SEARCH Builder
pub struct SsdpDiscoveryPacket;

impl SsdpDiscoveryPacket {
    pub fn build_msearch_request(search_target: &str) -> String {
        let mut req = String::new();
        req.push_str("M-SEARCH * HTTP/1.1\r\n");
        req.push_str("HOST: 239.255.255.250:1900\r\n");
        req.push_str("MAN: \"ssdp:discover\"\r\n");
        req.push_str("MX: 2\r\n");
        req.push_str("ST: ");
        req.push_str(search_target);
        req.push_str("\r\n\r\n");
        req
    }

    pub fn parse_notify_or_response(payload: &str) -> BTreeMap<String, String> {
        let mut headers = BTreeMap::new();
        for line in payload.lines() {
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                let key = parts[0].trim().to_uppercase();
                let val = parts[1].trim().to_string();
                headers.insert(key, val);
            }
        }
        headers
    }
}

/// IPv6 ICMPv6 Neighbor Discovery Protocol (NDP) Record
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Icmpv6NdpEntry {
    pub ipv6_address: String,
    pub mac_address: [u8; 6],
    pub is_router: bool,
    pub reachability_state: String, // "REACHABLE", "STALE", "DELAY", "PROBE"
}

/// LLMNR / NetBIOS Query Builder and Resolver
pub struct LlmnrNbnsResolver;

impl LlmnrNbnsResolver {
    pub fn build_llmnr_query(host_name: &str) -> Vec<u8> {
        let mut packet = Vec::new();
        // LLMNR Header: ID=0x1234, Flags=0x0000 (Query), QDCOUNT=1
        packet.extend_from_slice(&[
            0x12, 0x34, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]);
        // Question name labels
        for part in host_name.split('.') {
            if !part.is_empty() {
                packet.push(part.len() as u8);
                packet.extend_from_slice(part.as_bytes());
            }
        }
        packet.push(0x00); // Null label end
        packet.extend_from_slice(&[0x00, 0x01]); // Type A
        packet.extend_from_slice(&[0x00, 0x01]); // Class IN
        packet
    }
}

/// Sovereign Network Discovery Engine
pub struct SovereignNetworkDiscoveryEngine {
    pub local_services: Vec<DiscoveredNetworkService>,
    pub discovered_peers: BTreeMap<String, NetworkDevicePeer>,
    pub ndp_table: Vec<Icmpv6NdpEntry>,
    pub auto_announce_enabled: bool,
}

impl SovereignNetworkDiscoveryEngine {
    pub fn new() -> Self {
        Self {
            local_services: Vec::new(),
            discovered_peers: BTreeMap::new(),
            ndp_table: Vec::new(),
            auto_announce_enabled: true,
        }
    }

    pub fn announce_local_service(&mut self, service: DiscoveredNetworkService) {
        self.local_services.push(service);
    }

    pub fn process_ssdp_response(&mut self, ip_addr: &str, raw_payload: &str) {
        let headers = SsdpDiscoveryPacket::parse_notify_or_response(raw_payload);
        if let Some(st) = headers.get("ST").or_else(|| headers.get("NT")) {
            let location = headers.get("LOCATION").cloned().unwrap_or_default();
            let peer = self
                .discovered_peers
                .entry(ip_addr.to_string())
                .or_insert_with(|| NetworkDevicePeer::new(ip_addr, ip_addr, ip_addr));

            let mut service = DiscoveredNetworkService::new(
                "UPnP Service",
                st,
                ip_addr,
                ip_addr,
                1900,
                DiscoveryProtocolType::UpnpSsdp,
            );
            if !location.is_empty() {
                service.add_txt_record("location", &location);
            }
            peer.services.push(service);
        }
    }

    pub fn process_mdns_announcement(&mut self, service: DiscoveredNetworkService) {
        let peer_ip = service.ip_address.clone();
        let peer = self
            .discovered_peers
            .entry(peer_ip.clone())
            .or_insert_with(|| NetworkDevicePeer::new(&peer_ip, &service.host_name, &peer_ip));
        peer.services.push(service);
    }

    pub fn add_ndp_entry(&mut self, ipv6_addr: &str, mac: [u8; 6], is_router: bool) {
        self.ndp_table.push(Icmpv6NdpEntry {
            ipv6_address: ipv6_addr.to_string(),
            mac_address: mac,
            is_router,
            reachability_state: "REACHABLE".to_string(),
        });
    }

    pub fn list_active_peers(&self) -> Vec<&NetworkDevicePeer> {
        self.discovered_peers.values().collect()
    }
}

impl Default for SovereignNetworkDiscoveryEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mdns_service_and_peer_registration() {
        let mut engine = SovereignNetworkDiscoveryEngine::new();
        let mut svc = DiscoveredNetworkService::new(
            "SigmaOS Web Dashboard",
            "_http._tcp.local",
            "sigma-node-1.local",
            "192.168.1.50",
            8080,
            DiscoveryProtocolType::MdnsDnsSd,
        );
        svc.add_txt_record("path", "/api/v1");

        engine.process_mdns_announcement(svc);
        let peers = engine.list_active_peers();
        assert_eq!(peers.len(), 1);
        assert_eq!(peers[0].ip_address, "192.168.1.50");
        assert_eq!(peers[0].services.len(), 1);
        assert_eq!(peers[0].services[0].service_name, "SigmaOS Web Dashboard");
    }

    #[test]
    fn test_ssdp_msearch_and_notify_parsing() {
        let msearch = SsdpDiscoveryPacket::build_msearch_request("ssdp:all");
        assert!(msearch.contains("M-SEARCH * HTTP/1.1"));
        assert!(msearch.contains("ST: ssdp:all"));

        let notify_raw = "HTTP/1.1 200 OK\r\nST: urn:schemas-upnp-org:device:MediaServer:1\r\nLOCATION: http://192.168.1.100:49152/description.xml\r\n";
        let mut engine = SovereignNetworkDiscoveryEngine::new();
        engine.process_ssdp_response("192.168.1.100", notify_raw);

        let peers = engine.list_active_peers();
        assert_eq!(peers.len(), 1);
        assert_eq!(
            peers[0].services[0].protocol,
            DiscoveryProtocolType::UpnpSsdp
        );
    }

    #[test]
    fn test_llmnr_query_encoding() {
        let query = LlmnrNbnsResolver::build_llmnr_query("printer.local");
        assert_eq!(&query[0..2], &[0x12, 0x34]); // ID
        assert_eq!(query[12], 7); // "printer" length
    }

    #[test]
    fn test_icmpv6_ndp_table_updates() {
        let mut engine = SovereignNetworkDiscoveryEngine::new();
        engine.add_ndp_entry("fe80::1", [0x00, 0x11, 0x22, 0x33, 0x44, 0x55], true);
        assert_eq!(engine.ndp_table.len(), 1);
        assert_eq!(engine.ndp_table[0].reachability_state, "REACHABLE");
        assert!(engine.ndp_table[0].is_router);
    }
}

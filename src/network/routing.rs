#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
// SPDX-License-Identifier: MIT
/// SigmaOS: Network Routing Engine
/// Implements packet routing, forwarding, and network lookup

use super::zenithnet::{Ipv4Addr, NetworkError};
use std::string::String;
use std::vec::Vec;

/// Routing Table Entry
#[derive(Debug, Clone)]
pub struct RouteEntry {
    pub destination: Ipv4Addr,
    pub netmask: Ipv4Addr,
    pub gateway: Option<Ipv4Addr>,
    pub interface: String,
    pub metric: u32,
    pub flags: u32,
}

impl RouteEntry {
    pub fn new(dest: Ipv4Addr, netmask: Ipv4Addr, interface: String) -> Self {
        Self {
            destination: dest,
            netmask,
            gateway: None,
            interface,
            metric: 0,
            flags: 0,
        }
    }

    pub fn with_gateway(mut self, gateway: Ipv4Addr) -> Self {
        self.gateway = Some(gateway);
        self
    }

    pub fn with_metric(mut self, metric: u32) -> Self {
        self.metric = metric;
        self
    }

    pub fn matches(&self, addr: Ipv4Addr) -> bool {
        (addr.0 & self.netmask.0) == (self.destination.0 & self.netmask.0)
    }
}

/// Routing Table
pub struct RoutingTable {
    routes: Vec<RouteEntry>,
}

impl RoutingTable {
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
        }
    }

    /// Add route
    pub fn add_route(&mut self, route: RouteEntry) {
        self.routes.push(route);
        // Sort by netmask length (most specific first)
        self.routes.sort_by_key(|r| {
            let bits = u32::BITS - r.netmask.0.leading_zeros();
            -(bits as i32) // Negative for reverse sort (most specific first)
        });
    }

    /// Remove route
    pub fn remove_route(&mut self, dest: Ipv4Addr, netmask: Ipv4Addr) -> bool {
        if let Some(pos) = self
            .routes
            .iter()
            .position(|r| r.destination == dest && r.netmask == netmask)
        {
            self.routes.remove(pos);
            true
        } else {
            false
        }
    }

    /// Lookup route for destination
    pub fn lookup(&self, dest: Ipv4Addr) -> Option<&RouteEntry> {
        self.routes.iter().find(|r| r.matches(dest))
    }

    /// Get all routes
    pub fn routes(&self) -> &[RouteEntry] {
        &self.routes
    }
}

impl Default for RoutingTable {
    fn default() -> Self {
        Self::new()
    }
}

/// Forwarding Decision
#[derive(Debug, Clone)]
pub enum ForwardingDecision {
    LocalDelivery,
    Forward {
        interface: String,
        next_hop: Option<Ipv4Addr>,
    },
    DropPacket,
}

/// Routing Engine
pub struct RoutingEngine {
    routing_table: RoutingTable,
    local_addresses: Vec<Ipv4Addr>,
    default_gateway: Option<Ipv4Addr>,
}

impl RoutingEngine {
    pub fn new() -> Self {
        Self {
            routing_table: RoutingTable::new(),
            local_addresses: Vec::new(),
            default_gateway: None,
        }
    }

    /// Add local address
    pub fn add_local_address(&mut self, addr: Ipv4Addr) {
        self.local_addresses.push(addr);
    }

    /// Set default gateway
    pub fn set_default_gateway(&mut self, gateway: Ipv4Addr) {
        self.default_gateway = Some(gateway);
    }

    /// Add route
    pub fn add_route(&mut self, route: RouteEntry) {
        self.routing_table.add_route(route);
    }

    /// Make forwarding decision
    pub fn forward_packet(&self, dest: Ipv4Addr) -> Result<ForwardingDecision, NetworkError> {
        // Check if destination is local
        if self.local_addresses.contains(&dest) {
            return Ok(ForwardingDecision::LocalDelivery);
        }

        // Lookup in routing table
        if let Some(route) = self.routing_table.lookup(dest) {
            return Ok(ForwardingDecision::Forward {
                interface: route.interface.clone(),
                next_hop: route.gateway,
            });
        }

        // Use default gateway if available
        if let Some(gateway) = self.default_gateway {
            return Ok(ForwardingDecision::Forward {
                interface: "eth0".to_string(), // Default interface
                next_hop: Some(gateway),
            });
        }

        Err(NetworkError::RouteNotFound)
    }

    /// Get routing table
    pub fn get_routing_table(&self) -> &RoutingTable {
        &self.routing_table
    }
}

impl Default for RoutingEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_route_entry() {
        let route = RouteEntry::new(
            Ipv4Addr::new(192, 168, 0, 0),
            Ipv4Addr::new(255, 255, 255, 0),
            "eth0".to_string(),
        );

        assert!(route.matches(Ipv4Addr::new(192, 168, 0, 1)));
        assert!(!route.matches(Ipv4Addr::new(192, 168, 1, 1)));
    }

    #[test]
    fn test_routing_table() {
        let mut table = RoutingTable::new();

        let route1 = RouteEntry::new(
            Ipv4Addr::new(192, 168, 0, 0),
            Ipv4Addr::new(255, 255, 255, 0),
            "eth0".to_string(),
        );

        let route2 = RouteEntry::new(
            Ipv4Addr::new(10, 0, 0, 0),
            Ipv4Addr::new(255, 0, 0, 0),
            "eth1".to_string(),
        );

        table.add_route(route1);
        table.add_route(route2);

        assert_eq!(table.routes().len(), 2);
        assert!(table.lookup(Ipv4Addr::new(192, 168, 0, 1)).is_some());
        assert!(table.lookup(Ipv4Addr::new(10, 1, 2, 3)).is_some());
    }

    #[test]
    fn test_routing_engine() {
        let mut engine = RoutingEngine::new();
        engine.add_local_address(Ipv4Addr::new(192, 168, 1, 1));

        let decision = engine.forward_packet(Ipv4Addr::new(192, 168, 1, 1)).unwrap();
        assert!(matches!(decision, ForwardingDecision::LocalDelivery));
    }

    #[test]
    fn test_default_gateway() {
        let mut engine = RoutingEngine::new();
        engine.add_local_address(Ipv4Addr::new(192, 168, 1, 1));
        engine.set_default_gateway(Ipv4Addr::new(192, 168, 1, 254));

        let decision = engine.forward_packet(Ipv4Addr::new(8, 8, 8, 8)).unwrap();
        assert!(matches!(decision, ForwardingDecision::Forward { .. }));
    }
}

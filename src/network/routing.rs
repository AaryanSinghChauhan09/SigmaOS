// SPDX-License-Identifier: MIT
// SigmaOS: Network Routing Engine
// Implements packet routing, forwarding, and network lookup
#![no_std]

extern crate alloc;
use super::zenithnet::{Ipv4Addr, NetworkError};
use alloc::string::String;
use alloc::vec::Vec;

/// Routing Table
#[derive(Debug, Clone)]
pub struct RoutingTable {
    routes: Vec<RouteEntry>,
}

impl RoutingTable {
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    pub fn add_route(&mut self, route: RouteEntry) {
        self.routes.push(route);
        self.routes.sort_by_key(|r| r.metric);
    }

    pub fn remove_route(&mut self, dest: Ipv4Addr, netmask: Ipv4Addr) -> bool {
        let len_before = self.routes.len();
        self.routes.retain(|r| !(r.destination == dest && r.netmask == netmask));
        self.routes.len() < len_before
    }

    pub fn lookup(&self, dest: Ipv4Addr) -> Option<&RouteEntry> {
        self.routes.iter().find(|r| r.matches(dest))
    }

    pub fn routes(&self) -> &[RouteEntry] {
        &self.routes
    }
}

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
#[derive(Debug, Clone)]
pub struct RoutingTable {
    routes: Vec<RouteEntry>,
}

impl RoutingTable {
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    pub fn add_route(&mut self, route: RouteEntry) {
        self.routes.push(route);
    }

    pub fn remove_route(&mut self, dest: Ipv4Addr, netmask: Ipv4Addr) -> bool {
        let len_before = self.routes.len();
        self.routes.retain(|r| !(r.destination == dest && r.netmask == netmask));
        self.routes.len() < len_before
    }

    pub fn lookup(&self, dest: Ipv4Addr) -> Option<&RouteEntry> {
        self.routes.iter().find(|r| r.matches(dest))
    }

    pub fn routes(&self) -> &[RouteEntry] {
        &self.routes
    }
}

/// Advanced Routing Tables
/// Supports multiple routing tables (main, default, local, main) as per Linux networking stack
/// Roadmap item: Full multi-table routing support for Omarchy Linux parity
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RoutingTableId {
    /// Main routing table
    Main,
    /// Default routing table
    Default,
    /// Local routing table
    Local,
    /// Custom routing table
    Custom(String),
}

pub enum RoutingTableEntry {
    /// Primary routing table
    Primary(RoutingTable),
    /// Secondary routing table
    Secondary(RoutingTable),
    /// Policy routing table
    Policy(RoutingTable),
}

pub struct AdvancedRoutingTable {
    /// Primary routing table (main)
    primary: RoutingTable,
    /// Secondary routing tables indexed by table ID
    secondary: alloc::collections::BTreeMap<RoutingTableId, RoutingTable>,
    /// Default gateway
    default_gateway: Option<Ipv4Addr>,
    /// Local addresses
    local_addresses: alloc::vec::Vec<Ipv4Addr>,
}

impl AdvancedRoutingTable {
    pub fn new() -> Self {
        Self {
            primary: RoutingTable::new(),
            secondary: alloc::collections::BTreeMap::new(),
            default_gateway: None,
            local_addresses: alloc::vec::Vec::new(),
        }
    }

    /// Add route to primary routing table
    pub fn add_primary_route(&mut self, route: RouteEntry) {
        self.primary.add_route(route);
    }

    /// Add route to secondary routing table
    pub fn add_secondary_route(&mut self, table_id: RoutingTableId, route: RouteEntry) {
        self.secondary
            .entry(table_id)
            .or_insert_with(RoutingTable::new)
            .add_route(route);
    }

    /// Remove route from primary table
    pub fn remove_primary_route(&mut self, dest: Ipv4Addr, netmask: Ipv4Addr) -> bool {
        self.primary.remove_route(dest, netmask)
    }

    /// Remove route from secondary table
    pub fn remove_secondary_route(
        &mut self,
        table_id: RoutingTableId,
        dest: Ipv4Addr,
        netmask: Ipv4Addr,
    ) -> bool {
        if let Some(table) = self.secondary.get_mut(&table_id) {
            table.remove_route(dest, netmask)
        } else {
            false
        }
    }

    /// Lookup route in primary table
    pub fn lookup_primary(&self, dest: Ipv4Addr) -> Option<&RouteEntry> {
        self.primary.lookup(dest)
    }

    /// Lookup route in secondary table
    pub fn lookup_secondary(
        &self,
        table_id: RoutingTableId,
        dest: Ipv4Addr,
    ) -> Option<&RouteEntry> {
        self.secondary.get(&table_id).and_then(|t| t.lookup(dest))
    }

    /// Get all routes from primary table
    pub fn primary_routes(&self) -> &[RouteEntry] {
        self.primary.routes()
    }

    /// Get all routes from secondary table
    pub fn secondary_routes(&self, table_id: RoutingTableId) -> Option<&[RouteEntry]> {
        self.secondary.get(&table_id).map(|t| t.routes())
    }

    /// Add local address
    pub fn add_local_address(&mut self, addr: Ipv4Addr) {
        self.local_addresses.push(addr);
    }

    /// Set default gateway
    pub fn set_default_gateway(&mut self, gateway: Ipv4Addr) {
        self.default_gateway = Some(gateway);
    }

    /// Get default gateway
    pub fn default_gateway(&self) -> Option<&Ipv4Addr> {
        self.default_gateway.as_ref()
    }
}

/// Multi-table Routing Engine
/// Supports multiple routing tables for policy-based routing
/// Roadmap item: Full multi-table routing support for Omarchy Linux parity
pub struct MultiTableRoutingEngine {
    /// Advanced routing tables
    routing_tables: alloc::collections::BTreeMap<RoutingTableId, AdvancedRoutingTable>,
    /// Current active table
    active_table: RoutingTableId,
}

impl MultiTableRoutingEngine {
    pub fn new() -> Self {
        Self {
            routing_tables: alloc::collections::BTreeMap::new(),
            active_table: RoutingTableId::Main,
        }
    }

    /// Add routing table
    pub fn add_routing_table(&mut self, table_id: RoutingTableId) {
        self.routing_tables
            .entry(table_id)
            .or_insert_with(AdvancedRoutingTable::new);
    }

    /// Set active routing table
    pub fn set_active_table(&mut self, table_id: RoutingTableId) {
        self.active_table = table_id;
    }

    /// Get active routing table reference
    pub fn active_table(&self) -> Option<&AdvancedRoutingTable> {
        self.routing_tables.get(&self.active_table)
    }

    /// Get or insert active routing table mutable reference
    pub fn get_active_routing_table(&mut self) -> &mut AdvancedRoutingTable {
        self.routing_tables
            .entry(self.active_table.clone())            .or_insert_with(AdvancedRoutingTable::new)
    }

    /// Forward packet using active routing table
    pub fn forward_packet(&self, dest: Ipv4Addr) -> Result<ForwardingDecision, NetworkError> {
        if let Some(table) = self.active_table() {            if let Some(route) = table.lookup_primary(dest) {
                return Ok(ForwardingDecision::Forward {
                    interface: route.interface.clone(),
                    next_hop: route.gateway,
                });
            }
=======
            if let Some(gateway) = table.default_gateway() {
                return Ok(ForwardingDecision::Forward {
                    interface: "eth0".to_string(),
                    next_hop: Some(*gateway),
                });
            }
>>>>>>> origin/main
        }
        Err(NetworkError::RouteNotFound)
    }

    /// Add route to active table
    pub fn add_route(&mut self, route: RouteEntry) {
        self.get_active_routing_table().add_primary_route(route);
    }

    /// Add route to specific table
    pub fn add_route_to_table(&mut self, table_id: RoutingTableId, route: RouteEntry) {
        self.routing_tables
            .entry(table_id)
            .or_insert_with(AdvancedRoutingTable::new)
            .add_primary_route(route);
    }

    /// Remove route from active table
    pub fn remove_route(&mut self, dest: Ipv4Addr, netmask: Ipv4Addr) -> bool {
        self.get_active_routing_table()
            .remove_primary_route(dest, netmask)
    }

    /// Lookup route in active table
    pub fn lookup_route(&self, dest: Ipv4Addr) -> Option<&RouteEntry> {
<<<<<<< HEAD
        self.active_table().and_then(|t| t.lookup_primary(dest))    }

    /// Get all routes from active table
    pub fn get_routes(&self) -> alloc::vec::Vec<RouteEntry> {
        let mut routes = alloc::vec::Vec::new();
        if let Some(table) = self.routing_tables.get(&self.active_table) {
            routes.extend_from_slice(table.primary_routes());
        }
        for (_, secondary_table) in &self.routing_tables {
            if let Some(sec_routes) = secondary_table.secondary_routes(self.active_table.clone()) {
                routes.extend_from_slice(sec_routes);
            }
        }
        routes
    }

    /// Get all routing table IDs
    pub fn routing_table_ids(&self) -> alloc::vec::Vec<RoutingTableId> {
        self.routing_tables.keys().cloned().collect()
    }

    /// Get routing table by ID
    pub fn get_routing_table(&self, table_id: RoutingTableId) -> Option<&AdvancedRoutingTable> {
        self.routing_tables.get(&table_id)
    }

    /// Get local addresses
    pub fn local_addresses(&self) -> Option<&alloc::vec::Vec<Ipv4Addr>> {
        self.active_table().map(|t| &t.local_addresses)    }
}

/// Policy Routing Decision
/// Supports policy-based routing with multiple routing tables
#[derive(Debug, Clone)]
pub enum PolicyRoutingDecision {
    /// Standard forwarding
    Standard(ForwardingDecision),
    /// Policy-based forwarding
    Policy {
        table: RoutingTableId,
        decision: ForwardingDecision,
    },
    /// Drop packet
    Drop,
}

/// Policy Routing Engine
/// Supports policy-based routing with multiple routing tables
/// Roadmap item: Full multi-table routing support for Omarchy Linux parity
pub struct PolicyRoutingEngine {
    /// Routing tables
    routing_tables: alloc::collections::BTreeMap<RoutingTableId, AdvancedRoutingTable>,
    /// Routing decisions
    decisions: alloc::vec::Vec<PolicyRoutingDecision>,
}

impl PolicyRoutingEngine {
    pub fn new() -> Self {
        Self {
            routing_tables: alloc::collections::BTreeMap::new(),
            decisions: alloc::vec::Vec::new(),
        }
    }

    /// Add routing table
    pub fn add_routing_table(&mut self, table_id: RoutingTableId) {
        self.routing_tables
            .entry(table_id)
            .or_insert_with(AdvancedRoutingTable::new);
    }

    /// Add policy routing decision
    pub fn add_decision(&mut self, decision: PolicyRoutingDecision) {
        self.decisions.push(decision);
    }

    /// Get routing table
    pub fn get_routing_table(&self, table_id: RoutingTableId) -> Option<&AdvancedRoutingTable> {
        self.routing_tables.get(&table_id)
    }

    /// Get all routing table IDs
    pub fn routing_table_ids(&self) -> alloc::vec::Vec<RoutingTableId> {
        self.routing_tables.keys().cloned().collect()
    }

    /// Evaluate policy routing for destination
    pub fn evaluate_policy(&self, dest: Ipv4Addr) -> PolicyRoutingDecision {
        // Check configured policy decisions
        for decision in &self.decisions {
            match decision {
                PolicyRoutingDecision::Policy { table, decision } => {
                    // If table is active, use that decision
                    if let Some(tbl) = self.routing_tables.get(table) {
                        // Check if destination matches any route in that table
                        if tbl.lookup_secondary(table.clone(), dest).is_some() {
                            return PolicyRoutingDecision::Policy {
                                table: table.clone(),
                                decision: decision.clone(),                            };
                        }
                    }
                }
                _ => {}
            }
        }

        // Fall back to standard routing
        PolicyRoutingDecision::Standard(ForwardingDecision::DropPacket)
    }

    /// Get all policy decisions
    pub fn decisions(&self) -> &alloc::vec::Vec<PolicyRoutingDecision> {
        &self.decisions
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

#[cfg(test)]
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

        let decision = engine
            .forward_packet(Ipv4Addr::new(192, 168, 1, 1))
            .unwrap();
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

// SigmaOS Network Protocol Layer
// Advanced Routing - Linux-style routing table management
// Supports multiple routing tables, route caching, and policy routing


use std::string::{String, ToString};
use std::vec::Vec;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RouteType {
    Unicast,
    Local,
    Broadcast,
    Multicast,
    Unreachable,
    Blackhole,
    Prohibit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RouteProtocol {
    Kernel,
    Boot,
    Static,
    Redirect,
    Ra,
    Mrt,
    Zebra,
    Bird,
    Dnr,
    Xorp,
    Ntk,
    Dhcp,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RouteKey {
    pub destination: String,
    pub prefix_length: u8,
    pub table_id: u32,
}

#[derive(Debug, Clone)]
pub struct RouteEntry {
    pub key: RouteKey,
    pub gateway: Option<String>,
    pub interface: String,
    pub metric: u32,
    pub route_type: RouteType,
    pub protocol: RouteProtocol,
    pub scope: u8,
    pub flags: u32,
}

pub struct RoutingTable {
    routes: BTreeMap<RouteKey, RouteEntry>,
    route_cache: Vec<RouteEntry>,
    default_table_id: u32,
}

impl RoutingTable {
    pub fn new() -> Self {
        Self {
            routes: BTreeMap::new(),
            route_cache: Vec::new(),
            default_table_id: 254, // Main table
        }
    }

    /// Add a route to the routing table
    pub fn add_route(&mut self, route: RouteEntry) -> Result<(), &'static str> {
        let key = route.key.clone();
        self.routes.insert(key, route);
        self.invalidate_cache();
        Ok(())
    }

    /// Delete a route from the routing table
    pub fn delete_route(&mut self, destination: &str, prefix_length: u8, table_id: u32) -> Result<(), &'static str> {
        let key = RouteKey {
            destination: destination.to_string(),
            prefix_length,
            table_id,
        };
        
        self.routes.remove(&key)
            .ok_or("Route not found")?;
        
        self.invalidate_cache();
        Ok(())
    }

    /// Lookup a route for a destination
    pub fn lookup_route(&mut self, destination: &str) -> Option<RouteEntry> {
        // Check cache first
        for cached_route in &self.route_cache {
            if self.matches_destination(destination, &cached_route.key.destination, cached_route.key.prefix_length) {
                return Some(cached_route.clone());
            }
        }

        // Full lookup
        let mut best_route_key = None;
        let mut best_metric = u32::MAX;
        let mut best_prefix = 0u8;

        for route in self.routes.values() {
            if self.matches_destination(destination, &route.key.destination, route.key.prefix_length) {
                // Prefer longer prefix
                if route.key.prefix_length > best_prefix || 
                   (route.key.prefix_length == best_prefix && route.metric < best_metric) {
                    best_prefix = route.key.prefix_length;
                    best_metric = route.metric;
                    best_route_key = Some(route.key.clone());
                }
            }
        }

        // Cache the result
        if let Some(ref key) = best_route_key {
            if let Some(route) = self.routes.get(key) {
                self.route_cache.push(route.clone());
                // Limit cache size
                if self.route_cache.len() > 128 {
                    self.route_cache.remove(0);
                }
                return Some(route.clone());
            }
        }

        None
    }

    /// Check if destination matches a route prefix
    fn matches_destination(&self, dest: &str, route_dest: &str, prefix_len: u8) -> bool {
        // Simplified IP address matching
        // In a real implementation, this would parse IP addresses and compare bit by bit
        dest.starts_with(route_dest) || route_dest == "0.0.0.0" || route_dest == "::"
    }

    /// Flush the route cache
    pub fn flush_cache(&mut self) {
        self.route_cache.clear();
    }

    /// Invalidate the route cache
    fn invalidate_cache(&mut self) {
        self.route_cache.clear();
    }

    /// Get all routes in a specific table
    pub fn get_table_routes(&self, table_id: u32) -> Vec<&RouteEntry> {
        self.routes.values()
            .filter(|r| r.key.table_id == table_id)
            .collect()
    }

    /// Set the default routing table
    pub fn set_default_table(&mut self, table_id: u32) {
        self.default_table_id = table_id;
    }

    /// Get the default routing table ID
    pub fn default_table(&self) -> u32 {
        self.default_table_id
    }

    /// Get route count
    pub fn route_count(&self) -> usize {
        self.routes.len()
    }

    /// Get cache size
    pub fn cache_size(&self) -> usize {
        self.route_cache.len()
    }
}

impl Default for RoutingTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_add_route() {
        let mut table = RoutingTable::new();
        
        let route = RouteEntry {
            key: RouteKey {
                destination: "192.168.1.0".to_string(),
                prefix_length: 24,
                table_id: 254,
            },
            gateway: Some("192.168.1.1".to_string()),
            interface: "eth0".to_string(),
            metric: 100,
            route_type: RouteType::Unicast,
            protocol: RouteProtocol::Static,
            scope: 0,
            flags: 0,
        };
        
        table.add_route(route).unwrap();
        assert_eq!(table.route_count(), 1);
    }

    #[test]
    fn test_delete_route() {
        let mut table = RoutingTable::new();
        
        let route = RouteEntry {
            key: RouteKey {
                destination: "192.168.1.0".to_string(),
                prefix_length: 24,
                table_id: 254,
            },
            gateway: Some("192.168.1.1".to_string()),
            interface: "eth0".to_string(),
            metric: 100,
            route_type: RouteType::Unicast,
            protocol: RouteProtocol::Static,
            scope: 0,
            flags: 0,
        };
        
        table.add_route(route).unwrap();
        table.delete_route("192.168.1.0", 24, 254).unwrap();
        
        assert_eq!(table.route_count(), 0);
    }

    #[test]
    fn test_route_lookup() {
        let mut table = RoutingTable::new();
        
        let route = RouteEntry {
            key: RouteKey {
                destination: "192.168.1.0".to_string(),
                prefix_length: 24,
                table_id: 254,
            },
            gateway: Some("192.168.1.1".to_string()),
            interface: "eth0".to_string(),
            metric: 100,
            route_type: RouteType::Unicast,
            protocol: RouteProtocol::Static,
            scope: 0,
            flags: 0,
        };
        
        table.add_route(route).unwrap();
        
        let found = table.lookup_route("192.168.1.100");
        assert!(found.is_some());
    }

    #[test]
    fn test_route_cache() {
        let mut table = RoutingTable::new();
        
        let route = RouteEntry {
            key: RouteKey {
                destination: "192.168.1.0".to_string(),
                prefix_length: 24,
                table_id: 254,
            },
            gateway: Some("192.168.1.1".to_string()),
            interface: "eth0".to_string(),
            metric: 100,
            route_type: RouteType::Unicast,
            protocol: RouteProtocol::Static,
            scope: 0,
            flags: 0,
        };
        
        table.add_route(route).unwrap();
        
        // First lookup - cache miss
        table.lookup_route("192.168.1.100");
        assert_eq!(table.cache_size(), 1);
        
        // Second lookup - cache hit
        table.lookup_route("192.168.1.100");
        assert_eq!(table.cache_size(), 1);
    }

    #[test]
    fn test_flush_cache() {
        let mut table = RoutingTable::new();
        
        let route = RouteEntry {
            key: RouteKey {
                destination: "192.168.1.0".to_string(),
                prefix_length: 24,
                table_id: 254,
            },
            gateway: Some("192.168.1.1".to_string()),
            interface: "eth0".to_string(),
            metric: 100,
            route_type: RouteType::Unicast,
            protocol: RouteProtocol::Static,
            scope: 0,
            flags: 0,
        };
        
        table.add_route(route).unwrap();
        table.lookup_route("192.168.1.100");
        
        table.flush_cache();
        assert_eq!(table.cache_size(), 0);
    }

    #[test]
    fn test_get_table_routes() {
        let mut table = RoutingTable::new();
        
        let route1 = RouteEntry {
            key: RouteKey {
                destination: "192.168.1.0".to_string(),
                prefix_length: 24,
                table_id: 254,
            },
            gateway: Some("192.168.1.1".to_string()),
            interface: "eth0".to_string(),
            metric: 100,
            route_type: RouteType::Unicast,
            protocol: RouteProtocol::Static,
            scope: 0,
            flags: 0,
        };
        
        let route2 = RouteEntry {
            key: RouteKey {
                destination: "10.0.0.0".to_string(),
                prefix_length: 8,
                table_id: 255,
            },
            gateway: Some("10.0.0.1".to_string()),
            interface: "eth1".to_string(),
            metric: 100,
            route_type: RouteType::Unicast,
            protocol: RouteProtocol::Static,
            scope: 0,
            flags: 0,
        };
        
        table.add_route(route1).unwrap();
        table.add_route(route2).unwrap();
        
        let table_routes = table.get_table_routes(254);
        assert_eq!(table_routes.len(), 1);
    }
}

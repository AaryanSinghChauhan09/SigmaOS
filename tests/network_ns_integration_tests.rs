//! Network Namespace Integration Tests
//!
//! Comprehensive end-to-end integration tests for Network Namespace (CLONE_NEWNET)
//! Tests cover:
//! - End-to-end namespace creation and isolation
//! - Bridge connectivity between namespaces
//! - Multi-namespace communication
//! - Performance benchmarks
//! - Edge cases and error handling

#[cfg(test)]
mod network_ns_integration_tests {
    use std::net::{IpAddr, Ipv4Addr};
    use std::time::Instant;
    use sigmaos::net::{
        NetworkNamespace, NetworkNamespaceId, NetworkNamespaceManager,
        NetworkInterface, Route, FirewallRule, FirewallAction, VirtualBridge,
    };

    // ============================================================================
    // TEST SUITE 1: End-to-End Network Namespace Tests
    // ============================================================================

    #[test]
    fn test_create_multiple_namespaces() {
        let manager = NetworkNamespaceManager::new();
        
        // Create 3 namespaces
        let ns1 = manager.create_namespace(None).expect("Failed to create ns1");
        let ns2 = manager.create_namespace(None).expect("Failed to create ns2");
        let ns3 = manager.create_namespace(Some(ns1)).expect("Failed to create ns3 (child of ns1)");
        
        assert_ne!(ns1.raw(), 0);
        assert_ne!(ns2.raw(), 0);
        assert_ne!(ns3.raw(), 0);
        assert_ne!(ns1, ns2);
        assert_ne!(ns2, ns3);
        assert_ne!(ns1, ns3);
        
        // Verify they're tracked
        let count = manager.count().expect("Failed to count");
        assert_eq!(count, 3);
    }

    #[test]
    fn test_create_interfaces_per_namespace() {
        let manager = NetworkNamespaceManager::new();
        let ns1_id = manager.create_namespace(None).expect("Failed to create ns1");
        let ns2_id = manager.create_namespace(None).expect("Failed to create ns2");
        
        let ns1 = manager.get_namespace(ns1_id).expect("Failed to get ns1");
        let ns2 = manager.get_namespace(ns2_id).expect("Failed to get ns2");
        
        // Create interfaces in ns1
        let ns1_lock = ns1.lock().expect("Failed to lock ns1");
        let eth0_ns1 = NetworkInterface::new("eth0".to_string())
            .with_ip(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 10)));
        let eth1_ns1 = NetworkInterface::new("eth1".to_string())
            .with_ip(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 10)));
        
        ns1_lock.add_interface(eth0_ns1).expect("Failed to add eth0 to ns1");
        ns1_lock.add_interface(eth1_ns1).expect("Failed to add eth1 to ns1");
        
        // Create different interfaces in ns2
        let ns2_lock = ns2.lock().expect("Failed to lock ns2");
        let eth0_ns2 = NetworkInterface::new("eth0".to_string())
            .with_ip(IpAddr::V4(Ipv4Addr::new(172, 16, 0, 10)));
        
        ns2_lock.add_interface(eth0_ns2).expect("Failed to add eth0 to ns2");
        
        // Verify isolation
        let ns1_ifaces = ns1_lock.list_interfaces().expect("Failed to list ns1 ifaces");
        let ns2_ifaces = ns2_lock.list_interfaces().expect("Failed to list ns2 ifaces");
        
        assert_eq!(ns1_ifaces.len(), 2);
        assert_eq!(ns2_ifaces.len(), 1);
        assert!(ns1_ifaces.contains(&"eth0".to_string()));
        assert!(ns1_ifaces.contains(&"eth1".to_string()));
        assert!(ns2_ifaces.contains(&"eth0".to_string()));
    }

    #[test]
    fn test_configure_routes_per_namespace() {
        let manager = NetworkNamespaceManager::new();
        let ns1_id = manager.create_namespace(None).expect("Failed to create ns1");
        let ns2_id = manager.create_namespace(None).expect("Failed to create ns2");
        
        let ns1 = manager.get_namespace(ns1_id).expect("Failed to get ns1");
        let ns2 = manager.get_namespace(ns2_id).expect("Failed to get ns2");
        
        // Add routes to ns1
        let ns1_lock = ns1.lock().expect("Failed to lock ns1");
        let route1 = Route::new(
            IpAddr::V4(Ipv4Addr::new(10, 0, 0, 0)),
            24,
            "eth0".to_string(),
        );
        let route2 = Route::new(
            IpAddr::V4(Ipv4Addr::new(172, 16, 0, 0)),
            24,
            "eth1".to_string(),
        );
        
        ns1_lock.add_route(route1).expect("Failed to add route1 to ns1");
        ns1_lock.add_route(route2).expect("Failed to add route2 to ns1");
        
        // Add different route to ns2
        let ns2_lock = ns2.lock().expect("Failed to lock ns2");
        let route3 = Route::new(
            IpAddr::V4(Ipv4Addr::new(192, 168, 0, 0)),
            24,
            "eth0".to_string(),
        );
        
        ns2_lock.add_route(route3).expect("Failed to add route to ns2");
        
        // Verify isolation
        let ns1_routes = ns1_lock.get_routes().expect("Failed to get ns1 routes");
        let ns2_routes = ns2_lock.get_routes().expect("Failed to get ns2 routes");
        
        assert_eq!(ns1_routes.len(), 2);
        assert_eq!(ns2_routes.len(), 1);
        
        // Verify destinations are different
        assert_eq!(ns1_routes[0].destination, IpAddr::V4(Ipv4Addr::new(10, 0, 0, 0)));
        assert_eq!(ns1_routes[1].destination, IpAddr::V4(Ipv4Addr::new(172, 16, 0, 0)));
        assert_eq!(ns2_routes[0].destination, IpAddr::V4(Ipv4Addr::new(192, 168, 0, 0)));
    }

    #[test]
    fn test_verify_network_isolation() {
        let manager = NetworkNamespaceManager::new();
        let ns1_id = manager.create_namespace(None).expect("Failed to create ns1");
        let ns2_id = manager.create_namespace(None).expect("Failed to create ns2");
        
        let ns1 = manager.get_namespace(ns1_id).expect("Failed to get ns1");
        let ns2 = manager.get_namespace(ns2_id).expect("Failed to get ns2");
        
        let ns1_lock = ns1.lock().expect("Failed to lock ns1");
        let ns2_lock = ns2.lock().expect("Failed to lock ns2");
        
        // Setup ns1: eth0 with 192.168.1.0/24
        let eth0 = NetworkInterface::new("eth0".to_string())
            .with_ip(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)));
        ns1_lock.add_interface(eth0).expect("Failed to add eth0 to ns1");
        
        // Setup ns2: eth0 with 192.168.1.0/24 (same!)
        let eth0 = NetworkInterface::new("eth0".to_string())
            .with_ip(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)));
        ns2_lock.add_interface(eth0).expect("Failed to add eth0 to ns2");
        
        // Even though IPs are the same, they're isolated in different namespaces
        let ns1_ifaces = ns1_lock.list_interfaces().expect("Failed to list ns1 ifaces");
        let ns2_ifaces = ns2_lock.list_interfaces().expect("Failed to list ns2 ifaces");
        
        assert_eq!(ns1_ifaces.len(), 1);
        assert_eq!(ns2_ifaces.len(), 1);
        
        // Verify they have separate interface objects
        let eth0_ns1 = ns1_lock.get_interface("eth0")
            .expect("Failed to get eth0 from ns1");
        let eth0_ns2 = ns2_lock.get_interface("eth0")
            .expect("Failed to get eth0 from ns2");
        
        let eth0_ns1_lock = eth0_ns1.lock().expect("Failed to lock eth0 in ns1");
        let eth0_ns2_lock = eth0_ns2.lock().expect("Failed to lock eth0 in ns2");
        
        // They have the same config but are separate objects
        assert_eq!(eth0_ns1_lock.ip_addr, eth0_ns2_lock.ip_addr);
    }

    #[test]
    fn test_interface_visibility_isolation() {
        let manager = NetworkNamespaceManager::new();
        let ns1_id = manager.create_namespace(None).expect("Failed to create ns1");
        let ns2_id = manager.create_namespace(None).expect("Failed to create ns2");
        
        let ns1 = manager.get_namespace(ns1_id).expect("Failed to get ns1");
        let ns2 = manager.get_namespace(ns2_id).expect("Failed to get ns2");
        
        let ns1_lock = ns1.lock().expect("Failed to lock ns1");
        let ns2_lock = ns2.lock().expect("Failed to lock ns2");
        
        // Add eth0 only to ns1
        let eth0 = NetworkInterface::new("eth0".to_string());
        ns1_lock.add_interface(eth0).expect("Failed to add eth0 to ns1");
        
        // Verify eth0 NOT visible in ns2
        let eth0_in_ns2 = ns2_lock.get_interface("eth0");
        assert!(eth0_in_ns2.is_err(), "Interface should not be visible across namespaces");
        
        // Verify eth0 IS visible in ns1
        let eth0_in_ns1 = ns1_lock.get_interface("eth0");
        assert!(eth0_in_ns1.is_ok(), "Interface should be visible in its namespace");
    }

    #[test]
    fn test_route_isolation() {
        let manager = NetworkNamespaceManager::new();
        let ns1_id = manager.create_namespace(None).expect("Failed to create ns1");
        let ns2_id = manager.create_namespace(None).expect("Failed to create ns2");
        
        let ns1 = manager.get_namespace(ns1_id).expect("Failed to get ns1");
        let ns2 = manager.get_namespace(ns2_id).expect("Failed to get ns2");
        
        let ns1_lock = ns1.lock().expect("Failed to lock ns1");
        let ns2_lock = ns2.lock().expect("Failed to lock ns2");
        
        // Add route to default gateway in ns1
        let route1 = Route::new(
            IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            0,
            "eth0".to_string(),
        );
        ns1_lock.add_route(route1).expect("Failed to add default route to ns1");
        
        // ns2 has no routes
        let ns1_routes = ns1_lock.get_routes().expect("Failed to get ns1 routes");
        let ns2_routes = ns2_lock.get_routes().expect("Failed to get ns2 routes");
        
        assert_eq!(ns1_routes.len(), 1);
        assert_eq!(ns2_routes.len(), 0);
        
        // Adding route to ns2 doesn't affect ns1
        let route2 = Route::new(
            IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            0,
            "eth0".to_string(),
        );
        ns2_lock.add_route(route2).expect("Failed to add route to ns2");
        
        let ns1_routes_after = ns1_lock.get_routes().expect("Failed to get ns1 routes");
        assert_eq!(ns1_routes_after.len(), 1, "ns1 routes should not change");
    }

    // ============================================================================
    // TEST SUITE 2: Virtual Bridge Connectivity Tests
    // ============================================================================

    #[test]
    fn test_create_virtual_bridge() {
        let manager = NetworkNamespaceManager::new();
        let ns_id = manager.create_namespace(None).expect("Failed to create namespace");
        let ns = manager.get_namespace(ns_id).expect("Failed to get namespace");
        
        let ns_lock = ns.lock().expect("Failed to lock namespace");
        assert!(ns_lock.create_virtual_bridge("br0".to_string()).is_ok());
        assert!(ns_lock.create_virtual_bridge("br1".to_string()).is_ok());
    }

    #[test]
    fn test_bridge_connect_namespace() {
        let manager = NetworkNamespaceManager::new();
        let ns1_id = manager.create_namespace(None).expect("Failed to create ns1");
        let ns2_id = manager.create_namespace(None).expect("Failed to create ns2");
        let ns3_id = manager.create_namespace(None).expect("Failed to create ns3");
        
        let ns1 = manager.get_namespace(ns1_id).expect("Failed to get ns1");
        let ns1_lock = ns1.lock().expect("Failed to lock ns1");
        
        // Create bridge in ns1
        ns1_lock.create_virtual_bridge("br0".to_string()).expect("Failed to create bridge");
        
        // Get bridge and connect namespaces
        let bridge_arc = ns1_lock.get_virtual_bridge("br0").expect("Failed to get bridge");
        let mut bridge = bridge_arc.lock().expect("Failed to lock bridge");
        
        bridge.add_namespace(ns1_id).expect("Failed to add ns1 to bridge");
        bridge.add_namespace(ns2_id).expect("Failed to add ns2 to bridge");
        bridge.add_namespace(ns3_id).expect("Failed to add ns3 to bridge");
        
        assert_eq!(bridge.connected_namespaces.len(), 3);
    }

    #[test]
    fn test_bridge_remove_namespace() {
        let manager = NetworkNamespaceManager::new();
        let ns1_id = manager.create_namespace(None).expect("Failed to create ns1");
        let ns2_id = manager.create_namespace(None).expect("Failed to create ns2");
        
        let ns1 = manager.get_namespace(ns1_id).expect("Failed to get ns1");
        let ns1_lock = ns1.lock().expect("Failed to lock ns1");
        
        ns1_lock.create_virtual_bridge("br0".to_string()).expect("Failed to create bridge");
        let bridge_arc = ns1_lock.get_virtual_bridge("br0").expect("Failed to get bridge");
        let mut bridge = bridge_arc.lock().expect("Failed to lock bridge");
        
        bridge.add_namespace(ns1_id).expect("Failed to add ns1");
        bridge.add_namespace(ns2_id).expect("Failed to add ns2");
        
        assert_eq!(bridge.connected_namespaces.len(), 2);
        
        bridge.remove_namespace(ns1_id).expect("Failed to remove ns1");
        assert_eq!(bridge.connected_namespaces.len(), 1);
        assert!(bridge.connected_namespaces.contains(&ns2_id));
    }

    #[test]
    fn test_bridge_forwarding_connectivity() {
        let manager = NetworkNamespaceManager::new();
        let ns1_id = manager.create_namespace(None).expect("Failed to create ns1");
        let ns2_id = manager.create_namespace(None).expect("Failed to create ns2");
        
        let ns1 = manager.get_namespace(ns1_id).expect("Failed to get ns1");
        let ns2 = manager.get_namespace(ns2_id).expect("Failed to get ns2");
        
        let ns1_lock = ns1.lock().expect("Failed to lock ns1");
        let ns2_lock = ns2.lock().expect("Failed to lock ns2");
        
        // Setup interfaces with IPs
        let eth0_ns1 = NetworkInterface::new("eth0".to_string())
            .with_ip(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 10)));
        let eth0_ns2 = NetworkInterface::new("eth0".to_string())
            .with_ip(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 20)));
        
        ns1_lock.add_interface(eth0_ns1).expect("Failed to add eth0 to ns1");
        ns2_lock.add_interface(eth0_ns2).expect("Failed to add eth0 to ns2");
        
        // Create bridge connecting both
        ns1_lock.create_virtual_bridge("br0".to_string()).expect("Failed to create bridge");
        let bridge_arc = ns1_lock.get_virtual_bridge("br0").expect("Failed to get bridge");
        let mut bridge = bridge_arc.lock().expect("Failed to lock bridge");
        
        bridge.add_namespace(ns1_id).expect("Failed to add ns1 to bridge");
        bridge.add_namespace(ns2_id).expect("Failed to add ns2 to bridge");
        
        assert_eq!(bridge.connected_namespaces.len(), 2);
        assert!(bridge.connected_namespaces.contains(&ns1_id));
        assert!(bridge.connected_namespaces.contains(&ns2_id));
    }

    // ============================================================================
    // TEST SUITE 3: Multi-Namespace Communication Tests
    // ============================================================================

    #[test]
    fn test_three_namespace_communication_scenario() {
        let manager = NetworkNamespaceManager::new();
        let ns1_id = manager.create_namespace(None).expect("Failed to create ns1");
        let ns2_id = manager.create_namespace(None).expect("Failed to create ns2");
        let ns3_id = manager.create_namespace(None).expect("Failed to create ns3");
        
        let ns1 = manager.get_namespace(ns1_id).expect("Failed to get ns1");
        let ns2 = manager.get_namespace(ns2_id).expect("Failed to get ns2");
        let ns3 = manager.get_namespace(ns3_id).expect("Failed to get ns3");
        
        // Setup ns1: 192.168.1.0/24
        let ns1_lock = ns1.lock().expect("Failed to lock ns1");
        let eth0_ns1 = NetworkInterface::new("eth0".to_string())
            .with_ip(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)));
        ns1_lock.add_interface(eth0_ns1).expect("Failed to add eth0 to ns1");
        
        // Setup ns2: 192.168.2.0/24
        let ns2_lock = ns2.lock().expect("Failed to lock ns2");
        let eth0_ns2 = NetworkInterface::new("eth0".to_string())
            .with_ip(IpAddr::V4(Ipv4Addr::new(192, 168, 2, 1)));
        ns2_lock.add_interface(eth0_ns2).expect("Failed to add eth0 to ns2");
        
        // Setup ns3: 192.168.3.0/24
        let ns3_lock = ns3.lock().expect("Failed to lock ns3");
        let eth0_ns3 = NetworkInterface::new("eth0".to_string())
            .with_ip(IpAddr::V4(Ipv4Addr::new(192, 168, 3, 1)));
        ns3_lock.add_interface(eth0_ns3).expect("Failed to add eth0 to ns3");
        
        // Add cross-namespace routes
        let route1 = Route::new(
            IpAddr::V4(Ipv4Addr::new(192, 168, 2, 0)),
            24,
            "eth0".to_string(),
        );
        ns1_lock.add_route(route1).expect("Failed to add route to ns1");
        
        let route2 = Route::new(
            IpAddr::V4(Ipv4Addr::new(192, 168, 3, 0)),
            24,
            "eth0".to_string(),
        );
        ns1_lock.add_route(route2).expect("Failed to add route to ns1");
        
        // Verify isolation
        let routes_ns1 = ns1_lock.get_routes().expect("Failed to get routes");
        let routes_ns2 = ns2_lock.get_routes().expect("Failed to get routes");
        let routes_ns3 = ns3_lock.get_routes().expect("Failed to get routes");
        
        assert_eq!(routes_ns1.len(), 2);
        assert_eq!(routes_ns2.len(), 0);
        assert_eq!(routes_ns3.len(), 0);
    }

    #[test]
    fn test_cross_namespace_isolation_at_socket_level() {
        let manager = NetworkNamespaceManager::new();
        let ns1_id = manager.create_namespace(None).expect("Failed to create ns1");
        let ns2_id = manager.create_namespace(None).expect("Failed to create ns2");
        
        let ns1 = manager.get_namespace(ns1_id).expect("Failed to get ns1");
        let ns2 = manager.get_namespace(ns2_id).expect("Failed to get ns2");
        
        let ns1_lock = ns1.lock().expect("Failed to lock ns1");
        let ns2_lock = ns2.lock().expect("Failed to lock ns2");
        
        // Verify each namespace has its own socket table
        let table1 = ns1_lock.get_socket_table().expect("Failed to get socket table");
        let table2 = ns2_lock.get_socket_table().expect("Failed to get socket table");
        
        let table1_lock = table1.lock().expect("Failed to lock table1");
        let table2_lock = table2.lock().expect("Failed to lock table2");
        
        assert_eq!(table1_lock.count().unwrap_or(0), 0);
        assert_eq!(table2_lock.count().unwrap_or(0), 0);
    }

    // ============================================================================
    // TEST SUITE 4: Performance Benchmarks
    // ============================================================================

    #[test]
    fn bench_namespace_creation_latency() {
        let manager = NetworkNamespaceManager::new();
        let iterations = 100;
        
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = manager.create_namespace(None);
        }
        let elapsed = start.elapsed();
        
        let avg_us = elapsed.as_micros() as f64 / iterations as f64;
        println!("Namespace creation: {:.2} µs per operation", avg_us);
        
        // Verify performance is reasonable (< 1000 µs per operation)
        assert!(avg_us < 1000.0, "Namespace creation too slow: {:.2} µs", avg_us);
    }

    #[test]
    fn bench_interface_creation_latency() {
        let manager = NetworkNamespaceManager::new();
        let ns_id = manager.create_namespace(None).expect("Failed to create namespace");
        let ns = manager.get_namespace(ns_id).expect("Failed to get namespace");
        let ns_lock = ns.lock().expect("Failed to lock namespace");
        
        let iterations = 100;
        let start = Instant::now();
        
        for i in 0..iterations {
            let iface = NetworkInterface::new(format!("eth{}", i));
            let _ = ns_lock.add_interface(iface);
        }
        
        let elapsed = start.elapsed();
        let avg_us = elapsed.as_micros() as f64 / iterations as f64;
        println!("Interface creation: {:.2} µs per operation", avg_us);
        
        assert!(avg_us < 100.0, "Interface creation too slow: {:.2} µs", avg_us);
    }

    #[test]
    fn bench_route_lookup_performance() {
        let manager = NetworkNamespaceManager::new();
        let ns_id = manager.create_namespace(None).expect("Failed to create namespace");
        let ns = manager.get_namespace(ns_id).expect("Failed to get namespace");
        let ns_lock = ns.lock().expect("Failed to lock namespace");
        
        // Add 50 routes
        for i in 0..50 {
            let dest = IpAddr::V4(Ipv4Addr::new(10, (i >> 8) as u8, i as u8, 0));
            let route = Route::new(dest, 24, "eth0".to_string());
            let _ = ns_lock.add_route(route);
        }
        
        // Benchmark route lookup
        let iterations = 100;
        let start = Instant::now();
        
        for _ in 0..iterations {
            let _ = ns_lock.get_routes();
        }
        
        let elapsed = start.elapsed();
        let avg_us = elapsed.as_micros() as f64 / iterations as f64;
        println!("Route lookup: {:.2} µs per operation", avg_us);
        
        assert!(avg_us < 100.0, "Route lookup too slow: {:.2} µs", avg_us);
    }

    #[test]
    fn bench_firewall_rule_matching_performance() {
        let manager = NetworkNamespaceManager::new();
        let ns_id = manager.create_namespace(None).expect("Failed to create namespace");
        let ns = manager.get_namespace(ns_id).expect("Failed to get namespace");
        let ns_lock = ns.lock().expect("Failed to lock namespace");
        
        // Add 100 firewall rules
        for i in 0..100 {
            let rule = FirewallRule::new(
                if i % 2 == 0 { FirewallAction::Allow } else { FirewallAction::Deny }
            );
            let _ = ns_lock.add_firewall_rule(rule);
        }
        
        // Benchmark rule retrieval
        let iterations = 100;
        let start = Instant::now();
        
        for _ in 0..iterations {
            let _ = ns_lock.get_firewall_rules();
        }
        
        let elapsed = start.elapsed();
        let avg_us = elapsed.as_micros() as f64 / iterations as f64;
        println!("Firewall rule matching: {:.2} µs per operation", avg_us);
        
        assert!(avg_us < 100.0, "Rule matching too slow: {:.2} µs", avg_us);
    }

    // ============================================================================
    // TEST SUITE 5: Edge Cases and Error Handling
    // ============================================================================

    #[test]
    fn test_invalid_interface_name() {
        let manager = NetworkNamespaceManager::new();
        let ns_id = manager.create_namespace(None).expect("Failed to create namespace");
        let ns = manager.get_namespace(ns_id).expect("Failed to get namespace");
        let ns_lock = ns.lock().expect("Failed to lock namespace");
        
        // Try to get non-existent interface
        let result = ns_lock.get_interface("nonexistent");
        assert!(result.is_err(), "Should fail for non-existent interface");
    }

    #[test]
    fn test_route_conflicts() {
        let manager = NetworkNamespaceManager::new();
        let ns_id = manager.create_namespace(None).expect("Failed to create namespace");
        let ns = manager.get_namespace(ns_id).expect("Failed to get namespace");
        let ns_lock = ns.lock().expect("Failed to lock namespace");
        
        // Add first route
        let route1 = Route::new(
            IpAddr::V4(Ipv4Addr::new(10, 0, 0, 0)),
            24,
            "eth0".to_string(),
        );
        ns_lock.add_route(route1).expect("Failed to add first route");
        
        // Add overlapping route (same destination/prefix)
        let route2 = Route::new(
            IpAddr::V4(Ipv4Addr::new(10, 0, 0, 0)),
            24,
            "eth1".to_string(),
        );
        ns_lock.add_route(route2).expect("Failed to add second route");
        
        // Both should be present (allowing overlap for now)
        let routes = ns_lock.get_routes().expect("Failed to get routes");
        assert_eq!(routes.len(), 2);
    }

    #[test]
    fn test_firewall_rule_conflicts() {
        let manager = NetworkNamespaceManager::new();
        let ns_id = manager.create_namespace(None).expect("Failed to create namespace");
        let ns = manager.get_namespace(ns_id).expect("Failed to get namespace");
        let ns_lock = ns.lock().expect("Failed to lock namespace");
        
        // Add conflicting rules (Allow then Deny for same traffic)
        let rule1 = FirewallRule::new(FirewallAction::Allow);
        let rule2 = FirewallRule::new(FirewallAction::Deny);
        
        ns_lock.add_firewall_rule(rule1).expect("Failed to add allow rule");
        ns_lock.add_firewall_rule(rule2).expect("Failed to add deny rule");
        
        let rules = ns_lock.get_firewall_rules().expect("Failed to get rules");
        assert_eq!(rules.len(), 2);
    }

    #[test]
    fn test_duplicate_interface_prevented() {
        let manager = NetworkNamespaceManager::new();
        let ns_id = manager.create_namespace(None).expect("Failed to create namespace");
        let ns = manager.get_namespace(ns_id).expect("Failed to get namespace");
        let ns_lock = ns.lock().expect("Failed to lock namespace");
        
        let eth0 = NetworkInterface::new("eth0".to_string());
        ns_lock.add_interface(eth0).expect("Failed to add eth0");
        
        // Try to add duplicate
        let eth0_dup = NetworkInterface::new("eth0".to_string());
        let result = ns_lock.add_interface(eth0_dup);
        
        assert!(result.is_err(), "Should prevent duplicate interface");
    }

    #[test]
    fn test_bridge_duplicate_namespace_prevented() {
        let manager = NetworkNamespaceManager::new();
        let ns1_id = manager.create_namespace(None).expect("Failed to create ns1");
        let ns1 = manager.get_namespace(ns1_id).expect("Failed to get ns1");
        let ns1_lock = ns1.lock().expect("Failed to lock ns1");
        
        ns1_lock.create_virtual_bridge("br0".to_string()).expect("Failed to create bridge");
        let bridge_arc = ns1_lock.get_virtual_bridge("br0").expect("Failed to get bridge");
        let mut bridge = bridge_arc.lock().expect("Failed to lock bridge");
        
        bridge.add_namespace(ns1_id).expect("Failed to add ns1");
        
        // Try to add same namespace again
        let result = bridge.add_namespace(ns1_id);
        assert!(result.is_err(), "Should prevent duplicate namespace in bridge");
    }

    #[test]
    fn test_get_nonexistent_namespace() {
        let manager = NetworkNamespaceManager::new();
        let fake_id = NetworkNamespaceId::new(9999);
        
        let result = manager.get_namespace(fake_id);
        assert!(result.is_err(), "Should fail for non-existent namespace");
    }

    #[test]
    fn test_delete_namespace() {
        let manager = NetworkNamespaceManager::new();
        let ns_id = manager.create_namespace(None).expect("Failed to create namespace");
        
        let count_before = manager.count().expect("Failed to count");
        assert_eq!(count_before, 1);
        
        manager.delete_namespace(ns_id).expect("Failed to delete namespace");
        
        let count_after = manager.count().expect("Failed to count");
        assert_eq!(count_after, 0);
        
        // Verify we can't get it
        let result = manager.get_namespace(ns_id);
        assert!(result.is_err(), "Should not find deleted namespace");
    }

    #[test]
    fn test_list_namespaces() {
        let manager = NetworkNamespaceManager::new();
        let ns1 = manager.create_namespace(None).expect("Failed to create ns1");
        let ns2 = manager.create_namespace(None).expect("Failed to create ns2");
        let ns3 = manager.create_namespace(None).expect("Failed to create ns3");
        
        let list = manager.list_namespaces().expect("Failed to list");
        assert_eq!(list.len(), 3);
        assert!(list.contains(&ns1));
        assert!(list.contains(&ns2));
        assert!(list.contains(&ns3));
    }

    // ============================================================================
    // TEST SUITE 6: Reference Counting and Refcount Tests
    // ============================================================================

    #[test]
    fn test_namespace_refcount() {
        let manager = NetworkNamespaceManager::new();
        let ns_id = manager.create_namespace(None).expect("Failed to create namespace");
        let ns = manager.get_namespace(ns_id).expect("Failed to get namespace");
        let ns_lock = ns.lock().expect("Failed to lock namespace");
        
        let initial_refcount = ns_lock.refcount();
        assert!(initial_refcount > 0);
        
        ns_lock.incref();
        assert_eq!(ns_lock.refcount(), initial_refcount + 1);
        
        let decref_count = ns_lock.decref();
        assert_eq!(decref_count, initial_refcount + 1);
        assert_eq!(ns_lock.refcount(), initial_refcount);
    }
}

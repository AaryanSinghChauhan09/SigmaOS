//! Comprehensive Network Interface & Routing Management Test Suite
//!
//! Tests for Phase 9.2.3: Interface & Routing Management
//! Covers:
//! - Interface isolation between namespaces
//! - Route isolation and routing table consistency
//! - Firewall rule isolation and enforcement
//! - Socket creation with namespace support
//! - Cross-namespace communication scenarios
//! - Edge cases and error handling

#[cfg(test)]
mod tests {
    use sigmaos::net::{
        NetworkNamespaceManager, NetworkNamespaceId, NetworkInterface, Route, FirewallRule,
        FirewallAction, NetworkSyscalls, SocketFd, SockAddr, SocketState,
        AF_INET, SOCK_STREAM, SOCK_DGRAM, IPPROTO_TCP, IPPROTO_UDP,
    };
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    // ============================================================================
    // INTERFACE ISOLATION TESTS
    // ============================================================================

    #[test]
    fn test_interface_isolation_between_namespaces() {
        let manager = NetworkNamespaceManager::new();

        // Create two separate namespaces
        let ns1_id = manager.create_namespace(None).expect("Failed to create ns1");
        let ns2_id = manager.create_namespace(None).expect("Failed to create ns2");

        // Get namespace objects
        let ns1_arc = manager.get_namespace(ns1_id).expect("Failed to get ns1");
        let ns2_arc = manager.get_namespace(ns2_id).expect("Failed to get ns2");

        let ns1 = ns1_arc.lock().expect("Failed to lock ns1");
        let ns2 = ns2_arc.lock().expect("Failed to lock ns2");

        // Add eth0 interface to ns1
        let iface1 = NetworkInterface::new("eth0".to_string())
            .with_ip(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)));
        ns1.add_interface(iface1).expect("Failed to add eth0 to ns1");

        // Add eth0 interface to ns2 with different IP
        let iface2 = NetworkInterface::new("eth0".to_string())
            .with_ip(IpAddr::V4(Ipv4Addr::new(192, 168, 2, 1)));
        ns2.add_interface(iface2).expect("Failed to add eth0 to ns2");

        // Verify interfaces are separate
        let iface1_retrieved = ns1.get_interface("eth0").expect("Failed to get eth0 from ns1");
        let iface1_lock = iface1_retrieved.lock().expect("Failed to lock iface1");
        if let Some(IpAddr::V4(ip)) = iface1_lock.ip_addr {
            assert_eq!(ip, Ipv4Addr::new(192, 168, 1, 1));
        }

        let iface2_retrieved = ns2.get_interface("eth0").expect("Failed to get eth0 from ns2");
        let iface2_lock = iface2_retrieved.lock().expect("Failed to lock iface2");
        if let Some(IpAddr::V4(ip)) = iface2_lock.ip_addr {
            assert_eq!(ip, Ipv4Addr::new(192, 168, 2, 1));
        }
    }

    #[test]
    fn test_interface_not_visible_across_namespaces() {
        let manager = NetworkNamespaceManager::new();

        let ns1_id = manager.create_namespace(None).expect("Failed to create ns1");
        let ns2_id = manager.create_namespace(None).expect("Failed to create ns2");

        let ns1_arc = manager.get_namespace(ns1_id).expect("Failed to get ns1");
        let ns2_arc = manager.get_namespace(ns2_id).expect("Failed to get ns2");

        let ns1 = ns1_arc.lock().expect("Failed to lock ns1");
        let ns2 = ns2_arc.lock().expect("Failed to lock ns2");

        // Add multiple interfaces to ns1
        ns1.add_interface(NetworkInterface::new("eth0".to_string()))
            .expect("Failed to add eth0");
        ns1.add_interface(NetworkInterface::new("eth1".to_string()))
            .expect("Failed to add eth1");
        ns1.add_interface(NetworkInterface::new("eth2".to_string()))
            .expect("Failed to add eth2");

        // Verify ns2 has no interfaces
        let ns2_interfaces = ns2.list_interfaces().expect("Failed to list interfaces");
        assert_eq!(ns2_interfaces.len(), 0, "ns2 should have no interfaces");

        // Add interface to ns2
        ns2.add_interface(NetworkInterface::new("wlan0".to_string()))
            .expect("Failed to add wlan0");

        // Verify ns1 still has 3 interfaces (no change)
        let ns1_interfaces = ns1.list_interfaces().expect("Failed to list interfaces");
        assert_eq!(ns1_interfaces.len(), 3, "ns1 should still have 3 interfaces");

        // Verify ns1 doesn't see wlan0
        let result = ns1.get_interface("wlan0");
        assert!(result.is_err(), "ns1 should not see wlan0");
    }

    #[test]
    fn test_multiple_interfaces_per_namespace() {
        let manager = NetworkNamespaceManager::new();
        let ns_id = manager.create_namespace(None).expect("Failed to create namespace");
        let ns_arc = manager.get_namespace(ns_id).expect("Failed to get namespace");
        let ns = ns_arc.lock().expect("Failed to lock namespace");

        // Add multiple interfaces
        ns.add_interface(NetworkInterface::new("eth0".to_string()))
            .expect("Failed to add eth0");
        ns.add_interface(NetworkInterface::new("eth1".to_string()))
            .expect("Failed to add eth1");
        ns.add_interface(NetworkInterface::new("eth2".to_string()))
            .expect("Failed to add eth2");
        ns.add_interface(NetworkInterface::new("lo".to_string()))
            .expect("Failed to add lo");

        // List and verify
        let interfaces = ns.list_interfaces().expect("Failed to list interfaces");
        assert_eq!(interfaces.len(), 4);
        assert!(interfaces.contains(&"eth0".to_string()));
        assert!(interfaces.contains(&"eth1".to_string()));
        assert!(interfaces.contains(&"eth2".to_string()));
        assert!(interfaces.contains(&"lo".to_string()));
    }

    #[test]
    fn test_interface_duplicate_prevention() {
        let manager = NetworkNamespaceManager::new();
        let ns_id = manager.create_namespace(None).expect("Failed to create namespace");
        let ns_arc = manager.get_namespace(ns_id).expect("Failed to get namespace");
        let ns = ns_arc.lock().expect("Failed to lock namespace");

        let iface = NetworkInterface::new("eth0".to_string());
        ns.add_interface(iface).expect("Failed to add eth0 first time");

        // Try to add duplicate
        let iface_dup = NetworkInterface::new("eth0".to_string());
        let result = ns.add_interface(iface_dup);
        assert!(result.is_err(), "Should prevent duplicate interface");
    }

    // ============================================================================
    // ROUTING TABLE ISOLATION TESTS
    // ============================================================================

    #[test]
    fn test_route_isolation_between_namespaces() {
        let manager = NetworkNamespaceManager::new();

        let ns1_id = manager.create_namespace(None).expect("Failed to create ns1");
        let ns2_id = manager.create_namespace(None).expect("Failed to create ns2");

        let ns1_arc = manager.get_namespace(ns1_id).expect("Failed to get ns1");
        let ns2_arc = manager.get_namespace(ns2_id).expect("Failed to get ns2");

        let ns1 = ns1_arc.lock().expect("Failed to lock ns1");
        let ns2 = ns2_arc.lock().expect("Failed to lock ns2");

        // Add route to ns1
        let route1 = Route::new(
            IpAddr::V4(Ipv4Addr::new(10, 0, 0, 0)),
            24,
            "eth0".to_string(),
        );
        ns1.add_route(route1).expect("Failed to add route to ns1");

        // Add different route to ns2
        let route2 = Route::new(
            IpAddr::V4(Ipv4Addr::new(172, 16, 0, 0)),
            16,
            "eth1".to_string(),
        );
        ns2.add_route(route2).expect("Failed to add route to ns2");

        // Verify isolation
        let routes1 = ns1.get_routes().expect("Failed to get routes from ns1");
        let routes2 = ns2.get_routes().expect("Failed to get routes from ns2");

        assert_eq!(routes1.len(), 1, "ns1 should have 1 route");
        assert_eq!(routes2.len(), 1, "ns2 should have 1 route");

        // Verify route content
        if let IpAddr::V4(dst) = routes1[0].destination {
            assert_eq!(dst, Ipv4Addr::new(10, 0, 0, 0));
        }

        if let IpAddr::V4(dst) = routes2[0].destination {
            assert_eq!(dst, Ipv4Addr::new(172, 16, 0, 0));
        }
    }

    #[test]
    fn test_routing_table_consistency() {
        let manager = NetworkNamespaceManager::new();
        let ns_id = manager.create_namespace(None).expect("Failed to create namespace");
        let ns_arc = manager.get_namespace(ns_id).expect("Failed to get namespace");
        let ns = ns_arc.lock().expect("Failed to lock namespace");

        // Add multiple routes
        let routes = vec![
            Route::new(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 0)), 8, "eth0".to_string()),
            Route::new(IpAddr::V4(Ipv4Addr::new(192, 168, 0, 0)), 16, "eth1".to_string()),
            Route::new(IpAddr::V4(Ipv4Addr::new(172, 16, 0, 0)), 12, "eth2".to_string()),
        ];

        for route in routes.iter() {
            ns.add_route(route.clone()).expect("Failed to add route");
        }

        // Verify all routes exist
        let stored_routes = ns.get_routes().expect("Failed to get routes");
        assert_eq!(stored_routes.len(), 3, "Should have 3 routes");

        // Verify specific routes
        let has_10_0_0_0 = stored_routes.iter()
            .any(|r| if let IpAddr::V4(ip) = r.destination {
                ip == Ipv4Addr::new(10, 0, 0, 0)
            } else { false });
        assert!(has_10_0_0_0, "Should have route for 10.0.0.0");

        let has_172_16 = stored_routes.iter()
            .any(|r| if let IpAddr::V4(ip) = r.destination {
                ip == Ipv4Addr::new(172, 16, 0, 0)
            } else { false });
        assert!(has_172_16, "Should have route for 172.16.0.0");
    }

    #[test]
    fn test_route_metrics_preserved() {
        let manager = NetworkNamespaceManager::new();
        let ns_id = manager.create_namespace(None).expect("Failed to create namespace");
        let ns_arc = manager.get_namespace(ns_id).expect("Failed to get namespace");
        let ns = ns_arc.lock().expect("Failed to lock namespace");

        let mut route = Route::new(
            IpAddr::V4(Ipv4Addr::new(10, 0, 0, 0)),
            24,
            "eth0".to_string(),
        );
        route.metric = 50;

        ns.add_route(route).expect("Failed to add route");

        let routes = ns.get_routes().expect("Failed to get routes");
        assert_eq!(routes[0].metric, 50, "Route metric should be preserved");
    }

    // ============================================================================
    // FIREWALL RULE ISOLATION TESTS
    // ============================================================================

    #[test]
    fn test_firewall_rule_isolation() {
        let manager = NetworkNamespaceManager::new();

        let ns1_id = manager.create_namespace(None).expect("Failed to create ns1");
        let ns2_id = manager.create_namespace(None).expect("Failed to create ns2");

        let ns1_arc = manager.get_namespace(ns1_id).expect("Failed to get ns1");
        let ns2_arc = manager.get_namespace(ns2_id).expect("Failed to get ns2");

        let ns1 = ns1_arc.lock().expect("Failed to lock ns1");
        let ns2 = ns2_arc.lock().expect("Failed to lock ns2");

        // Add ALLOW rule to ns1
        let rule1 = FirewallRule::new(FirewallAction::Allow);
        ns1.add_firewall_rule(rule1).expect("Failed to add rule to ns1");

        // Add DENY rule to ns2
        let rule2 = FirewallRule::new(FirewallAction::Deny);
        ns2.add_firewall_rule(rule2).expect("Failed to add rule to ns2");

        // Verify isolation
        let rules1 = ns1.get_firewall_rules().expect("Failed to get rules from ns1");
        let rules2 = ns2.get_firewall_rules().expect("Failed to get rules from ns2");

        assert_eq!(rules1.len(), 1);
        assert_eq!(rules2.len(), 1);
        assert_eq!(rules1[0].action, FirewallAction::Allow);
        assert_eq!(rules2[0].action, FirewallAction::Deny);
    }

    #[test]
    fn test_firewall_rule_with_addresses() {
        let manager = NetworkNamespaceManager::new();
        let ns_id = manager.create_namespace(None).expect("Failed to create namespace");
        let ns_arc = manager.get_namespace(ns_id).expect("Failed to get namespace");
        let ns = ns_arc.lock().expect("Failed to lock namespace");

        let mut rule = FirewallRule::new(FirewallAction::Allow);
        rule.src_addr = Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 0)));
        rule.dst_addr = Some(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 0)));
        rule.src_port = Some(8080);
        rule.dst_port = Some(80);
        rule.protocol = Some(6); // TCP

        ns.add_firewall_rule(rule).expect("Failed to add rule");

        let rules = ns.get_firewall_rules().expect("Failed to get rules");
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].src_port, Some(8080));
        assert_eq!(rules[0].dst_port, Some(80));
        assert_eq!(rules[0].protocol, Some(6));
    }

    #[test]
    fn test_multiple_firewall_rules() {
        let manager = NetworkNamespaceManager::new();
        let ns_id = manager.create_namespace(None).expect("Failed to create namespace");
        let ns_arc = manager.get_namespace(ns_id).expect("Failed to get namespace");
        let ns = ns_arc.lock().expect("Failed to lock namespace");

        // Add multiple rules with different actions
        for i in 0..5 {
            let action = if i % 2 == 0 {
                FirewallAction::Allow
            } else {
                FirewallAction::Deny
            };
            let rule = FirewallRule::new(action);
            ns.add_firewall_rule(rule).expect("Failed to add rule");
        }

        let rules = ns.get_firewall_rules().expect("Failed to get rules");
        assert_eq!(rules.len(), 5, "Should have 5 firewall rules");

        // Verify alternating pattern
        assert_eq!(rules[0].action, FirewallAction::Allow);
        assert_eq!(rules[1].action, FirewallAction::Deny);
        assert_eq!(rules[2].action, FirewallAction::Allow);
        assert_eq!(rules[3].action, FirewallAction::Deny);
        assert_eq!(rules[4].action, FirewallAction::Allow);
    }

    // ============================================================================
    // SOCKET SYSCALLS TESTS
    // ============================================================================

    #[test]
    fn test_socket_creation_with_namespace() {
        let syscalls = NetworkSyscalls::new();
        let ns_id = NetworkNamespaceId::new(1);

        let fd = syscalls.sys_socket(AF_INET, SOCK_STREAM, IPPROTO_TCP, ns_id)
            .expect("Failed to create socket");
        
        assert_ne!(fd.raw(), 0, "Socket FD should be valid");
    }

    #[test]
    fn test_socket_bind_to_address() {
        let syscalls = NetworkSyscalls::new();
        let ns_id = NetworkNamespaceId::new(1);

        let fd = syscalls.sys_socket(AF_INET, SOCK_STREAM, IPPROTO_TCP, ns_id)
            .expect("Failed to create socket");

        let addr = SockAddr::new_ipv4(Ipv4Addr::new(127, 0, 0, 1), 8080);
        syscalls.sys_bind(fd, addr, ns_id).expect("Failed to bind socket");

        // Verify binding
        let bound_addr = syscalls.sys_getsockname(fd, ns_id)
            .expect("Failed to get socket name");
        assert_eq!(bound_addr.port, 8080);
    }

    #[test]
    fn test_socket_listen_and_accept() {
        let syscalls = NetworkSyscalls::new();
        let ns_id = NetworkNamespaceId::new(1);

        let fd = syscalls.sys_socket(AF_INET, SOCK_STREAM, IPPROTO_TCP, ns_id)
            .expect("Failed to create socket");

        let addr = SockAddr::new_ipv4(Ipv4Addr::new(127, 0, 0, 1), 8080);
        syscalls.sys_bind(fd, addr, ns_id).expect("Failed to bind");
        syscalls.sys_listen(fd, 5, ns_id).expect("Failed to listen");

        let (conn_fd, peer_addr) = syscalls.sys_accept(fd, ns_id)
            .expect("Failed to accept");
        
        assert_ne!(conn_fd.raw(), fd.raw(), "Connection FD should be different");
        assert_ne!(peer_addr.port, 0, "Peer address should be set");
    }

    #[test]
    fn test_socket_connect() {
        let syscalls = NetworkSyscalls::new();
        let ns_id = NetworkNamespaceId::new(1);

        let fd = syscalls.sys_socket(AF_INET, SOCK_STREAM, IPPROTO_TCP, ns_id)
            .expect("Failed to create socket");

        let addr = SockAddr::new_ipv4(Ipv4Addr::new(127, 0, 0, 1), 8080);
        syscalls.sys_connect(fd, addr, ns_id).expect("Failed to connect");

        // Verify connection
        let peer = syscalls.sys_getpeername(fd, ns_id)
            .expect("Failed to get peer name");
        assert_eq!(peer.port, 8080);
    }

    // ============================================================================
    // NAMESPACE SOCKET ISOLATION TESTS
    // ============================================================================

    #[test]
    fn test_socket_isolation_across_namespaces() {
        let syscalls = NetworkSyscalls::new();
        let ns1 = NetworkNamespaceId::new(1);
        let ns2 = NetworkNamespaceId::new(2);

        let fd1 = syscalls.sys_socket(AF_INET, SOCK_STREAM, IPPROTO_TCP, ns1)
            .expect("Failed to create socket in ns1");
        let fd2 = syscalls.sys_socket(AF_INET, SOCK_STREAM, IPPROTO_TCP, ns2)
            .expect("Failed to create socket in ns2");

        // Same FD number in different namespaces is allowed
        assert_eq!(fd1.raw(), fd2.raw());

        // Bind to different addresses
        let addr1 = SockAddr::new_ipv4(Ipv4Addr::new(127, 0, 0, 1), 8080);
        let addr2 = SockAddr::new_ipv4(Ipv4Addr::new(127, 0, 0, 1), 9090);

        syscalls.sys_bind(fd1, addr1, ns1).expect("Failed to bind in ns1");
        syscalls.sys_bind(fd2, addr2, ns2).expect("Failed to bind in ns2");

        // Verify different bindings
        let bound1 = syscalls.sys_getsockname(fd1, ns1)
            .expect("Failed to get name from ns1");
        let bound2 = syscalls.sys_getsockname(fd2, ns2)
            .expect("Failed to get name from ns2");

        assert_eq!(bound1.port, 8080, "ns1 socket should be on port 8080");
        assert_eq!(bound2.port, 9090, "ns2 socket should be on port 9090");
    }

    #[test]
    fn test_socket_not_visible_across_namespaces() {
        let syscalls = NetworkSyscalls::new();
        let ns1 = NetworkNamespaceId::new(1);
        let ns2 = NetworkNamespaceId::new(2);

        let fd = syscalls.sys_socket(AF_INET, SOCK_STREAM, IPPROTO_TCP, ns1)
            .expect("Failed to create socket in ns1");

        // ns2 should not see the socket from ns1
        let result = syscalls.sys_getsockname(fd, ns2);
        assert!(result.is_err(), "ns2 should not see socket from ns1");
    }

    #[test]
    fn test_multiple_sockets_in_namespace() {
        let syscalls = NetworkSyscalls::new();
        let ns_id = NetworkNamespaceId::new(1);

        // Create multiple sockets
        let fds: Vec<_> = (0..5)
            .map(|_| syscalls.sys_socket(AF_INET, SOCK_STREAM, IPPROTO_TCP, ns_id)
                .expect("Failed to create socket"))
            .collect();

        // All should have different FD numbers
        for i in 0..fds.len() {
            for j in (i + 1)..fds.len() {
                assert_ne!(fds[i].raw(), fds[j].raw(), "FDs should be unique");
            }
        }
    }

    // ============================================================================
    // EDGE CASES AND ERROR HANDLING
    // ============================================================================

    #[test]
    fn test_invalid_socket_domain() {
        let syscalls = NetworkSyscalls::new();
        let ns_id = NetworkNamespaceId::new(1);

        let result = syscalls.sys_socket(999, SOCK_STREAM, IPPROTO_TCP, ns_id);
        assert!(result.is_err(), "Should reject invalid domain");
    }

    #[test]
    fn test_invalid_socket_type() {
        let syscalls = NetworkSyscalls::new();
        let ns_id = NetworkNamespaceId::new(1);

        let result = syscalls.sys_socket(AF_INET, 999, IPPROTO_TCP, ns_id);
        assert!(result.is_err(), "Should reject invalid socket type");
    }

    #[test]
    fn test_bind_without_socket() {
        let syscalls = NetworkSyscalls::new();
        let ns_id = NetworkNamespaceId::new(1);
        let fd = SocketFd::new(999); // Non-existent socket

        let addr = SockAddr::new_ipv4(Ipv4Addr::new(127, 0, 0, 1), 8080);
        let result = syscalls.sys_bind(fd, addr, ns_id);
        assert!(result.is_err(), "Should fail to bind non-existent socket");
    }

    #[test]
    fn test_listen_without_bind() {
        let syscalls = NetworkSyscalls::new();
        let ns_id = NetworkNamespaceId::new(1);

        let fd = syscalls.sys_socket(AF_INET, SOCK_STREAM, IPPROTO_TCP, ns_id)
            .expect("Failed to create socket");

        let result = syscalls.sys_listen(fd, 5, ns_id);
        assert!(result.is_err(), "Should fail to listen without binding");
    }

    #[test]
    fn test_listen_on_dgram_socket() {
        let syscalls = NetworkSyscalls::new();
        let ns_id = NetworkNamespaceId::new(1);

        let fd = syscalls.sys_socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP, ns_id)
            .expect("Failed to create UDP socket");

        let addr = SockAddr::new_ipv4(Ipv4Addr::new(127, 0, 0, 1), 8080);
        syscalls.sys_bind(fd, addr, ns_id).expect("Failed to bind");

        let result = syscalls.sys_listen(fd, 5, ns_id);
        assert!(result.is_err(), "Should not allow listen on SOCK_DGRAM");
    }

    #[test]
    fn test_accept_without_listen() {
        let syscalls = NetworkSyscalls::new();
        let ns_id = NetworkNamespaceId::new(1);

        let fd = syscalls.sys_socket(AF_INET, SOCK_STREAM, IPPROTO_TCP, ns_id)
            .expect("Failed to create socket");

        let result = syscalls.sys_accept(fd, ns_id);
        assert!(result.is_err(), "Should fail to accept on non-listening socket");
    }

    // ============================================================================
    // HIERARCHICAL NAMESPACE TESTS
    // ============================================================================

    #[test]
    fn test_hierarchical_namespaces_with_interfaces() {
        let manager = NetworkNamespaceManager::new();

        let parent_id = manager.create_namespace(None)
            .expect("Failed to create parent namespace");
        let child_id = manager.create_namespace(Some(parent_id))
            .expect("Failed to create child namespace");

        let parent_arc = manager.get_namespace(parent_id).expect("Failed to get parent");
        let child_arc = manager.get_namespace(child_id).expect("Failed to get child");

        let parent = parent_arc.lock().expect("Failed to lock parent");
        let child = child_arc.lock().expect("Failed to lock child");

        // Add interfaces to both
        parent.add_interface(NetworkInterface::new("eth0".to_string()))
            .expect("Failed to add eth0 to parent");
        child.add_interface(NetworkInterface::new("eth0".to_string()))
            .expect("Failed to add eth0 to child");

        // Verify they are separate
        let parent_ifaces = parent.list_interfaces().expect("Failed to list parent");
        let child_ifaces = child.list_interfaces().expect("Failed to list child");

        assert_eq!(parent_ifaces.len(), 1);
        assert_eq!(child_ifaces.len(), 1);

        // Parent should know its parent
        assert_eq!(child.parent_id(), Some(parent_id));
        assert_eq!(parent.parent_id(), None);
    }

    // ============================================================================
    // CROSS-NAMESPACE COMMUNICATION SCENARIOS
    // ============================================================================

    #[test]
    fn test_virtual_bridge_connection() {
        let manager = NetworkNamespaceManager::new();

        let ns1_id = manager.create_namespace(None).expect("Failed to create ns1");
        let ns2_id = manager.create_namespace(None).expect("Failed to create ns2");

        let ns1_arc = manager.get_namespace(ns1_id).expect("Failed to get ns1");
        let ns2_arc = manager.get_namespace(ns2_id).expect("Failed to get ns2");

        let ns1 = ns1_arc.lock().expect("Failed to lock ns1");
        let ns2 = ns2_arc.lock().expect("Failed to lock ns2");

        // Create virtual bridge in ns1
        ns1.create_virtual_bridge("br0".to_string())
            .expect("Failed to create bridge");

        // Verify bridge was created
        let bridge_result = ns1.get_virtual_bridge("br0");
        assert!(bridge_result.is_ok(), "Bridge should exist");

        // ns2 should not see the bridge
        let bridge_result_ns2 = ns2.get_virtual_bridge("br0");
        assert!(bridge_result_ns2.is_err(), "ns2 should not see ns1's bridge");
    }

    #[test]
    fn test_namespace_reference_counting() {
        let manager = NetworkNamespaceManager::new();
        let ns_id = manager.create_namespace(None).expect("Failed to create namespace");

        let ns_arc = manager.get_namespace(ns_id).expect("Failed to get namespace");
        let ns = ns_arc.lock().expect("Failed to lock namespace");

        // Initial refcount should be 1
        let initial_refcount = ns.refcount();
        assert_eq!(initial_refcount, 1);

        // Increment refcount
        ns.incref();
        let after_incref = ns.refcount();
        assert_eq!(after_incref, 2);

        // Decrement refcount
        let after_decref = ns.decref();
        assert_eq!(after_decref, 2);
    }

    // ============================================================================
    // STRESS AND SCALE TESTS
    // ============================================================================

    #[test]
    fn test_many_namespaces() {
        let manager = NetworkNamespaceManager::new();

        // Create 100 namespaces
        let mut ns_ids = Vec::new();
        for _ in 0..100 {
            let ns_id = manager.create_namespace(None)
                .expect("Failed to create namespace");
            ns_ids.push(ns_id);
        }

        let count = manager.count().expect("Failed to get count");
        assert_eq!(count, 100, "Should have 100 namespaces");

        // Verify each namespace is independent
        for (i, ns_id) in ns_ids.iter().enumerate() {
            let ns_arc = manager.get_namespace(*ns_id).expect("Failed to get namespace");
            let ns = ns_arc.lock().expect("Failed to lock namespace");

            ns.add_interface(NetworkInterface::new(format!("eth{}", i)))
                .expect("Failed to add interface");
        }

        // Verify each namespace has only its own interface
        for (i, ns_id) in ns_ids.iter().enumerate() {
            let ns_arc = manager.get_namespace(*ns_id).expect("Failed to get namespace");
            let ns = ns_arc.lock().expect("Failed to lock namespace");

            let ifaces = ns.list_interfaces().expect("Failed to list interfaces");
            assert_eq!(ifaces.len(), 1, "Each namespace should have 1 interface");
            assert_eq!(ifaces[0], format!("eth{}", i));
        }
    }

    #[test]
    fn test_many_interfaces_per_namespace() {
        let manager = NetworkNamespaceManager::new();
        let ns_id = manager.create_namespace(None).expect("Failed to create namespace");
        let ns_arc = manager.get_namespace(ns_id).expect("Failed to get namespace");
        let ns = ns_arc.lock().expect("Failed to lock namespace");

        // Add 50 interfaces
        for i in 0..50 {
            ns.add_interface(NetworkInterface::new(format!("iface{}", i)))
                .expect("Failed to add interface");
        }

        let ifaces = ns.list_interfaces().expect("Failed to list interfaces");
        assert_eq!(ifaces.len(), 50, "Should have 50 interfaces");
    }

    #[test]
    fn test_many_routes_per_namespace() {
        let manager = NetworkNamespaceManager::new();
        let ns_id = manager.create_namespace(None).expect("Failed to create namespace");
        let ns_arc = manager.get_namespace(ns_id).expect("Failed to get namespace");
        let ns = ns_arc.lock().expect("Failed to lock namespace");

        // Add 50 routes
        for i in 0..50 {
            let ip = Ipv4Addr::new(10, i as u8, 0, 0);
            let route = Route::new(
                IpAddr::V4(ip),
                16,
                format!("eth{}", i % 5),
            );
            ns.add_route(route).expect("Failed to add route");
        }

        let routes = ns.get_routes().expect("Failed to get routes");
        assert_eq!(routes.len(), 50, "Should have 50 routes");
    }

    #[test]
    fn test_many_firewall_rules_per_namespace() {
        let manager = NetworkNamespaceManager::new();
        let ns_id = manager.create_namespace(None).expect("Failed to create namespace");
        let ns_arc = manager.get_namespace(ns_id).expect("Failed to get namespace");
        let ns = ns_arc.lock().expect("Failed to lock namespace");

        // Add 50 firewall rules
        for i in 0..50 {
            let action = if i % 3 == 0 {
                FirewallAction::Allow
            } else if i % 3 == 1 {
                FirewallAction::Deny
            } else {
                FirewallAction::Drop
            };
            let rule = FirewallRule::new(action);
            ns.add_firewall_rule(rule).expect("Failed to add rule");
        }

        let rules = ns.get_firewall_rules().expect("Failed to get rules");
        assert_eq!(rules.len(), 50, "Should have 50 firewall rules");
    }
}

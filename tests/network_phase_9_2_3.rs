//! Network Interface & Routing Management Tests - Phase 9.2.3
//!
//! Standalone test suite that focuses on the new network_syscalls and
//! extended network_namespace functionality.
//! 
//! These tests validate:
//! - Socket syscalls with namespace support
//! - Network namespace isolation
//! - Interface, route, and firewall management
//! - Cross-namespace communication scenarios

use std::net::{IpAddr, Ipv4Addr};

// Import from sigmaos::net module
use sigmaos::net::network_namespace::{
    NetworkNamespaceManager, NetworkNamespaceId, NetworkInterface, Route, FirewallRule,
    FirewallAction,
};

use sigmaos::net::network_syscalls::{
    NetworkSyscalls, SocketFd, SockAddr,
    AF_INET, SOCK_STREAM, SOCK_DGRAM, IPPROTO_TCP, IPPROTO_UDP,
};

#[test]
fn test_basic_namespace_creation() {
    let manager = NetworkNamespaceManager::new();
    let ns_id = manager.create_namespace(None).expect("Failed to create namespace");
    assert_ne!(ns_id.raw(), 0);
}

#[test]
fn test_interface_management() {
    let manager = NetworkNamespaceManager::new();
    let ns_id = manager.create_namespace(None).expect("Failed to create namespace");
    let ns_arc = manager.get_namespace(ns_id).expect("Failed to get namespace");
    let ns = ns_arc.lock().expect("Failed to lock namespace");

    let iface = NetworkInterface::new("eth0".to_string());
    ns.add_interface(iface).expect("Failed to add interface");

    let interfaces = ns.list_interfaces().expect("Failed to list interfaces");
    assert_eq!(interfaces.len(), 1);
    assert!(interfaces.contains(&"eth0".to_string()));
}

#[test]
fn test_route_management() {
    let manager = NetworkNamespaceManager::new();
    let ns_id = manager.create_namespace(None).expect("Failed to create namespace");
    let ns_arc = manager.get_namespace(ns_id).expect("Failed to get namespace");
    let ns = ns_arc.lock().expect("Failed to lock namespace");

    let route = Route::new(
        IpAddr::V4(Ipv4Addr::new(10, 0, 0, 0)),
        24,
        "eth0".to_string(),
    );
    ns.add_route(route).expect("Failed to add route");

    let routes = ns.get_routes().expect("Failed to get routes");
    assert_eq!(routes.len(), 1);
}

#[test]
fn test_firewall_rule_management() {
    let manager = NetworkNamespaceManager::new();
    let ns_id = manager.create_namespace(None).expect("Failed to create namespace");
    let ns_arc = manager.get_namespace(ns_id).expect("Failed to get namespace");
    let ns = ns_arc.lock().expect("Failed to lock namespace");

    let rule = FirewallRule::new(FirewallAction::Allow);
    ns.add_firewall_rule(rule).expect("Failed to add rule");

    let rules = ns.get_firewall_rules().expect("Failed to get rules");
    assert_eq!(rules.len(), 1);
    assert_eq!(rules[0].action, FirewallAction::Allow);
}

#[test]
fn test_socket_syscall_creation() {
    let syscalls = NetworkSyscalls::new();
    let ns_id = NetworkNamespaceId::new(1);

    let fd = syscalls.sys_socket(AF_INET, SOCK_STREAM, IPPROTO_TCP, ns_id)
        .expect("Failed to create socket");
    
    assert_ne!(fd.raw(), 0);
}

#[test]
fn test_socket_bind() {
    let syscalls = NetworkSyscalls::new();
    let ns_id = NetworkNamespaceId::new(1);

    let fd = syscalls.sys_socket(AF_INET, SOCK_STREAM, IPPROTO_TCP, ns_id)
        .expect("Failed to create socket");

    let addr = SockAddr::new_ipv4(Ipv4Addr::new(127, 0, 0, 1), 8080);
    syscalls.sys_bind(fd, addr, ns_id).expect("Failed to bind");
}

#[test]
fn test_socket_listen() {
    let syscalls = NetworkSyscalls::new();
    let ns_id = NetworkNamespaceId::new(1);

    let fd = syscalls.sys_socket(AF_INET, SOCK_STREAM, IPPROTO_TCP, ns_id)
        .expect("Failed to create socket");

    let addr = SockAddr::new_ipv4(Ipv4Addr::new(127, 0, 0, 1), 8080);
    syscalls.sys_bind(fd, addr, ns_id).expect("Failed to bind");
    syscalls.sys_listen(fd, 5, ns_id).expect("Failed to listen");
}

#[test]
fn test_socket_accept() {
    let syscalls = NetworkSyscalls::new();
    let ns_id = NetworkNamespaceId::new(1);

    let fd = syscalls.sys_socket(AF_INET, SOCK_STREAM, IPPROTO_TCP, ns_id)
        .expect("Failed to create socket");

    let addr = SockAddr::new_ipv4(Ipv4Addr::new(127, 0, 0, 1), 8080);
    syscalls.sys_bind(fd, addr, ns_id).expect("Failed to bind");
    syscalls.sys_listen(fd, 5, ns_id).expect("Failed to listen");

    let (conn_fd, _peer_addr) = syscalls.sys_accept(fd, ns_id)
        .expect("Failed to accept");
    
    assert_ne!(conn_fd.raw(), fd.raw());
}

#[test]
fn test_socket_connect() {
    let syscalls = NetworkSyscalls::new();
    let ns_id = NetworkNamespaceId::new(1);

    let fd = syscalls.sys_socket(AF_INET, SOCK_STREAM, IPPROTO_TCP, ns_id)
        .expect("Failed to create socket");

    let addr = SockAddr::new_ipv4(Ipv4Addr::new(127, 0, 0, 1), 8080);
    syscalls.sys_connect(fd, addr, ns_id).expect("Failed to connect");
}

#[test]
fn test_namespace_isolation_interfaces() {
    let manager = NetworkNamespaceManager::new();

    let ns1_id = manager.create_namespace(None).expect("Failed to create ns1");
    let ns2_id = manager.create_namespace(None).expect("Failed to create ns2");

    let ns1_arc = manager.get_namespace(ns1_id).expect("Failed to get ns1");
    let ns2_arc = manager.get_namespace(ns2_id).expect("Failed to get ns2");

    let ns1 = ns1_arc.lock().expect("Failed to lock ns1");
    let ns2 = ns2_arc.lock().expect("Failed to lock ns2");

    ns1.add_interface(NetworkInterface::new("eth0".to_string()))
        .expect("Failed to add eth0 to ns1");

    let ns1_ifaces = ns1.list_interfaces().expect("Failed to list ns1");
    let ns2_ifaces = ns2.list_interfaces().expect("Failed to list ns2");

    assert_eq!(ns1_ifaces.len(), 1);
    assert_eq!(ns2_ifaces.len(), 0);
}

#[test]
fn test_namespace_isolation_routes() {
    let manager = NetworkNamespaceManager::new();

    let ns1_id = manager.create_namespace(None).expect("Failed to create ns1");
    let ns2_id = manager.create_namespace(None).expect("Failed to create ns2");

    let ns1_arc = manager.get_namespace(ns1_id).expect("Failed to get ns1");
    let ns2_arc = manager.get_namespace(ns2_id).expect("Failed to get ns2");

    let ns1 = ns1_arc.lock().expect("Failed to lock ns1");
    let ns2 = ns2_arc.lock().expect("Failed to lock ns2");

    let route1 = Route::new(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 0)), 24, "eth0".to_string());
    let route2 = Route::new(IpAddr::V4(Ipv4Addr::new(172, 16, 0, 0)), 16, "eth1".to_string());

    ns1.add_route(route1).expect("Failed to add route to ns1");
    ns2.add_route(route2).expect("Failed to add route to ns2");

    let ns1_routes = ns1.get_routes().expect("Failed to get ns1 routes");
    let ns2_routes = ns2.get_routes().expect("Failed to get ns2 routes");

    assert_eq!(ns1_routes.len(), 1);
    assert_eq!(ns2_routes.len(), 1);
}

#[test]
fn test_namespace_isolation_firewall() {
    let manager = NetworkNamespaceManager::new();

    let ns1_id = manager.create_namespace(None).expect("Failed to create ns1");
    let ns2_id = manager.create_namespace(None).expect("Failed to create ns2");

    let ns1_arc = manager.get_namespace(ns1_id).expect("Failed to get ns1");
    let ns2_arc = manager.get_namespace(ns2_id).expect("Failed to get ns2");

    let ns1 = ns1_arc.lock().expect("Failed to lock ns1");
    let ns2 = ns2_arc.lock().expect("Failed to lock ns2");

    ns1.add_firewall_rule(FirewallRule::new(FirewallAction::Allow))
        .expect("Failed to add rule to ns1");
    ns2.add_firewall_rule(FirewallRule::new(FirewallAction::Deny))
        .expect("Failed to add rule to ns2");

    let ns1_rules = ns1.get_firewall_rules().expect("Failed to get ns1 rules");
    let ns2_rules = ns2.get_firewall_rules().expect("Failed to get ns2 rules");

    assert_eq!(ns1_rules[0].action, FirewallAction::Allow);
    assert_eq!(ns2_rules[0].action, FirewallAction::Deny);
}

#[test]
fn test_socket_isolation_across_namespaces() {
    let syscalls = NetworkSyscalls::new();
    let ns1 = NetworkNamespaceId::new(1);
    let ns2 = NetworkNamespaceId::new(2);

    let fd1 = syscalls.sys_socket(AF_INET, SOCK_STREAM, IPPROTO_TCP, ns1)
        .expect("Failed to create socket in ns1");
    let fd2 = syscalls.sys_socket(AF_INET, SOCK_STREAM, IPPROTO_TCP, ns2)
        .expect("Failed to create socket in ns2");

    let addr1 = SockAddr::new_ipv4(Ipv4Addr::new(127, 0, 0, 1), 8080);
    let addr2 = SockAddr::new_ipv4(Ipv4Addr::new(127, 0, 0, 1), 9090);

    syscalls.sys_bind(fd1, addr1, ns1).expect("Failed to bind in ns1");
    syscalls.sys_bind(fd2, addr2, ns2).expect("Failed to bind in ns2");

    let bound1 = syscalls.sys_getsockname(fd1, ns1)
        .expect("Failed to get socket name from ns1");
    let bound2 = syscalls.sys_getsockname(fd2, ns2)
        .expect("Failed to get socket name from ns2");

    assert_eq!(bound1.port, 8080);
    assert_eq!(bound2.port, 9090);
}

#[test]
fn test_hierarchical_namespaces() {
    let manager = NetworkNamespaceManager::new();

    let parent_id = manager.create_namespace(None).expect("Failed to create parent");
    let child_id = manager.create_namespace(Some(parent_id)).expect("Failed to create child");

    let child_arc = manager.get_namespace(child_id).expect("Failed to get child");
    let child = child_arc.lock().expect("Failed to lock child");

    assert_eq!(child.parent_id(), Some(parent_id));
}

#[test]
fn test_many_interfaces() {
    let manager = NetworkNamespaceManager::new();
    let ns_id = manager.create_namespace(None).expect("Failed to create namespace");
    let ns_arc = manager.get_namespace(ns_id).expect("Failed to get namespace");
    let ns = ns_arc.lock().expect("Failed to lock namespace");

    for i in 0..20 {
        ns.add_interface(NetworkInterface::new(format!("eth{}", i)))
            .expect("Failed to add interface");
    }

    let ifaces = ns.list_interfaces().expect("Failed to list interfaces");
    assert_eq!(ifaces.len(), 20);
}

#[test]
fn test_many_routes() {
    let manager = NetworkNamespaceManager::new();
    let ns_id = manager.create_namespace(None).expect("Failed to create namespace");
    let ns_arc = manager.get_namespace(ns_id).expect("Failed to get namespace");
    let ns = ns_arc.lock().expect("Failed to lock namespace");

    for i in 0..20 {
        let ip = Ipv4Addr::new(10, i as u8, 0, 0);
        let route = Route::new(IpAddr::V4(ip), 16, format!("eth{}", i % 5));
        ns.add_route(route).expect("Failed to add route");
    }

    let routes = ns.get_routes().expect("Failed to get routes");
    assert_eq!(routes.len(), 20);
}

#[test]
fn test_many_firewall_rules() {
    let manager = NetworkNamespaceManager::new();
    let ns_id = manager.create_namespace(None).expect("Failed to create namespace");
    let ns_arc = manager.get_namespace(ns_id).expect("Failed to get namespace");
    let ns = ns_arc.lock().expect("Failed to lock namespace");

    for i in 0..20 {
        let action = if i % 2 == 0 {
            FirewallAction::Allow
        } else {
            FirewallAction::Deny
        };
        let rule = FirewallRule::new(action);
        ns.add_firewall_rule(rule).expect("Failed to add rule");
    }

    let rules = ns.get_firewall_rules().expect("Failed to get rules");
    assert_eq!(rules.len(), 20);
}

#[test]
fn test_socket_state_transitions() {
    let syscalls = NetworkSyscalls::new();
    let ns_id = NetworkNamespaceId::new(1);

    let fd = syscalls.sys_socket(AF_INET, SOCK_STREAM, IPPROTO_TCP, ns_id)
        .expect("Failed to create socket");

    let addr = SockAddr::new_ipv4(Ipv4Addr::new(127, 0, 0, 1), 8080);
    syscalls.sys_bind(fd, addr, ns_id).expect("Failed to bind");
    syscalls.sys_listen(fd, 5, ns_id).expect("Failed to listen");
    syscalls.sys_close(fd, ns_id).expect("Failed to close");
}

#[test]
fn test_interface_with_ip_configuration() {
    let manager = NetworkNamespaceManager::new();
    let ns_id = manager.create_namespace(None).expect("Failed to create namespace");
    let ns_arc = manager.get_namespace(ns_id).expect("Failed to get namespace");
    let ns = ns_arc.lock().expect("Failed to lock namespace");

    let iface = NetworkInterface::new("eth0".to_string())
        .with_ip(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)));
    
    ns.add_interface(iface).expect("Failed to add interface");

    let retrieved = ns.get_interface("eth0").expect("Failed to get interface");
    let iface_lock = retrieved.lock().expect("Failed to lock interface");
    
    if let Some(IpAddr::V4(ip)) = iface_lock.ip_addr {
        assert_eq!(ip, Ipv4Addr::new(192, 168, 1, 1));
    }
}

#[test]
fn test_multiple_namespaces() {
    let manager = NetworkNamespaceManager::new();

    let mut ns_ids = Vec::new();
    for _ in 0..10 {
        let ns_id = manager.create_namespace(None).expect("Failed to create namespace");
        ns_ids.push(ns_id);
    }

    let count = manager.count().expect("Failed to get count");
    assert_eq!(count, 10);

    // Verify each namespace is independent
    for (i, ns_id) in ns_ids.iter().enumerate() {
        let ns_arc = manager.get_namespace(*ns_id).expect("Failed to get namespace");
        let ns = ns_arc.lock().expect("Failed to lock namespace");

        ns.add_interface(NetworkInterface::new(format!("eth{}", i)))
            .expect("Failed to add interface");
    }

    for (i, ns_id) in ns_ids.iter().enumerate() {
        let ns_arc = manager.get_namespace(*ns_id).expect("Failed to get namespace");
        let ns = ns_arc.lock().expect("Failed to lock namespace");

        let ifaces = ns.list_interfaces().expect("Failed to list interfaces");
        assert_eq!(ifaces.len(), 1);
        assert_eq!(ifaces[0], format!("eth{}", i));
    }
}

#[test]
fn test_socket_error_cases() {
    let syscalls = NetworkSyscalls::new();
    let ns_id = NetworkNamespaceId::new(1);

    // Invalid domain
    assert!(syscalls.sys_socket(999, SOCK_STREAM, IPPROTO_TCP, ns_id).is_err());

    // Invalid socket type
    assert!(syscalls.sys_socket(AF_INET, 999, IPPROTO_TCP, ns_id).is_err());
}

#[test]
fn test_bind_error_cases() {
    let syscalls = NetworkSyscalls::new();
    let ns_id = NetworkNamespaceId::new(1);

    let fd = SocketFd::new(999); // Non-existent socket
    let addr = SockAddr::new_ipv4(Ipv4Addr::new(127, 0, 0, 1), 8080);

    // Should fail because socket doesn't exist
    assert!(syscalls.sys_bind(fd, addr, ns_id).is_err());
}

#[test]
fn test_multiple_sockets_per_namespace() {
    let syscalls = NetworkSyscalls::new();
    let ns_id = NetworkNamespaceId::new(1);

    let fd1 = syscalls.sys_socket(AF_INET, SOCK_STREAM, IPPROTO_TCP, ns_id)
        .expect("Failed to create TCP socket");
    let fd2 = syscalls.sys_socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP, ns_id)
        .expect("Failed to create UDP socket");

    assert_ne!(fd1.raw(), fd2.raw());
}

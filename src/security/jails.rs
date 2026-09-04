//! FreeBSD-style & Linux Container (OCI/cgroups) Hybrid Jails Isolation System for SigmaOS
//! Implements advanced OS-level virtualization by isolating directory subtrees, hostnames,
//! VNET network stacks, User/Mount namespaces, and resource quotas (FreeBSD rctl / cgroups v2).
use std::format;

use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};

/// Administrative capabilities inside a jail
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JailCapabilities {
    pub allow_raw_sockets: bool,
    pub allow_mounting: bool,
    pub allow_chflags: bool,
    pub allow_sysvipc: bool,
}

impl JailCapabilities {
    pub fn secure_default() -> Self {
        Self {
            allow_raw_sockets: false,
            allow_mounting: false,
            allow_chflags: false,
            allow_sysvipc: false,
        }
    }

    pub fn unrestricted() -> Self {
        Self {
            allow_raw_sockets: true,
            allow_mounting: true,
            allow_chflags: true,
            allow_sysvipc: true,
        }
    }
}

/// Route entry in VNET isolated routing table
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VnetRouteEntry {
    pub destination_cidr: String,
    pub gateway_ip: String,
    pub interface_name: String,
}

/// Virtual Interface Pair (epair - veth equivalent in FreeBSD VNET)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VnetEpairPair {
    pub host_side_if: String,
    pub jail_side_if: String,
    pub mac_host: [u8; 6],
    pub mac_jail: [u8; 6],
}

/// Fully virtualized FreeBSD VNET Network Stack (independent routing, sockets, ARP, epairs)
#[derive(Debug, Clone)]
pub struct VnetStack {
    pub vnet_id: u32,
    pub epairs: Vec<VnetEpairPair>,
    pub routing_table: Vec<VnetRouteEntry>,
    pub arp_cache: Vec<(String, [u8; 6])>,
    pub bound_sockets: Vec<(String, u16)>, // (IP, port)
}

impl VnetStack {
    pub fn new(vnet_id: u32) -> Self {
        Self {
            vnet_id,
            epairs: Vec::new(),
            routing_table: Vec::new(),
            arp_cache: Vec::new(),
            bound_sockets: Vec::new(),
        }
    }

    pub fn attach_epair(&mut self, epair: VnetEpairPair) {
        self.epairs.push(epair);
    }

    pub fn add_route(&mut self, cidr: &str, gateway: &str, if_name: &str) {
        self.routing_table.push(VnetRouteEntry {
            destination_cidr: cidr.to_string(),
            gateway_ip: gateway.to_string(),
            interface_name: if_name.to_string(),
        });
    }

    pub fn is_port_available(&self, ip: &str, port: u16) -> bool {
        !self
            .bound_sockets
            .iter()
            .any(|(s_ip, s_port)| s_ip == ip && *s_port == port)
    }

    pub fn bind_socket(&mut self, ip: &str, port: u16) -> Result<(), &'static str> {
        if !self.is_port_available(ip, port) {
            return Err("Port already bound in VNET stack");
        }
        self.bound_sockets.push((ip.to_string(), port));
        Ok(())
    }
}

/// FreeBSD VNET / Linux Network Namespace Virtual Network Config
#[derive(Debug, Clone)]
pub struct JailVnetConfig {
    pub is_vnet_enabled: bool,
    pub virtual_interface_name: String,
    pub bridge_name: String,
    pub isolated_gateway: String,
    pub stack: Option<VnetStack>,
}

impl JailVnetConfig {
    pub fn disabled() -> Self {
        Self {
            is_vnet_enabled: false,
            virtual_interface_name: String::new(),
            bridge_name: String::new(),
            isolated_gateway: String::new(),
            stack: None,
        }
    }

    pub fn new_vnet(if_name: &str, bridge: &str, gateway: &str) -> Self {
        Self {
            is_vnet_enabled: true,
            virtual_interface_name: if_name.to_string(),
            bridge_name: bridge.to_string(),
            isolated_gateway: gateway.to_string(),
            stack: Some(VnetStack::new(1)),
        }
    }
}

/// Resource Quotas & Limits (FreeBSD rctl / Linux cgroups v2 Parity)
#[derive(Debug, Clone, Copy)]
pub struct JailResourceLimits {
    pub max_cpu_percentage: u32,
    pub max_memory_mb: usize,
    pub max_pids: u32,
    pub max_iops: u32,
}

impl JailResourceLimits {
    pub fn unlimited() -> Self {
        Self {
            max_cpu_percentage: 100,
            max_memory_mb: usize::MAX,
            max_pids: u32::MAX,
            max_iops: u32::MAX,
        }
    }

    pub fn strict_sandbox(memory_mb: usize, max_pids: u32) -> Self {
        Self {
            max_cpu_percentage: 50,
            max_memory_mb: memory_mb,
            max_pids,
            max_iops: 1000,
        }
    }
}

/// Linux User & Mount Namespace Parity (UID/GID Mapping & Ephemeral OverlayFS)
#[derive(Debug, Clone)]
pub struct JailNamespaceIsolation {
    pub uid_offset: u32,
    pub gid_offset: u32,
    pub read_only_root_overlay: bool,
    pub ephemeral_tmpfs_size_mb: usize,
}

impl JailNamespaceIsolation {
    pub fn standard() -> Self {
        Self {
            uid_offset: 100_000,
            gid_offset: 100_000,
            read_only_root_overlay: true,
            ephemeral_tmpfs_size_mb: 128,
        }
    }
}

/// Structure representing a FreeBSD-style & OCI Hybrid Jail Container
#[derive(Debug, Clone)]
pub struct Jail {
    pub jid: u32,
    pub parent_jid: Option<u32>, // Parent JID for Hierarchical Sub-Jails (FreeBSD 13+)
    pub name: String,
    pub path_root: String,
    pub hostname: String,
    pub ip_addresses: Vec<String>,
    pub capabilities: JailCapabilities,
    pub vnet_config: JailVnetConfig,
    pub resource_limits: JailResourceLimits,
    pub namespace_config: JailNamespaceIsolation,
    pub active_processes_count: u32,
    pub current_memory_used_mb: usize,
}

impl Jail {
    pub fn new(
        jid: u32,
        name: &str,
        path_root: &str,
        hostname: &str,
        ip_addresses: Vec<String>,
        capabilities: JailCapabilities,
    ) -> Self {
        Self {
            jid,
            parent_jid: None,
            name: name.to_string(),
            path_root: path_root.to_string(),
            hostname: hostname.to_string(),
            ip_addresses,
            capabilities,
            vnet_config: JailVnetConfig::disabled(),
            resource_limits: JailResourceLimits::unlimited(),
            namespace_config: JailNamespaceIsolation::standard(),
            active_processes_count: 0,
            current_memory_used_mb: 0,
        }
    }

    /// Verifies if a given path is within the jail's isolated path root directory subtree
    pub fn verify_path_isolation(&self, path: &str) -> bool {
        path.starts_with(&self.path_root)
    }

    /// Verifies if a jail can bind to a specific IP address
    pub fn verify_ip_binding_allowed(&self, ip: &str) -> bool {
        self.ip_addresses.iter().any(|allowed_ip| allowed_ip == ip)
    }

    /// Checks resource limits against current process spawning request
    pub fn can_spawn_process(&self) -> bool {
        self.active_processes_count < self.resource_limits.max_pids
    }

    /// Checks memory quota against allocation request
    pub fn can_allocate_memory(&self, size_mb: usize) -> bool {
        self.current_memory_used_mb + size_mb <= self.resource_limits.max_memory_mb
    }
}

/// Global tracking manager for Jails
pub struct JailManager {
    pub jid_allocator: AtomicU32,
    pub active_jails: Vec<Jail>,
}

impl Default for JailManager {
    fn default() -> Self {
        Self::new()
    }
}

impl JailManager {
    pub fn new() -> Self {
        Self {
            jid_allocator: AtomicU32::new(1),
            active_jails: Vec::new(),
        }
    }

    /// Spawn/register a new top-level jail
    pub fn spawn_jail(
        &mut self,
        name: &str,
        path_root: &str,
        hostname: &str,
        ips: Vec<String>,
        caps: JailCapabilities,
    ) -> u32 {
        let jid = self.jid_allocator.fetch_add(1, Ordering::SeqCst);
        let jail = Jail::new(jid, name, path_root, hostname, ips, caps);
        self.active_jails.push(jail);
        jid
    }

    /// Spawn a hierarchical child sub-jail inside a parent jail (FreeBSD 13+ sub-jails)
    pub fn spawn_sub_jail(
        &mut self,
        parent_jid: u32,
        name: &str,
        relative_path: &str,
        hostname: &str,
        ips: Vec<String>,
        caps: JailCapabilities,
    ) -> Result<u32, &'static str> {
        let parent = self
            .lookup_jail(parent_jid)
            .ok_or("Parent jail not found")?
            .clone();
        let full_path = format!("{}/{}", parent.path_root, relative_path);

        let jid = self.jid_allocator.fetch_add(1, Ordering::SeqCst);
        let mut child_jail = Jail::new(jid, name, &full_path, hostname, ips, caps);
        child_jail.parent_jid = Some(parent_jid);
        // Inherit parent resource bounds
        child_jail.resource_limits = parent.resource_limits;

        self.active_jails.push(child_jail);
        Ok(jid)
    }

    /// Lookup a jail by its ID
    pub fn lookup_jail(&self, jid: u32) -> Option<&Jail> {
        self.active_jails.iter().find(|j| j.jid == jid)
    }

    /// Remove a jail and perform cascading termination of all its child sub-jails
    pub fn terminate_jail(&mut self, jid: u32) -> usize {
        let mut to_remove = vec![jid];
        let mut i = 0;

        while i < to_remove.len() {
            let current = to_remove[i];
            for jail in &self.active_jails {
                if jail.parent_jid == Some(current) && !to_remove.contains(&jail.jid) {
                    to_remove.push(jail.jid);
                }
            }
            i += 1;
        }

        let terminated_count = to_remove.len();
        self.active_jails.retain(|j| !to_remove.contains(&j.jid));
        terminated_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jail_resource_restrictions() {
        let mut manager = JailManager::new();

        let ips = vec!["192.168.1.50".to_string(), "10.0.0.5".to_string()];
        let jid = manager.spawn_jail(
            "strict_sandbox",
            "/jails/sandbox",
            "sandbox.local",
            ips,
            JailCapabilities::secure_default(),
        );

        let jail = manager.lookup_jail(jid).unwrap();
        assert_eq!(jail.name, "strict_sandbox");
        assert_eq!(jail.hostname, "sandbox.local");
        assert!(jail.verify_path_isolation("/jails/sandbox/usr/bin"));
        assert!(!jail.verify_path_isolation("/etc/passwd"));
        assert!(jail.verify_ip_binding_allowed("192.168.1.50"));
        assert!(!jail.verify_ip_binding_allowed("8.8.8.8"));
    }

    #[test]
    fn test_hierarchical_sub_jails() {
        let mut manager = JailManager::new();

        let parent_jid = manager.spawn_jail(
            "parent_jail",
            "/jails/parent",
            "parent.local",
            vec!["10.0.0.1".to_string()],
            JailCapabilities::secure_default(),
        );

        let child_jid = manager
            .spawn_sub_jail(
                parent_jid,
                "child_subjail",
                "child",
                "child.local",
                vec!["10.0.0.2".to_string()],
                JailCapabilities::secure_default(),
            )
            .unwrap();

        let child = manager.lookup_jail(child_jid).unwrap();
        assert_eq!(child.parent_jid, Some(parent_jid));
        assert_eq!(child.path_root, "/jails/parent/child");

        // Test cascading termination
        let count = manager.terminate_jail(parent_jid);
        assert_eq!(count, 2); // Parent and child both terminated
        assert!(manager.lookup_jail(parent_jid).is_none());
        assert!(manager.lookup_jail(child_jid).is_none());
    }

    #[test]
    fn test_vnet_and_resource_quotas() {
        let mut jail = Jail::new(
            1,
            "vnet_app",
            "/jails/vnet_app",
            "vnet.local",
            vec!["10.0.1.100".to_string()],
            JailCapabilities::secure_default(),
        );

        jail.vnet_config = JailVnetConfig::new_vnet("veth0", "bridge0", "10.0.1.1");
        jail.resource_limits = JailResourceLimits::strict_sandbox(512, 10);

        assert!(jail.vnet_config.is_vnet_enabled);
        assert_eq!(jail.vnet_config.virtual_interface_name, "veth0");

        assert!(jail.can_spawn_process());
        jail.active_processes_count = 10;
        assert!(!jail.can_spawn_process());

        assert!(jail.can_allocate_memory(256));
        assert!(!jail.can_allocate_memory(600));
    }
}

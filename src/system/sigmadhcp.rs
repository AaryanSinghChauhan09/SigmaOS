// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaDHCP - System DHCP Client
//!
//! SigmaDHCP provides system DHCP client functionality for SigmaOS, inspired by dhclient from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// DHCP state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DhcpState {
    /// Initializing
    Initializing,
    /// Selecting
    Selecting,
    /// Requesting
    Requesting,
    /// Bound
    Bound,
    /// Renewing
    Renewing,
    /// Rebinding
    Rebinding,
}

impl fmt::Display for DhcpState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DhcpState::Initializing => write!(f, "initializing"),
            DhcpState::Selecting => write!(f, "selecting"),
            DhcpState::Requesting => write!(f, "requesting"),
            DhcpState::Bound => write!(f, "bound"),
            DhcpState::Renewing => write!(f, "renewing"),
            DhcpState::Rebinding => write!(f, "rebinding"),
        }
    }
}

/// DHCP lease
#[derive(Debug, Clone)]
pub struct DhcpLease {
    /// Interface name
    pub interface: String,
    /// IP address
    pub ip_address: String,
    /// Subnet mask
    pub subnet_mask: String,
    /// Gateway
    pub gateway: String,
    /// DNS servers
    pub dns_servers: Vec<String>,
    /// Lease time in seconds
    pub lease_time: u32,
    /// State
    pub state: DhcpState,
}

impl DhcpLease {
    /// Create a new DHCP lease
    pub fn new(
        interface: String,
        ip_address: String,
        subnet_mask: String,
        gateway: String,
    ) -> Self {
        Self {
            interface,
            ip_address,
            subnet_mask,
            gateway,
            dns_servers: Vec::new(),
            lease_time: 0,
            state: DhcpState::Initializing,
        }
    }

    /// Set DNS servers
    pub fn with_dns_servers(mut self, dns_servers: Vec<String>) -> Self {
        self.dns_servers = dns_servers;
        self
    }

    /// Set lease time
    pub fn with_lease_time(mut self, lease_time: u32) -> Self {
        self.lease_time = lease_time;
        self
    }

    /// Set state
    pub fn with_state(mut self, state: DhcpState) -> Self {
        self.state = state;
        self
    }
}

/// SigmaDHCP - System DHCP Client
pub struct SigmaDHCP {
    /// DHCP leases (interface -> lease)
    leases: BTreeMap<String, DhcpLease>,
    /// Running state
    running: bool,
    /// Lease counter
    lease_counter: u32,
}

impl SigmaDHCP {
    /// Create a new SigmaDHCP instance
    pub fn new() -> Self {
        Self {
            leases: BTreeMap::new(),
            running: false,
            lease_counter: 0,
        }
    }

    /// Start the DHCP client
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the DHCP client
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the client is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Add a DHCP lease
    pub fn add_lease(&mut self, lease: DhcpLease) -> Result<(), String> {
        if self.leases.contains_key(&lease.interface) {
            return Err(format!(
                "Lease for interface '{}' already exists",
                lease.interface
            ));
        }

        self.leases.insert(lease.interface.clone(), lease);
        self.lease_counter += 1;
        Ok(())
    }

    /// Remove a DHCP lease
    pub fn remove_lease(&mut self, interface: &str) -> Result<(), String> {
        if self.leases.remove(interface).is_none() {
            return Err(format!("Lease for interface '{}' not found", interface));
        }
        Ok(())
    }

    /// Request a DHCP lease
    pub fn request_lease(&mut self, interface: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaDHCP not running".to_string());
        }

        match self.leases.get_mut(interface) {
            Some(lease) => {
                lease.state = DhcpState::Requesting;
                Ok(())
            }
            None => Err(format!("Lease for interface '{}' not found", interface)),
        }
    }

    /// Release a DHCP lease
    pub fn release_lease(&mut self, interface: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaDHCP not running".to_string());
        }

        match self.leases.get_mut(interface) {
            Some(lease) => {
                lease.state = DhcpState::Initializing;
                Ok(())
            }
            None => Err(format!("Lease for interface '{}' not found", interface)),
        }
    }

    /// Renew a DHCP lease
    pub fn renew_lease(&mut self, interface: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaDHCP not running".to_string());
        }

        match self.leases.get_mut(interface) {
            Some(lease) => {
                lease.state = DhcpState::Renewing;
                Ok(())
            }
            None => Err(format!("Lease for interface '{}' not found", interface)),
        }
    }

    /// Get a DHCP lease
    pub fn get_lease(&self, interface: &str) -> Option<&DhcpLease> {
        self.leases.get(interface)
    }

    /// List all leases
    pub fn list_leases(&self) -> Vec<&DhcpLease> {
        self.leases.values().collect()
    }

    /// List bound leases
    pub fn list_bound_leases(&self) -> Vec<&DhcpLease> {
        self.leases
            .values()
            .filter(|l| l.state == DhcpState::Bound)
            .collect()
    }

    /// Get lease count
    pub fn lease_count(&self) -> usize {
        self.leases.len()
    }

    /// Get DHCP client statistics
    pub fn dhcp_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert("total_leases".to_string(), self.lease_count().to_string());
        stats.insert(
            "bound_leases".to_string(),
            self.list_bound_leases().len().to_string(),
        );
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
}

impl Default for SigmaDHCP {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaDHCP instance (placeholder for actual global state)
static mut SIGMADHCP_INSTANCE: Option<SigmaDHCP> = None;

/// Initialize the global SigmaDHCP instance
pub fn init_sigmadhcp() {
    unsafe {
        SIGMADHCP_INSTANCE = Some(SigmaDHCP::new());
    }
}

/// Get the global SigmaDHCP instance
pub fn get_sigmadhcp() -> Option<&'static mut SigmaDHCP> {
    unsafe { SIGMADHCP_INSTANCE.as_mut() }
}

/// Convenience function to request a DHCP lease
pub fn request_dhcp_lease(interface: &str) -> Result<(), String> {
    if let Some(dhcp) = get_sigmadhcp() {
        dhcp.request_lease(interface)
    } else {
        Err("SigmaDHCP not initialized".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dhcp_state_display() {
        assert_eq!(DhcpState::Initializing.to_string(), "initializing");
        assert_eq!(DhcpState::Bound.to_string(), "bound");
    }

    #[test]
    fn test_dhcp_lease() {
        let lease = DhcpLease::new(
            "eth0".to_string(),
            "192.168.1.100".to_string(),
            "255.255.255.0".to_string(),
            "192.168.1.1".to_string(),
        )
        .with_dns_servers(vec!["8.8.8.8".to_string()])
        .with_lease_time(3600)
        .with_state(DhcpState::Bound);

        assert_eq!(lease.interface, "eth0");
        assert_eq!(lease.lease_time, 3600);
        assert_eq!(lease.state, DhcpState::Bound);
    }

    #[test]
    fn test_sigmadhcp() {
        let mut dhcp = SigmaDHCP::new();
        assert!(!dhcp.is_running());
        dhcp.start();
        assert!(dhcp.is_running());
        dhcp.stop();
        assert!(!dhcp.is_running());
    }

    #[test]
    fn test_sigmadhcp_add_lease() {
        let mut dhcp = SigmaDHCP::new();

        let lease = DhcpLease::new(
            "eth0".to_string(),
            "192.168.1.100".to_string(),
            "255.255.255.0".to_string(),
            "192.168.1.1".to_string(),
        );

        assert!(dhcp.add_lease(lease).is_ok());
        assert_eq!(dhcp.lease_count(), 1);
    }

    #[test]
    fn test_sigmadhcp_request_lease() {
        let mut dhcp = SigmaDHCP::new();
        dhcp.start();

        let lease = DhcpLease::new(
            "eth0".to_string(),
            "192.168.1.100".to_string(),
            "255.255.255.0".to_string(),
            "192.168.1.1".to_string(),
        );

        dhcp.add_lease(lease).unwrap();
        assert!(dhcp.request_lease("eth0").is_ok());

        let lease = dhcp.get_lease("eth0").unwrap();
        assert_eq!(lease.state, DhcpState::Requesting);
    }

    #[test]
    fn test_sigmadhcp_renew_lease() {
        let mut dhcp = SigmaDHCP::new();
        dhcp.start();

        let lease = DhcpLease::new(
            "eth0".to_string(),
            "192.168.1.100".to_string(),
            "255.255.255.0".to_string(),
            "192.168.1.1".to_string(),
        );

        dhcp.add_lease(lease).unwrap();
        assert!(dhcp.renew_lease("eth0").is_ok());

        let lease = dhcp.get_lease("eth0").unwrap();
        assert_eq!(lease.state, DhcpState::Renewing);
    }

    #[test]
    fn test_sigmadhcp_list_bound_leases() {
        let mut dhcp = SigmaDHCP::new();

        let lease = DhcpLease::new(
            "eth0".to_string(),
            "192.168.1.100".to_string(),
            "255.255.255.0".to_string(),
            "192.168.1.1".to_string(),
        )
        .with_state(DhcpState::Bound);

        dhcp.add_lease(lease).unwrap();

        let bound = dhcp.list_bound_leases();
        assert_eq!(bound.len(), 1);
    }

    #[test]
    fn test_sigmadhcp_stats() {
        let mut dhcp = SigmaDHCP::new();

        let lease = DhcpLease::new(
            "eth0".to_string(),
            "192.168.1.100".to_string(),
            "255.255.255.0".to_string(),
            "192.168.1.1".to_string(),
        )
        .with_state(DhcpState::Bound);

        dhcp.add_lease(lease).unwrap();

        let stats = dhcp.dhcp_stats();
        assert_eq!(stats.get("total_leases"), Some(&"1".to_string()));
        assert_eq!(stats.get("bound_leases"), Some(&"1".to_string()));
    }

    #[test]
    fn test_request_dhcp_lease_convenience() {
        init_sigmadhcp();

        let mut dhcp = get_sigmadhcp().unwrap();
        dhcp.start();

        let lease = DhcpLease::new(
            "eth0".to_string(),
            "192.168.1.100".to_string(),
            "255.255.255.0".to_string(),
            "192.168.1.1".to_string(),
        );

        dhcp.add_lease(lease).unwrap();

        let result = request_dhcp_lease("eth0");
        assert!(result.is_ok());
    }
}

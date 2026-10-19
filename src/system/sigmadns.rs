// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaDNS - System DNS Manager
//!
//! SigmaDNS provides system DNS management for SigmaOS, inspired by systemd-resolved from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// DNS record type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DnsRecordType {
    /// A record
    A,
    /// AAAA record
    AAAA,
    /// CNAME record
    CNAME,
    /// MX record
    MX,
    /// TXT record
    TXT,
}

impl fmt::Display for DnsRecordType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DnsRecordType::A => write!(f, "A"),
            DnsRecordType::AAAA => write!(f, "AAAA"),
            DnsRecordType::CNAME => write!(f, "CNAME"),
            DnsRecordType::MX => write!(f, "MX"),
            DnsRecordType::TXT => write!(f, "TXT"),
        }
    }
}

/// DNS resolver state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolverState {
    /// Active
    Active,
    /// Inactive
    Inactive,
}

impl fmt::Display for ResolverState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResolverState::Active => write!(f, "active"),
            ResolverState::Inactive => write!(f, "inactive"),
        }
    }
}

/// DNS server
#[derive(Debug, Clone)]
pub struct DnsServer {
    /// Server ID
    pub server_id: String,
    /// Server address
    pub address: String,
    /// Port
    pub port: u16,
    /// State
    pub state: ResolverState,
}

impl DnsServer {
    /// Create a new DNS server
    pub fn new(server_id: String, address: String, port: u16) -> Self {
        Self {
            server_id,
            address,
            port,
            state: ResolverState::Inactive,
        }
    }

    /// Set state
    pub fn with_state(mut self, state: ResolverState) -> Self {
        self.state = state;
        self
    }
}

/// DNS cache entry
#[derive(Debug, Clone)]
pub struct DnsCacheEntry {
    /// Domain name
    pub domain: String,
    /// Record type
    pub record_type: DnsRecordType,
    /// Record value
    pub value: String,
    /// TTL (time to live)
    pub ttl: u32,
}

impl DnsCacheEntry {
    /// Create a new DNS cache entry
    pub fn new(domain: String, record_type: DnsRecordType, value: String, ttl: u32) -> Self {
        Self {
            domain,
            record_type,
            value,
            ttl,
        }
    }
}

/// SigmaDNS - System DNS Manager
pub struct SigmaDNS {
    /// DNS servers (server_id -> server)
    servers: BTreeMap<String, DnsServer>,
    /// DNS cache (domain -> entries)
    cache: BTreeMap<String, Vec<DnsCacheEntry>>,
    /// Running state
    running: bool,
    /// Server counter
    server_counter: u32,
}

impl SigmaDNS {
    /// Create a new SigmaDNS instance
    pub fn new() -> Self {
        Self {
            servers: BTreeMap::new(),
            cache: BTreeMap::new(),
            running: false,
            server_counter: 0,
        }
    }

    /// Start the DNS manager
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the DNS manager
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Add a DNS server
    pub fn add_server(&mut self, server: DnsServer) -> Result<(), String> {
        if self.servers.contains_key(&server.server_id) {
            return Err(format!("Server '{}' already exists", server.server_id));
        }

        self.servers.insert(server.server_id.clone(), server);
        self.server_counter += 1;
        Ok(())
    }

    /// Remove a DNS server
    pub fn remove_server(&mut self, server_id: &str) -> Result<(), String> {
        if self.servers.remove(server_id).is_none() {
            return Err(format!("Server '{}' not found", server_id));
        }
        Ok(())
    }

    /// Activate a DNS server
    pub fn activate_server(&mut self, server_id: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaDNS not running".to_string());
        }

        match self.servers.get_mut(server_id) {
            Some(server) => {
                server.state = ResolverState::Active;
                Ok(())
            }
            None => Err(format!("Server '{}' not found", server_id)),
        }
    }

    /// Deactivate a DNS server
    pub fn deactivate_server(&mut self, server_id: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaDNS not running".to_string());
        }

        match self.servers.get_mut(server_id) {
            Some(server) => {
                server.state = ResolverState::Inactive;
                Ok(())
            }
            None => Err(format!("Server '{}' not found", server_id)),
        }
    }

    /// Add a DNS cache entry
    pub fn add_cache_entry(&mut self, entry: DnsCacheEntry) -> Result<(), String> {
        if !self.running {
            return Err("SigmaDNS not running".to_string());
        }

        self.cache
            .entry(entry.domain.clone())
            .or_insert_with(Vec::new)
            .push(entry);
        Ok(())
    }

    /// Get a DNS server
    pub fn get_server(&self, server_id: &str) -> Option<&DnsServer> {
        self.servers.get(server_id)
    }

    /// Get DNS cache entries for a domain
    pub fn get_cache_entries(&self, domain: &str) -> Option<&Vec<DnsCacheEntry>> {
        self.cache.get(domain)
    }

    /// List all servers
    pub fn list_servers(&self) -> Vec<&DnsServer> {
        self.servers.values().collect()
    }

    /// List active servers
    pub fn list_active_servers(&self) -> Vec<&DnsServer> {
        self.servers
            .values()
            .filter(|s| s.state == ResolverState::Active)
            .collect()
    }

    /// Get server count
    pub fn server_count(&self) -> usize {
        self.servers.len()
    }

    /// Get cache entry count
    pub fn cache_entry_count(&self) -> usize {
        self.cache.values().map(|v| v.len()).sum()
    }

    /// Get DNS manager statistics
    pub fn dns_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert("total_servers".to_string(), self.server_count().to_string());
        stats.insert(
            "active_servers".to_string(),
            self.list_active_servers().len().to_string(),
        );
        stats.insert(
            "cache_entries".to_string(),
            self.cache_entry_count().to_string(),
        );
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
}

impl Default for SigmaDNS {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaDNS instance (placeholder for actual global state)
static mut SIGMADNS_INSTANCE: Option<SigmaDNS> = None;

/// Initialize the global SigmaDNS instance
pub fn init_sigmadns() {
    unsafe {
        SIGMADNS_INSTANCE = Some(SigmaDNS::new());
    }
}

/// Get the global SigmaDNS instance
pub fn get_sigmadns() -> Option<&'static mut SigmaDNS> {
    unsafe { SIGMADNS_INSTANCE.as_mut() }
}

/// Convenience function to activate a DNS server
pub fn activate_dns_server(server_id: &str) -> Result<(), String> {
    if let Some(dns) = get_sigmadns() {
        dns.activate_server(server_id)
    } else {
        Err("SigmaDNS not initialized".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dns_record_type_display() {
        assert_eq!(DnsRecordType::A.to_string(), "A");
        assert_eq!(DnsRecordType::AAAA.to_string(), "AAAA");
    }

    #[test]
    fn test_resolver_state_display() {
        assert_eq!(ResolverState::Active.to_string(), "active");
        assert_eq!(ResolverState::Inactive.to_string(), "inactive");
    }

    #[test]
    fn test_dns_server() {
        let server = DnsServer::new("server1".to_string(), "8.8.8.8".to_string(), 53)
            .with_state(ResolverState::Active);

        assert_eq!(server.server_id, "server1");
        assert_eq!(server.state, ResolverState::Active);
    }

    #[test]
    fn test_dns_cache_entry() {
        let entry = DnsCacheEntry::new(
            "example.com".to_string(),
            DnsRecordType::A,
            "93.184.216.34".to_string(),
            300,
        );

        assert_eq!(entry.domain, "example.com");
        assert_eq!(entry.ttl, 300);
    }

    #[test]
    fn test_sigmadns() {
        let mut dns = SigmaDNS::new();
        assert!(!dns.is_running());
        dns.start();
        assert!(dns.is_running());
        dns.stop();
        assert!(!dns.is_running());
    }

    #[test]
    fn test_sigmadns_add_server() {
        let mut dns = SigmaDNS::new();

        let server = DnsServer::new("server1".to_string(), "8.8.8.8".to_string(), 53);

        assert!(dns.add_server(server).is_ok());
        assert_eq!(dns.server_count(), 1);
    }

    #[test]
    fn test_sigmadns_activate_server() {
        let mut dns = SigmaDNS::new();
        dns.start();

        let server = DnsServer::new("server1".to_string(), "8.8.8.8".to_string(), 53);

        dns.add_server(server).unwrap();
        assert!(dns.activate_server("server1").is_ok());

        let server = dns.get_server("server1").unwrap();
        assert_eq!(server.state, ResolverState::Active);
    }

    #[test]
    fn test_sigmadns_add_cache_entry() {
        let mut dns = SigmaDNS::new();
        dns.start();

        let entry = DnsCacheEntry::new(
            "example.com".to_string(),
            DnsRecordType::A,
            "93.184.216.34".to_string(),
            300,
        );

        assert!(dns.add_cache_entry(entry).is_ok());
        assert_eq!(dns.cache_entry_count(), 1);
    }

    #[test]
    fn test_sigmadns_list_active_servers() {
        let mut dns = SigmaDNS::new();

        let server = DnsServer::new("server1".to_string(), "8.8.8.8".to_string(), 53)
            .with_state(ResolverState::Active);

        dns.add_server(server).unwrap();

        let active = dns.list_active_servers();
        assert_eq!(active.len(), 1);
    }

    #[test]
    fn test_sigmadns_stats() {
        let mut dns = SigmaDNS::new();

        let server = DnsServer::new("server1".to_string(), "8.8.8.8".to_string(), 53)
            .with_state(ResolverState::Active);

        dns.add_server(server).unwrap();

        let stats = dns.dns_stats();
        assert_eq!(stats.get("total_servers"), Some(&"1".to_string()));
        assert_eq!(stats.get("active_servers"), Some(&"1".to_string()));
    }

    #[test]
    fn test_activate_dns_server_convenience() {
        init_sigmadns();

        let mut dns = get_sigmadns().unwrap();
        dns.start();

        let server = DnsServer::new("server1".to_string(), "8.8.8.8".to_string(), 53);

        dns.add_server(server).unwrap();

        let result = activate_dns_server("server1");
        assert!(result.is_ok());
    }
}

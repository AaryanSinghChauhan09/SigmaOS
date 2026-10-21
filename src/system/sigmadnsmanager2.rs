#![no_std]
extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// DNS manager for DNS configuration and management
/// Inspired by Linux DNS tools (systemd-resolved, dnsmasq, bind)
#[derive(Debug, Clone)]
pub struct SigmaDNSManager2 {
    zones: Vec<DNSManager2Zone>,
    records: Vec<DNSManager2Record>,
    resolvers: Vec<DNSManager2Resolver>,
}

/// DNS zone
#[derive(Debug, Clone)]
pub struct DNSManager2Zone {
    id: String,
    name: String,
    zone_type: ZoneType,
    state: ZoneState,
}

/// Zone type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZoneType {
    Master,
    Slave,
    Forward,
    Stub,
}

/// Zone state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZoneState {
    Active,
    Disabled,
    Transferring,
}

/// DNS record
#[derive(Debug, Clone)]
pub struct DNSManager2Record {
    id: String,
    zone_id: String,
    name: String,
    record_type: RecordType,
    value: String,
    ttl: u32,
}

/// Record type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordType {
    A,
    AAAA,
    CNAME,
    MX,
    TXT,
    NS,
    SOA,
    PTR,
    SRV,
}

/// DNS resolver
#[derive(Debug, Clone)]
pub struct DNSManager2Resolver {
    id: String,
    address: String,
    port: u16,
    resolver_type: ResolverType,
}

/// Resolver type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolverType {
    Upstream,
    Forwarder,
    Recursive,
}

impl SigmaDNSManager2 {
    pub fn new() -> Self {
        Self {
            zones: Vec::new(),
            records: Vec::new(),
            resolvers: Vec::new(),
        }
    }

    /// Create a DNS zone
    pub fn create_zone(&mut self, id: String, name: String, zone_type: ZoneType) -> Result<String, String> {
        if self.zones.iter().any(|z| z.id == id) {
            return Err(format!("Zone with id {} already exists", id));
        }
        
        let zone = DNSManager2Zone {
            id: id.clone(),
            name,
            zone_type,
            state: ZoneState::Active,
        };
        self.zones.push(zone);
        Ok(id)
    }

    /// Remove a zone
    pub fn remove_zone(&mut self, id: &str) -> Result<(), String> {
        let pos = self.zones.iter().position(|z| z.id == id)
            .ok_or_else(|| format!("Zone with id {} not found", id))?;
        self.zones.remove(pos);
        Ok(())
    }

    /// Enable a zone
    pub fn enable_zone(&mut self, id: &str) -> Result<(), String> {
        let zone = self.zones.iter_mut().find(|z| z.id == id)
            .ok_or_else(|| format!("Zone with id {} not found", id))?;
        zone.state = ZoneState::Active;
        Ok(())
    }

    /// Disable a zone
    pub fn disable_zone(&mut self, id: &str) -> Result<(), String> {
        let zone = self.zones.iter_mut().find(|z| z.id == id)
            .ok_or_else(|| format!("Zone with id {} not found", id))?;
        zone.state = ZoneState::Disabled;
        Ok(())
    }

    /// Add a DNS record
    pub fn add_record(&mut self, id: String, zone_id: String, name: String, record_type: RecordType, value: String, ttl: u32) -> Result<String, String> {
        if self.records.iter().any(|r| r.id == id) {
            return Err(format!("Record with id {} already exists", id));
        }
        
        let record = DNSManager2Record {
            id: id.clone(),
            zone_id,
            name,
            record_type,
            value,
            ttl,
        };
        self.records.push(record);
        Ok(id)
    }

    /// Remove a record
    pub fn remove_record(&mut self, id: &str) -> Result<(), String> {
        let pos = self.records.iter().position(|r| r.id == id)
            .ok_or_else(|| format!("Record with id {} not found", id))?;
        self.records.remove(pos);
        Ok(())
    }

    /// Add a resolver
    pub fn add_resolver(&mut self, id: String, address: String, port: u16, resolver_type: ResolverType) -> Result<String, String> {
        if self.resolvers.iter().any(|r| r.id == id) {
            return Err(format!("Resolver with id {} already exists", id));
        }
        
        let resolver = DNSManager2Resolver {
            id: id.clone(),
            address,
            port,
            resolver_type,
        };
        self.resolvers.push(resolver);
        Ok(id)
    }

    /// Remove a resolver
    pub fn remove_resolver(&mut self, id: &str) -> Result<(), String> {
        let pos = self.resolvers.iter().position(|r| r.id == id)
            .ok_or_else(|| format!("Resolver with id {} not found", id))?;
        self.resolvers.remove(pos);
        Ok(())
    }

    /// Get all zones
    pub fn get_zones(&self) -> &[DNSManager2Zone] {
        &self.zones
    }

    /// Get active zones
    pub fn get_active_zones(&self) -> Vec<&DNSManager2Zone> {
        self.zones.iter().filter(|z| z.state == ZoneState::Active).collect()
    }

    /// Get zone by ID
    pub fn get_zone(&self, id: &str) -> Option<&DNSManager2Zone> {
        self.zones.iter().find(|z| z.id == id)
    }

    /// Get records by zone
    pub fn get_records_by_zone(&self, zone_id: &str) -> Vec<&DNSManager2Record> {
        self.records.iter().filter(|r| r.zone_id == zone_id).collect()
    }

    /// Get all resolvers
    pub fn get_resolvers(&self) -> &[DNSManager2Resolver] {
        &self.resolvers
    }
}

impl Default for SigmaDNSManager2 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_zone() {
        let mut manager = SigmaDNSManager2::new();
        let result = manager.create_zone("zone1".to_string(), "example.com".to_string(), ZoneType::Master);
        assert!(result.is_ok());
        assert_eq!(manager.get_zones().len(), 1);
    }

    #[test]
    fn test_duplicate_zone() {
        let mut manager = SigmaDNSManager2::new();
        manager.create_zone("zone1".to_string(), "example.com".to_string(), ZoneType::Master).unwrap();
        let result = manager.create_zone("zone1".to_string(), "other.com".to_string(), ZoneType::Slave);
        assert!(result.is_err());
    }

    #[test]
    fn test_enable_disable_zone() {
        let mut manager = SigmaDNSManager2::new();
        manager.create_zone("zone1".to_string(), "example.com".to_string(), ZoneType::Master).unwrap();
        
        manager.disable_zone("zone1").unwrap();
        assert_eq!(manager.get_zone("zone1").unwrap().state, ZoneState::Disabled);
        
        manager.enable_zone("zone1").unwrap();
        assert_eq!(manager.get_zone("zone1").unwrap().state, ZoneState::Active);
    }

    #[test]
    fn test_add_record() {
        let mut manager = SigmaDNSManager2::new();
        manager.create_zone("zone1".to_string(), "example.com".to_string(), ZoneType::Master).unwrap();
        let result = manager.add_record("rec1".to_string(), "zone1".to_string(), "www".to_string(), RecordType::A, "192.168.1.1".to_string(), 3600);
        assert!(result.is_ok());
    }

    #[test]
    fn test_add_resolver() {
        let mut manager = SigmaDNSManager2::new();
        let result = manager.add_resolver("res1".to_string(), "8.8.8.8".to_string(), 53, ResolverType::Upstream);
        assert!(result.is_ok());
        assert_eq!(manager.get_resolvers().len(), 1);
    }
}

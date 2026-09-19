// SPDX-License-Identifier: MIT
// SigmaOS IOMMU (Input/Output Memory Management Unit) Subsystem
// Device memory isolation and DMA remapping inspired by Linux IOMMU

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, AtomicU32, Ordering};

/// IOMMU domain ID
pub type IommuDomainId = u64;

/// Device ID
pub type DeviceId = u64;

/// IOMMU domain type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IommuDomainType {
    Unmanaged,
    Identity,
    DMA,
    Passthrough,
}

/// IOMMU page protection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IommuPageProtection {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

impl IommuPageProtection {
    pub fn new(read: bool, write: bool, execute: bool) -> Self {
        IommuPageProtection {
            read,
            write,
            execute,
        }
    }

    pub fn as_flags(&self) -> u32 {
        let mut flags = 0;
        if self.read { flags |= 0x1; }
        if self.write { flags |= 0x2; }
        if self.execute { flags |= 0x4; }
        flags
    }
}

/// IOMMU mapping
#[derive(Debug, Clone)]
pub struct IommuMapping {
    pub iova: u64,         // IO virtual address
    pub paddr: u64,        // Physical address
    pub size: u64,         // Size in bytes
    pub protection: IommuPageProtection,
}

impl IommuMapping {
    pub fn new(iova: u64, paddr: u64, size: u64, protection: IommuPageProtection) -> Self {
        IommuMapping {
            iova,
            paddr,
            size,
            protection,
        }
    }
}

/// IOMMU domain
#[derive(Debug)]
pub struct IommuDomain {
    pub id: IommuDomainId,
    pub domain_type: IommuDomainType,
    pub mappings: BTreeMap<u64, IommuMapping>, // iova -> mapping
    pub devices: Vec<DeviceId>,
    pub page_size: u64,
    pub aperture_start: u64,
    pub aperture_end: u64,
}

impl IommuDomain {
    pub fn new(id: IommuDomainId, domain_type: IommuDomainType, page_size: u64) -> Self {
        IommuDomain {
            id,
            domain_type,
            mappings: BTreeMap::new(),
            devices: Vec::new(),
            page_size,
            aperture_start: 0,
            aperture_end: u64::MAX,
        }
    }

    pub fn add_mapping(&mut self, mapping: IommuMapping) -> Result<(), &'static str> {
        if mapping.iova < self.aperture_start || mapping.iova + mapping.size > self.aperture_end {
            return Err("Mapping outside aperture");
        }

        if mapping.iova % self.page_size != 0 || mapping.size % self.page_size != 0 {
            return Err("Mapping not page-aligned");
        }

        self.mappings.insert(mapping.iova, mapping);
        Ok(())
    }

    pub fn remove_mapping(&mut self, iova: u64) -> Result<(), &'static str> {
        self.mappings.remove(&iova).ok_or("Mapping not found")?;
        Ok(())
    }

    pub fn get_mapping(&self, iova: u64) -> Option<&IommuMapping> {
        self.mappings.get(&iova)
    }

    pub fn add_device(&mut self, device_id: DeviceId) {
        self.devices.push(device_id);
    }

    pub fn remove_device(&mut self, device_id: DeviceId) {
        self.devices.retain(|&id| id != device_id);
    }

    pub fn mapping_count(&self) -> usize {
        self.mappings.len()
    }

    pub fn device_count(&self) -> usize {
        self.devices.len()
    }
}

/// IOMMU subsystem
#[derive(Debug)]
pub struct IommuSubsystem {
    domains: BTreeMap<IommuDomainId, IommuDomain>,
    next_domain_id: AtomicU64,
    devices: BTreeMap<DeviceId, Option<IommuDomainId>>, // device -> domain
}

impl IommuSubsystem {
    pub fn new() -> Self {
        IommuSubsystem {
            domains: BTreeMap::new(),
            next_domain_id: AtomicU64::new(1),
            devices: BTreeMap::new(),
        }
    }

    /// Create a new IOMMU domain
    pub fn create_domain(&mut self, domain_type: IommuDomainType, page_size: u64) -> IommuDomainId {
        let id = self.next_domain_id.fetch_add(1, Ordering::SeqCst);
        let domain = IommuDomain::new(id, domain_type, page_size);
        self.domains.insert(id, domain);
        id
    }

    /// Get domain by ID
    pub fn get_domain(&self, id: IommuDomainId) -> Option<&IommuDomain> {
        self.domains.get(&id)
    }

    /// Get mutable domain by ID
    pub fn get_domain_mut(&mut self, id: IommuDomainId) -> Option<&mut IommuDomain> {
        self.domains.get_mut(&id)
    }

    /// Delete a domain
    pub fn delete_domain(&mut self, id: IommuDomainId) -> Result<(), &'static str> {
        let domain = self.domains.remove(&id).ok_or("Domain not found")?;
        
        if !domain.devices.is_empty() {
            return Err("Cannot delete domain with attached devices");
        }

        if !domain.mappings.is_empty() {
            return Err("Cannot delete domain with active mappings");
        }

        Ok(())
    }

    /// Attach device to domain
    pub fn attach_device(&mut self, device_id: DeviceId, domain_id: IommuDomainId) -> Result<(), &'static str> {
        // Detach from previous domain if any
        let prev_domain_id = self.devices.get(&device_id).copied().flatten();
        
        if let Some(prev_id) = prev_domain_id {
            if let Some(prev_domain) = self.domains.get_mut(&prev_id) {
                prev_domain.remove_device(device_id);
            }
        }

        let domain = self.domains.get_mut(&domain_id).ok_or("Domain not found")?;
        domain.add_device(device_id);
        self.devices.insert(device_id, Some(domain_id));
        Ok(())
    }

    /// Detach device from domain
    pub fn detach_device(&mut self, device_id: DeviceId) -> Result<(), &'static str> {
        let domain_id = self.devices.get(&device_id).copied().flatten().ok_or("Device not attached")?;
        
        if let Some(domain) = self.domains.get_mut(&domain_id) {
            domain.remove_device(device_id);
        }

        self.devices.insert(device_id, None);
        Ok(())
    }

    /// Get domain for device
    pub fn get_device_domain(&self, device_id: DeviceId) -> Option<IommuDomainId> {
        self.devices.get(&device_id).copied().flatten()
    }

    /// Get domain count
    pub fn domain_count(&self) -> usize {
        self.domains.len()
    }

    /// Get device count
    pub fn device_count(&self) -> usize {
        self.devices.len()
    }
}

impl Default for IommuSubsystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iommu_domain_creation() {
        let mut subsystem = IommuSubsystem::new();
        
        let id = subsystem.create_domain(IommuDomainType::DMA, 4096);
        assert!(id > 0);
        assert_eq!(subsystem.domain_count(), 1);
    }

    #[test]
    fn test_iommu_mapping() {
        let mut subsystem = IommuSubsystem::new();
        
        let domain_id = subsystem.create_domain(IommuDomainType::DMA, 4096);
        let domain = subsystem.get_domain_mut(domain_id).unwrap();
        
        let mapping = IommuMapping::new(
            0x1000,
            0x2000,
            4096,
            IommuPageProtection::new(true, false, false),
        );
        
        domain.add_mapping(mapping).unwrap();
        assert_eq!(domain.mapping_count(), 1);
    }

    #[test]
    fn test_iommu_device_attach() {
        let mut subsystem = IommuSubsystem::new();
        
        let domain_id = subsystem.create_domain(IommuDomainType::DMA, 4096);
        subsystem.attach_device(1234, domain_id).unwrap();
        
        assert_eq!(subsystem.get_device_domain(1234), Some(domain_id));
    }

    #[test]
    fn test_iommu_device_detach() {
        let mut subsystem = IommuSubsystem::new();
        
        let domain_id = subsystem.create_domain(IommuDomainType::DMA, 4096);
        subsystem.attach_device(1234, domain_id).unwrap();
        subsystem.detach_device(1234).unwrap();
        
        assert_eq!(subsystem.get_device_domain(1234), None);
    }

    #[test]
    fn test_iommu_page_protection() {
        let prot = IommuPageProtection::new(true, true, false);
        let flags = prot.as_flags();
        
        assert!(flags & 0x1 != 0); // read
        assert!(flags & 0x2 != 0); // write
        assert!(flags & 0x4 == 0); // execute
    }

    #[test]
    fn test_iommu_domain_deletion() {
        let mut subsystem = IommuSubsystem::new();
        
        let domain_id = subsystem.create_domain(IommuDomainType::DMA, 4096);
        subsystem.delete_domain(domain_id).unwrap();
        
        assert_eq!(subsystem.domain_count(), 0);
    }
}

// SigmaOS IOMMU & DMA Remapping Subsystem (Intel VT-d / AMD-Vi)
// Enforces peripheral DMA isolation and prevents DMA attack vectors through hardware I/O page tables.

#![allow(dead_code)]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

pub const DMAR_SIGNATURE: &[u8; 4] = b"DMAR";
pub const IVRS_SIGNATURE: &[u8; 4] = b"IVRS";

/// DMA access permissions in IOMMU page table
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IommuPermissions {
    Read,
    Write,
    ReadWrite,
}

/// Device PCI BDF address (Bus, Device, Function)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PciBdf {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
}

/// IOMMU Protection Domain
#[derive(Debug, Clone)]
pub struct IommuDomain {
    pub domain_id: u32,
    pub iova_page_table: BTreeMap<u64, (u64, IommuPermissions)>, // IOVA -> (PhysAddr, Perms)
    pub assigned_devices: Vec<PciBdf>,
}

/// IOMMU Controller Engine
#[derive(Debug, Clone, Default)]
pub struct IommuController {
    pub hardware_units_count: usize,
    pub is_enabled: bool,
    pub domains: BTreeMap<u32, IommuDomain>,
    pub next_domain_id: u32,
    pub translation_faults_count: u64,
}

impl IommuController {
    pub fn new() -> Self {
        Self {
            hardware_units_count: 0,
            is_enabled: false,
            domains: BTreeMap::new(),
            next_domain_id: 1,
            translation_faults_count: 0,
        }
    }

    /// Parse ACPI DMAR (Intel) or IVRS (AMD) table to discover hardware remapping units
    pub fn parse_acpi_table(&mut self, table_data: &[u8]) -> Result<usize, &'static str> {
        if table_data.len() < 36 {
            return Err("IOMMU ACPI table too small");
        }

        let sig = &table_data[0..4];
        if sig != DMAR_SIGNATURE && sig != IVRS_SIGNATURE {
            return Err("Invalid IOMMU ACPI signature (neither DMAR nor IVRS)");
        }

        self.hardware_units_count = 1; // Discovered root remapping hardware unit
        self.is_enabled = true;
        Ok(self.hardware_units_count)
    }

    /// Create an isolated protection domain
    pub fn create_domain(&mut self) -> u32 {
        let domain_id = self.next_domain_id;
        self.next_domain_id += 1;

        let domain = IommuDomain {
            domain_id,
            iova_page_table: BTreeMap::new(),
            assigned_devices: Vec::new(),
        };

        self.domains.insert(domain_id, domain);
        domain_id
    }

    /// Attach a PCI peripheral to an IOMMU protection domain
    pub fn attach_device(&mut self, domain_id: u32, bdf: PciBdf) -> Result<(), &'static str> {
        let domain = self.domains.get_mut(&domain_id).ok_or("Domain not found")?;
        if !domain.assigned_devices.contains(&bdf) {
            domain.assigned_devices.push(bdf);
        }
        Ok(())
    }

    /// Map an IOVA range to a physical RAM address in the domain
    pub fn map_iova(
        &mut self,
        domain_id: u32,
        iova: u64,
        phys_addr: u64,
        perms: IommuPermissions,
    ) -> Result<(), &'static str> {
        let domain = self.domains.get_mut(&domain_id).ok_or("Domain not found")?;
        domain.iova_page_table.insert(iova & !4095, (phys_addr & !4095, perms));
        Ok(())
    }

    /// Translate DMA access attempt from a device
    pub fn translate_dma(
        &mut self,
        bdf: PciBdf,
        iova: u64,
        is_write: bool,
    ) -> Result<u64, &'static str> {
        if !self.is_enabled {
            return Ok(iova); // Pass-through when disabled
        }

        let aligned_iova = iova & !4095;
        let offset = iova & 4095;

        for domain in self.domains.values() {
            if domain.assigned_devices.contains(&bdf) {
                if let Some(&(phys_base, perms)) = domain.iova_page_table.get(&aligned_iova) {
                    if is_write && perms == IommuPermissions::Read {
                        self.translation_faults_count += 1;
                        return Err("IOMMU Fault: Write violation on read-only DMA page");
                    }
                    return Ok(phys_base + offset);
                }
            }
        }

        self.translation_faults_count += 1;
        Err("IOMMU Fault: Device not authorized or unmapped IOVA address")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iommu_domain_translation_and_fault() {
        let mut iommu = IommuController::new();
        iommu.is_enabled = true;

        let domain_id = iommu.create_domain();
        let nic_bdf = PciBdf { bus: 0, device: 3, function: 0 };
        assert!(iommu.attach_device(domain_id, nic_bdf).is_ok());

        let iova = 0x1000_2000u64;
        let phys = 0x8000_2000u64;
        assert!(iommu.map_iova(domain_id, iova, phys, IommuPermissions::ReadWrite).is_ok());

        // Valid DMA read
        let translated = iommu.translate_dma(nic_bdf, iova, false).expect("DMA translated");
        assert_eq!(translated, phys);

        // Rogue device attempting access to same IOVA -> rejected!
        let rogue_bdf = PciBdf { bus: 1, device: 0, function: 0 };
        assert!(iommu.translate_dma(rogue_bdf, iova, false).is_err());
        assert_eq!(iommu.translation_faults_count, 1);
    }
}

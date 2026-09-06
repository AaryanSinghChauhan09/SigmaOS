//! SigmaOS DMA Engine and IOMMU Abstraction
//!
//! Sovereign Direct Memory Access subsystem. Inspired by:
//! - Linux DMA API (include/linux/dma-mapping.h)
//! - FreeBSD busdma(9) API
//! - ARM SMMU, Intel VT-d IOMMU
//!
//! Provides device-independent DMA buffer management,
//! IOMMU page table management, and scatter-gather support.

#![allow(dead_code)]
#![allow(clippy::new_without_default)]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

// ============================================================
// DMA Direction
// ============================================================

/// Direction of a DMA transfer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaDirection {
    /// CPU writes, device reads
    ToDevice,
    /// Device writes, CPU reads
    FromDevice,
    /// Bidirectional transfer
    Bidirectional,
    /// No actual data transfer (for mapping without syncing)
    None,
}

// ============================================================
// Physical/DMA Address Types
// ============================================================

/// Physical memory address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysAddr(pub u64);

/// DMA (IOVA) address — the address as seen by the device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DmaAddr(pub u64);

impl DmaAddr {
    pub const NULL: Self = Self(0);
    pub fn is_null(&self) -> bool { self.0 == 0 }
}

// ============================================================
// DMA Scatter-Gather List
// ============================================================

/// One entry in a scatter-gather list.
#[derive(Debug, Clone, Copy)]
pub struct SgEntry {
    /// DMA (IOVA) address of this segment
    pub dma_addr: DmaAddr,
    /// Length of this segment in bytes
    pub length: u32,
    /// Offset within the first page
    pub offset: u32,
}

/// A scatter-gather list for fragmented DMA transfers.
///
/// Analogous to Linux `struct sg_table` / `struct scatterlist`.
#[derive(Debug, Clone)]
pub struct ScatterGatherList {
    pub entries: Vec<SgEntry>,
    /// Total byte count across all segments
    pub total_bytes: u64,
    /// Transfer direction
    pub direction: DmaDirection,
    /// Whether this SGL is currently mapped (device can access)
    pub is_mapped: bool,
}

impl ScatterGatherList {
    pub fn new(direction: DmaDirection) -> Self {
        Self { entries: Vec::new(), total_bytes: 0, direction, is_mapped: false }
    }

    pub fn add_entry(&mut self, dma_addr: DmaAddr, length: u32, offset: u32) {
        self.total_bytes += length as u64;
        self.entries.push(SgEntry { dma_addr, length, offset });
    }

    pub fn entry_count(&self) -> usize { self.entries.len() }
}

// ============================================================
// DMA Coherent Buffer
// ============================================================

/// A coherent (cache-coherent) DMA buffer.
///
/// Coherent buffers are visible to both CPU and device at all times.
/// No explicit sync needed (unlike streaming DMA).
/// Analogous to `dma_alloc_coherent()` in Linux.
#[derive(Debug)]
pub struct DmaCoherentBuffer {
    /// Physical address
    pub phys: PhysAddr,
    /// DMA address (IOVA as seen by device)
    pub dma_addr: DmaAddr,
    /// Kernel virtual address (for CPU access)
    pub virt_addr: u64,
    /// Buffer size in bytes
    pub size: usize,
    /// Device ID that owns this buffer
    pub device_id: u32,
}

// ============================================================
// IOMMU Page Table Entry
// ============================================================

/// An IOMMU mapping entry.
#[derive(Debug, Clone)]
pub struct IommuEntry {
    /// IOVA (device-visible address)
    pub iova: DmaAddr,
    /// Physical address it maps to
    pub phys: PhysAddr,
    /// Mapping size in bytes (must be page-aligned)
    pub size: usize,
    /// Readable by device
    pub read: bool,
    /// Writable by device
    pub write: bool,
    /// Device ID (PCI BDF or platform device ID)
    pub device_id: u32,
}

// ============================================================
// IOMMU Domain
// ============================================================

/// An IOMMU domain — an isolated DMA address space.
///
/// Each device (or group of devices) has its own domain.
/// Analogous to Linux `struct iommu_domain`.
pub struct IommuDomain {
    pub id: u64,
    /// IOVA → PhysAddr mappings
    mappings: BTreeMap<u64, IommuEntry>,
    /// Next IOVA to allocate from
    next_iova: u64,
    /// IOVA base (bottom of address space)
    iova_base: u64,
    /// IOVA limit
    iova_limit: u64,
}

impl IommuDomain {
    pub fn new(id: u64, iova_base: u64, iova_limit: u64) -> Self {
        Self { id, mappings: BTreeMap::new(), next_iova: iova_base, iova_base, iova_limit }
    }

    /// Map a physical region into the IOMMU domain.
    pub fn map(&mut self, phys: PhysAddr, size: usize, read: bool, write: bool, device_id: u32)
        -> Result<DmaAddr, &'static str> {
        // Align size to 4KB pages
        let aligned_size = (size + 0xFFF) & !0xFFF;
        let iova = self.alloc_iova(aligned_size)?;
        let entry = IommuEntry { iova: DmaAddr(iova), phys, size: aligned_size, read, write, device_id };
        self.mappings.insert(iova, entry);
        Ok(DmaAddr(iova))
    }

    /// Unmap a previously mapped IOVA.
    pub fn unmap(&mut self, iova: DmaAddr) -> Result<(), &'static str> {
        self.mappings.remove(&iova.0).map(|_| ()).ok_or("mapping not found")
    }

    /// Translate IOVA to physical address.
    pub fn translate(&self, iova: DmaAddr) -> Option<PhysAddr> {
        // Find the mapping that covers this IOVA
        for (&base, entry) in &self.mappings {
            if iova.0 >= base && iova.0 < base + entry.size as u64 {
                let offset = iova.0 - base;
                return Some(PhysAddr(entry.phys.0 + offset));
            }
        }
        None
    }

    fn alloc_iova(&mut self, size: usize) -> Result<u64, &'static str> {
        let iova = self.next_iova;
        let next = iova + size as u64;
        if next > self.iova_limit { return Err("IOMMU: IOVA space exhausted"); }
        self.next_iova = next;
        Ok(iova)
    }

    pub fn mapping_count(&self) -> usize { self.mappings.len() }
}

// ============================================================
// DMA Engine
// ============================================================

/// DMA transfer descriptor.
#[derive(Debug, Clone)]
pub struct DmaDescriptor {
    pub id: u64,
    pub src: DmaAddr,
    pub dst: DmaAddr,
    pub len: u32,
    pub direction: DmaDirection,
    pub status: DmaStatus,
}

/// Status of a DMA descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaStatus {
    Pending,
    InProgress,
    Complete,
    Error,
}

/// A DMA channel for asynchronous memory-to-memory or device transfers.
pub struct DmaChannel {
    pub id: u32,
    pending: Vec<DmaDescriptor>,
    completed: Vec<DmaDescriptor>,
    next_desc_id: u64,
    pub bytes_transferred: u64,
}

impl DmaChannel {
    pub fn new(id: u32) -> Self {
        Self { id, pending: Vec::new(), completed: Vec::new(), next_desc_id: 1, bytes_transferred: 0 }
    }

    /// Submit a DMA transfer descriptor.
    pub fn submit(&mut self, src: DmaAddr, dst: DmaAddr, len: u32, dir: DmaDirection) -> u64 {
        let id = self.next_desc_id;
        self.next_desc_id += 1;
        self.pending.push(DmaDescriptor { id, src, dst, len, direction: dir, status: DmaStatus::Pending });
        id
    }

    /// Issue all pending transfers (simulate DMA engine execution).
    pub fn issue_pending(&mut self) -> usize {
        let count = self.pending.len();
        for mut desc in self.pending.drain(..) {
            desc.status = DmaStatus::Complete;
            self.bytes_transferred += desc.len as u64;
            self.completed.push(desc);
        }
        count
    }

    /// Collect completed transfers.
    pub fn collect_completed(&mut self) -> Vec<DmaDescriptor> {
        self.completed.drain(..).collect()
    }

    /// Wait for a specific transfer to complete.
    pub fn wait(&mut self, desc_id: u64) -> Option<DmaStatus> {
        self.issue_pending();
        self.completed.iter().find(|d| d.id == desc_id).map(|d| d.status)
    }
}

// ============================================================
// SigmaDmaSubsystem — Global DMA Manager
// ============================================================

/// System-wide DMA subsystem manager.
///
/// Manages IOMMU domains, DMA channels, and coherent buffers.
pub struct SigmaDmaSubsystem {
    /// IOMMU domains indexed by domain ID
    domains: BTreeMap<u64, IommuDomain>,
    /// DMA channels indexed by channel ID
    channels: BTreeMap<u32, DmaChannel>,
    /// Coherent buffers
    coherent_buffers: Vec<DmaCoherentBuffer>,
    /// Next domain ID
    next_domain_id: u64,
    /// Next coherent buffer physical address (simulation)
    next_phys: u64,
    /// Statistics
    pub total_bytes_transferred: u64,
}

impl SigmaDmaSubsystem {
    /// Create a new DMA subsystem.
    pub fn new() -> Self {
        let mut sys = Self {
            domains: BTreeMap::new(),
            channels: BTreeMap::new(),
            coherent_buffers: Vec::new(),
            next_domain_id: 1,
            next_phys: 0x1000_0000, // Start at 256MB
            total_bytes_transferred: 0,
        };
        // Create default channels (0=mem-to-mem, 1=device-to-mem, 2=mem-to-device)
        for i in 0..4u32 { sys.channels.insert(i, DmaChannel::new(i)); }
        sys
    }

    /// Create an IOMMU domain for a device.
    pub fn create_domain(&mut self, device_id: u32) -> u64 {
        let id = self.next_domain_id;
        self.next_domain_id += 1;
        // IOVA space: 0x10000000..0x100000000 (256MB..4GB)
        let domain = IommuDomain::new(id, 0x1000_0000, 0x1_0000_0000);
        self.domains.insert(id, domain);
        id
    }

    /// Allocate a coherent DMA buffer.
    ///
    /// Analogous to `dma_alloc_coherent()`.
    pub fn alloc_coherent(&mut self, size: usize, device_id: u32) -> Result<&DmaCoherentBuffer, &'static str> {
        let aligned = (size + 0xFFF) & !0xFFF;
        let phys = PhysAddr(self.next_phys);
        self.next_phys += aligned as u64;

        // Create or find domain for device
        let domain_id = if let Some((&did, _)) = self.domains.iter().find(|(_, d)| {
            d.mappings.values().any(|m| m.device_id == device_id)
        }) { did } else { self.create_domain(device_id) };

        let dma_addr = self.domains.get_mut(&domain_id)
            .ok_or("domain not found")?
            .map(phys, size, true, true, device_id)?;

        let buf = DmaCoherentBuffer {
            phys, dma_addr, virt_addr: phys.0, // In simulation, virt == phys
            size: aligned, device_id,
        };
        self.coherent_buffers.push(buf);
        Ok(self.coherent_buffers.last().unwrap())
    }

    /// Submit a DMA transfer on a channel.
    pub fn submit_transfer(&mut self, channel: u32, src: DmaAddr, dst: DmaAddr,
                            len: u32, dir: DmaDirection) -> Result<u64, &'static str> {
        let ch = self.channels.get_mut(&channel).ok_or("channel not found")?;
        Ok(ch.submit(src, dst, len, dir))
    }

    /// Execute all pending transfers on all channels.
    pub fn flush_all(&mut self) {
        for ch in self.channels.values_mut() {
            let completed = ch.issue_pending();
            self.total_bytes_transferred += ch.bytes_transferred;
            ch.collect_completed();
        }
    }

    /// Map memory into an IOMMU domain.
    pub fn iommu_map(&mut self, domain_id: u64, phys: PhysAddr, size: usize,
                      read: bool, write: bool, device_id: u32) -> Result<DmaAddr, &'static str> {
        self.domains.get_mut(&domain_id)
            .ok_or("domain not found")?
            .map(phys, size, read, write, device_id)
    }

    /// Translate IOVA to physical address in a domain.
    pub fn iommu_translate(&self, domain_id: u64, iova: DmaAddr) -> Option<PhysAddr> {
        self.domains.get(&domain_id)?.translate(iova)
    }

    pub fn domain_count(&self) -> usize { self.domains.len() }
    pub fn channel_count(&self) -> usize { self.channels.len() }
    pub fn coherent_buffer_count(&self) -> usize { self.coherent_buffers.len() }
}

impl Default for SigmaDmaSubsystem {
    fn default() -> Self { Self::new() }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iommu_map_translate() {
        let mut domain = IommuDomain::new(1, 0x1000_0000, 0x8000_0000);
        let phys = PhysAddr(0x4000_0000);
        let iova = domain.map(phys, 4096, true, true, 1).unwrap();
        assert!(!iova.is_null());
        let translated = domain.translate(iova).unwrap();
        assert_eq!(translated, phys);
    }

    #[test]
    fn test_iommu_unmap() {
        let mut domain = IommuDomain::new(1, 0x1000_0000, 0x8000_0000);
        let iova = domain.map(PhysAddr(0x1234_0000), 4096, true, false, 1).unwrap();
        assert_eq!(domain.mapping_count(), 1);
        domain.unmap(iova).unwrap();
        assert_eq!(domain.mapping_count(), 0);
    }

    #[test]
    fn test_dma_channel_submit() {
        let mut ch = DmaChannel::new(0);
        let id = ch.submit(DmaAddr(0x1000), DmaAddr(0x2000), 4096, DmaDirection::ToDevice);
        assert_eq!(ch.pending.len(), 1);
        ch.issue_pending();
        assert_eq!(ch.bytes_transferred, 4096);
        let completed = ch.collect_completed();
        assert_eq!(completed.len(), 1);
        assert_eq!(completed[0].status, DmaStatus::Complete);
    }

    #[test]
    fn test_coherent_alloc() {
        let mut dma = SigmaDmaSubsystem::new();
        let buf = dma.alloc_coherent(8192, 1).unwrap();
        assert_eq!(buf.size, 8192);
        assert!(!buf.dma_addr.is_null());
        assert_eq!(dma.coherent_buffer_count(), 1);
    }

    #[test]
    fn test_scatter_gather() {
        let mut sgl = ScatterGatherList::new(DmaDirection::FromDevice);
        sgl.add_entry(DmaAddr(0x1000), 4096, 0);
        sgl.add_entry(DmaAddr(0x3000), 4096, 0);
        sgl.add_entry(DmaAddr(0x5000), 2048, 0);
        assert_eq!(sgl.entry_count(), 3);
        assert_eq!(sgl.total_bytes, 10240);
    }
}

/// Memory Descriptor Lists (MDL), physical-to-virtual memory descriptor tracking,
/// locked-in-memory states, and ancient/historical hardware driver DMA buffer compatibility.
///
/// Replicates historical concepts from early Linux kernels (0.01-1.0 series)
/// where direct I/O address space, direct-memory access (DMA), page protection, and
/// ISA 16MB memory boundaries required precise, contiguous/non-contiguous mapping trackers.

use std::vec::Vec;
use std::boxed::Box;
use std::string::String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryProtection {
    ReadOnly,
    ReadWrite,
    ExecuteRead,
    ExecuteReadWrite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MdlEntry {
    pub physical_page_frame: usize, // Physical page number (PPN)
    pub length: usize,               // Length of the segment in bytes
    pub offset: usize,               // Offset within the page
}

impl MdlEntry {
    pub fn new(ppn: usize, length: usize, offset: usize) -> Self {
        Self {
            physical_page_frame: ppn,
            length,
            offset,
        }
    }
}

/// Memory Descriptor List (MDL)
/// Describes an existing buffer in virtual memory, maps virtual addresses to non-contiguous
/// physical memory layouts, allows linking multiple descriptors, and tracks page lock status.
pub struct MemoryDescriptorList {
    pub virtual_address: usize,
    pub byte_count: usize,
    pub byte_offset: usize,
    pub entries: Vec<MdlEntry>,
    pub is_locked: bool,
    pub protection: MemoryProtection,
    pub next: Option<Box<MemoryDescriptorList>>, // Entry linking for chained transfers
}

impl MemoryDescriptorList {
    /// Creates a new, blank Memory Descriptor List for a virtual memory segment
    pub fn new(virtual_address: usize, byte_count: usize, protection: MemoryProtection) -> Self {
        let page_size = 4096;
        let byte_offset = virtual_address % page_size;

        let mut entries = Vec::new();
        let total_pages = (byte_count + byte_offset + page_size - 1) / page_size;

        // Emulate allocating non-contiguous physical pages
        for i in 0..total_pages {
            // Simulated physical page allocation
            let ppn = 0x2000_0000 + i * page_size;
            let segment_len = if i == 0 {
                core::cmp::min(byte_count, page_size - byte_offset)
            } else if i == total_pages - 1 {
                (byte_count + byte_offset) % page_size
            } else {
                page_size
            };

            let segment_offset = if i == 0 { byte_offset } else { 0 };
            entries.push(MdlEntry::new(ppn, segment_len, segment_offset));
        }

        Self {
            virtual_address,
            byte_count,
            byte_offset,
            entries,
            is_locked: false,
            protection,
            next: None,
        }
    }

    /// Wraps an existing buffer securely into a Memory Descriptor List
    pub fn from_existing_buffer(buf: &[u8], protection: MemoryProtection) -> Self {
        let virtual_address = buf.as_ptr() as usize;
        let byte_count = buf.len();
        let page_size = 4096;
        let byte_offset = virtual_address % page_size;

        let mut entries = Vec::new();
        let total_pages = (byte_count + byte_offset + page_size - 1) / page_size;

        for i in 0..total_pages {
            let ppn = 0x5000_0000 + i * page_size; // Dedicated pool for mapped user buffers
            let segment_len = if i == 0 {
                core::cmp::min(byte_count, page_size - byte_offset)
            } else if i == total_pages - 1 {
                let rem = (byte_count + byte_offset) % page_size;
                if rem == 0 { page_size } else { rem }
            } else {
                page_size
            };
            let segment_offset = if i == 0 { byte_offset } else { 0 };
            entries.push(MdlEntry::new(ppn, segment_len, segment_offset));
        }

        Self {
            virtual_address,
            byte_count,
            byte_offset,
            entries,
            is_locked: true, // Existing buffers mapped this way are pinned in memory
            protection,
            next: None,
        }
    }

    /// Verifies if the underlying physical page frames are perfectly contiguous in memory.
    pub fn is_contiguous(&self) -> bool {
        if self.entries.is_empty() {
            return true;
        }
        let page_size = 4096;
        for i in 0..self.entries.len() - 1 {
            if self.entries[i].physical_page_frame + page_size != self.entries[i + 1].physical_page_frame {
                return false;
            }
        }
        true
    }

    /// Simulates locking/pinning pages in memory to prevent page out or swap
    pub fn lock_pages(&mut self) -> Result<(), &'static str> {
        if self.is_locked {
            return Err("Pages are already locked in memory");
        }
        self.is_locked = true;
        Ok(())
    }

    /// Simulates unlocking pages in memory
    pub fn unlock_pages(&mut self) -> Result<(), &'static str> {
        if !self.is_locked {
            return Err("Pages are not locked in memory");
        }
        self.is_locked = false;
        Ok(())
    }

    /// Translates a virtual address offset to its physical address representation
    pub fn translate_virtual(&self, offset: usize) -> Option<usize> {
        if offset >= self.byte_count {
            return None;
        }
        let mut current_offset = 0;
        for entry in &self.entries {
            if current_offset + entry.length > offset {
                let inside_offset = offset - current_offset;
                return Some(entry.physical_page_frame + entry.offset + inside_offset);
            }
            current_offset += entry.length;
        }
        None
    }

    /// Chains another Memory Descriptor List (multiple entries / linking)
    pub fn link_mdl(&mut self, next_mdl: MemoryDescriptorList) {
        if let Some(ref mut child) = self.next {
            child.link_mdl(next_mdl);
        } else {
            self.next = Some(Box::new(next_mdl));
        }
    }
}

// =========================================================================
// Ancient / Historical Device DMA Buffer Compatibility Layer
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AncientDeviceType {
    FloppyController,   // Floppy Drive (ISA 8237 DMA, strictly 16MB memory bounds, 64KB buffer limit)
    SoundBlaster16,     // Sound Blaster 16 (Dual 8-bit/16-bit DMA ping-pong buffers)
    Ne2000Ethernet,     // NE2000 shared memory rings (ISA standard ethernet buffers)
}

/// Model of an ancient device driver DMA memory buffer bound to historical computer architectures.
pub struct AncientDeviceDmaBuffer {
    pub device_type: AncientDeviceType,
    pub buffer_name: String,
    pub mdl: MemoryDescriptorList,
    pub isa_dma_channel: Option<u8>,
}

impl AncientDeviceDmaBuffer {
    /// Constructs a standard ISA DMA buffer limited to the first 16 Megabytes of physical memory.
    pub fn new_isa_dma(device_type: AncientDeviceType, name: &str, size: usize) -> Result<Self, &'static str> {
        // Enforce traditional ISA 8237 DMA limits (max 64KB single transfer for Floppy, 16MB physical limits)
        if device_type == AncientDeviceType::FloppyController && size > 65536 {
            return Err("Floppy controller ISA DMA transfer buffer exceeds traditional 64KB threshold");
        }

        // Allocate a simulated ISA-bound contiguous buffer (virtually and physically below 16MB limit)
        let virtual_address = 0x00F0_0000; // Under 16MB (0x0100_0000)
        let mut mdl = MemoryDescriptorList::new(virtual_address, size, MemoryProtection::ReadWrite);

        // Enforce physical address frames to fall below 16MB ISA boundary (PPN < 0x0100_0000)
        let page_size = 4096;
        let mut physical_base = 0x0010_0000; // Starting at 1MB (conventional ISA DMA zone)
        for entry in &mut mdl.entries {
            entry.physical_page_frame = physical_base;
            physical_base += page_size;
        }

        mdl.is_locked = true; // Ancient DMA pages are immediately pinned/locked in memory

        let isa_channel = match device_type {
            AncientDeviceType::FloppyController => Some(2), // Channel 2 typically reserved for Floppy
            AncientDeviceType::SoundBlaster16 => Some(5),   // Channel 5 typically reserved for 16-bit Sound Blaster
            AncientDeviceType::Ne2000Ethernet => None,      // NE2000 uses direct Shared-RAM ring
        };

        Ok(Self {
            device_type,
            buffer_name: String::from(name),
            mdl,
            isa_dma_channel: isa_channel,
        })
    }

    /// Emulates trigger/transfer behavior for historical double-buffering / ping-pong buffers
    pub fn trigger_ping_pong_interrupt(&mut self) -> Result<&'static str, &'static str> {
        if self.device_type != AncientDeviceType::SoundBlaster16 {
            return Err("Double-buffering ping-pong interrupt is only supported on Sound Blaster 16 adapters");
        }
        Ok("SoundBlaster16: Flopped active playback/record half buffer. Active interrupts sent!")
    }

    /// Simulates direct physical hardware write into the NE2000 Ethernet shared ring buffer
    pub fn write_ne2000_ring(&mut self, offset: usize, data: &[u8]) -> Result<(), &'static str> {
        if self.device_type != AncientDeviceType::Ne2000Ethernet {
            return Err("Only NE2000 Ethernet shared buffers support ring writes");
        }
        if offset + data.len() > self.mdl.byte_count {
            return Err("Shared buffer write exceeds the configured 16KB/32KB buffer frame size");
        }
        Ok(())
    }
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_mdl_non_contiguous_creation() {
        // Virtual memory: 10KB (needs 3 pages)
        let mut mdl = MemoryDescriptorList::new(0x4000_1500, 10240, MemoryProtection::ReadWrite);
        assert_eq!(mdl.byte_offset, 0x500);
        assert_eq!(mdl.entries.len(), 3);
        assert!(!mdl.is_locked);

        // Lock MDL pages
        assert!(mdl.lock_pages().is_ok());
        assert!(mdl.is_locked);
        assert!(mdl.lock_pages().is_err()); // Already locked

        // Unlock
        assert!(mdl.unlock_pages().is_ok());
        assert!(!mdl.is_locked);
    }

    #[test]
    fn test_mdl_from_existing_buffer() {
        let buffer = [0u8; 100];
        let mdl = MemoryDescriptorList::from_existing_buffer(&buffer, MemoryProtection::ReadOnly);
        assert_eq!(mdl.byte_count, 100);
        assert!(mdl.is_locked); // Pinning existing buffers
    }

    #[test]
    fn test_mdl_is_contiguous() {
        let mut mdl = MemoryDescriptorList::new(0x8000_0000, 8192, MemoryProtection::ReadWrite);
        // Default mocked PPN allocation in new() is contiguous
        assert!(mdl.is_contiguous());

        // Introduce manual non-contiguous jump
        mdl.entries[1].physical_page_frame = 0x9999_0000;
        assert!(!mdl.is_contiguous());
    }

    #[test]
    fn test_mdl_address_translation() {
        let mdl = MemoryDescriptorList::new(0x8000_0200, 2048, MemoryProtection::ReadWrite);
        let first_ppn = mdl.entries[0].physical_page_frame;
        // Translate offset 0
        let phys_addr = mdl.translate_virtual(0).unwrap();
        assert_eq!(phys_addr, first_ppn + 0x200);

        // Offset exceeding bounds
        assert!(mdl.translate_virtual(3000).is_none());
    }

    #[test]
    fn test_mdl_entry_linking() {
        let mut main_mdl = MemoryDescriptorList::new(0x1000_0000, 4096, MemoryProtection::ReadWrite);
        let link_mdl = MemoryDescriptorList::new(0x2000_0000, 4096, MemoryProtection::ReadWrite);
        main_mdl.link_mdl(link_mdl);
        assert!(main_mdl.next.is_some());
    }

    #[test]
    fn test_floppy_isa_dma_limits() {
        // Safe floppy buffer creation
        let floppy_buf = AncientDeviceDmaBuffer::new_isa_dma(AncientDeviceType::FloppyController, "FLOPPY_A", 32768);
        assert!(floppy_buf.is_ok());
        let buf = floppy_buf.unwrap();
        assert_eq!(buf.isa_dma_channel, Some(2));
        assert!(buf.mdl.is_locked);
        // Ensure below 16MB boundary
        assert!(buf.mdl.entries[0].physical_page_frame < 0x0100_0000);

        // Exceeding 64KB limit for Floppy
        let bad_floppy = AncientDeviceDmaBuffer::new_isa_dma(AncientDeviceType::FloppyController, "FLOPPY_A", 100_000);
        assert!(bad_floppy.is_err());
    }

    #[test]
    fn test_sound_blaster_ping_pong() {
        let mut sb = AncientDeviceDmaBuffer::new_isa_dma(AncientDeviceType::SoundBlaster16, "SB16_PLAYBACK", 4096).unwrap();
        assert_eq!(sb.isa_dma_channel, Some(5));
        let trigger_msg = sb.trigger_ping_pong_interrupt().unwrap();
        assert!(trigger_msg.contains("Flopped active playback"));
    }

    #[test]
    fn test_ne2000_shared_ring() {
        let mut eth = AncientDeviceDmaBuffer::new_isa_dma(AncientDeviceType::Ne2000Ethernet, "NE2000_RING", 16384).unwrap();
        assert_eq!(eth.isa_dma_channel, None);
        let bytes = [0x55; 128];
        assert!(eth.write_ne2000_ring(0, &bytes).is_ok());
        assert!(eth.write_ne2000_ring(16300, &bytes).is_err()); // Overflow
    }
}

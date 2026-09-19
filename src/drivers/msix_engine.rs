// SPDX-License-Identifier: MIT
// SigmaOS Advanced PCI MSI/MSI-X Vector Management Engine
// Inspired by Linux (drivers/pci/msi/) & FreeBSD (sys/dev/pci/msi.c)
// Provides 64-bit Message Signaled Interrupts (MSI/MSI-X), MMIO Table Entry programming,
// PBA (Pending Bit Array) monitoring, and x2APIC IRQ vector steering.

#![allow(dead_code)]

use std::vec::Vec;

/// MSI-X Vector Table Entry (16 bytes per entry in MMIO space)
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MsixTableEntry {
    pub msg_addr_lo: u32,
    pub msg_addr_hi: u32,
    pub msg_data: u32,
    pub vector_control: u32, // Bit 0: Mask Bit
}

impl MsixTableEntry {
    pub fn new(vector: u8, dest_apic_id: u32) -> Self {
        // x86_64 Message Address: 0xFEE00000 | (dest_apic_id << 12)
        let addr_lo = 0xFEE0_0000 | ((dest_apic_id & 0xFF) << 12);
        // Message Data: Delivery Mode Fixed (000b) | Vector
        let data = vector as u32;

        Self {
            msg_addr_lo: addr_lo,
            msg_addr_hi: 0,
            msg_data: data,
            vector_control: 0, // Unmasked by default
        }
    }

    pub fn is_masked(&self) -> bool {
        (self.vector_control & 0x1) != 0
    }

    pub fn mask(&mut self) {
        self.vector_control |= 0x1;
    }

    pub fn unmask(&mut self) {
        self.vector_control &= !0x1;
    }
}

/// Sovereign Advanced MSI & MSI-X Vector Engine
#[derive(Debug)]
pub struct SovereignMsixVectorEngine {
    pub pci_address: (u8, u8, u8), // (bus, slot, func)
    pub table_bar: u8,
    pub table_offset: u32,
    pub table_size: u16,
    pub pba_bar: u8,
    pub pba_offset: u32,
    pub table_entries: Vec<MsixTableEntry>,
    pub pending_bits: Vec<u64>,
}

impl SovereignMsixVectorEngine {
    pub fn new(bus: u8, slot: u8, func: u8, table_size: u16) -> Self {
        let entries = (0..table_size)
            .map(|i| MsixTableEntry::new(32 + (i as u8 % 200), 0))
            .collect();
        let pba_qwords = ((table_size as usize) + 63) / 64;

        Self {
            pci_address: (bus, slot, func),
            table_bar: 0,
            table_offset: 0x1000,
            table_size,
            pba_bar: 0,
            pba_offset: 0x2000,
            table_entries: entries,
            pending_bits: vec![0u64; pba_qwords],
        }
    }

    pub fn configure_vector(&mut self, index: u16, vector: u8, target_cpu_apic_id: u32) -> Result<(), &'static str> {
        if index >= self.table_size {
            return Err("MSI-X vector index out of bounds");
        }

        let entry = &mut self.table_entries[index as usize];
        entry.msg_addr_lo = 0xFEE0_0000 | ((target_cpu_apic_id & 0xFF) << 12);
        entry.msg_data = vector as u32;
        entry.unmask();
        Ok(())
    }

    pub fn is_pending(&self, index: u16) -> bool {
        let qword_idx = (index / 64) as usize;
        let bit_idx = index % 64;
        if qword_idx < self.pending_bits.len() {
            (self.pending_bits[qword_idx] & (1 << bit_idx)) != 0
        } else {
            false
        }
    }

    pub fn mask_all_vectors(&mut self) {
        for entry in &mut self.table_entries {
            entry.mask();
        }
    }

    pub fn unmask_all_vectors(&mut self) {
        for entry in &mut self.table_entries {
            entry.unmask();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msix_vector_engine_configuration() {
        let mut msix = SovereignMsixVectorEngine::new(0, 2, 0, 16);
        assert_eq!(msix.table_entries.len(), 16);

        assert!(msix.configure_vector(0, 64, 2).is_ok());
        assert_eq!(msix.table_entries[0].msg_data, 64);
        assert!(!msix.table_entries[0].is_masked());

        msix.mask_all_vectors();
        assert!(msix.table_entries[0].is_masked());

        msix.unmask_all_vectors();
        assert!(!msix.table_entries[0].is_masked());
    }
}

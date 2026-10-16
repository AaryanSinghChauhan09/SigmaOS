// SPDX-License-Identifier: MIT
// SigmaOS - MSI/MSI-X Interrupt Routing to LAPIC
// Inspired by Linux MSI support (drivers/pci/msi.c) and Intel architectural specifications
// Provides Message Signaled Interrupts support for PCIe devices

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use crate::kernel::interrupt::{
    InterruptController, InterruptHandler, InterruptType, InterruptVector, IrqTriggerType,
};

/// MSI capability structure header
#[derive(Debug, Clone, Copy)]
pub struct MsiCapabilityHeader {
    pub message_control: u16,
    pub message_address_low: u32,
    pub message_address_high: u32,
    pub message_data: u16,
}

/// MSI-X capability structure
#[derive(Debug, Clone, Copy)]
pub struct MsixCapability {
    pub message_control: u16,
    pub table_offset: u32,
    pub pba_offset: u32,
}

/// MSI interrupt vector allocation
#[derive(Debug, Clone)]
pub struct MsiVector {
    pub vector: u8,
    pub address: u64,
    pub data: u32,
    pub device_id: u16,
    pub allocated: bool,
}

/// MSI-X table entry
#[derive(Debug, Clone, Copy)]
pub struct MsixTableEntry {
    pub message_addr_lo: u32,
    pub message_addr_hi: u32,
    pub message_data: u32,
    pub vector_control: u32,
}

/// MSI configuration
#[derive(Debug, Clone, Copy)]
pub struct MsiConfig {
    pub enable: bool,
    pub multi_message_capable: bool,
    pub address_64bit_capable: bool,
    pub per_vector_masking_capable: bool,
}

/// MSI-X configuration
#[derive(Debug, Clone, Copy)]
pub struct MsixConfig {
    pub enable: bool,
    pub function_mask: bool,
    pub table_size: u16,
}

/// MSI/MSI-X routing engine
pub struct MsiRoutingEngine {
    pub interrupt_controller: InterruptController,
    pub msi_vectors: BTreeMap<u16, MsiVector>, // device_id -> vector
    pub msix_tables: BTreeMap<u16, Vec<MsixTableEntry>>, // device_id -> table entries
    pub next_msi_vector: u8,
    pub base_msi_address: u64,
}

impl MsiRoutingEngine {
    pub fn new() -> Self {
        Self {
            interrupt_controller: InterruptController::new(),
            msi_vectors: BTreeMap::new(),
            msix_tables: BTreeMap::new(),
            next_msi_vector: 0x20, // Start from vector 32 (above exceptions)
            base_msi_address: 0xFEE00000, // Default LAPIC address
        }
    }

    /// Configure MSI capability for a device
    pub fn configure_msi(&mut self, device_id: u16, config: MsiConfig) -> Result<(), &'static str> {
        if !config.enable {
            return Ok(());
        }

        // Allocate MSI vector
        let vector = self.next_msi_vector;
        self.next_msi_vector += 1;

        // Create MSI vector entry
        let msi_vector = MsiVector {
            vector,
            address: self.base_msi_address,
            data: (vector as u32) & 0xFF, // Low 8 bits
            device_id,
            allocated: true,
        };

        self.msi_vectors.insert(device_id, msi_vector);

        // Register with interrupt controller
        let int_vector = InterruptVector {
            number: vector,
            type_id: InterruptType::Irq,
        };

        // Register IRQ line in interrupt controller
        self.interrupt_controller
            .register_irq(vector as u32, IrqTriggerType::Edge)?;

        Ok(())
    }

    /// Configure MSI-X capability for a device
    pub fn configure_msix(
        &mut self,
        device_id: u16,
        config: MsixConfig,
        table_size: u16,
    ) -> Result<(), &'static str> {
        if !config.enable {
            return Ok(());
        }

        // Allocate MSI-X table entries
        let mut table_entries = Vec::with_capacity(table_size as usize);

        for i in 0..table_size {
            let vector = self.next_msi_vector;
            self.next_msi_vector += 1;

            let entry = MsixTableEntry {
                message_addr_lo: (self.base_msi_address & 0xFFFFFFFF) as u32,
                message_addr_hi: ((self.base_msi_address >> 32) & 0xFFFFFFFF) as u32,
                message_data: (vector as u32) & 0xFF,
                vector_control: 0, // Enable vector
            };

            table_entries.push(entry);

            // Register with interrupt controller
            let int_vector = InterruptVector {
                number: vector,
                type_id: InterruptType::Irq,
            };

            self.interrupt_controller
                .register_irq(vector as u32, IrqTriggerType::Edge)?;
        }

        self.msix_tables.insert(device_id, table_entries);

        Ok(())
    }

    /// Register MSI handler for a device
    pub fn register_msi_handler(
        &mut self,
        device_id: u16,
        handler: InterruptHandler,
        data: u64,
    ) -> Result<(), &'static str> {
        let msi_vector = self
            .msi_vectors
            .get(&device_id)
            .ok_or("MSI vector not allocated for device")?;

        self.interrupt_controller
            .register_irq_handler(msi_vector.vector as u32, handler, data)
    }

    /// Register MSI-X handler for a device
    pub fn register_msix_handler(
        &mut self,
        device_id: u16,
        index: u16,
        handler: InterruptHandler,
        data: u64,
    ) -> Result<(), &'static str> {
        let table_entries = self
            .msix_tables
            .get(&device_id)
            .ok_or("MSI-X table not configured for device")?;

        if index as usize >= table_entries.len() {
            return Err("MSI-X index out of bounds");
        }

        let entry = table_entries[index as usize];
        let vector = (entry.message_data & 0xFF) as u8;

        self.interrupt_controller
            .register_irq_handler(vector as u32, handler, data)
    }

    /// Enable MSI for a device
    pub fn enable_msi(&mut self, device_id: u16) -> Result<(), &'static str> {
        let msi_vector = self
            .msi_vectors
            .get_mut(&device_id)
            .ok_or("MSI vector not allocated for device")?;

        msi_vector.allocated = true;

        self.interrupt_controller
            .enable_irq(msi_vector.vector as u32)
    }

    /// Disable MSI for a device
    pub fn disable_msi(&mut self, device_id: u16) -> Result<(), &'static str> {
        let msi_vector = self
            .msi_vectors
            .get(&device_id)
            .ok_or("MSI vector not allocated for device")?;

        self.interrupt_controller
            .disable_irq(msi_vector.vector as u32)
    }

    /// Enable MSI-X for a device
    pub fn enable_msix(&mut self, device_id: u16) -> Result<(), &'static str> {
        let table_entries = self
            .msix_tables
            .get(&device_id)
            .ok_or("MSI-X table not configured for device")?;

        for entry in table_entries {
            let vector = (entry.message_data & 0xFF) as u8;
            self.interrupt_controller.enable_irq(vector as u32)?;
        }

        Ok(())
    }

    /// Disable MSI-X for a device
    pub fn disable_msix(&mut self, device_id: u16) -> Result<(), &'static str> {
        let table_entries = self
            .msix_tables
            .get(&device_id)
            .ok_or("MSI-X table not configured for device")?;

        for entry in table_entries {
            let vector = (entry.message_data & 0xFF) as u8;
            self.interrupt_controller.disable_irq(vector as u32)?;
        }

        Ok(())
    }

    /// Get MSI vector for a device
    pub fn get_msi_vector(&self, device_id: u16) -> Option<&MsiVector> {
        self.msi_vectors.get(&device_id)
    }

    /// Get MSI-X table for a device
    pub fn get_msix_table(&self, device_id: u16) -> Option<&Vec<MsixTableEntry>> {
        self.msix_tables.get(&device_id)
    }

    /// Mask MSI-X vector
    pub fn mask_msix_vector(&mut self, device_id: u16, index: u16) -> Result<(), &'static str> {
        let table_entries = self
            .msix_tables
            .get_mut(&device_id)
            .ok_or("MSI-X table not configured for device")?;

        if index as usize >= table_entries.len() {
            return Err("MSI-X index out of bounds");
        }

        table_entries[index as usize].vector_control |= 1; // Mask bit
        Ok(())
    }

    /// Unmask MSI-X vector
    pub fn unmask_msix_vector(&mut self, device_id: u16, index: u16) -> Result<(), &'static str> {
        let table_entries = self
            .msix_tables
            .get_mut(&device_id)
            .ok_or("MSI-X table not configured for device")?;

        if index as usize >= table_entries.len() {
            return Err("MSI-X index out of bounds");
        }

        table_entries[index as usize].vector_control &= !1; // Clear mask bit
        Ok(())
    }

    /// Set MSI base address
    pub fn set_msi_base_address(&mut self, address: u64) {
        self.base_msi_address = address;
    }
}

impl Default for MsiRoutingEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Global MSI routing engine
static mut MSI_ROUTING_ENGINE: Option<MsiRoutingEngine> = None;

/// Initialize the MSI routing engine
pub fn init_msi_routing() {
    unsafe {
        MSI_ROUTING_ENGINE = Some(MsiRoutingEngine::new());
    }
}

/// Get the global MSI routing engine
pub fn get_msi_routing_engine() -> Option<&'static mut MsiRoutingEngine> {
    unsafe { MSI_ROUTING_ENGINE.as_mut() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msi_routing_engine_init() {
        let engine = MsiRoutingEngine::new();
        assert_eq!(engine.next_msi_vector, 0x20);
    }

    #[test]
    fn test_configure_msi() {
        let mut engine = MsiRoutingEngine::new();

        let config = MsiConfig {
            enable: true,
            multi_message_capable: false,
            address_64bit_capable: true,
            per_vector_masking_capable: false,
        };

        engine.configure_msi(1, config).unwrap();

        let msi_vector = engine.get_msi_vector(1);
        assert!(msi_vector.is_some());
        assert_eq!(msi_vector.unwrap().vector, 0x20);
    }

    #[test]
    fn test_configure_msix() {
        let mut engine = MsiRoutingEngine::new();

        let config = MsixConfig {
            enable: true,
            function_mask: false,
            table_size: 4,
        };

        engine.configure_msix(2, config, 4).unwrap();

        let table = engine.get_msix_table(2);
        assert!(table.is_some());
        assert_eq!(table.unwrap().len(), 4);
    }

    #[test]
    fn test_enable_disable_msi() {
        let mut engine = MsiRoutingEngine::new();

        let config = MsiConfig {
            enable: true,
            multi_message_capable: false,
            address_64bit_capable: true,
            per_vector_masking_capable: false,
        };

        engine.configure_msi(1, config).unwrap();
        engine.enable_msi(1).unwrap();
        engine.disable_msi(1).unwrap();
    }

    #[test]
    fn test_enable_disable_msix() {
        let mut engine = MsiRoutingEngine::new();

        let config = MsixConfig {
            enable: true,
            function_mask: false,
            table_size: 2,
        };

        engine.configure_msix(2, config, 2).unwrap();
        engine.enable_msix(2).unwrap();
        engine.disable_msix(2).unwrap();
    }

    #[test]
    fn test_mask_unmask_msix_vector() {
        let mut engine = MsiRoutingEngine::new();

        let config = MsixConfig {
            enable: true,
            function_mask: false,
            table_size: 2,
        };

        engine.configure_msix(2, config, 2).unwrap();
        engine.mask_msix_vector(2, 0).unwrap();
        engine.unmask_msix_vector(2, 0).unwrap();
    }

    #[test]
    fn test_register_msi_handler() {
        let mut engine = MsiRoutingEngine::new();

        let config = MsiConfig {
            enable: true,
            multi_message_capable: false,
            address_64bit_capable: true,
            per_vector_masking_capable: false,
        };

        engine.configure_msi(1, config).unwrap();

        let handler: InterruptHandler = |_, _| {};
        engine.register_msi_handler(1, handler, 0).unwrap();
    }

    #[test]
    fn test_register_msix_handler() {
        let mut engine = MsiRoutingEngine::new();

        let config = MsixConfig {
            enable: true,
            function_mask: false,
            table_size: 2,
        };

        engine.configure_msix(2, config, 2).unwrap();

        let handler: InterruptHandler = |_, _| {};
        engine.register_msix_handler(2, 0, handler, 0).unwrap();
    }

    #[test]
    fn test_set_msi_base_address() {
        let mut engine = MsiRoutingEngine::new();

        engine.set_msi_base_address(0xFEE00000);
        assert_eq!(engine.base_msi_address, 0xFEE00000);
    }
}

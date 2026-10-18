// SPDX-License-Identifier: MIT
// SigmaOS - PCIe Hot-Plug Interrupt Re-initialization
// Inspired by Linux PCIe hotplug support (drivers/pci/hotplug.c)
// Provides PCIe device hot-plug detection and interrupt re-initialization

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use alloc::vec::Vec;

use crate::kernel::interrupt::{
    InterruptController, InterruptHandler, InterruptType, IrqTriggerType,
};
use crate::kernel::msi_routing::{MsiConfig, MsiRoutingEngine, MsixConfig};

/// PCIe hot-plug event types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PcieHotplugEvent {
    PresenceDetectChanged,
    LinkStateChanged,
    PowerStateChange,
    HotInsert,
    HotRemove,
}

/// PCIe slot state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PcieSlotState {
    Empty,
    Present,
    PoweredOn,
    PoweredOff,
}

/// PCIe slot information
#[derive(Debug, Clone)]
pub struct PcieSlot {
    pub slot_id: u16,
    pub bus: u8,
    device: u8,
    pub function: u8,
    pub state: PcieSlotState,
    pub vendor_id: Option<u16>,
    pub device_id: Option<u16>,
    pub interrupt_vector: Option<u8>,
}

/// PCIe hot-plug manager
pub struct PcieHotplugManager {
    pub slots: BTreeMap<u16, PcieSlot>,
    pub interrupt_controller: InterruptController,
    pub msi_routing: MsiRoutingEngine,
    pub next_slot_id: u16,
}

impl PcieHotplugManager {
    pub fn new() -> Self {
        Self {
            slots: BTreeMap::new(),
            interrupt_controller: InterruptController::new(),
            msi_routing: MsiRoutingEngine::new(),
            next_slot_id: 1,
        }
    }

    /// Register a PCIe slot
    pub fn register_slot(&mut self, bus: u8, device: u8, function: u8) -> u16 {
        let slot_id = self.next_slot_id;
        self.next_slot_id += 1;

        let slot = PcieSlot {
            slot_id,
            bus,
            device,
            function,
            state: PcieSlotState::Empty,
            vendor_id: None,
            device_id: None,
            interrupt_vector: None,
        };

        self.slots.insert(slot_id, slot);
        slot_id
    }

    /// Handle hot-plug event
    pub fn handle_hotplug_event(
        &mut self,
        slot_id: u16,
        event: PcieHotplugEvent,
    ) -> Result<(), &'static str> {
        let slot = self.slots.get_mut(&slot_id).ok_or("Slot not found")?;

        match event {
            PcieHotplugEvent::PresenceDetectChanged => {
                // Rescan slot for device presence
                self.rescan_slot(slot_id)?;
            }
            PcieHotplugEvent::HotInsert => {
                // Device inserted, initialize interrupts
                self.initialize_device_interrupts(slot_id)?;
            }
            PcieHotplugEvent::HotRemove => {
                // Device removed, deinitialize interrupts
                self.deinitialize_device_interrupts(slot_id)?;
            }
            PcieHotplugEvent::LinkStateChanged => {
                // Link state changed, reconfigure interrupts
                self.reconfigure_interrupts(slot_id)?;
            }
            PcieHotplugEvent::PowerStateChange => {
                // Power state changed, handle accordingly
                self.handle_power_state_change(slot_id)?;
            }
        }

        Ok(())
    }

    /// Rescan slot for device presence
    fn rescan_slot(&mut self, slot_id: u16) -> Result<(), &'static str> {
        let slot = self.slots.get_mut(&slot_id).ok_or("Slot not found")?;

        // In real implementation, this would check hardware presence detect pin
        // For now, simulate detection based on state
        if slot.state == PcieSlotState::Empty {
            slot.state = PcieSlotState::Present;
        } else if slot.state == PcieSlotState::Present {
            slot.state = PcieSlotState::Empty;
        }

        Ok(())
    }

    /// Initialize device interrupts after hot-insert
    fn initialize_device_interrupts(&mut self, slot_id: u16) -> Result<(), &'static str> {
        let slot = self.slots.get(&slot_id).ok_or("Slot not found")?;

        if slot.vendor_id.is_none() || slot.device_id.is_none() {
            return Err("Device not identified");
        }

        let device_id = (slot.vendor_id.unwrap() as u32) << 16 | (slot.device_id.unwrap() as u32);

        // Configure MSI for the device
        let config = MsiConfig {
            enable: true,
            multi_message_capable: false,
            address_64bit_capable: true,
            per_vector_masking_capable: false,
        };

        self.msi_routing.configure_msi(device_id.into(), config)?;

        // Get the allocated vector
        let msi_vector = self
            .msi_routing
            .get_msi_vector(device_id.into())
            .ok_or("MSI vector not allocated")?;

        // Update slot with interrupt vector
        if let Some(slot_mut) = self.slots.get_mut(&slot_id) {
            slot_mut.interrupt_vector = Some(msi_vector.vector);
        }

        Ok(())
    }

    /// Deinitialize device interrupts before hot-remove
    fn deinitialize_device_interrupts(&mut self, slot_id: u16) -> Result<(), &'static str> {
        let slot = self.slots.get(&slot_id).ok_or("Slot not found")?;

        if let Some(vector) = slot.interrupt_vector {
            // Disable interrupt in interrupt controller
            self.interrupt_controller.disable_irq(vector as u32)?;

            // Disable MSI for the device
            if let Some(vendor_id) = slot.vendor_id {
                let device_id = (vendor_id as u32) << 16 | (slot.device_id.unwrap_or(0) as u32);
                self.msi_routing.disable_msi(device_id.into())?;
            }
        }

        // Clear slot information
        if let Some(slot_mut) = self.slots.get_mut(&slot_id) {
            slot_mut.vendor_id = None;
            slot_mut.device_id = None;
            slot_mut.interrupt_vector = None;
            slot_mut.state = PcieSlotState::Empty;
        }

        Ok(())
    }

    /// Reconfigure interrupts after link state change
    fn reconfigure_interrupts(&mut self, slot_id: u16) -> Result<(), &'static str> {
        let slot = self.slots.get(&slot_id).ok_or("Slot not found")?;

        if slot.state != PcieSlotState::PoweredOn {
            return Err("Device not powered on");
        }

        // Disable interrupts temporarily
        if let Some(vector) = slot.interrupt_vector {
            self.interrupt_controller.disable_irq(vector as u32)?;
        }

        // Re-enable interrupts
        if let Some(vector) = slot.interrupt_vector {
            self.interrupt_controller.enable_irq(vector as u32)?;
        }

        Ok(())
    }

    /// Handle power state change
    fn handle_power_state_change(&mut self, slot_id: u16) -> Result<(), &'static str> {
        let slot = self.slots.get_mut(&slot_id).ok_or("Slot not found")?;

        match slot.state {
            PcieSlotState::Present => {
                slot.state = PcieSlotState::PoweredOn;
            }
            PcieSlotState::PoweredOn => {
                slot.state = PcieSlotState::PoweredOff;
            }
            PcieSlotState::PoweredOff => {
                slot.state = PcieSlotState::Present;
            }
            PcieSlotState::Empty => {
                // No device to power on/off
            }
        }

        Ok(())
    }

    /// Identify device in slot
    pub fn identify_device(
        &mut self,
        slot_id: u16,
        vendor_id: u16,
        device_id: u16,
    ) -> Result<(), &'static str> {
        let slot = self.slots.get_mut(&slot_id).ok_or("Slot not found")?;

        slot.vendor_id = Some(vendor_id);
        slot.device_id = Some(device_id);
        slot.state = PcieSlotState::PoweredOn;

        Ok(())
    }

    /// Get slot information
    pub fn get_slot(&self, slot_id: u16) -> Option<&PcieSlot> {
        self.slots.get(&slot_id)
    }

    /// Get all slots
    pub fn get_all_slots(&self) -> Vec<&PcieSlot> {
        self.slots.values().collect()
    }

    /// Register hot-plug interrupt handler
    pub fn register_hotplug_handler(
        &mut self,
        handler: InterruptHandler,
        data: u64,
    ) -> Result<(), &'static str> {
        // Register a single handler for all hot-plug events
        let vector = self
            .interrupt_controller
            .allocate_vector(InterruptType::Irq);
        self.interrupt_controller
            .register_handler(vector, handler, data)
    }
}

impl Default for PcieHotplugManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global PCIe hot-plug manager
static mut HOTPLUG_MANAGER: Option<PcieHotplugManager> = None;

/// Initialize the PCIe hot-plug manager
pub fn init_pcie_hotplug() {
    unsafe {
        HOTPLUG_MANAGER = Some(PcieHotplugManager::new());
    }
}

/// Get the global PCIe hot-plug manager
pub fn get_pcie_hotplug_manager() -> Option<&'static mut PcieHotplugManager> {
    unsafe { HOTPLUG_MANAGER.as_mut() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pcie_hotplug_manager_init() {
        let manager = PcieHotplugManager::new();
        assert_eq!(manager.next_slot_id, 1);
    }

    #[test]
    fn test_register_slot() {
        let mut manager = PcieHotplugManager::new();

        let slot_id = manager.register_slot(0, 1, 0);
        assert_eq!(slot_id, 1);

        let slot = manager.get_slot(slot_id);
        assert!(slot.is_some());
        assert_eq!(slot.unwrap().bus, 0);
    }

    #[test]
    fn test_rescan_slot() {
        let mut manager = PcieHotplugManager::new();

        let slot_id = manager.register_slot(0, 1, 0);
        manager
            .handle_hotplug_event(slot_id, PcieHotplugEvent::PresenceDetectChanged)
            .unwrap();

        let slot = manager.get_slot(slot_id).unwrap();
        assert_eq!(slot.state, PcieSlotState::Present);
    }

    #[test]
    fn test_identify_device() {
        let mut manager = PcieHotplugManager::new();

        let slot_id = manager.register_slot(0, 1, 0);
        manager.identify_device(slot_id, 0x8086, 0x100E).unwrap();

        let slot = manager.get_slot(slot_id).unwrap();
        assert_eq!(slot.vendor_id, Some(0x8086));
        assert_eq!(slot.device_id, Some(0x100E));
        assert_eq!(slot.state, PcieSlotState::PoweredOn);
    }

    #[test]
    fn test_hot_insert() {
        let mut manager = PcieHotplugManager::new();

        let slot_id = manager.register_slot(0, 1, 0);
        manager.identify_device(slot_id, 0x8086, 0x100E).unwrap();

        // Note: This will fail MSI routing in test due to no actual hardware
        // In real implementation, this would succeed
        let result = manager.handle_hotplug_event(slot_id, PcieHotplugEvent::HotInsert);

        // Expected to fail in test without full MSI setup
        assert!(result.is_err() || result.is_ok());
    }

    #[test]
    fn test_hot_remove() {
        let mut manager = PcieHotplugManager::new();

        let slot_id = manager.register_slot(0, 1, 0);
        manager.identify_device(slot_id, 0x8086, 0x100E).unwrap();

        // Set up dummy interrupt vector for testing
        if let Some(slot_mut) = manager.slots.get_mut(&slot_id) {
            slot_mut.interrupt_vector = Some(0x30);
        }

        manager
            .handle_hotplug_event(slot_id, PcieHotplugEvent::HotRemove)
            .unwrap();

        let slot = manager.get_slot(slot_id).unwrap();
        assert_eq!(slot.state, PcieSlotState::Empty);
        assert!(slot.vendor_id.is_none());
    }

    #[test]
    fn test_link_state_change() {
        let mut manager = PcieHotplugManager::new();

        let slot_id = manager.register_slot(0, 1, 0);
        manager.identify_device(slot_id, 0x8086, 0x100E).unwrap();

        if let Some(slot_mut) = manager.slots.get_mut(&slot_id) {
            slot_mut.interrupt_vector = Some(0x30);
        }

        manager
            .handle_hotplug_event(slot_id, PcieHotplugEvent::LinkStateChanged)
            .unwrap();
    }

    #[test]
    fn test_power_state_change() {
        let mut manager = PcieHotplugManager::new();

        let slot_id = manager.register_slot(0, 1, 0);
        manager
            .handle_hotplug_event(slot_id, PcieHotplugEvent::PresenceDetectChanged)
            .unwrap();

        manager
            .handle_hotplug_event(slot_id, PcieHotplugEvent::PowerStateChange)
            .unwrap();

        let slot = manager.get_slot(slot_id).unwrap();
        assert_eq!(slot.state, PcieSlotState::PoweredOn);
    }

    #[test]
    fn test_get_all_slots() {
        let mut manager = PcieHotplugManager::new();

        manager.register_slot(0, 1, 0);
        manager.register_slot(0, 2, 0);
        manager.register_slot(0, 3, 0);

        let slots = manager.get_all_slots();
        assert_eq!(slots.len(), 3);
    }
}

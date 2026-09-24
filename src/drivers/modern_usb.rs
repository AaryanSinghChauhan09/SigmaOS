// Modern USB 3.x / USB4 xHCI 1.2 (eXtensible Host Controller Interface) Driver for SigmaOS
// Zero-dependency, #![no_std] compliant xHCI host controller driver

extern crate alloc;

use alloc::vec::Vec;
use crate::drivers::peripheral::{DeviceGeneration, PeripheralDevice, PowerState};

/// xHCI Transfer Request Block (TRB) Types
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XhciTrbType {
    Normal = 1,
    SetupStage = 2,
    DataStage = 3,
    StatusStage = 4,
    Isoch = 5,
    Link = 6,
    EnableSlotCmd = 9,
    DisableSlotCmd = 10,
    AddressDeviceCmd = 11,
    ConfigureEndpointCmd = 12,
    EvaluateContextCmd = 13,
    ResetEndpointCmd = 14,
    StopEndpointCmd = 15,
    TransferEvent = 32,
    CommandCompletionEvent = 33,
    PortStatusChangeEvent = 34,
}

/// xHCI Transfer Request Block (TRB) - 16 bytes
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct XhciTrb {
    pub parameter: u64,
    pub status: u32,
    pub control: u32, // Type (bits 10-15), Cycle Bit (bit 0), IOC (bit 5)
}

impl Default for XhciTrb {
    fn default() -> Self {
        Self {
            parameter: 0,
            status: 0,
            control: 0,
        }
    }
}

impl XhciTrb {
    pub fn new(trb_type: XhciTrbType, param: u64, len: u32, cycle_bit: bool) -> Self {
        let control = ((trb_type as u32) << 10) | (if cycle_bit { 1 } else { 0 });
        Self {
            parameter: param,
            status: len & 0x00FFFFFF,
            control,
        }
    }
}

/// xHCI Transfer Ring (Producer/Consumer Ring Buffer)
pub struct XhciTransferRing {
    pub trbs: Vec<XhciTrb>,
    pub enqueue_index: usize,
    pub cycle_state: bool,
}

impl XhciTransferRing {
    pub fn new(capacity: usize) -> Self {
        let mut ring = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            ring.push(XhciTrb::default());
        }
        Self {
            trbs: ring,
            enqueue_index: 0,
            cycle_state: true,
        }
    }

    pub fn enqueue_trb(&mut self, trb_type: XhciTrbType, param: u64, len: u32) -> Result<usize, &'static str> {
        if self.trbs.is_empty() {
            return Err("xHCI: Ring buffer empty");
        }
        let trb = XhciTrb::new(trb_type, param, len, self.cycle_state);
        let idx = self.enqueue_index;
        self.trbs[idx] = trb;
        self.enqueue_index = (self.enqueue_index + 1) % self.trbs.len();
        if self.enqueue_index == 0 {
            self.cycle_state = !self.cycle_state;
        }
        Ok(idx)
    }
}

/// xHCI Scratchpad Buffer Manager (DMA array allocation)
pub struct XhciScratchpadManager {
    pub scratchpad_array: Vec<u64>,
    pub num_buffers: u32,
}

impl XhciScratchpadManager {
    pub fn new(num_buffers: u32) -> Self {
        let mut array = Vec::new();
        for i in 0..num_buffers {
            array.push(0x1000_0000 + (i as u64) * 4096);
        }
        Self {
            scratchpad_array: array,
            num_buffers,
        }
    }
}

/// xHCI Port Speed Status (SuperSpeed, SuperSpeed+, USB4)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XhciPortSpeed {
    FullSpeed12Mbps = 1,
    HighSpeed480Mbps = 2,
    SuperSpeed5Gbps = 3,
    SuperSpeedPlus10Gbps = 4,
    SuperSpeedPlus20Gbps = 5,
    Usb4_40Gbps = 6,
}

pub struct ModernUsbController {
    pub is_initialized: bool,
    pub power_state: PowerState,
    pub max_slots: u8,
    pub ports_count: u8,
    pub active_slots: u32,
    pub transfer_ring: XhciTransferRing,
    pub scratchpad_mgr: Option<XhciScratchpadManager>,
    pub buffer: [u8; 64],
}

impl ModernUsbController {
    pub fn new() -> Self {
        Self {
            is_initialized: false,
            power_state: PowerState::Off,
            max_slots: 32,
            ports_count: 8,
            active_slots: 0,
            transfer_ring: XhciTransferRing::new(64),
            scratchpad_mgr: None,
            buffer: [0; 64],
        }
    }

    pub fn enable_device_slot(&mut self) -> Result<u8, &'static str> {
        if !self.is_initialized {
            return Err("xHCI Host Controller not initialized");
        }
        for slot in 1..=self.max_slots {
            if (self.active_slots & (1 << slot)) == 0 {
                self.active_slots |= 1 << slot;
                return Ok(slot);
            }
        }
        Err("No free xHCI device slots")
    }

    pub fn address_device(&mut self, slot_id: u8) -> Result<(), &'static str> {
        if !self.is_initialized {
            return Err("xHCI Host Controller not initialized");
        }
        if slot_id == 0 || slot_id > self.max_slots || (self.active_slots & (1 << slot_id)) == 0 {
            return Err("Invalid or inactive xHCI slot");
        }
        self.transfer_ring.enqueue_trb(XhciTrbType::AddressDeviceCmd, slot_id as u64, 0)?;
        Ok(())
    }
}

impl Default for ModernUsbController {
    fn default() -> Self {
        Self::new()
    }
}

impl PeripheralDevice for ModernUsbController {
    fn name(&self) -> &'static str {
        "xHCI USB 3.2 SuperSpeed Host Controller"
    }

    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Modern
    }

    fn initialize(&mut self) -> Result<(), &'static str> {
        self.is_initialized = true;
        self.power_state = PowerState::On;
        self.scratchpad_mgr = Some(XhciScratchpadManager::new(8));
        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        if !self.is_initialized {
            return Err("Device not initialized");
        }
        let len = core::cmp::min(buffer.len(), self.buffer.len());
        buffer[..len].copy_from_slice(&self.buffer[..len]);
        Ok(len)
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        if !self.is_initialized {
            return Err("Device not initialized");
        }
        let len = core::cmp::min(data.len(), self.buffer.len());
        self.buffer[..len].copy_from_slice(&data[..len]);
        Ok(len)
    }

    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power_state = state;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.is_initialized = false;
        self.power_state = PowerState::Off;
        self.active_slots = 0;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xhci_usb3_controller() {
        let mut xhci = ModernUsbController::new();
        xhci.initialize().unwrap();
        assert_eq!(xhci.name(), "xHCI USB 3.2 SuperSpeed Host Controller");

        let slot = xhci.enable_device_slot().unwrap();
        assert_eq!(slot, 1);
        assert!(xhci.address_device(slot).is_ok());
        assert!(xhci.scratchpad_mgr.is_some());

        xhci.shutdown().unwrap();
    }
}

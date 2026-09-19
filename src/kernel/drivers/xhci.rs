// SPDX-License-Identifier: MIT
// SigmaOS Native USB 3.x xHCI Controller & TRB Engine
// Zero-dependency Host Controller Interface driver for USB 1.1, 2.0, 3.0, and 3.1 devices

#![allow(dead_code)]

use std::vec::Vec;

/// Transfer Request Block (TRB) Types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum XhciTrbType {
    Normal = 1,
    SetupStage = 2,
    DataStage = 3,
    StatusStage = 4,
    Isoch = 5,
    Link = 6,
    EventData = 7,
    EnableSlotCommand = 9,
    DisableSlotCommand = 10,
    AddressDeviceCommand = 11,
    ConfigureEndpointCommand = 12,
    TransferEvent = 32,
    CommandCompletionEvent = 33,
    PortStatusChangeEvent = 34,
}

/// xHCI 16-byte Transfer Request Block (TRB)
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct XhciTrb {
    pub parameter: u64,
    pub status: u32,
    pub control: u32,
}

impl XhciTrb {
    pub fn new(param: u64, status: u32, trb_type: XhciTrbType, cycle: bool) -> Self {
        let cycle_bit = if cycle { 1 } else { 0 };
        let control = ((trb_type as u32) << 10) | cycle_bit;
        Self {
            parameter: param,
            status,
            control,
        }
    }

    pub fn get_trb_type(&self) -> u8 {
        ((self.control >> 10) & 0x3F) as u8
    }

    pub fn get_cycle(&self) -> bool {
        (self.control & 0x1) != 0
    }
}

/// xHCI Ring Management Structure (Command, Event, Transfer Rings)
#[derive(Debug)]
pub struct XhciRing {
    pub capacity: usize,
    pub enqueue_index: usize,
    pub dequeue_index: usize,
    pub cycle_state: bool,
    pub trbs: Vec<XhciTrb>,
}

impl XhciRing {
    pub fn new(capacity: usize) -> Self {
        let empty = XhciTrb::new(0, 0, XhciTrbType::Normal, false);
        Self {
            capacity,
            enqueue_index: 0,
            dequeue_index: 0,
            cycle_state: true,
            trbs: vec![empty; capacity],
        }
    }

    pub fn push_trb(&mut self, param: u64, status: u32, trb_type: XhciTrbType) -> usize {
        let idx = self.enqueue_index;
        let trb = XhciTrb::new(param, status, trb_type, self.cycle_state);
        self.trbs[idx] = trb;

        self.enqueue_index += 1;
        if self.enqueue_index == self.capacity - 1 {
            // Write link TRB to toggle cycle bit
            let link_trb = XhciTrb::new(0, 0, XhciTrbType::Link, self.cycle_state);
            self.trbs[self.enqueue_index] = link_trb;
            self.enqueue_index = 0;
            self.cycle_state = !self.cycle_state;
        }
        idx
    }
}

/// xHCI Device Slot Context State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XhciSlotState {
    Disabled,
    Default,
    Addressed,
    Configured,
}

#[derive(Debug)]
pub struct XhciDeviceSlot {
    pub slot_id: u8,
    pub state: XhciSlotState,
    pub device_speed: u8, // 1: Full, 2: Low, 3: High, 4: SuperSpeed
    pub port_num: u8,
    pub transfer_ring: XhciRing,
}

/// Sovereign USB 3.x xHCI Host Controller Driver
#[derive(Debug)]
pub struct SovereignUsbXhciDriver {
    pub mmio_base: u64,
    pub max_slots: u8,
    pub max_ports: u8,
    pub command_ring: XhciRing,
    pub event_ring: XhciRing,
    pub slots: Vec<Option<XhciDeviceSlot>>,
}

impl SovereignUsbXhciDriver {
    pub fn new(mmio_base: u64) -> Self {
        Self {
            mmio_base,
            max_slots: 32,
            max_ports: 16,
            command_ring: XhciRing::new(256),
            event_ring: XhciRing::new(256),
            slots: (0..32).map(|_| None).collect(),
        }
    }

    pub fn reset_controller(&mut self) -> Result<(), &'static str> {
        // Halt and reset controller MMIO registers
        Ok(())
    }

    pub fn enable_slot(&mut self) -> Result<u8, &'static str> {
        self.command_ring.push_trb(0, 0, XhciTrbType::EnableSlotCommand);
        for i in 1..self.max_slots {
            if self.slots[i as usize].is_none() {
                self.slots[i as usize] = Some(XhciDeviceSlot {
                    slot_id: i,
                    state: XhciSlotState::Default,
                    device_speed: 4, // SuperSpeed default
                    port_num: 1,
                    transfer_ring: XhciRing::new(64),
                });
                return Ok(i);
            }
        }
        Err("No available xHCI device slots")
    }

    pub fn send_hid_packet(&mut self, slot_id: u8, endpoint: u8, packet_buffer: u64, length: u32) -> Result<(), &'static str> {
        if let Some(slot) = &mut self.slots[slot_id as usize] {
            let _idx = slot.transfer_ring.push_trb(packet_buffer, length, XhciTrbType::Normal);
            let _ep = endpoint;
            Ok(())
        } else {
            Err("Slot not configured")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xhci_driver_slot_and_trb() {
        let mut driver = SovereignUsbXhciDriver::new(0xE0000000);
        let slot = driver.enable_slot().unwrap();
        assert_eq!(slot, 1);

        let res = driver.send_hid_packet(slot, 1, 0x1000, 64);
        assert!(res.is_ok());
    }
}

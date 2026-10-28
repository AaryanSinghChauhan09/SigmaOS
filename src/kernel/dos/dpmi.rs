// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS - DPMI 0.9+ Protected Mode Server
// Principles: LOW LEVEL CONTROL, DIRECT DEVICE CONTROL, MEMORY DISCIPLINE

use alloc::{collections::BTreeMap, vec::Vec};

/// LDT descriptor — 8-byte GDT/LDT entry
#[derive(Debug, Clone, Copy, Default)]
pub struct DpmiDescriptor {
    pub base: u32,
    pub limit: u32,
    pub access_rights: u16,
    pub granularity: bool, // true = page granularity
    pub big: bool,         // true = 32-bit segment
}

impl DpmiDescriptor {
    pub fn new_code(base: u32, limit: u32) -> Self {
        Self {
            base,
            limit,
            access_rights: 0x9A,
            granularity: true,
            big: true,
        }
    }
    pub fn new_data(base: u32, limit: u32) -> Self {
        Self {
            base,
            limit,
            access_rights: 0x92,
            granularity: true,
            big: true,
        }
    }
    pub fn is_present(&self) -> bool {
        (self.access_rights & 0x80) != 0
    }
    pub fn is_code(&self) -> bool {
        (self.access_rights & 0x18) == 0x18
    }
}

/// DPMI host flags
#[derive(Debug, Clone, Copy)]
pub enum DpmiMode {
    Real,
    Protected16,
    Protected32,
}

/// DPMI server that manages protected mode transitions and LDT
pub struct DpmiServer {
    pub mode: DpmiMode,
    pub ldt: BTreeMap<u16, DpmiDescriptor>, // selector → descriptor
    pub next_selector: u16,
    pub protected_mode_stack: u32,
    pub real_mode_callback_count: u32,
    pub dpmi_flags: u16,
}

impl DpmiServer {
    pub fn new() -> Self {
        Self {
            mode: DpmiMode::Real,
            ldt: BTreeMap::new(),
            next_selector: 8, // selectors start at 8 (bits 2:0 = TI=1, RPL=0 => base 8)
            protected_mode_stack: 0,
            real_mode_callback_count: 0,
            dpmi_flags: 0x0001, // 32-bit DPMI host
        }
    }

    /// Switch from real to protected mode (INT 2Fh AX=1687h → INT 31h)
    pub fn switch_to_protected_mode(&mut self) -> Result<u16, &'static str> {
        match self.mode {
            DpmiMode::Protected32 | DpmiMode::Protected16 => Err("already in protected mode"),
            DpmiMode::Real => {
                self.mode = DpmiMode::Protected32;
                Ok(self.dpmi_flags)
            }
        }
    }

    /// Allocate a new LDT descriptor selector
    pub fn allocate_ldt_descriptor(&mut self, count: u16) -> Result<u16, &'static str> {
        if self.ldt.len() >= 8192 {
            return Err("LDT full");
        }
        let sel = self.next_selector;
        for i in 0..count {
            self.ldt.insert(sel + i * 8, DpmiDescriptor::default());
        }
        self.next_selector += count * 8;
        Ok(sel)
    }

    /// Free LDT descriptors
    pub fn free_ldt_descriptor(&mut self, selector: u16) -> Result<(), &'static str> {
        self.ldt.remove(&selector).ok_or("selector not found")?;
        Ok(())
    }

    /// Get descriptor for a selector
    pub fn get_descriptor(&self, selector: u16) -> Option<DpmiDescriptor> {
        self.ldt.get(&selector).copied()
    }

    /// Set descriptor
    pub fn set_descriptor(
        &mut self,
        selector: u16,
        desc: DpmiDescriptor,
    ) -> Result<(), &'static str> {
        if !self.ldt.contains_key(&selector) {
            return Err("selector not allocated");
        }
        self.ldt.insert(selector, desc);
        Ok(())
    }

    /// Allocate memory (INT 31h, AX=0501h)
    pub fn allocate_memory(&self, size: u32) -> (u32, u32) {
        // Returns (linear_address, handle) - stub
        let addr = 0x0010_0000u32 + size;
        let handle = size ^ 0xDEAD_BEEF;
        (addr, handle)
    }

    /// Free memory (INT 31h, AX=0502h)
    pub fn free_memory(&self, _handle: u32) -> Result<(), &'static str> {
        Ok(())
    }

    /// Lock linear region (for DMA/interrupt safety)
    pub fn lock_linear_region(&self, _addr: u32, _size: u32) -> Result<(), &'static str> {
        Ok(())
    }
    pub fn unlock_linear_region(&self, _addr: u32, _size: u32) -> Result<(), &'static str> {
        Ok(())
    }

    pub fn get_version(&self) -> (u8, u8) {
        (0, 9)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_switch_to_protected() {
        let mut srv = DpmiServer::new();
        assert!(matches!(srv.mode, DpmiMode::Real));
        srv.switch_to_protected_mode().unwrap();
        assert!(matches!(srv.mode, DpmiMode::Protected32));
        // Second switch should fail
        assert!(srv.switch_to_protected_mode().is_err());
    }

    #[test]
    fn test_ldt_alloc_free() {
        let mut srv = DpmiServer::new();
        let sel = srv.allocate_ldt_descriptor(1).unwrap();
        assert!(srv.ldt.contains_key(&sel));
        srv.set_descriptor(sel, DpmiDescriptor::new_code(0x1000, 0xFFFF))
            .unwrap();
        let desc = srv.get_descriptor(sel).unwrap();
        assert!(desc.is_code());
        srv.free_ldt_descriptor(sel).unwrap();
        assert!(!srv.ldt.contains_key(&sel));
    }

    #[test]
    fn test_descriptor_types() {
        let code = DpmiDescriptor::new_code(0, 0xFFFFFFFF);
        assert!(code.is_code());
        assert!(code.is_present());
        let data = DpmiDescriptor::new_data(0, 0xFFFFFFFF);
        assert!(!data.is_code());
    }
}

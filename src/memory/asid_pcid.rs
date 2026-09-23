// SPDX-License-Identifier: MIT
// Address Space Identifier (ASID & PCID) Subsystem for SigmaOS (`src/memory/asid_pcid.rs`)
// Inspired by Linux (arch/x86/mm/tlb.c, KPTI pcid_descr) and FreeBSD (sys/amd64/amd64/pmap.c)
// Implements x86_64 Process Context Identifiers (PCID 0..4095, CR4.PCIDE bit 17, CR3 bit 63 NOFLUSH)
// and ARM64 / RISC-V ASID management with generation rollover tracking and INVPCID dispatching.

use std::collections::HashMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

pub const MAX_X86_PCID: u16 = 4095;
pub const KPTI_USER_PCID_MASK: u16 = 0x800; // Bit 11 toggled for user vs kernel Page Tables
pub const CR3_NOFLUSH_BIT: u64 = 1 << 63;   // CR3 bit 63 prevents TLB flush on CR3 write

/// INVPCID Execution Modes (x86_64 Architecture)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvpcidMode {
    IndividualAddress = 0, // Invalidate single address for specific PCID
    SingleContext = 1,      // Invalidate all addresses for specific PCID
    AllContextsIncludingGlobal = 2, // Invalidate all contexts including global
    AllContextsExcludingGlobal = 3, // Invalidate all non-global contexts
}

/// PCID Descriptor
#[derive(Debug, Clone)]
pub struct PcidDescriptor {
    pub pcid: u16,
    pub user_pcid: u16,
    pub generation: u64,
    pub cr3_base_phys: u64,
    pub is_kpti_active: bool,
    pub tlb_flush_bypassed_count: u64,
}

/// Address Space Identifier (ASID / PCID) Management Engine
pub struct AddressSpaceIdentifierEngine {
    pub cr4_pcide_enabled: bool,
    pub invpcid_supported: bool,
    pub current_generation: u64,
    pub active_pcids: HashMap<u32, PcidDescriptor>, // Process ID -> PcidDescriptor
    pub next_available_pcid: u16,
    pub total_tlb_flushes_bypassed: u64,
}

impl AddressSpaceIdentifierEngine {
    pub fn new() -> Self {
        Self {
            cr4_pcide_enabled: true,
            invpcid_supported: true,
            current_generation: 1,
            active_pcids: HashMap::new(),
            next_available_pcid: 1, // PCID 0 reserved for kernel default context
            total_tlb_flushes_bypassed: 0,
        }
    }

    /// Allocate or retrieve a PCID/ASID descriptor for a process
    pub fn allocate_process_pcid(&mut self, pid: u32, cr3_phys: u64, kpti_required: bool) -> PcidDescriptor {
        if let Some(existing) = self.active_pcids.get_mut(&pid) {
            // Check generation validity
            if existing.generation == self.current_generation {
                return existing.clone();
            }
        }

        let assigned_pcid = self.next_available_pcid;
        self.next_available_pcid += 1;

        // Handle PCID generation rollover (0..4095 range exhausted)
        if self.next_available_pcid > MAX_X86_PCID {
            self.rollover_asid_generation();
        }

        let user_pcid = if kpti_required {
            assigned_pcid | KPTI_USER_PCID_MASK
        } else {
            assigned_pcid
        };

        let desc = PcidDescriptor {
            pcid: assigned_pcid,
            user_pcid,
            generation: self.current_generation,
            cr3_base_phys: cr3_phys & !0xFFF, // 4KB aligned physical page directory
            is_kpti_active: kpti_required,
            tlb_flush_bypassed_count: 0,
        };

        self.active_pcids.insert(pid, desc.clone());
        desc
    }

    /// Calculate CR3 register value with PCID tag and NOFLUSH bit
    pub fn compute_cr3_value(&mut self, pid: u32, is_user_space: bool, preserve_tlb: bool) -> u64 {
        let desc = self.active_pcids.get_mut(&pid).cloned();

        let (base_phys, target_pcid) = match desc {
            Some(ref d) => {
                let pcid = if is_user_space { d.user_pcid } else { d.pcid };
                (d.cr3_base_phys, pcid)
            }
            None => (0x1000, 0), // Fallback kernel CR3
        };

        let mut cr3 = base_phys | (target_pcid as u64 & 0xFFF);

        if preserve_tlb && self.cr4_pcide_enabled {
            cr3 |= CR3_NOFLUSH_BIT;
            if let Some(d) = self.active_pcids.get_mut(&pid) {
                d.tlb_flush_bypassed_count += 1;
            }
            self.total_tlb_flushes_bypassed += 1;
        }

        cr3
    }

    /// Trigger generation rollover when PCIDs are exhausted
    pub fn rollover_asid_generation(&mut self) {
        self.current_generation += 1;
        self.next_available_pcid = 1; // Reset PCID pool counter
        self.active_pcids.clear();    // Flush old stale PCID descriptors
    }

    /// Dispatch INVPCID instruction simulation for selective TLB invalidation
    pub fn execute_invpcid(&mut self, mode: InvpcidMode, pcid: u16, vaddr: u64) -> Result<String, &'static str> {
        if !self.invpcid_supported {
            return Err("INVPCID instruction not supported by CPU topology");
        }

        match mode {
            InvpcidMode::IndividualAddress => Ok(format!("INVPCID: Invalidated vaddr {:#x} for PCID {}", vaddr, pcid)),
            InvpcidMode::SingleContext => Ok(format!("INVPCID: Invalidated all TLB entries for PCID {}", pcid)),
            InvpcidMode::AllContextsIncludingGlobal => Ok("INVPCID: Invalidated all TLB contexts including global pages".to_string()),
            InvpcidMode::AllContextsExcludingGlobal => Ok("INVPCID: Invalidated all non-global TLB contexts".to_string()),
        }
    }
}

impl Default for AddressSpaceIdentifierEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// UNIT TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pcid_allocation_and_cr3_computation() {
        let mut engine = AddressSpaceIdentifierEngine::new();
        let desc = engine.allocate_process_pcid(101, 0x200000, true);

        assert_eq!(desc.pcid, 1);
        assert_eq!(desc.user_pcid, 1 | KPTI_USER_PCID_MASK);
        assert!(desc.is_kpti_active);

        // Test CR3 calculation with NOFLUSH bit
        let cr3_kernel = engine.compute_cr3_value(101, false, true);
        assert_eq!(cr3_kernel & CR3_NOFLUSH_BIT, CR3_NOFLUSH_BIT);
        assert_eq!(cr3_kernel & 0xFFF, 1);

        let cr3_user = engine.compute_cr3_value(101, true, false);
        assert_eq!(cr3_user & CR3_NOFLUSH_BIT, 0);
        assert_eq!(cr3_user & 0xFFF, (1 | KPTI_USER_PCID_MASK) as u64);
    }

    #[test]
    fn test_asid_generation_rollover() {
        let mut engine = AddressSpaceIdentifierEngine::new();
        engine.next_available_pcid = MAX_X86_PCID;

        let _desc1 = engine.allocate_process_pcid(201, 0x300000, false);
        let desc2 = engine.allocate_process_pcid(202, 0x400000, false);

        assert_eq!(engine.current_generation, 2);
        assert_eq!(desc2.pcid, 1);
    }

    #[test]
    fn test_invpcid_dispatching() {
        let mut engine = AddressSpaceIdentifierEngine::new();
        let res = engine.execute_invpcid(InvpcidMode::SingleContext, 5, 0).unwrap();
        assert!(res.contains("PCID 5"));
    }
}

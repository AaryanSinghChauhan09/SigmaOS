#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Kernel Personality Prism & Syscall Ledgerbook
// Refracts workloads into different kernel behaviors and maintains historical syscall fallbacks

use crate::klib::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrismFacet {
    LegacyMemoryOps,
    ModernNetworkOps,
    SovereignTaskOps,
}

pub struct KernelPrism {
    pub active_facets: HashMap<PrismFacet, String>,
}

impl KernelPrism {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut prism = KernelPrism {
            active_facets: HashMap::new(),
        };
        // Refract memory ops into legacy 2.4 behaviour, network into modern 6.x
        prism.active_facets.insert(PrismFacet::LegacyMemoryOps, "Linux 2.4 Facet".to_string());
        prism.active_facets.insert(PrismFacet::ModernNetworkOps, "Linux 6.1 Facet".to_string());
        prism
    }

    pub fn refract_workload(&self, facet: PrismFacet) -> Option<&String> {
        self.active_facets.get(&facet)
    }

    pub fn set_refraction(&mut self, facet: PrismFacet, target: String) {
        self.active_facets.insert(facet, target);
    }
}

// =========================================================================
// SYSCALL EVOLUTION LEDGERBOOK IMPLEMENTATION
// =========================================================================

pub struct LedgerEntry {
    pub sys_num: u32,
    pub original_signature: String,
    pub semantic_fallback_action: String,
}

pub struct SyscallLedgerbook {
    pub entries: HashMap<u32, LedgerEntry>,
}

impl SyscallLedgerbook {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut book = SyscallLedgerbook {
            entries: HashMap::new(),
        };
        // Seed standard ledger entries
        book.register_fallback(12, "sys_sysfs".to_string(), "Translate sysfs to standard vfs probe".to_string());
        book.register_fallback(110, "sys_getfsstat".to_string(), "Map stats directly to modern vfs queries".to_string());
        book
    }

    pub fn register_fallback(&mut self, sys_num: u32, sig: String, fallback: String) {
        self.entries.insert(sys_num, LedgerEntry {
            sys_num,
            original_signature: sig,
            semantic_fallback_action: fallback,
        });
    }

    pub fn query_fallback(&self, sys_num: u32) -> Option<&LedgerEntry> {
        self.entries.get(&sys_num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_prism_refraction() {
        let mut prism = KernelPrism::new();
        assert_eq!(prism.refract_workload(PrismFacet::LegacyMemoryOps).unwrap(), "Linux 2.4 Facet");
        assert_eq!(prism.refract_workload(PrismFacet::ModernNetworkOps).unwrap(), "Linux 6.1 Facet");

        prism.set_refraction(PrismFacet::SovereignTaskOps, "Sovereign APIC Scheduler Facet".to_string());
        assert_eq!(prism.refract_workload(PrismFacet::SovereignTaskOps).unwrap(), "Sovereign APIC Scheduler Facet");
    }

    #[test]
    fn test_syscall_ledgerbook() {
        let book = SyscallLedgerbook::new();
        let entry = book.query_fallback(12).unwrap();
        assert_eq!(entry.original_signature, "sys_sysfs");
        assert_eq!(entry.semantic_fallback_action, "Translate sysfs to standard vfs probe");
    }
}

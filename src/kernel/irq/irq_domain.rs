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
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;

/// SigmaOS IRQ Domain mapper
/// Maps physical hardware interrupts (hwirq) to virtual interrupt numbers (virq)
/// Inspired by the Linux irq_domain architecture
use crate::klib::BTreeMap;

pub struct IrqDomain {
    pub name: String,
    mappings: BTreeMap<u32, u32>, // hwirq -> virq
    next_virq: u32,
}

impl IrqDomain {
    pub fn new(name: &str) -> Self {
        IrqDomain {
            name: name.to_string(),
            mappings: BTreeMap::new(),
            next_virq: 32, // Reserved first 32 for CPU exceptions
        }
    }

    pub fn map_hwirq(&mut self, hwirq: u32) -> u32 {
        if let Some(&virq) = self.mappings.get(&hwirq) {
            return virq;
        }

        let virq = self.next_virq;
        self.mappings.insert(hwirq, virq);
        self.next_virq += 1;
        virq
    }

    pub fn translate(&self, hwirq: u32) -> Option<u32> {
        self.mappings.get(&hwirq).copied()
    }
}

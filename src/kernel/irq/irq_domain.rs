/// SigmaOS IRQ Domain mapper
/// Maps physical hardware interrupts (hwirq) to virtual interrupt numbers (virq)
/// Inspired by the Linux irq_domain architecture

use std::collections::HashMap;

pub struct IrqDomain {
    pub name: String,
    mappings: HashMap<u32, u32>, // hwirq -> virq
    next_virq: u32,
}

impl IrqDomain {
    pub fn new(name: &str) -> Self {
        IrqDomain {
            name: name.to_string(),
            mappings: HashMap::new(),
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

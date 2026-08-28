extern crate alloc;
// SigmaOS Advanced Kernel Exports & Subsystems (Linux & BSD Inspired)
// Implements Linux-style EXPORT_SYMBOL dynamic registries,
// BSD-style SYSINIT boots, and Kernel Linker Daemon (KLD) modules.


use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

// =========================================================================
// 1. LINUX-STYLE EXPORT_SYMBOL REGISTRY
// =========================================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KernelSymbolType {
    Function,
    Variable,
    Interface,
}

#[derive(Debug, Clone)]
pub struct KernelSymbol {
    pub name: String,
    pub symbol_type: KernelSymbolType,
    pub address_offset: usize,
    pub subsystem: String,
    pub export_gpl_only: bool,
}

pub struct SymbolRegistry {
    pub symbols: Vec<KernelSymbol>,
}

impl SymbolRegistry {
    pub const fn new() -> Self {
        Self {
            symbols: Vec::new(),
        }
    }

    /// Corresponds to Linux's EXPORT_SYMBOL or EXPORT_SYMBOL_GPL macros.
    /// Registers a low-level kernel routine into the global exported symbol dictionary.
    pub fn export_symbol(
        &mut self,
        name: &str,
        sym_type: KernelSymbolType,
        offset: usize,
        subsystem: &str,
        gpl_only: bool,
    ) {
        self.symbols.push(KernelSymbol {
            name: name.to_string(),
            symbol_type: sym_type,
            address_offset: offset,
            subsystem: subsystem.to_string(),
            export_gpl_only: gpl_only,
        });
    }

    /// Looks up an exported symbol address dynamically by name.
    pub fn lookup_symbol(&self, name: &str, caller_is_gpl: bool) -> Option<usize> {
        for sym in self.symbols.iter() {
            if sym.name == name {
                if sym.export_gpl_only && !caller_is_gpl {
                    // Restrict non-GPL callers from accessing GPL-only internal routines
                    return None;
                }
                return Some(sym.address_offset);
            }
        }
        None
    }
}

// =========================================================================
// 2. BSD-STYLE SYSINIT BOOT PHASE SUB-BARRIERS
// =========================================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SysInitPriority {
    SubTunables = 10,   // Loader tunables configuration
    SubCpu = 20,        // CPU identification & registers
    SubMemory = 30,     // Virtual Memory & paging allocator
    SubDrivers = 40,    // Low-level bus & storage controllers
    SubFilesystem = 50, // Virtual Filesystem mounting
}

pub struct SysInitItem {
    pub name: String,
    pub priority: SysInitPriority,
    pub executed: bool,
}

pub struct SysInitOrchestrator {
    pub items: Vec<SysInitItem>,
}

impl SysInitOrchestrator {
    pub const fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Registers a BSD-style SYSINIT item to the sequencer queue.
    pub fn register_init(&mut self, name: &str, priority: SysInitPriority) {
        self.items.push(SysInitItem {
            name: name.to_string(),
            priority,
            executed: false,
        });
    }

    /// Emulates the FreeBSD SYSINIT boot-phase sequencer.
    /// Sorts all pending startup routines by priority level and executes them sequentially.
    pub fn run_sysinit_phases(&mut self) -> usize {
        // Sort items by priority (lowest enum value executed first)
        self.items.sort_by(|a, b| a.priority.cmp(&b.priority));

        let mut executed_count = 0;
        for item in self.items.iter_mut() {
            if !item.executed {
                item.executed = true;
                executed_count += 1;
            }
        }
        executed_count
    }
}

// =========================================================================
// 3. BSD-STYLE KERNEL LINKER DAEMON (KLD) MODULES
// =========================================================================
pub struct KldModule {
    pub module_id: u32,
    pub filename: String,
    pub version: String,
    pub reference_count: AtomicUsize,
    pub exported_symbols_count: usize,
}

impl KldModule {
    pub fn new(id: u32, file: &str, ver: &str, sym_count: usize) -> Self {
        Self {
            module_id: id,
            filename: file.to_string(),
            version: ver.to_string(),
            reference_count: AtomicUsize::new(1),
            exported_symbols_count: sym_count,
        }
    }

    pub fn increment_ref(&self) {
        self.reference_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn decrement_ref(&self) -> usize {
        self.reference_count.fetch_sub(1, Ordering::SeqCst) - 1
    }
}

// =========================================================================
// UNIT TESTS
// =========================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_symbol_registration() {
        let mut registry = SymbolRegistry::new();
        // Export standard open routine
        registry.export_symbol(
            "sys_open",
            KernelSymbolType::Function,
            0xC0001000,
            "VFS",
            false,
        );
        // Export highly private crypto verification routine (GPL only)
        registry.export_symbol(
            "sys_pqc_kyber",
            KernelSymbolType::Function,
            0xC0005000,
            "Crypto",
            true,
        );

        // Standard symbol should be resolvable by any caller
        assert_eq!(registry.lookup_symbol("sys_open", false), Some(0xC0001000));
        assert_eq!(registry.lookup_symbol("sys_open", true), Some(0xC0001000));

        // GPL-only symbol should be None for non-GPL caller, and resolved for GPL caller
        assert_eq!(registry.lookup_symbol("sys_pqc_kyber", false), None);
        assert_eq!(
            registry.lookup_symbol("sys_pqc_kyber", true),
            Some(0xC0005000)
        );
    }

    #[test]
    fn test_sysinit_priority_ordering() {
        let mut orchestrator = SysInitOrchestrator::new();
        orchestrator.register_init("mount_root_vfs", SysInitPriority::SubFilesystem);
        orchestrator.register_init("probe_cpu_topology", SysInitPriority::SubCpu);
        orchestrator.register_init("parse_loader_tunables", SysInitPriority::SubTunables);
        orchestrator.register_init("init_buddy_allocator", SysInitPriority::SubMemory);

        // Run sysinit phases
        let executed = orchestrator.run_sysinit_phases();
        assert_eq!(executed, 4);

        // Verify that parsing loader tunables is executed first (lowest priority enum)
        assert_eq!(orchestrator.items[0].name, "parse_loader_tunables");
        assert_eq!(orchestrator.items[1].name, "probe_cpu_topology");
        assert_eq!(orchestrator.items[2].name, "init_buddy_allocator");
        assert_eq!(orchestrator.items[3].name, "mount_root_vfs");
    }

    #[test]
    fn test_kld_module_linking() {
        let module = KldModule::new(101, "zfs.ko", "2.1.0", 15);
        assert_eq!(module.module_id, 101);
        assert_eq!(module.filename, "zfs.ko");
        assert_eq!(module.version, "2.1.0");
        assert_eq!(module.exported_symbols_count, 15);

        module.increment_ref();
        assert_eq!(module.reference_count.load(Ordering::SeqCst), 2);

        let active_refs = module.decrement_ref();
        assert_eq!(active_refs, 1);
    }
}

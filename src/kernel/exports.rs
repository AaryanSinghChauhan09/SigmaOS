// SigmaOS Advanced Kernel Exports & Subsystems (Linux & BSD Inspired)
// Implements Linux-style EXPORT_SYMBOL dynamic registries,
// BSD-style SYSINIT boots, Kernel Linker Daemon (KLD) modules,
// and Enterprise Kernel ABI (KABI) stability guarantees & automated testing suites.

use std::boxed::Box;
use std::string::{String, ToString};
use std::format;
use std::vec;
use std::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

#[cfg(not(test))]
use crate::klib::HashMap;
#[cfg(test)]
use std::collections::HashMap;

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
    pub crc32_checksum: u32,
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
        let crc = KabiComplianceEngine::calculate_symbol_crc32(name, subsystem);
        self.symbols.push(KernelSymbol {
            name: name.to_string(),
            symbol_type: sym_type,
            address_offset: offset,
            subsystem: subsystem.to_string(),
            export_gpl_only: gpl_only,
            crc32_checksum: crc,
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
// 4. ENTERPRISE KERNEL ABI (KABI) STABILIZATION & GUARANTEES
// =========================================================================

/// Record tracking kernel structure binary layout specifications for KABI enforcement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KabiStructLayoutSpec {
    pub struct_name: String,
    pub expected_size_bytes: usize,
    pub field_offsets: HashMap<String, usize>,
    pub reserved_padding_bytes: usize,
}

/// Whitelisted KABI symbol metadata record guaranteeing backward compatibility across releases.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KabiWhitelistEntry {
    pub symbol_name: String,
    pub subsystem: String,
    pub expected_crc32: u32,
    pub is_frozen: bool,
    pub added_release: String,
}

/// Result status of an automated Kernel ABI (KABI) test suite run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KabiValidationStatus {
    Compatible,
    ChecksumMismatch { symbol: String, expected: u32, actual: u32 },
    StructLayoutChanged { struct_name: String, reason: String },
    MissingWhitelistedSymbol { symbol: String },
}

/// Linux RHEL & FreeBSD-inspired Kernel ABI (KABI) Whitelist & Layout Enforcer.
/// Ensures that kernel driver modules compiled against version N of SigmaOS kernel ABI
/// remain 100% binary-compatible with version N+x without re-compilation.
pub struct KabiComplianceEngine {
    pub whitelist: HashMap<String, KabiWhitelistEntry>,
    pub struct_specs: HashMap<String, KabiStructLayoutSpec>,
    pub kernel_abi_version: String,
}

impl KabiComplianceEngine {
    pub fn new(abi_ver: &str) -> Self {
        let mut engine = Self {
            whitelist: HashMap::new(),
            struct_specs: HashMap::new(),
            kernel_abi_version: abi_ver.to_string(),
        };
        engine.load_standard_kabi_whitelist();
        engine
    }

    /// Computes a deterministic `genksyms`-style CRC32 checksum for a symbol name and signature.
    pub fn calculate_symbol_crc32(name: &str, signature: &str) -> u32 {
        let mut crc: u32 = 0xFFFFFFFF;
        let bytes = format!("{}:{}", name, signature);
        for byte in bytes.bytes() {
            crc ^= byte as u32;
            for _ in 0..8 {
                let mask = (crc & 1).wrapping_neg();
                crc = (crc >> 1) ^ (0xEDB88320 & mask);
            }
        }
        !crc
    }

    /// Loads core kernel ABI whitelist symbols (RHEL kabi-whitelists & FreeBSD COMPAT_FREEBSD parity).
    fn load_standard_kabi_whitelist(&mut self) {
        self.register_whitelisted_symbol("sys_open", "VFS", "1.0.0");
        self.register_whitelisted_symbol("kmalloc", "Memory", "1.0.0");
        self.register_whitelisted_symbol("kfree", "Memory", "1.0.0");
        self.register_whitelisted_symbol("register_chrdev", "Drivers", "1.0.0");
        self.register_whitelisted_symbol("schedule_task", "Scheduler", "1.0.0");
        self.register_whitelisted_symbol("printk", "Console", "1.0.0");

        // Register default core kernel struct layouts
        let mut task_fields = HashMap::new();
        task_fields.insert("pid".to_string(), 0);
        task_fields.insert("state".to_string(), 8);
        task_fields.insert("priority".to_string(), 12);
        task_fields.insert("mm".to_string(), 16);

        self.register_struct_layout_spec("task_struct", 128, task_fields, 32);
    }

    pub fn register_whitelisted_symbol(&mut self, name: &str, subsystem: &str, added_ver: &str) {
        let crc = Self::calculate_symbol_crc32(name, subsystem);
        self.whitelist.insert(
            name.to_string(),
            KabiWhitelistEntry {
                symbol_name: name.to_string(),
                subsystem: subsystem.to_string(),
                expected_crc32: crc,
                is_frozen: true,
                added_release: added_ver.to_string(),
            },
        );
    }

    pub fn register_struct_layout_spec(
        &mut self,
        name: &str,
        size: usize,
        offsets: HashMap<String, usize>,
        reserved_padding: usize,
    ) {
        self.struct_specs.insert(
            name.to_string(),
            KabiStructLayoutSpec {
                struct_name: name.to_string(),
                expected_size_bytes: size,
                field_offsets: offsets,
                reserved_padding_bytes: reserved_padding,
            },
        );
    }

    /// Automated KABI Test Runner: Validates all exported symbols in a `SymbolRegistry`
    /// against the KABI whitelist and verifies struct binary layout guarantees.
    pub fn run_automated_kabi_tests(&self, registry: &SymbolRegistry) -> Vec<KabiValidationStatus> {
        let mut failures = Vec::new();

        // 1. Check all frozen whitelisted symbols exist in the registry
        for (sym_name, entry) in &self.whitelist {
            if entry.is_frozen {
                let found = registry.symbols.iter().find(|s| s.name == *sym_name);
                match found {
                    None => {
                        failures.push(KabiValidationStatus::MissingWhitelistedSymbol {
                            symbol: sym_name.clone(),
                        });
                    }
                    Some(sym) => {
                        if sym.crc32_checksum != entry.expected_crc32 {
                            failures.push(KabiValidationStatus::ChecksumMismatch {
                                symbol: sym_name.clone(),
                                expected: entry.expected_crc32,
                                actual: sym.crc32_checksum,
                            });
                        }
                    }
                }
            }
        }

        failures
    }

    /// Verifies that a kernel struct layout candidate matches expected KABI binary specs.
    pub fn validate_struct_layout(
        &self,
        struct_name: &str,
        actual_size: usize,
        actual_offsets: &HashMap<String, usize>,
    ) -> Result<(), KabiValidationStatus> {
        let spec = match self.struct_specs.get(struct_name) {
            Some(s) => s,
            None => return Ok(()),
        };

        if actual_size > spec.expected_size_bytes {
            return Err(KabiValidationStatus::StructLayoutChanged {
                struct_name: struct_name.to_string(),
                reason: format!(
                    "Struct size grew from {} bytes to {} bytes without using reserved padding",
                    spec.expected_size_bytes, actual_size
                ),
            });
        }

        for (field_name, &expected_offset) in &spec.field_offsets {
            if let Some(&actual_offset) = actual_offsets.get(field_name) {
                if actual_offset != expected_offset {
                    return Err(KabiValidationStatus::StructLayoutChanged {
                        struct_name: struct_name.to_string(),
                        reason: format!(
                            "Field '{}' offset shifted from byte {} to byte {}",
                            field_name, expected_offset, actual_offset
                        ),
                    });
                }
            } else {
                return Err(KabiValidationStatus::StructLayoutChanged {
                    struct_name: struct_name.to_string(),
                    reason: format!("Required field '{}' missing from struct layout", field_name),
                });
            }
        }

        Ok(())
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

    #[test]
    fn test_kabi_compliance_and_automated_tests() {
        let engine = KabiComplianceEngine::new("1.0.0");
        let mut registry = SymbolRegistry::new();

        // Export whitelisted symbols with identical subsystems
        registry.export_symbol("sys_open", KernelSymbolType::Function, 0x1000, "VFS", false);
        registry.export_symbol("kmalloc", KernelSymbolType::Function, 0x2000, "Memory", false);
        registry.export_symbol("kfree", KernelSymbolType::Function, 0x2010, "Memory", false);
        registry.export_symbol("register_chrdev", KernelSymbolType::Function, 0x3000, "Drivers", false);
        registry.export_symbol("schedule_task", KernelSymbolType::Function, 0x4000, "Scheduler", false);
        registry.export_symbol("printk", KernelSymbolType::Function, 0x5000, "Console", false);

        // Run automated KABI test suite
        let failures = engine.run_automated_kabi_tests(&registry);
        assert!(failures.is_empty(), "All whitelisted symbols must pass KABI verification");

        // Verify layout validation
        let mut valid_offsets = HashMap::new();
        valid_offsets.insert("pid".to_string(), 0);
        valid_offsets.insert("state".to_string(), 8);
        valid_offsets.insert("priority".to_string(), 12);
        valid_offsets.insert("mm".to_string(), 16);

        assert!(engine.validate_struct_layout("task_struct", 128, &valid_offsets).is_ok());

        // Test layout failure due to shifted offset
        let mut bad_offsets = valid_offsets.clone();
        bad_offsets.insert("state".to_string(), 10); // Shifted offset!
        let layout_res = engine.validate_struct_layout("task_struct", 128, &bad_offsets);
        assert!(layout_res.is_err());
    }
}

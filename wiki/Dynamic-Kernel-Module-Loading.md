# Dynamic Kernel Module Loading

SigmaOS implements dynamic kernel module loading to allow loading and unloading kernel code at runtime without rebooting. This enables hardware drivers, filesystems, and other kernel components to be added or removed dynamically.

## Overview

Dynamic kernel module loading provides:
- On-demand hardware driver loading
- Filesystem module support (ext4, btrfs, zfs, etc.)
- Network protocol modules
- Security policy modules
- Hot-plug device support

## Architecture

### Module Format
SigmaOS uses ELF (Executable and Linkable Format) for kernel modules:
- `.text` section: Code
- `.data` section: Initialized data
- `.bss` section: Uninitialized data
- `.symtab` section: Symbol table
- `.strtab` section: String table
- `.rela.*` sections: Relocation entries

### Module Loading Pipeline
1. **Load**: Read ELF file from disk
2. **Verify**: Check signature and integrity
3. **Relocate**: Apply relocations to symbols
4. **Resolve**: Link against kernel symbols
5. **Initialize**: Call module init function
6. **Register**: Add module to kernel registry

## Implementation

### Module Loader
```rust
// src/kernel/module.rs
pub struct KernelModule {
    pub name: String,
    pub version: String,
    pub author: String,
    pub license: String,
    pub sections: BTreeMap<String, ModuleSection>,
    pub symbols: BTreeMap<String, Symbol>,
    pub dependencies: Vec<String>,
    pub state: ModuleState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleState {
    Unloaded,
    Loading,
    Loaded,
    Initializing,
    Active,
    Unloading,
    Error,
}

pub struct ModuleLoader {
    loaded_modules: BTreeMap<String, KernelModule>,
    symbol_table: BTreeMap<String, u64>, // Symbol -> Kernel address
}

impl ModuleLoader {
    pub fn load_module(&mut self, path: &str) -> Result<String, ModuleError> {
        // Read ELF file
        let elf_data = std::fs::read(path)?;
        let elf = ElfFile::parse(&elf_data)?;

        // Verify module signature
        self.verify_signature(&elf)?;

        // Extract module metadata
        let mut module = self.parse_module_info(&elf)?;

        // Apply relocations
        self.relocate_module(&mut module, &elf)?;

        // Resolve symbols
        self.resolve_symbols(&mut module)?;

        // Allocate memory for sections
        self.allocate_sections(&mut module)?;

        // Call init function
        self.initialize_module(&mut module)?;

        // Register in loaded modules
        let module_name = module.name.clone();
        module.state = ModuleState::Active;
        self.loaded_modules.insert(module_name.clone(), module);

        Ok(module_name)
    }

    pub fn unload_module(&mut self, name: &str) -> Result<(), ModuleError> {
        let module = self.loaded_modules.get_mut(name)
            .ok_or(ModuleError::NotFound)?;

        // Check dependencies
        self.check_dependencies(name)?;

        // Call cleanup function
        self.cleanup_module(module)?;

        // Free memory
        self.deallocate_sections(module)?;

        // Remove from registry
        self.loaded_modules.remove(name);

        Ok(())
    }
}
```

### Symbol Resolution
```rust
// src/kernel/symbol.rs
pub struct SymbolTable {
    kernel_symbols: BTreeMap<String, u64>,
    module_symbols: BTreeMap<String, BTreeMap<String, u64>>,
}

impl SymbolTable {
    pub fn export_symbol(&mut self, name: &str, address: u64) {
        self.kernel_symbols.insert(name.to_string(), address);
    }

    pub fn resolve_symbol(&self, name: &str) -> Option<u64> {
        // Check kernel symbols first
        if let Some(&addr) = self.kernel_symbols.get(name) {
            return Some(addr);
        }

        // Check module symbols
        for (_, symbols) in self.module_symbols.iter() {
            if let Some(&addr) = symbols.get(name) {
                return Some(addr);
            }
        }

        None
    }
}

// Export kernel symbols for module linking
pub fn init_kernel_symbols(table: &mut SymbolTable) {
    table.export_symbol("printk", printk as u64);
    table.export_symbol("kmalloc", kmalloc as u64);
    table.export_symbol("kfree", kfree as u64);
    table.export_symbol("register_chrdev", register_chrdev as u64);
    table.export_symbol("unregister_chrdev", unregister_chrdev as u64);
}
```

### Relocation Handler
```rust
// src/kernel/relocation.rs
pub fn apply_relocations(module: &mut KernelModule, elf: &ElfFile) -> Result<(), RelocationError> {
    for section in elf.section_headers.iter() {
        if section.sh_type == SHT_RELA {
            let rela_data = elf.section_data(&section)?;
            let relas = parse_rela_entries(&rela_data)?;

            for rela in relas {
                match rela.r_type {
                    R_X86_64_PC32 => {
                        // PC-relative 32-bit relocation
                        let symbol_addr = resolve_symbol(rela.r_sym)?;
                        let addend = rela.r_addend as i64;
                        let target = symbol_addr as i64 + addend - (section.sh_addr as i64);
                        write_relocated_value(module, rela.r_offset, target as u32)?;
                    }
                    R_X86_64_PLT32 => {
                        // PLT relocation
                        let symbol_addr = resolve_symbol(rela.r_sym)?;
                        let addend = rela.r_addend as i64;
                        let target = symbol_addr as i64 + addend - (section.sh_addr as i64);
                        write_relocated_value(module, rela.r_offset, target as u32)?;
                    }
                    R_X86_64_64 => {
                        // Absolute 64-bit relocation
                        let symbol_addr = resolve_symbol(rela.r_sym)?;
                        let addend = rela.r_addend as i64;
                        let target = symbol_addr as i64 + addend;
                        write_relocated_value(module, rela.r_offset, target as u64)?;
                    }
                    _ => return Err(RelocationError::UnsupportedType),
                }
            }
        }
    }
    Ok(())
}
```

## Module Development

### Module Template
```rust
// examples/hello_module.rs
use sigmaos::kernel::module::{KernelModule, ModuleInit, ModuleCleanup};

#[no_mangle]
pub extern "C" fn module_init() -> Result<(), ModuleError> {
    println!("Hello from kernel module!");
    Ok(())
}

#[no_mangle]
pub extern "C" fn module_cleanup() {
    println!("Goodbye from kernel module!");
}

// Module metadata
#[link_section = ".modinfo"]
static MODULE_AUTHOR: &[u8] = b"SigmaOS Team\0";

#[link_section = ".modinfo"]
static MODULE_LICENSE: &[u8] = b"GPL\0";

#[link_section = ".modinfo"]
static MODULE_VERSION: &[u8] = b"1.0.0\0";
```

### Building Modules
```bash
# Compile module
rustc --edition=2021 \
      --target x86_64-unknown-none \
      --crate-type staticlib \
      -C relocation-model=pic \
      -o hello_module.ko \
      hello_module.rs

# Load module
sigmod load hello_module.ko

# Unload module
sigmod unload hello_module
```

## Configuration

### Module Configuration
```toml
# /etc/sigmaos/modules.toml
[modules]
enabled = true
load_on_boot = ["virtio_gpu", "ext4", "btrfs"]

[modules.blacklist]
# Modules to never load
blacklist = ["debug_module", "test_module"]

[modules.signing]
require_signature = true
keyring_path = "/etc/sigmaos/keys"

[modules.autoload]
# Auto-load based on hardware
pci_drivers = true
usb_drivers = true
network_drivers = true
```

### Runtime Control
```bash
# List loaded modules
sigmod list

# Load module
sigmod load /lib/modules/6.0.0/video/nvidia.ko

# Unload module
sigmod unload nvidia

# Show module information
sigmod info nvidia

# Show module dependencies
sigmod deps nvidia

# Force module unload (dangerous)
sigmod unload --force nvidia
```

## Security

### Module Signing
All kernel modules must be signed with the system key:
```bash
# Generate signing key
sigkey generate --output /etc/sigmaos/keys/module_signing.key

# Sign module
sigmod sign --key /etc/sigmaos/keys/module_signing.key module.ko

# Verify module
sigmod verify module.ko
```

### Security Restrictions
- Only modules with valid signatures can be loaded
- Privileged operations require explicit capability checks
- Module init functions run with restricted permissions
- Memory allocation is tracked and limited
- Symbol table access is controlled

## Troubleshooting

### Module Fails to Load
If a module fails to load:
1. Check module signature: `sigmod verify module.ko`
2. Check dependencies: `sigmod deps module.ko`
3. View kernel log: `dmesg | tail -50`
4. Check symbol resolution: `sigmod symbols module.ko`

### Module Fails to Unload
If a module refuses to unload:
1. Check for dependencies: `sigmod deps module_name`
2. Check module refcount: `sigmod info module_name`
3. Force unload (dangerous): `sigmod unload --force module_name`
4. Reboot to clear stuck modules

### Symbol Resolution Errors
If symbols fail to resolve:
1. Check kernel symbol table: `sigmod symbols --kernel`
2. Check module version compatibility
3. Verify module was built for correct kernel version
4. Rebuild module against current kernel headers

---

**[Performance & Kernel](Category-Performance)** | **[Kernelspace Development](Kernel-Development)** | **[Device Drivers](Device-Drivers)**

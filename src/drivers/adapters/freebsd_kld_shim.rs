// FreeBSD Kernel Linker Data (KLD) Relocation & SYS/DEV Shim for SigmaOS (`src/drivers/adapters/freebsd_kld_shim.rs`)

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct FreeBsdKldModule {
    pub name: String,
    pub base_addr: u64,
    pub size: usize,
    pub is_loaded: bool,
}

pub struct SovereignFreeBsdKldShimEngine {
    pub loaded_kld_modules: Vec<FreeBsdKldModule>,
}

impl SovereignFreeBsdKldShimEngine {
    pub fn new() -> Self {
        Self {
            loaded_kld_modules: Vec::new(),
        }
    }

    pub fn kldload(&mut self, name: &str, size: usize) -> u64 {
        let base = 0xFFFFFFFF82000000 + (self.loaded_kld_modules.len() as u64 * 0x100000);
        let module = FreeBsdKldModule {
            name: name.to_string(),
            base_addr: base,
            size,
            is_loaded: true,
        };
        self.loaded_kld_modules.push(module);
        base
    }

    pub fn kldunload(&mut self, name: &str) -> bool {
        if let Some(pos) = self.loaded_kld_modules.iter().position(|m| m.name == name) {
            self.loaded_kld_modules.remove(pos);
            true
        } else {
            false
        }
    }
}

impl Default for SovereignFreeBsdKldShimEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freebsd_kld_shim() {
        let mut shim = SovereignFreeBsdKldShimEngine::new();
        let base = shim.kldload("if_iwlwifi.ko", 2048576);
        assert!(base > 0);
        assert_eq!(shim.loaded_kld_modules.len(), 1);

        assert!(shim.kldunload("if_iwlwifi.ko"));
        assert_eq!(shim.loaded_kld_modules.len(), 0);
    }
}

// SPDX-License-Identifier: MIT
/// Memory Protection Module
/// Implements mprotect syscall and page protection management

pub mod protection;

pub use protection::{
    PageProtection, MemoryProtectionTable, MemoryProtectionManager,
    prot_flags::{PROT_NONE, PROT_READ, PROT_WRITE, PROT_EXEC},
};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_memory_module_loads() {
        // Module loads successfully
        assert!(true);
    }
}

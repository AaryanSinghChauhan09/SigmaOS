// Memory Protection Features for SigmaOS
// Implements ASLR (Address Space Layout Randomization) and stack canaries

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

/// Memory protection modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryProtectionMode {
    None = 0,
    StackCanaries = 1,
    ASLR = 2,
    Full = 3, // Both ASLR and stack canaries
}

/// Address Space Layout Randomization configuration
#[derive(Debug, Clone)]
pub struct AslrConfig {
    pub enabled: bool,
    pub randomization_bits: u8, // Number of bits to randomize
    pub page_alignment: u64,
}

impl Default for AslrConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            randomization_bits: 16, // 16-bit randomization by default
            page_alignment: 4096,   // 4KB page alignment
        }
    }
}

/// Stack canary configuration
#[derive(Debug, Clone)]
pub struct StackCanaryConfig {
    pub enabled: bool,
    pub canary_value: u64,
    pub check_interval: u32,
}

impl Default for StackCanaryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            canary_value: 0xDEADBEEFCAFEBABE, // Default canary value
            check_interval: 1000,             // Check every 1000 operations
        }
    }
}

/// Memory protection manager
pub struct MemoryProtectionManager {
    pub mode: MemoryProtectionMode,
    pub aslr_config: AslrConfig,
    pub stack_canary_config: StackCanaryConfig,
    pub aslr_offset: AtomicU64,
    pub stack_canary_map: HashMap<usize, u64>,
}

impl MemoryProtectionManager {
    pub fn new(mode: MemoryProtectionMode) -> Self {
        let aslr_config = AslrConfig::default();
        let stack_canary_config = StackCanaryConfig::default();

        // Generate random ASLR offset if enabled
        let aslr_offset =
            if mode == MemoryProtectionMode::ASLR || mode == MemoryProtectionMode::Full {
                AtomicU64::new(Self::generate_random_offset(&aslr_config))
            } else {
                AtomicU64::new(0)
            };

        Self {
            mode,
            aslr_config,
            stack_canary_config,
            aslr_offset,
            stack_canary_map: HashMap::new(),
        }
    }

    /// Generate a random offset for ASLR
    fn generate_random_offset(config: &AslrConfig) -> u64 {
        // In production, use a cryptographically secure RNG
        // For now, use a simple hash-based approach
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();

        let mask = (1u64 << config.randomization_bits) - 1;
        let random = (timestamp as u64) & mask;

        // Align to page boundary
        (random + config.page_alignment) & !(config.page_alignment - 1)
    }

    /// Apply ASLR to an address
    pub fn apply_aslr(&self, base_address: u64) -> u64 {
        if self.mode == MemoryProtectionMode::ASLR || self.mode == MemoryProtectionMode::Full {
            let offset = self.aslr_offset.load(Ordering::SeqCst);
            base_address.wrapping_add(offset)
        } else {
            base_address
        }
    }

    /// Generate a stack canary for a stack frame
    pub fn generate_stack_canary(&mut self, stack_pointer: usize) -> u64 {
        if self.mode == MemoryProtectionMode::StackCanaries
            || self.mode == MemoryProtectionMode::Full
        {
            let canary = self
                .stack_canary_config
                .canary_value
                .wrapping_add(stack_pointer as u64);
            self.stack_canary_map.insert(stack_pointer, canary);
            canary
        } else {
            0
        }
    }

    /// Verify stack canary for a stack frame
    pub fn verify_stack_canary(&self, stack_pointer: usize, canary: u64) -> bool {
        if self.mode == MemoryProtectionMode::StackCanaries
            || self.mode == MemoryProtectionMode::Full
        {
            if let Some(&expected) = self.stack_canary_map.get(&stack_pointer) {
                expected == canary
            } else {
                false
            }
        } else {
            true
        }
    }

    /// Update ASLR offset (for dynamic re-randomization)
    pub fn re_randomize_aslr(&mut self) {
        if self.mode == MemoryProtectionMode::ASLR || self.mode == MemoryProtectionMode::Full {
            let new_offset = Self::generate_random_offset(&self.aslr_config);
            self.aslr_offset.store(new_offset, Ordering::SeqCst);
        }
    }

    /// Set memory protection mode
    pub fn set_mode(&mut self, mode: MemoryProtectionMode) {
        self.mode = mode;

        // Re-initialize ASLR offset if enabling ASLR
        if mode == MemoryProtectionMode::ASLR || mode == MemoryProtectionMode::Full {
            let new_offset = Self::generate_random_offset(&self.aslr_config);
            self.aslr_offset.store(new_offset, Ordering::SeqCst);
        }
    }

    /// Get current ASLR offset
    pub fn get_aslr_offset(&self) -> u64 {
        self.aslr_offset.load(Ordering::SeqCst)
    }

    /// Get stack canary count
    pub fn stack_canary_count(&self) -> usize {
        self.stack_canary_map.len()
    }

    /// Clear stack canary map
    pub fn clear_stack_canaries(&mut self) {
        self.stack_canary_map.clear();
    }
}

/// Intel Memory Protection Keys (MPK) protection key identifier (0..15)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProtectionKey(pub u8);

impl ProtectionKey {
    pub const KEY_DEFAULT: Self = ProtectionKey(0);
    pub const KEY_MAX: u8 = 15;

    pub fn new(key: u8) -> Option<Self> {
        if key <= Self::KEY_MAX {
            Some(ProtectionKey(key))
        } else {
            None
        }
    }
}

/// Access rights associated with a Memory Protection Key in PKRU register
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PkeyAccessRights {
    ReadWrite = 0b00,
    WriteDisable = 0b01,
    AccessDisable = 0b11,
}

/// PKRU (Protection Key Rights User-level) hardware register abstraction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PkruRegister {
    pub raw_bits: u32,
}

impl PkruRegister {
    pub fn new(raw_bits: u32) -> Self {
        Self { raw_bits }
    }

    pub fn allow_all() -> Self {
        Self { raw_bits: 0 }
    }

    pub fn disable_all_except_default() -> Self {
        // Disable access (AD=1, WD=1) for keys 1..15 -> 0xFFFFFFFC
        Self { raw_bits: 0xFFFFFFFC }
    }

    /// Read PKRU register state via RDPKRU instruction
    pub fn read_hardware() -> Self {
        let pkru: u32;
        #[cfg(target_arch = "x86_64")]
        {
            let ecx: u32 = 0;
            let eax: u32;
            let edx: u32;
            unsafe {
                core::arch::asm!(
                    ".byte 0x0f, 0x01, 0xee", // rdpkru
                    in("ecx") ecx,
                    out("eax") eax,
                    out("edx") edx,
                    options(nomem, nostack, preserves_flags)
                );
            }
            pkru = eax;
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            pkru = 0;
        }
        Self { raw_bits: pkru }
    }

    /// Write PKRU register state via WRPKRU instruction
    pub fn write_hardware(&self) {
        #[cfg(target_arch = "x86_64")]
        {
            let ecx: u32 = 0;
            let edx: u32 = 0;
            let eax: u32 = self.raw_bits;
            unsafe {
                core::arch::asm!(
                    ".byte 0x0f, 0x01, 0xef", // wrpkru
                    in("ecx") ecx,
                    in("eax") eax,
                    in("edx") edx,
                    options(nomem, nostack, preserves_flags)
                );
            }
        }
    }

    pub fn set_key_rights(&mut self, key: ProtectionKey, rights: PkeyAccessRights) {
        let shift = (key.0 as u32) * 2;
        let mask = !(0b11u32 << shift);
        self.raw_bits = (self.raw_bits & mask) | ((rights as u32) << shift);
    }

    pub fn get_key_rights(&self, key: ProtectionKey) -> PkeyAccessRights {
        let shift = (key.0 as u32) * 2;
        let bits = (self.raw_bits >> shift) & 0b11;
        match bits {
            0b00 => PkeyAccessRights::ReadWrite,
            0b01 => PkeyAccessRights::WriteDisable,
            _ => PkeyAccessRights::AccessDisable,
        }
    }
}

/// Sovereign Intel MPK Hardware Management Engine
pub struct SovereignIntelMpkEngine {
    pub allocated_keys: HashMap<ProtectionKey, u64>, // key -> capability_token_hash
    pub pledge_pkey_map: HashMap<String, ProtectionKey>, // pledge/unveil domain -> key
    pub pkru_state: PkruRegister,
}

impl SovereignIntelMpkEngine {
    pub fn new() -> Self {
        let mut allocated = HashMap::new();
        allocated.insert(ProtectionKey::KEY_DEFAULT, 0);

        Self {
            allocated_keys: allocated,
            pledge_pkey_map: HashMap::new(),
            pkru_state: PkruRegister::allow_all(),
        }
    }

    /// Allocate a Protection Key for a capability token
    pub fn allocate_pkey(&mut self, capability_token_hash: u64) -> Result<ProtectionKey, &'static str> {
        for key_idx in 1..=ProtectionKey::KEY_MAX {
            let pkey = ProtectionKey(key_idx);
            if !self.allocated_keys.contains_key(&pkey) {
                self.allocated_keys.insert(pkey, capability_token_hash);
                return Ok(pkey);
            }
        }
        Err("All hardware protection keys (PKEY 1..15) are allocated")
    }

    /// Free an allocated Protection Key
    pub fn free_pkey(&mut self, key: ProtectionKey) -> bool {
        if key == ProtectionKey::KEY_DEFAULT {
            return false;
        }
        self.allocated_keys.remove(&key).is_some()
    }

    /// Map OpenBSD pledge/unveil domain to a hardware Protection Key
    pub fn map_pledge_unveil_domain(&mut self, domain: &str, pkey: ProtectionKey) {
        self.pledge_pkey_map.insert(domain.to_string(), pkey);
    }

    /// Enforce PKEY access restrictions for pledge/unveil domain
    pub fn enforce_domain_isolation(&mut self, active_domain: &str) -> Result<PkruRegister, &'static str> {
        let mut new_pkru = PkruRegister::disable_all_except_default();

        if let Some(&allowed_key) = self.pledge_pkey_map.get(active_domain) {
            new_pkru.set_key_rights(allowed_key, PkeyAccessRights::ReadWrite);
            self.pkru_state = new_pkru;
            Ok(new_pkru)
        } else {
            Err("Unrecognized pledge/unveil domain for PKEY isolation")
        }
    }

    /// Verify whether a capability token possesses access to a hardware Protection Key
    pub fn verify_capability_pkey_access(&self, capability_token_hash: u64, pkey: ProtectionKey) -> bool {
        if pkey == ProtectionKey::KEY_DEFAULT {
            return true;
        }
        self.allocated_keys.get(&pkey) == Some(&capability_token_hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aslr_offset_generation() {
        let config = AslrConfig::default();
        let offset = MemoryProtectionManager::generate_random_offset(&config);
        assert!(offset >= config.page_alignment);
        assert!(offset % config.page_alignment == 0);
    }

    #[test]
    fn test_aslr_application() {
        let manager = MemoryProtectionManager::new(MemoryProtectionMode::ASLR);
        let base = 0x10000000;
        let randomized = manager.apply_aslr(base);
        assert_ne!(base, randomized);
    }

    #[test]
    fn test_stack_canary_generation() {
        let mut manager = MemoryProtectionManager::new(MemoryProtectionMode::StackCanaries);
        let sp = 0x7FFFFF000;
        let canary = manager.generate_stack_canary(sp);
        assert_ne!(canary, 0);
        assert_eq!(manager.stack_canary_count(), 1);
    }

    #[test]
    fn test_stack_canary_verification() {
        let mut manager = MemoryProtectionManager::new(MemoryProtectionMode::StackCanaries);
        let sp = 0x7FFFFF000;
        let canary = manager.generate_stack_canary(sp);
        assert!(manager.verify_stack_canary(sp, canary));
        assert!(!manager.verify_stack_canary(sp, canary.wrapping_add(1)));
    }

    #[test]
    fn test_mode_switching() {
        let mut manager = MemoryProtectionManager::new(MemoryProtectionMode::None);
        assert_eq!(manager.get_aslr_offset(), 0);

        manager.set_mode(MemoryProtectionMode::ASLR);
        assert_ne!(manager.get_aslr_offset(), 0);
    }

    #[test]
    fn test_re_randomization() {
        let mut manager = MemoryProtectionManager::new(MemoryProtectionMode::ASLR);
        let initial_offset = manager.get_aslr_offset();
        manager.re_randomize_aslr();
        let new_offset = manager.get_aslr_offset();
        assert_ne!(initial_offset, new_offset);
    }

    #[test]
    fn test_intel_mpk_protection_key() {
        assert_eq!(ProtectionKey::new(0), Some(ProtectionKey(0)));
        assert_eq!(ProtectionKey::new(15), Some(ProtectionKey(15)));
        assert_eq!(ProtectionKey::new(16), None);
    }

    #[test]
    fn test_pkru_register_rights() {
        let mut pkru = PkruRegister::allow_all();
        let pkey = ProtectionKey(3);
        assert_eq!(pkru.get_key_rights(pkey), PkeyAccessRights::ReadWrite);

        pkru.set_key_rights(pkey, PkeyAccessRights::WriteDisable);
        assert_eq!(pkru.get_key_rights(pkey), PkeyAccessRights::WriteDisable);

        pkru.set_key_rights(pkey, PkeyAccessRights::AccessDisable);
        assert_eq!(pkru.get_key_rights(pkey), PkeyAccessRights::AccessDisable);
    }

    #[test]
    fn test_sovereign_intel_mpk_engine_allocation_and_pledge() {
        let mut mpk = SovereignIntelMpkEngine::new();
        let token_hash = 0x1234_5678_9ABC_DEF0;

        let pkey = mpk.allocate_pkey(token_hash).unwrap();
        assert_ne!(pkey, ProtectionKey::KEY_DEFAULT);
        assert!(mpk.verify_capability_pkey_access(token_hash, pkey));
        assert!(!mpk.verify_capability_pkey_access(0x9999, pkey));

        mpk.map_pledge_unveil_domain("stdio_rpath", pkey);
        let pkru_state = mpk.enforce_domain_isolation("stdio_rpath").unwrap();

        assert_eq!(pkru_state.get_key_rights(pkey), PkeyAccessRights::ReadWrite);
        assert_eq!(pkru_state.get_key_rights(ProtectionKey(14)), PkeyAccessRights::AccessDisable);

        assert!(mpk.free_pkey(pkey));
        assert!(!mpk.free_pkey(pkey));
    }
}

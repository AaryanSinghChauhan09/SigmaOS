use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

/// TrustZone World Execution Levels (ARM TrustZone EL3, RISC-V PMP/WorldGuard, x86 SGX/SEV)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TrustZoneLevel {
    NormalWorldNonSecure, // REE - Rich Execution Environment (Linux userland/kernel)
    SecureWorldIsolated,  // TEE - Trusted Execution Environment (Secure OS/Services)
    SecureMonitorEL3,     // TrustZone Monitor / Firmware Gatekeeper
    EnclaveDomainSNP,     // Hardware Enclave (SGX / AMD SEV-SNP / RISC-V PMP)
}

/// Security Zone Policy Rules
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityZoneRule {
    pub zone_id: u32,
    pub name: String,
    pub level: TrustZoneLevel,
    pub allowed_subnets: Vec<String>,
    pub memory_base: u64,
    pub memory_size: u64,
    pub allow_dma: bool,
    pub require_pqc_attestation: bool,
}

impl SecurityZoneRule {
    pub fn new(zone_id: u32, name: &str, level: TrustZoneLevel, base: u64, size: u64) -> Self {
        Self {
            zone_id,
            name: name.to_string(),
            level,
            allowed_subnets: Vec::new(),
            memory_base: base,
            memory_size: size,
            allow_dma: false,
            require_pqc_attestation: true,
        }
    }

    pub fn is_memory_access_allowed(&self, virt_addr: u64, size: u64) -> bool {
        let end_addr = virt_addr.saturating_add(size);
        let zone_end = self.memory_base.saturating_add(self.memory_size);

        virt_addr >= self.memory_base && end_addr <= zone_end
    }
}

/// Sovereign Hardware & Software TrustZone Security Engine
#[derive(Debug, Clone)]
pub struct SovereignTrustZoneEngine {
    pub current_world: TrustZoneLevel,
    pub active_zones: BTreeMap<u32, SecurityZoneRule>,
    pub smc_dispatch_counter: u64,
    pub attestation_log: Vec<String>,
}

impl Default for SovereignTrustZoneEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SovereignTrustZoneEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            current_world: TrustZoneLevel::NormalWorldNonSecure,
            active_zones: BTreeMap::new(),
            smc_dispatch_counter: 0,
            attestation_log: Vec::new(),
        };

        // Register default system trust zones
        let mut normal_zone = SecurityZoneRule::new(
            1,
            "normal_world_userland",
            TrustZoneLevel::NormalWorldNonSecure,
            0x0000_0000_1000_0000,
            0x0000_0000_F000_0000,
        );
        normal_zone.allow_dma = true;
        normal_zone.require_pqc_attestation = false;

        let mut secure_zone = SecurityZoneRule::new(
            2,
            "secure_world_tee",
            TrustZoneLevel::SecureWorldIsolated,
            0x0000_0001_0000_0000,
            0x0000_0000_8000_0000,
        );
        secure_zone.allow_dma = false;
        secure_zone.require_pqc_attestation = true;

        let monitor_zone = SecurityZoneRule::new(
            3,
            "secure_monitor_el3",
            TrustZoneLevel::SecureMonitorEL3,
            0x0000_0002_0000_0000,
            0x0000_0000_1000_0000,
        );

        engine.active_zones.insert(1, normal_zone);
        engine.active_zones.insert(2, secure_zone);
        engine.active_zones.insert(3, monitor_zone);

        engine
    }

    /// Perform Secure Monitor Call (SMC) context switch between execution worlds
    pub fn secure_monitor_call(&mut self, target_world: TrustZoneLevel, smc_fid: u32) -> Result<String, String> {
        if self.current_world == target_world {
            return Ok(format!("Already in target world {:?} (FID 0x{:X})", target_world, smc_fid));
        }

        self.smc_dispatch_counter += 1;
        let prev_world = self.current_world;
        self.current_world = target_world;

        let log_msg = format!(
            "SMC #{} Switched from {:?} to {:?} via FID 0x{:X}",
            self.smc_dispatch_counter, prev_world, target_world, smc_fid
        );
        self.attestation_log.push(log_msg.clone());
        Ok(log_msg)
    }

    /// Evaluate memory access permissions across TrustZone boundaries
    pub fn validate_trust_zone_access(&self, zone_id: u32, address: u64, size: u64, is_dma: bool) -> bool {
        if let Some(zone) = self.active_zones.get(&zone_id) {
            if is_dma && !zone.allow_dma {
                return false; // DMA blocked in secure zone
            }
            if self.current_world < zone.level {
                return false; // Privilege isolation mismatch
            }
            zone.is_memory_access_allowed(address, size)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trust_zone_levels_and_smc_switch() {
        let mut tz = SovereignTrustZoneEngine::new();
        assert_eq!(tz.current_world, TrustZoneLevel::NormalWorldNonSecure);

        let res = tz.secure_monitor_call(TrustZoneLevel::SecureWorldIsolated, 0x84000001).unwrap();
        assert!(res.contains("Switched from NormalWorldNonSecure to SecureWorldIsolated"));
        assert_eq!(tz.current_world, TrustZoneLevel::SecureWorldIsolated);
    }

    #[test]
    fn test_trust_zone_memory_and_dma_validation() {
        let mut tz = SovereignTrustZoneEngine::new();

        // Normal world DMA access
        assert!(tz.validate_trust_zone_access(1, 0x1000_0000, 4096, true));

        // Switch to Secure World
        tz.secure_monitor_call(TrustZoneLevel::SecureWorldIsolated, 0x84000001).unwrap();

        // Secure World DMA access should be denied
        assert!(!tz.validate_trust_zone_access(2, 0x1_0000_0000, 4096, true));

        // Secure World Non-DMA access should be allowed
        assert!(tz.validate_trust_zone_access(2, 0x1_0000_0000, 4096, false));
    }
}

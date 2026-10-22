// VLAN (Virtual Local Area Network) Support
// Inspired by Linux 802.1Q VLAN implementation for SigmaOS

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::mem::MaybeUninit;

// VLAN TAG control information
// 802.1Q VLAN header: 2 bytes TPI + 2 bytes TCI
// TCI: 1bit PRI + 1bit CFI + 1bit DEI + 12 bits VLAN ID

/// VLAN ID range (1-4095)
pub const VLAN_MIN_ID: u16 = 1;
pub const VLAN_MAX_ID: u16 = 4095;
pub const VLAN_ANY_ID: u16 = 0;

/// VLAN TAG: 16 bits
/// Bit 15: CFI (Canonical Format Indicator)
/// Bits 14-13: Priority (3 bits)
/// Bit 12: DEI (Drop Eligible Indicator)
/// Bits 11-0: VLAN ID (12 bits)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VlanTag(u16);

impl VlanTag {
    /// Create a VLAN tag from priority and VLAN ID
    pub fn new(priority: u8, vlan_id: u16, dei: bool) -> Result<Self, String> {
        if vlan_id < VLAN_MIN_ID || vlan_id > VLAN_MAX_ID {
            return Err(format!("VLAN ID must be {}-{}, got {}", VLAN_MIN_ID, VLAN_MAX_ID, vlan_id));
        }
        if priority > 7 {
            return Err("Priority must be 0-7".to_string());
        }
        
        let tci: u16 = (dei as u16) << 13 | (priority as u16) << 13 | vlan_id as u16;
        // Wait, that's wrong. Let me fix:
        // Bit 15: CFI (always 0 for legacy)
        // Bits 14-12: Priority (3 bits)
        // Bit 11: DEI (Drop Eligible Indicator)
        // Bits 10-0: VLAN ID (12 bits)
        
        let tci_correct: u16 = (priory as u16) << 13 | (dei as u16) << 12 | (vlan_id as u16);
        
        // Actually let me redo this properly:
        // 15: CFI (1 bit)
        // 14-13: Priority (2 bits) - wait, that's only 2 bits for 4 priorities
        // 12: DEI (1 bit)
        // 11-0: VLAN ID (12 bits)
        
        // Correct bit layout:
        // 15: CFI (always 0)
        // 14-12: Priority (3 bits, 0-7)
        // 11: DEI (0 or 1)
        // 10-0: VLAN ID (0-4095)
        
        let tci_value: u16 = ((priority as u16) << 13) | (if dei { 1 } else { 0 } << 12) | (vlan_id as u16);
        
        Ok(VlanTag(tci_value))
    }
    
    /// Get the VLAN ID
    pub fn vlan_id(&self) -> u16 {
        self.0 & 0x0FFF
    }
    
    /// Get the priority (0-7)
    pub fn priority(&self) -> u8 {
        ((self.0 >> 13) & 0x7) as u8
    }
    
    /// Get the DEI flag
    pub fn dei(&self) -> bool {
        (self.0 >> 12) & 1 == 1
    }
    
    /// Get the CFI flag
    pub fn cfi(&self) -> bool {
        (self.0 >> 15) & 1 == 1
    }
    
    /// Convert to u16 (raw VLAN tag)
    pub fn to_u16(&self) -> u16 {
        self.0
    }
}

/// VLAN interface extension
#[derive(Debug, Clone)]
pub struct VlanInterface {
    pub physical_interface: String,
    pub vlan_id: VlanTag,
    pub tagged_frames: bool,
}

impl VlanInterface {
    /// Create a new VLAN interface
    pub fn new(physical: &str, vlan_id: u16) -> Result<Self, String> {
        let tag = VlanTag::new(0, vlan_id, false)?;
        Ok(VlanInterface {
            physical_interface: physical.to_string(),
            vlan_id: tag,
            tagged_frames: true,
        })
    }
    
    /// Get the VLAN ID as u16
    pub fn vlan_id_u16(&self) -> u16 {
        self.vlan_id.vlan_id()
    }
    
    /// Check if frames are tagged
    pub fn is_tagged(&self) -> bool {
        self.tagged_frames
    }
}

/// VLAN translation entry
#[derive(Debug, Clone)]
pub struct VlanTranslation {
    pub input_vlan: VlanTag,
    pub output_vlan: VlanTag,
    pub rewrite: bool,
}

impl VlanTranslation {
    /// Create a VLAN translation entry
    pub fn new(input: u16, output: u16, rewrite: bool) -> Result<Self, String> {
        let input_tag = VlanTag::new(0, input, false)?;
        let output_tag = VlanTag::new(0, output, false)?;
        
        Ok(VlanTranslation {
            input_vlan: input_tag,
            output_vlan: output_tag,
            rewrite,
        })
    }
}

/// VLAN manager
#[derive(Debug, Clone)]
pub struct VlanManager {
    /// Mapping of physical interface to VLAN tags
    pub interface_vlans: BTreeMap<String, Vec<VlanTag>>,
    /// VLAN translation table
    pub translations: BTreeMap<(u16, u16), VlanTranslation>,
    /// VLAN counters
    pub counters: VlanCounters,
}

/// VLAN counters/statistics
#[derive(Debug, Clone, Default)]
pub struct VlanCounters {
    pub frames_tagged: u64,
    pub frames_untagged: u64,
    pub frames_dropped: u64,
    pub frames_with_unknown_vlan: u64,
}

impl VlanManager {
    /// Create a new VLAN manager
    pub fn new() -> Self {
        VlanManager {
            interface_vlans: BTreeMap::new(),
            translations: BTreeMap::new(),
            counters: VlanCounters::default(),
        }
    }
    
    /// Add VLAN to physical interface
    pub fn add_vlan(&mut self, physical: &str, vlan_id: u16) -> Result<(), String> {
        let tag = VlanTag::new(0, vlan_id, false)?;
        self.interface_vlans
            .entry(physical.to_string())
            .or_insert_with(Vec::new)
            .push(tag);
        self.counters.frames_tagged += 1;
        Ok(())
    }
    
    /// Remove VLAN from physical interface
    pub fn remove_vlan(&mut self, physical: &str, vlan_id: u16) -> Result<(), String> {
        let tag = VlanTag::new(0, vlan_id, false)?;
        if let Some(vlans) = self.interface_vlans.get_mut(physical) {
            vlans.retain(|t| t.vlan_id() != vlan_id);
        }
        Ok(())
    }
    
    /// Translate VLAN tag
    pub fn translate(&self, input_vlan: u16) -> Option<VlanTag> {
        let key = (0u16, input_vlan);  // source VLAN
        self.translations.get(&key).map(|t| t.output_vlan)
    }
    
    /// Add VLAN translation
    pub fn add_translation(&mut self, input_vlan: u16, output_vlan: u16, rewrite: bool) -> Result<(), String> {
        let translation = VlanTranslation::new(input_vlan, output_vlan, rewrite)?;
        let key = (input_vlan, output_vlan);
        self.translations.insert(key, translation);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vlan_tag_creation() {
        let tag = VlanTag::new(5, 100, false).unwrap();
        assert_eq!(tag.vlan_id(), 100);
        assert_eq!(tag.priority(), 5);
        assert!(!tag.dei());
    }
    
    #[test]
    fn test_vlan_id_range() {
        let tag = VlanTag::new(0, 1, false).unwrap();
        assert_eq!(tag.vlan_id(), 1);
        
        let tag = VlanTag::new(0, 4095, false).unwrap();
        assert_eq!(tag.vlan_id(), 4095);
    }
    
    #[test]
    fn test_vlan_invalid_id() {
        let result = VlanTag::new(0, 0, false);
        assert!(result.is_err());
        
        let result = VlanTag::new(0, 4096, false);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_vlan_invalid_priority() {
        let result = VlanTag::new(8, 100, false);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_vlan_manager() {
        let mut manager = VlanManager::new();
        manager.add_vlan("eth0", 10).unwrap();
        manager.add_vlan("eth0", 20).unwrap();
        
        assert_eq!(manager.interface_vlans.get("eth0").unwrap().len(), 2);
        
        manager.remove_vlan("eth0", 10).unwrap();
        assert_eq!(manager.interface_vlans.get("eth0").unwrap().len(), 1);
    }
    
    #[test]
    fn test_vlan_translation() {
        let mut manager = VlanManager::new();
        manager.add_translation(10, 20, true).unwrap();
        
        let translated = manager.translate(10);
        assert!(translated.is_some());
        assert_eq!(translated.unwrap().vlan_id(), 20);
    }
}

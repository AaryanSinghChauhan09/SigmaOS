// Kernel Pointer Restriction for SigmaOS
// Kernel pointer restriction per Wiki 04-Kernel.md
// Provides kernel pointer exposure control

use std::string::{String, ToString};

/// Kernel pointer restriction level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KptrRestrictLevel {
    /// No restriction - kernel pointers visible to all
    None = 0,
    /// Restrict to processes with CAP_SYSLOG
    Restricted = 1,
    /// Completely hide kernel pointers
    Hidden = 2,
    Strict,}

impl KptrRestrictLevel {
    pub fn from_u32(value: u32) -> Self {
        match value {
            0 => KptrRestrictLevel::None,
            1 => KptrRestrictLevel::Restricted,
            2 => KptrRestrictLevel::Hidden,
            _ => KptrRestrictLevel::Restricted,
        }
    }

    pub fn as_u32(&self) -> u32 {
        *self as u32
    }

    pub fn as_str(&self) -> &str {
        match self {
            KptrRestrictLevel::None => "none",
            KptrRestrictLevel::Restricted => "restricted",
            KptrRestrictLevel::Hidden | KptrRestrictLevel::Strict => "hidden",
        }
    }

    pub fn is_restricted(&self) -> bool {
        matches!(self, KptrRestrictLevel::Restricted | KptrRestrictLevel::Hidden)
    }

    pub fn is_hidden(&self) -> bool {
        matches!(self, KptrRestrictLevel::Hidden)
    }
}

/// Kernel pointer restriction manager
#[derive(Debug, Clone)]
pub struct KptrRestrict {
    pub level: KptrRestrictLevel,
}

impl Default for KptrRestrict {
    fn default() -> Self {
        KptrRestrict {
            level: KptrRestrictLevel::Restricted,
        }
    }
}

impl KptrRestrict {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_level(mut self, level: KptrRestrictLevel) -> Self {
        self.level = level;
        self
    }

    pub fn set_level(&mut self, level: KptrRestrictLevel) {
        self.level = level;
    }

    pub fn get_level(&self) -> KptrRestrictLevel {
        self.level
    }

    pub fn restrict_all(&mut self) {
        self.level = KptrRestrictLevel::Hidden;
    }

    pub fn unrestrict(&mut self) {
        self.level = KptrRestrictLevel::None;
    }

    pub fn restrict_capable(&mut self) {
        self.level = KptrRestrictLevel::Restricted;
    }

    pub fn is_pointer_visible(&self, has_cap_syslog: bool) -> bool {
        match self.level {
            KptrRestrictLevel::None => true,
            KptrRestrictLevel::Restricted => has_cap_syslog,
            KptrRestrictLevel::Hidden | KptrRestrictLevel::Strict => false,
        }
    }

    pub fn is_hidden(&self) -> bool {
        self.level.is_hidden()
    }

    pub fn is_restricted(&self) -> bool {
        self.level.is_restricted()
    }

    pub fn mask_pointer(&self, pointer: usize, has_cap_syslog: bool) -> String {
        if self.is_pointer_visible(has_cap_syslog) {
            format!("{:#x}", pointer)
        } else {
            String::from("0x0000000000000000")
        }
    }

    pub fn mask_pointer_short(&self, pointer: usize, has_cap_syslog: bool) -> String {
        if self.is_pointer_visible(has_cap_syslog) {
            format!("{:#x}", pointer)
        } else {
            String::from("(ptr)")
        }
    }

    pub fn get_sysctl_value(&self) -> String {
        self.level.as_u32().to_string()
    }

    pub fn set_from_sysctl(&mut self, value: &str) -> Result<String, String> {
        let parsed = value.parse::<u32>()
            .map_err(|_| String::from("Invalid numeric value"))?;

        self.level = KptrRestrictLevel::from_u32(parsed);
        Ok(String::from("kptr_restrict updated"))
    }

    pub fn get_description(&self) -> String {
        match self.level {
            KptrRestrictLevel::None => String::from("No restriction - kernel pointers visible to all"),
            KptrRestrictLevel::Restricted => String::from("Restrict to processes with CAP_SYSLOG"),
            KptrRestrictLevel::Hidden | KptrRestrictLevel::Strict => String::from("Completely hide kernel pointers"),
        }
    }
}

/// Dmesg restriction level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmesgRestrictLevel {
    /// No restriction - dmesg visible to all
    None = 0,
    /// Restrict to processes with CAP_SYSLOG
    Restricted = 1,
    Strict,}

impl DmesgRestrictLevel {
    pub fn from_u32(value: u32) -> Self {
        match value {
            0 => DmesgRestrictLevel::None,
            1 => DmesgRestrictLevel::Restricted,
            _ => DmesgRestrictLevel::Restricted,
        }
    }

    pub fn as_u32(&self) -> u32 {
        *self as u32
    }

    /// Check if kernel pointer should be sanitized
    pub fn should_sanitize_pointer(&self) -> bool {
        matches!(
            self.get_kptr_restrict(),
            KptrRestrictLevel::Restricted | KptrRestrictLevel::Strict
        )
    }

    /// Sanitize kernel pointer for display
    pub fn sanitize_pointer(&self, ptr: usize) -> usize {
        if self.should_sanitize_pointer() {
            0 // Return null pointer
        } else {
            ptr
        }
    }

    /// Check if dmesg message should be displayed
    pub fn should_show_dmesg(&self, level: u32) -> bool {
        match self.get_dmesg_restrict() {
            DmesgRestrictLevel::None => true,
            DmesgRestrictLevel::Restricted => level >= 6, // Only show critical messages
            DmesgRestrictLevel::Strict => level >= 7,     // Only show emergency messages
        }
    }
}

/// Dmesg restriction manager
#[derive(Debug, Clone)]
pub struct DmesgRestrict {
    pub level: DmesgRestrictLevel,
}

impl Default for DmesgRestrict {
    fn default() -> Self {
        DmesgRestrict {
            level: DmesgRestrictLevel::Restricted,
        }
    }
}

impl DmesgRestrict {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_level(mut self, level: DmesgRestrictLevel) -> Self {
        self.level = level;
        self
    }

    pub fn set_level(&mut self, level: DmesgRestrictLevel) {
        self.level = level;
    }

    pub fn get_level(&self) -> DmesgRestrictLevel {
        self.level
    }

    pub fn restrict(&mut self) {
        self.level = DmesgRestrictLevel::Restricted;
    }

    pub fn unrestrict(&mut self) {
        self.level = DmesgRestrictLevel::None;
    }

    pub fn is_dmesg_visible(&self, has_cap_syslog: bool) -> bool {
        match self.level {
            DmesgRestrictLevel::None => true,
            DmesgRestrictLevel::Restricted | DmesgRestrictLevel::Strict => has_cap_syslog,
        }
    }

    pub fn is_restricted(&self) -> bool {
        self.level.is_restricted()
    }

    pub fn get_sysctl_value(&self) -> String {
        self.level.as_u32().to_string()
    }

    pub fn set_from_sysctl(&mut self, value: &str) -> Result<String, String> {
        let parsed = value.parse::<u32>()
            .map_err(|_| String::from("Invalid numeric value"))?;

        self.level = DmesgRestrictLevel::from_u32(parsed);
        Ok(String::from("dmesg_restrict updated"))
    }

    pub fn get_description(&self) -> String {
        match self.level {
            DmesgRestrictLevel::None => String::from("No restriction - dmesg visible to all"),
            DmesgRestrictLevel::Restricted | DmesgRestrictLevel::Strict => String::from("Restrict to processes with CAP_SYSLOG"),
        }
    }
}

/// Kernel security parameters manager
#[derive(Debug, Clone)]
pub struct KernelSecurityParams {
    pub kptr_restrict: KptrRestrict,
    pub dmesg_restrict: DmesgRestrict,
    pub modules_disabled: bool,
}

impl Default for KernelSecurityParams {
    fn default() -> Self {
        KernelSecurityParams {
            kptr_restrict: KptrRestrict::default(),
            dmesg_restrict: DmesgRestrict::default(),
            modules_disabled: false,
        }
    }
}

impl KernelSecurityParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_kptr_level(mut self, level: KptrRestrictLevel) -> Self {
        self.kptr_restrict.set_level(level);
        self
    }

    pub fn with_dmesg_level(mut self, level: DmesgRestrictLevel) -> Self {
        self.dmesg_restrict.set_level(level);
        self
    }

    pub fn disable_module_loading(&mut self) {
        self.modules_disabled = true;
    }

    pub fn enable_module_loading(&mut self) {
        self.modules_disabled = false;
    }

    pub fn is_module_loading_enabled(&self) -> bool {
        !self.modules_disabled
    }

    pub fn maximize_security(&mut self) {
        self.kptr_restrict.restrict_all();
        self.dmesg_restrict.restrict();
        self.modules_disabled = true;
    }

    pub fn minimize_security(&mut self) {
        self.kptr_restrict.unrestrict();
        self.dmesg_restrict.unrestrict();
        self.modules_disabled = false;
    }

    pub fn apply_default_hardening(&mut self) {
        self.kptr_restrict.restrict_capable();
        self.dmesg_restrict.restrict();
        self.modules_disabled = false;
    }

    pub fn get_security_level(&self) -> SecurityLevel {
        if self.kptr_restrict.is_hidden() && self.dmesg_restrict.is_restricted() && self.modules_disabled {
            SecurityLevel::Maximum
        } else if self.kptr_restrict.is_restricted() && self.dmesg_restrict.is_restricted() {
            SecurityLevel::High
        } else if self.kptr_restrict.is_restricted() || self.dmesg_restrict.is_restricted() {
            SecurityLevel::Medium
        } else {
            SecurityLevel::Low
        }
    }

    pub fn get_sysctl_configs(&self) -> Vec<(String, String)> {
        vec![
            (String::from("kernel.kptr_restrict"), self.kptr_restrict.get_sysctl_value()),
            (String::from("kernel.dmesg_restrict"), self.dmesg_restrict.get_sysctl_value()),
            (String::from("kernel.modules_disabled"), if self.modules_disabled { String::from("1") } else { String::from("0") }),
        ]
    }

    pub fn apply_sysctl(&mut self, key: &str, value: &str) -> Result<String, String> {
        match key {
            "kernel.kptr_restrict" => self.kptr_restrict.set_from_sysctl(value),
            "kernel.dmesg_restrict" => self.dmesg_restrict.set_from_sysctl(value),
            "kernel.modules_disabled" => {
                let parsed = value.parse::<u32>()
                    .map_err(|_| String::from("Invalid numeric value"))?;
                self.modules_disabled = parsed == 1;
                Ok(String::from("modules_disabled updated"))
            }
            _ => Err(String::from("Unknown sysctl key")),
        }
    }
}

/// Security level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityLevel {
    Low,
    Medium,
    High,
    Maximum,
}

impl SecurityLevel {
    pub fn as_str(&self) -> &str {
        match self {
            SecurityLevel::Low => "low",
            SecurityLevel::Medium => "medium",
            SecurityLevel::High => "high",
            SecurityLevel::Maximum => "maximum",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kptr_restrict_levels() {
        let mitigations = KernelSecurityMitigations::new();
        assert_eq!(mitigations.get_kptr_restrict(), KptrRestrictLevel::None);

        mitigations.set_kptr_restrict(KptrRestrictLevel::Strict);
        assert_eq!(mitigations.get_kptr_restrict(), KptrRestrictLevel::Strict);
    }

    #[test]
    fn test_dmesg_restrict_levels() {
        let mitigations = KernelSecurityMitigations::new();
        assert_eq!(mitigations.get_dmesg_restrict(), DmesgRestrictLevel::None);

        mitigations.set_dmesg_restrict(DmesgRestrictLevel::Restricted);
        assert_eq!(
            mitigations.get_dmesg_restrict(),
            DmesgRestrictLevel::Restricted
        );
    }

    #[test]
    fn test_pointer_sanitization() {
        let mitigations = KernelSecurityMitigations::new();
        let ptr = 0xdeadbeefusize;

        // No restriction - pointer should not be sanitized
        assert_eq!(mitigations.sanitize_pointer(ptr), ptr);

        mitigations.set_kptr_restrict(KptrRestrictLevel::Strict);
        assert_eq!(mitigations.sanitize_pointer(ptr), 0);
    }

    #[test]
    fn test_module_loading_control() {
        let mitigations = KernelSecurityMitigations::new();
        assert!(!mitigations.are_modules_disabled());

        mitigations.disable_modules();
        assert!(mitigations.are_modules_disabled());

        mitigations.enable_modules();
        assert!(!mitigations.are_modules_disabled());
    }

    #[test]
    fn test_dmesg_filtering() {
        let mitigations = KernelSecurityMitigations::new();

        // No restriction - all messages shown
        assert!(mitigations.should_show_dmesg(1));
        assert!(mitigations.should_show_dmesg(7));

        mitigations.set_dmesg_restrict(DmesgRestrictLevel::Restricted);
        assert!(!mitigations.should_show_dmesg(1)); // Info level filtered
        assert!(mitigations.should_show_dmesg(6)); // Critical level shown
    }

    #[test]
    fn test_kptr_restrict_is_pointer_visible() {
        let mut kptr = KptrRestrict::new();

        kptr.set_level(KptrRestrictLevel::None);
        assert!(kptr.is_pointer_visible(false));

        kptr.set_level(KptrRestrictLevel::Restricted);
        assert!(!kptr.is_pointer_visible(false));
        assert!(kptr.is_pointer_visible(true));

        kptr.set_level(KptrRestrictLevel::Hidden);
        assert!(!kptr.is_pointer_visible(true));
    }

    #[test]
    fn test_kptr_restrict_mask_pointer() {
        let kptr = KptrRestrict::new().with_level(KptrRestrictLevel::Hidden);

        let masked = kptr.mask_pointer(0x1234567890abcdef, false);
        assert_eq!(masked, "0x0000000000000000");

        let masked = kptr.mask_pointer(0x1234567890abcdef, true);
        assert_eq!(masked, "0x0000000000000000");
    }

    #[test]
    fn test_dmesg_restrict_level_from_u32() {
        assert_eq!(DmesgRestrictLevel::from_u32(0), DmesgRestrictLevel::None);
        assert_eq!(DmesgRestrictLevel::from_u32(1), DmesgRestrictLevel::Restricted);
    }

    #[test]
    fn test_dmesg_restrict_creation() {
        let dmesg = DmesgRestrict::new();
        assert_eq!(dmesg.level, DmesgRestrictLevel::Restricted);
    }

    #[test]
    fn test_dmesg_restrict_is_dmesg_visible() {
        let mut dmesg = DmesgRestrict::new();

        dmesg.set_level(DmesgRestrictLevel::None);
        assert!(dmesg.is_dmesg_visible(false));

        dmesg.set_level(DmesgRestrictLevel::Restricted);
        assert!(!dmesg.is_dmesg_visible(false));
        assert!(dmesg.is_dmesg_visible(true));
    }

    #[test]
    fn test_kernel_security_params_creation() {
        let params = KernelSecurityParams::new();
        assert!(!params.modules_disabled);
        assert_eq!(params.kptr_restrict.level, KptrRestrictLevel::Restricted);
    }

    #[test]
    fn test_kernel_security_params_maximize_security() {
        let mut params = KernelSecurityParams::new();
        params.maximize_security();

        assert!(params.modules_disabled);
        assert_eq!(params.kptr_restrict.level, KptrRestrictLevel::Hidden);
        assert_eq!(params.dmesg_restrict.level, DmesgRestrictLevel::Restricted);
    }

    #[test]
    fn test_kernel_security_params_minimize_security() {
        let mut params = KernelSecurityParams::new();
        params.minimize_security();

        assert!(!params.modules_disabled);
        assert_eq!(params.kptr_restrict.level, KptrRestrictLevel::None);
        assert_eq!(params.dmesg_restrict.level, DmesgRestrictLevel::None);
    }

    #[test]
    fn test_kernel_security_params_get_security_level() {
        let mut params = KernelSecurityParams::new();

        params.minimize_security();
        assert_eq!(params.get_security_level(), SecurityLevel::Low);

        params.apply_default_hardening();
        assert_eq!(params.get_security_level(), SecurityLevel::High);

        params.maximize_security();
        assert_eq!(params.get_security_level(), SecurityLevel::Maximum);
    }

    #[test]
    fn test_kernel_security_params_get_sysctl_configs() {
        let params = KernelSecurityParams::new();
        let configs = params.get_sysctl_configs();

        assert_eq!(configs.len(), 3);
        assert!(configs.iter().any(|(k, _)| k == "kernel.kptr_restrict"));
        assert!(configs.iter().any(|(k, _)| k == "kernel.dmesg_restrict"));
        assert!(configs.iter().any(|(k, _)| k == "kernel.modules_disabled"));
    }

    #[test]
    fn test_kernel_security_params_apply_sysctl() {
        let mut params = KernelSecurityParams::new();

        assert!(params.apply_sysctl("kernel.kptr_restrict", "2").is_ok());
        assert_eq!(params.kptr_restrict.level, KptrRestrictLevel::Hidden);

        assert!(params.apply_sysctl("kernel.modules_disabled", "1").is_ok());
        assert!(params.modules_disabled);

        assert!(params.apply_sysctl("unknown.key", "1").is_err());
    }

    #[test]
    fn test_security_level_as_str() {
        assert_eq!(SecurityLevel::Low.as_str(), "low");
        assert_eq!(SecurityLevel::Maximum.as_str(), "maximum");
    }
}

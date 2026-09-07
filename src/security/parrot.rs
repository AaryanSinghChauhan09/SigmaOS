// SigmaOS Parrot Security OS Security Fingerprint & Sandbox Spoofing Subsystem

use std::string::String;
use std::string::ToString;

pub struct ParrotSecurityFingerprintEngine {
    pub spoofed_os_name: String,
    pub active_sandbox_mode: String,
}

impl ParrotSecurityFingerprintEngine {
    pub fn new() -> Self {
        Self {
            spoofed_os_name: "Parrot Security OS 6.0".to_string(),
            active_sandbox_mode: "Strict".to_string(),
        }
    }

    pub fn get_user_agent_signature(&self) -> String {
        self.spoofed_os_name.clone()
    }
}

impl Default for ParrotSecurityFingerprintEngine {
    fn default() -> Self {
        Self::new()
    }
}

use std::format;
use std::string::{String, ToString};
use std::vec::Vec;
// Sovereign APM (Agent Package Manager)
// Core native package manager for isolated, reproducible sovereign application deployments.

use crate::klib::HashMap;
use crate::security::CapabilityToken;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IsolationLevel {
    FullSandbox,     // Complete isolation (no IPC/network unless granted)
    SharedNamespace, // Shared namespace (restricted access)
    BareMetalDirect, // Real-time bare metal direct thread (highly restricted, verified signature required)
}

#[derive(Debug, Clone)]
pub struct SovereignApp {
    pub name: String,
    pub version: String,
    pub executable_hash: [u8; 32],
    pub memory_limit_mb: usize,
    pub cpu_shares: usize,
    pub required_permissions: Vec<String>,
    pub isolation: IsolationLevel,
    pub is_verified: bool,
}

impl SovereignApp {
    pub fn new(name: &str, version: &str, executable_hash: [u8; 32]) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            executable_hash,
            memory_limit_mb: 128,
            cpu_shares: 1024,
            required_permissions: Vec::new(),
            isolation: IsolationLevel::FullSandbox,
            is_verified: false,
        }
    }

    pub fn with_limits(mut self, memory_mb: usize, cpu_shares: usize) -> Self {
        self.memory_limit_mb = memory_mb;
        self.cpu_shares = cpu_shares;
        self
    }

    pub fn with_permission(mut self, perm: &str) -> Self {
        self.required_permissions.push(perm.to_string());
        self
    }

    pub fn with_isolation(mut self, level: IsolationLevel) -> Self {
        self.isolation = level;
        self
    }
}

pub struct SovereignApm {
    pub registry: HashMap<String, SovereignApp>,
    pub installed: HashMap<String, SovereignApp>,
    pub active_containers: HashMap<String, IsolationLevel>,
    pub authority_key: [u8; 32], // Secure root verification key
}

impl SovereignApm {
    pub fn new(authority_key: [u8; 32]) -> Self {
        Self {
            registry: HashMap::new(),
            installed: HashMap::new(),
            active_containers: HashMap::new(),
            authority_key,
        }
    }

    pub fn register_app(&mut self, app: SovereignApp) {
        self.registry.insert(app.name.clone(), app);
    }

    /// Verifies the app signature recursively before deployment
    pub fn cryptographically_verify(&self, app_name: &str, signature: &[u8]) -> bool {
        if let Some(_app) = self.registry.get(app_name) {
            // Simulated post-quantum cryptographic signature check against authority_key
            if signature.len() == 32 && signature == self.authority_key {
                return true;
            }
        }
        false
    }

    /// Deploys and installs the sovereign application, enforcing sandboxing boundaries
    pub fn install_app(&mut self, app_name: &str, signature: &[u8]) -> Result<(), String> {
        let mut app = self
            .registry
            .get(app_name)
            .cloned()
            .ok_or("Application not registered in APM registry")?;

        // 1. Check signature verification
        if !self.cryptographically_verify(app_name, signature) {
            return Err("Cryptographic signature verification failed!".to_string());
        }
        app.is_verified = true;

        // 2. Enforce Sovereign constraints (No application can request BareMetalDirect unless verified and limited to < 512MB RAM)
        if app.isolation == IsolationLevel::BareMetalDirect && app.memory_limit_mb > 512 {
            return Err(
                "Resource violation: BareMetalDirect isolation level must restrict RAM below 512MB"
                    .to_string(),
            );
        }

        self.installed.insert(app_name.to_string(), app);
        Ok(())
    }

    /// Launch application in isolated container
    pub fn launch_app(
        &mut self,
        app_name: &str,
        granted_permissions: &CapabilityToken,
    ) -> Result<IsolationLevel, String> {
        let app = self
            .installed
            .get(app_name)
            .ok_or("Application not installed")?;

        // Check if sandbox permissions are satisfied by the security capability token
        for perm in &app.required_permissions {
            if perm == "Network" && (granted_permissions.bits() & 1 == 0) {
                return Err(format!(
                    "Security Violation: App '{}' requires Network permission",
                    app_name
                ));
            }
            if perm == "HardwareAccess" && (granted_permissions.bits() & 2 == 0) {
                return Err(format!(
                    "Security Violation: App '{}' requires HardwareAccess permission",
                    app_name
                ));
            }
        }

        self.active_containers
            .insert(app_name.to_string(), app.isolation);
        Ok(app.isolation)
    }

    /// Terminate and uninstall the application cleanly
    pub fn uninstall_app(&mut self, app_name: &str) -> Result<(), String> {
        if self.installed.remove(app_name).is_some() {
            self.active_containers.remove(app_name);
            Ok(())
        } else {
            Err("Application not installed".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sovereign_apm_registration() {
        let root_key = [7u8; 32];
        let mut apm = SovereignApm::new(root_key);

        let app = SovereignApp::new("sov-db", "1.2.0", [0xAA; 32])
            .with_limits(256, 512)
            .with_permission("Network")
            .with_isolation(IsolationLevel::FullSandbox);

        apm.register_app(app);
        assert!(apm.registry.contains_key("sov-db"));
    }

    #[test]
    fn test_sovereign_apm_install_verification() {
        let root_key = [7u8; 32];
        let mut apm = SovereignApm::new(root_key);

        let app = SovereignApp::new("sov-db", "1.2.0", [0xAA; 32])
            .with_limits(256, 512)
            .with_permission("Network")
            .with_isolation(IsolationLevel::FullSandbox);

        apm.register_app(app);

        // Invalid signature -> fail
        assert!(apm.install_app("sov-db", &[0u8; 32]).is_err());

        // Valid signature -> success
        assert!(apm.install_app("sov-db", &root_key).is_ok());
        assert!(apm.installed.contains_key("sov-db"));
        assert!(apm.installed.get("sov-db").unwrap().is_verified);
    }

    #[test]
    fn test_resource_violation_rejection() {
        let root_key = [7u8; 32];
        let mut apm = SovereignApm::new(root_key);

        // BareMetalDirect with excessive memory -> fail
        let app = SovereignApp::new("sov-core", "1.0.0", [0xCC; 32])
            .with_limits(1024, 2048)
            .with_isolation(IsolationLevel::BareMetalDirect);

        apm.register_app(app);
        assert!(apm.install_app("sov-core", &root_key).is_err());
    }

    #[test]
    fn test_sandbox_permission_gating() {
        let root_key = [7u8; 32];
        let mut apm = SovereignApm::new(root_key);

        let app = SovereignApp::new("sov-net", "1.0.0", [0xDD; 32])
            .with_permission("Network")
            .with_isolation(IsolationLevel::FullSandbox);

        apm.register_app(app);
        apm.install_app("sov-net", &root_key).unwrap();

        // 1. Launch with empty capability token (fails)
        let empty_token = CapabilityToken::new();
        assert!(apm.launch_app("sov-net", &empty_token).is_err());

        // 2. Launch with Network capability token (bits=1) (succeeds)
        let net_token = CapabilityToken::new().allow_network("tcp", 80);
        assert!(apm.launch_app("sov-net", &net_token).is_ok());
    }
}

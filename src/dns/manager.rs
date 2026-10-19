//! SigmaOS DNS Manager
//!
//! Inspired by: Omarchy DNS (omancy-dns)
//! Following principles: SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY,
//! LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING,
//! SECURE_DEFAULTS, OPEN_STANDARD_AND_INTEROPERABILITY, NETWORK_ISOLATION.

/// DNS provider
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DnsProvider {
    Cloudflare,
    Google,
    Quad9,
    OpenDns,
    Custom(String),
}

/// DNS configuration
#[derive(Debug, Clone)]
pub struct DnsConfig {
    pub provider: DnsProvider,
    pub primary: String,
    pub secondary: Option<String>,
    pub doh_enabled: bool,
    pub doh_url: Option<String>,
}

/// DNS manager
pub struct DnsManager {
    pub config: DnsConfig,
    pub available_providers: Vec<DnsProvider>,
}

impl DnsManager {
    pub fn new() -> Self {
        Self {
            config: DnsConfig {
                provider: DnsProvider::Cloudflare,
                primary: "1.1.1.1".to_string(),
                secondary: Some("1.0.0.1".to_string()),
                doh_enabled: false,
                doh_url: Some("https://cloudflare-dns.com/dns-query".to_string()),
            },
            available_providers: vec![
                DnsProvider::Cloudflare,
                DnsProvider::Google,
                DnsProvider::Quad9,
                DnsProvider::OpenDns,
            ],
        }
    }

    /// Set DNS provider
    pub fn set_provider(&mut self, provider: DnsProvider) -> Result<(), String> {
        match provider {
            DnsProvider::Cloudflare => {
                self.config.primary = "1.1.1.1".to_string();
                self.config.secondary = Some("1.0.0.1".to_string());
                self.config.doh_url = Some("https://cloudflare-dns.com/dns-query".to_string());
            }
            DnsProvider::Google => {
                self.config.primary = "8.8.8.8".to_string();
                self.config.secondary = Some("8.8.4.4".to_string());
                self.config.doh_url = Some("https://dns.google/dns-query".to_string());
            }
            DnsProvider::Quad9 => {
                self.config.primary = "9.9.9.9".to_string();
                self.config.secondary = Some("149.112.112.112".to_string());
                self.config.doh_url = Some("https://dns.quad9.net/dns-query".to_string());
            }
            DnsProvider::OpenDns => {
                self.config.primary = "208.67.222.222".to_string();
                self.config.secondary = Some("208.67.220.220".to_string());
                self.config.doh_url = Some("https://doh.opendns.com/dns-query".to_string());
            }
            DnsProvider::Custom(name) => {
                return Err(format!("Custom provider {} not configured", name));
            }
        }
        self.config.provider = provider;
        Ok(())
    }

    /// Set custom DNS servers
    pub fn set_custom(&mut self, primary: String, secondary: Option<String>) -> Result<(), String> {
        if primary.is_empty() {
            return Err("Primary DNS cannot be empty".to_string());
        }
        self.config.provider = DnsProvider::Custom("custom".to_string());
        self.config.primary = primary;
        self.config.secondary = secondary;
        self.config.doh_enabled = false;
        self.config.doh_url = None;
        Ok(())
    }

    /// Enable DoH
    pub fn enable_doh(&mut self, url: Option<String>) -> Result<(), String> {
        self.config.doh_enabled = true;
        self.config.doh_url = url;
        Ok(())
    }

    /// Disable DoH
    pub fn disable_doh(&mut self) {
        self.config.doh_enabled = false;
        self.config.doh_url = None;
    }

    /// List available providers
    pub fn list_providers(&self) -> Vec<&DnsProvider> {
        self.available_providers.iter().collect()
    }

    /// Get config
    pub fn get_config(&self) -> &DnsConfig {
        &self.config
    }
}

impl Default for DnsManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let mgr = DnsManager::new();
        assert_eq!(mgr.config.primary, "1.1.1.1");
        assert!(matches!(mgr.config.provider, DnsProvider::Cloudflare));
    }

    #[test]
    fn test_set_provider() {
        let mut mgr = DnsManager::new();
        assert!(mgr.set_provider(DnsProvider::Google).is_ok());
        assert_eq!(mgr.config.primary, "8.8.8.8");
        assert_eq!(mgr.config.secondary, Some("8.8.4.4".to_string()));
    }

    #[test]
    fn test_set_custom() {
        let mut mgr = DnsManager::new();
        assert!(mgr
            .set_custom("192.168.1.1".to_string(), Some("8.8.8.8".to_string()))
            .is_ok());
        assert_eq!(mgr.config.primary, "192.168.1.1");
        assert!(matches!(mgr.config.provider, DnsProvider::Custom(_)));
    }

    #[test]
    fn test_empty_primary_fails() {
        let mut mgr = DnsManager::new();
        assert!(mgr.set_custom(String::new(), None).is_err());
    }

    #[test]
    fn test_enable_doh() {
        let mut mgr = DnsManager::new();
        assert!(mgr.enable_doh(Some("https://doh.example.com".to_string())).is_ok());
        assert!(mgr.config.doh_enabled);
    }

    #[test]
    fn test_list_providers() {
        let mgr = DnsManager::new();
        assert_eq!(mgr.list_providers().len(), 4);
    }
}
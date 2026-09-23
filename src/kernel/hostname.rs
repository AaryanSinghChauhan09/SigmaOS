// Linux-inspired Hostname Management
// Provides system hostname and domain name management

use std::sync::{Arc, Mutex};

/// Hostname manager for system-wide hostname management
pub struct HostnameManager {
    hostname: Arc<Mutex<String>>,
    domainname: Arc<Mutex<String>>,
}

impl HostnameManager {
    pub fn new() -> Self {
        Self {
            hostname: Arc::new(Mutex::new("localhost".to_string())),
            domainname: Arc::new(Mutex::new(String::new())),
        }
    }

    /// Set the hostname
    pub fn set_hostname(&self, hostname: String) -> Result<(), String> {
        if hostname.is_empty() {
            return Err("Hostname cannot be empty".to_string());
        }
        if hostname.len() > 253 {
            return Err("Hostname too long (max 253 characters)".to_string());
        }

        // Validate hostname characters
        for c in hostname.chars() {
            if !c.is_alphanumeric() && c != '-' && c != '.' {
                return Err(format!("Invalid character in hostname: {}", c));
            }
        }

        let mut hn = self.hostname.lock().unwrap();
        *hn = hostname;
        Ok(())
    }

    /// Get the hostname
    pub fn get_hostname(&self) -> String {
        let hn = self.hostname.lock().unwrap();
        hn.clone()
    }

    /// Set the domain name
    pub fn set_domainname(&self, domainname: String) -> Result<(), String> {
        if domainname.len() > 253 {
            return Err("Domain name too long (max 253 characters)".to_string());
        }

        // Validate domain name characters
        for c in domainname.chars() {
            if !c.is_alphanumeric() && c != '-' && c != '.' {
                return Err(format!("Invalid character in domain name: {}", c));
            }
        }

        let mut dn = self.domainname.lock().unwrap();
        *dn = domainname;
        Ok(())
    }

    /// Get the domain name
    pub fn get_domainname(&self) -> String {
        let dn = self.domainname.lock().unwrap();
        dn.clone()
    }

    /// Get the fully qualified domain name (FQDN)
    pub fn get_fqdn(&self) -> String {
        let hostname = self.get_hostname();
        let domainname = self.get_domainname();

        if domainname.is_empty() {
            hostname
        } else {
            format!("{}.{}", hostname, domainname)
        }
    }

    /// Set both hostname and domain name
    pub fn set_both(&self, hostname: String, domainname: String) -> Result<(), String> {
        self.set_hostname(hostname)?;
        self.set_domainname(domainname)?;
        Ok(())
    }
}

impl Default for HostnameManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hostname_default() {
        let manager = HostnameManager::new();
        assert_eq!(manager.get_hostname(), "localhost");
        assert_eq!(manager.get_domainname(), "");
    }

    #[test]
    fn test_hostname_set() {
        let manager = HostnameManager::new();
        manager.set_hostname("myhost".to_string()).unwrap();
        assert_eq!(manager.get_hostname(), "myhost");
    }

    #[test]
    fn test_hostname_empty() {
        let manager = HostnameManager::new();
        assert!(manager.set_hostname("".to_string()).is_err());
    }

    #[test]
    fn test_hostname_too_long() {
        let manager = HostnameManager::new();
        let long_name = "a".repeat(254);
        assert!(manager.set_hostname(long_name).is_err());
    }

    #[test]
    fn test_hostname_invalid_chars() {
        let manager = HostnameManager::new();
        assert!(manager.set_hostname("my_host".to_string()).is_err());
        assert!(manager.set_hostname("my@host".to_string()).is_err());
    }

    #[test]
    fn test_hostname_valid_chars() {
        let manager = HostnameManager::new();
        assert!(manager.set_hostname("my-host".to_string()).is_ok());
        assert!(manager.set_hostname("my.host".to_string()).is_ok());
        assert!(manager.set_hostname("myhost123".to_string()).is_ok());
    }

    #[test]
    fn test_domainname_set() {
        let manager = HostnameManager::new();
        manager.set_domainname("example.com".to_string()).unwrap();
        assert_eq!(manager.get_domainname(), "example.com");
    }

    #[test]
    fn test_domainname_too_long() {
        let manager = HostnameManager::new();
        let long_name = "a".repeat(254);
        assert!(manager.set_domainname(long_name).is_err());
    }

    #[test]
    fn test_domainname_invalid_chars() {
        let manager = HostnameManager::new();
        assert!(manager.set_domainname("example_com".to_string()).is_err());
    }

    #[test]
    fn test_fqdn() {
        let manager = HostnameManager::new();
        manager.set_hostname("myhost".to_string()).unwrap();
        manager.set_domainname("example.com".to_string()).unwrap();

        assert_eq!(manager.get_fqdn(), "myhost.example.com");
    }

    #[test]
    fn test_fqdn_no_domain() {
        let manager = HostnameManager::new();
        manager.set_hostname("myhost".to_string()).unwrap();

        assert_eq!(manager.get_fqdn(), "myhost");
    }

    #[test]
    fn test_set_both() {
        let manager = HostnameManager::new();
        manager.set_both("myhost".to_string(), "example.com".to_string()).unwrap();

        assert_eq!(manager.get_hostname(), "myhost");
        assert_eq!(manager.get_domainname(), "example.com");
    }

    #[test]
    fn test_set_both_invalid_hostname() {
        let manager = HostnameManager::new();
        assert!(manager.set_both("".to_string(), "example.com".to_string()).is_err());
    }
}

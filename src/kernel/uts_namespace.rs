// Linux-inspired UTS Namespace
// Provides per-process hostname and domain name isolation

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// UTS namespace data
#[derive(Debug, Clone)]
pub struct UtsNamespace {
    pub id: u64,
    pub hostname: String,
    pub domainname: String,
    pub parent_id: Option<u64>,
}

impl UtsNamespace {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            hostname: "localhost".to_string(),
            domainname: String::new(),
            parent_id: None,
        }
    }

    pub fn with_parent(mut self, parent_id: u64) -> Self {
        self.parent_id = Some(parent_id);
        self
    }

    /// Set hostname
    pub fn set_hostname(&mut self, hostname: String) -> Result<(), String> {
        if hostname.is_empty() {
            return Err("Hostname cannot be empty".to_string());
        }
        if hostname.len() > 253 {
            return Err("Hostname too long (max 253 characters)".to_string());
        }
        self.hostname = hostname;
        Ok(())
    }

    /// Get hostname
    pub fn get_hostname(&self) -> &str {
        &self.hostname
    }

    /// Set domainname
    pub fn set_domainname(&mut self, domainname: String) -> Result<(), String> {
        if domainname.len() > 253 {
            return Err("Domain name too long (max 253 characters)".to_string());
        }
        self.domainname = domainname;
        Ok(())
    }

    /// Get domainname
    pub fn get_domainname(&self) -> &str {
        &self.domainname
    }

    /// Get fully qualified domain name
    pub fn get_fqdn(&self) -> String {
        if self.domainname.is_empty() {
            self.hostname.clone()
        } else {
            format!("{}.{}", self.hostname, self.domainname)
        }
    }
}

/// UTS namespace manager for system-wide namespace management
pub struct UtsNamespaceManager {
    namespaces: Arc<Mutex<HashMap<u64, UtsNamespace>>>,
    next_namespace_id: Arc<Mutex<u64>>,
}

impl UtsNamespaceManager {
    pub fn new() -> Self {
        Self {
            namespaces: Arc::new(Mutex::new(HashMap::new())),
            next_namespace_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Create a new UTS namespace
    pub fn create_namespace(&self, parent_id: Option<u64>) -> u64 {
        let mut next_id = self.next_namespace_id.lock().unwrap();
        let namespace_id = *next_id;
        *next_id += 1;
        drop(next_id);

        let namespace = match parent_id {
            Some(pid) => UtsNamespace::new(namespace_id).with_parent(pid),
            None => UtsNamespace::new(namespace_id),
        };

        let mut namespaces = self.namespaces.lock().unwrap();
        namespaces.insert(namespace_id, namespace);

        namespace_id
    }

    /// Get a namespace by ID
    pub fn get_namespace(&self, namespace_id: u64) -> Option<UtsNamespace> {
        let namespaces = self.namespaces.lock().unwrap();
        namespaces.get(&namespace_id).cloned()
    }

    /// Remove a namespace
    pub fn remove_namespace(&self, namespace_id: u64) -> Result<(), String> {
        let mut namespaces = self.namespaces.lock().unwrap();
        match namespaces.remove(&namespace_id) {
            Some(_) => Ok(()),
            None => Err(format!("Namespace {} not found", namespace_id)),
        }
    }

    /// Set hostname for a namespace
    pub fn set_hostname(&self, namespace_id: u64, hostname: String) -> Result<(), String> {
        let mut namespaces = self.namespaces.lock().unwrap();
        match namespaces.get_mut(&namespace_id) {
            Some(namespace) => namespace.set_hostname(hostname),
            None => Err(format!("Namespace {} not found", namespace_id)),
        }
    }

    /// Set domainname for a namespace
    pub fn set_domainname(&self, namespace_id: u64, domainname: String) -> Result<(), String> {
        let mut namespaces = self.namespaces.lock().unwrap();
        match namespaces.get_mut(&namespace_id) {
            Some(namespace) => namespace.set_domainname(domainname),
            None => Err(format!("Namespace {} not found", namespace_id)),
        }
    }

    /// Get namespace count
    pub fn namespace_count(&self) -> usize {
        let namespaces = self.namespaces.lock().unwrap();
        namespaces.len()
    }
}

impl Default for UtsNamespaceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uts_namespace() {
        let namespace = UtsNamespace::new(1);
        assert_eq!(namespace.id, 1);
        assert_eq!(namespace.get_hostname(), "localhost");
        assert_eq!(namespace.get_domainname(), "");
    }

    #[test]
    fn test_uts_namespace_with_parent() {
        let namespace = UtsNamespace::new(1).with_parent(0);
        assert_eq!(namespace.parent_id, Some(0));
    }

    #[test]
    fn test_uts_namespace_set_hostname() {
        let mut namespace = UtsNamespace::new(1);
        namespace.set_hostname("myhost".to_string()).unwrap();
        assert_eq!(namespace.get_hostname(), "myhost");
    }

    #[test]
    fn test_uts_namespace_set_hostname_empty() {
        let mut namespace = UtsNamespace::new(1);
        assert!(namespace.set_hostname("".to_string()).is_err());
    }

    #[test]
    fn test_uts_namespace_set_hostname_too_long() {
        let mut namespace = UtsNamespace::new(1);
        let long_name = "a".repeat(254);
        assert!(namespace.set_hostname(long_name).is_err());
    }

    #[test]
    fn test_uts_namespace_set_domainname() {
        let mut namespace = UtsNamespace::new(1);
        namespace.set_domainname("example.com".to_string()).unwrap();
        assert_eq!(namespace.get_domainname(), "example.com");
    }

    #[test]
    fn test_uts_namespace_get_fqdn() {
        let mut namespace = UtsNamespace::new(1);
        namespace.set_hostname("myhost".to_string()).unwrap();
        namespace.set_domainname("example.com".to_string()).unwrap();

        assert_eq!(namespace.get_fqdn(), "myhost.example.com");
    }

    #[test]
    fn test_uts_namespace_get_fqdn_no_domain() {
        let mut namespace = UtsNamespace::new(1);
        namespace.set_hostname("myhost".to_string()).unwrap();

        assert_eq!(namespace.get_fqdn(), "myhost");
    }

    #[test]
    fn test_uts_namespace_manager() {
        let manager = UtsNamespaceManager::new();

        let namespace_id = manager.create_namespace(None);
        assert_eq!(namespace_id, 1);

        manager.set_hostname(namespace_id, "myhost".to_string()).unwrap();
        let namespace = manager.get_namespace(namespace_id).unwrap();
        assert_eq!(namespace.get_hostname(), "myhost");
    }

    #[test]
    fn test_uts_namespace_manager_with_parent() {
        let manager = UtsNamespaceManager::new();

        let parent_id = manager.create_namespace(None);
        let child_id = manager.create_namespace(Some(parent_id));

        let namespace = manager.get_namespace(child_id).unwrap();
        assert_eq!(namespace.parent_id, Some(parent_id));
    }

    #[test]
    fn test_uts_namespace_manager_multiple_namespaces() {
        let manager = UtsNamespaceManager::new();

        let _ns_id1 = manager.create_namespace(None);
        let _ns_id2 = manager.create_namespace(None);

        assert_eq!(manager.namespace_count(), 2);
    }

    #[test]
    fn test_uts_namespace_manager_remove() {
        let manager = UtsNamespaceManager::new();

        let namespace_id = manager.create_namespace(None);
        manager.remove_namespace(namespace_id).unwrap();

        assert_eq!(manager.namespace_count(), 0);
    }

    #[test]
    fn test_uts_namespace_manager_invalid() {
        let manager = UtsNamespaceManager::new();
        assert!(manager.set_hostname(999, "test".to_string()).is_err());
    }
}

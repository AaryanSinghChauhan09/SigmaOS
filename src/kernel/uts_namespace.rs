#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
//! UTS Namespace Implementation
//!
//! Provides hostname and domainname isolation per namespace (CLONE_NEWUTS equivalent).
//! Enables processes to have independent UTS (hostname) information.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, atomic::{AtomicU64, Ordering}};

/// Unique identifier for a UTS namespace
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NamespaceId(u64);

impl NamespaceId {
    /// Create a new namespace ID
    pub fn new(id: u64) -> Self {
        NamespaceId(id)
    }

    /// Get the raw ID value
    pub fn raw(&self) -> u64 {
        self.0
    }
}

/// UTS Namespace - isolates hostname, domainname, and other UTS properties
#[derive(Debug, Clone)]
pub struct UtsNamespace {
    /// Unique identifier for this namespace
    id: NamespaceId,
    
    /// Hostname for this namespace (max 255 bytes)
    hostname: String,
    
    /// Domainname for this namespace (max 255 bytes)
    domainname: String,
    
    /// Nodename for this namespace
    nodename: String,
    
    /// Release version
    release: String,
    
    /// Version information
    version: String,
    
    /// Machine type (e.g., "x86_64")
    machine: String,
    
    /// Parent namespace ID (for hierarchical namespaces)
    parent_id: Option<NamespaceId>,
    
    /// Reference count for this namespace
    refcount: Arc<AtomicU64>,
}

impl UtsNamespace {
    /// Create a new UTS namespace
    pub fn new(
        id: NamespaceId,
        hostname: String,
        domainname: String,
        parent_id: Option<NamespaceId>,
    ) -> Self {
        UtsNamespace {
            id,
            hostname,
            domainname,
            nodename: "sigma-node".to_string(),
            release: "0.9.0".to_string(),
            version: "1".to_string(),
            machine: "x86_64".to_string(),
            parent_id,
            refcount: Arc::new(AtomicU64::new(1)),
        }
    }

    /// Get the namespace ID
    pub fn id(&self) -> NamespaceId {
        self.id
    }

    /// Get the hostname
    pub fn hostname(&self) -> &str {
        &self.hostname
    }

    /// Set the hostname (max 255 bytes)
    pub fn set_hostname(&mut self, hostname: String) -> Result<(), String> {
        if hostname.len() > 255 {
            return Err("Hostname too long (max 255 bytes)".to_string());
        }
        if hostname.is_empty() {
            return Err("Hostname cannot be empty".to_string());
        }
        self.hostname = hostname;
        Ok(())
    }

    /// Get the domainname
    pub fn domainname(&self) -> &str {
        &self.domainname
    }

    /// Set the domainname (max 255 bytes)
    pub fn set_domainname(&mut self, domainname: String) -> Result<(), String> {
        if domainname.len() > 255 {
            return Err("Domainname too long (max 255 bytes)".to_string());
        }
        self.domainname = domainname;
        Ok(())
    }

    /// Get the nodename
    pub fn nodename(&self) -> &str {
        &self.nodename
    }

    /// Get the release
    pub fn release(&self) -> &str {
        &self.release
    }

    /// Get the version
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Get the machine type
    pub fn machine(&self) -> &str {
        &self.machine
    }

    /// Get the parent namespace ID
    pub fn parent_id(&self) -> Option<NamespaceId> {
        self.parent_id
    }

    /// Increment reference count
    pub fn incref(&self) {
        self.refcount.fetch_add(1, Ordering::SeqCst);
    }

    /// Decrement reference count
    pub fn decref(&self) -> u64 {
        self.refcount.fetch_sub(1, Ordering::SeqCst)
    }

    /// Get current reference count
    pub fn refcount(&self) -> u64 {
        self.refcount.load(Ordering::SeqCst)
    }
}

/// UTS Namespace Manager - manages all active UTS namespaces
pub struct UtsNamespaceManager {
    /// Map of namespace ID to UTS namespace
    namespaces: Arc<Mutex<HashMap<NamespaceId, Arc<Mutex<UtsNamespace>>>>>,
    
    /// Atomic counter for generating unique namespace IDs
    id_counter: Arc<AtomicU64>,
}

impl UtsNamespaceManager {
    /// Create a new UTS namespace manager
    pub fn new() -> Self {
        UtsNamespaceManager {
            namespaces: Arc::new(Mutex::new(HashMap::new())),
            id_counter: Arc::new(AtomicU64::new(1)),
        }
    }

    /// Create a new UTS namespace
    pub fn create_namespace(
        &self,
        parent_id: Option<NamespaceId>,
    ) -> Result<NamespaceId, String> {
        // Generate new namespace ID
        let new_id = self.id_counter.fetch_add(1, Ordering::SeqCst);
        let ns_id = NamespaceId::new(new_id);

        // Create default hostname based on namespace ID
        let hostname = format!("sigma-{}", new_id);
        let domainname = "localdomain".to_string();

        // Create the namespace
        let namespace = Arc::new(Mutex::new(
            UtsNamespace::new(ns_id, hostname, domainname, parent_id)
        ));

        // Register it
        let mut namespaces = self.namespaces.lock().map_err(|e| e.to_string())?;
        namespaces.insert(ns_id, namespace);

        Ok(ns_id)
    }

    /// Get a namespace by ID
    pub fn get_namespace(&self, ns_id: NamespaceId) -> Result<Arc<Mutex<UtsNamespace>>, String> {
        let namespaces = self.namespaces.lock().map_err(|e| e.to_string())?;
        namespaces.get(&ns_id)
            .cloned()
            .ok_or_else(|| format!("Namespace {:?} not found", ns_id))
    }

    /// Set hostname for a namespace
    pub fn set_hostname(
        &self,
        ns_id: NamespaceId,
        hostname: String,
    ) -> Result<(), String> {
        let ns_arc = self.get_namespace(ns_id)?;
        let mut ns = ns_arc.lock().map_err(|e| e.to_string())?;
        ns.set_hostname(hostname)
    }

    /// Get hostname for a namespace
    pub fn get_hostname(&self, ns_id: NamespaceId) -> Result<String, String> {
        let ns_arc = self.get_namespace(ns_id)?;
        let ns = ns_arc.lock().map_err(|e| e.to_string())?;
        Ok(ns.hostname().to_string())
    }

    /// Set domainname for a namespace
    pub fn set_domainname(
        &self,
        ns_id: NamespaceId,
        domainname: String,
    ) -> Result<(), String> {
        let ns_arc = self.get_namespace(ns_id)?;
        let mut ns = ns_arc.lock().map_err(|e| e.to_string())?;
        ns.set_domainname(domainname)
    }

    /// Get domainname for a namespace
    pub fn get_domainname(&self, ns_id: NamespaceId) -> Result<String, String> {
        let ns_arc = self.get_namespace(ns_id)?;
        let ns = ns_arc.lock().map_err(|e| e.to_string())?;
        Ok(ns.domainname().to_string())
    }

    /// Delete a namespace
    pub fn delete_namespace(&self, ns_id: NamespaceId) -> Result<(), String> {
        let mut namespaces = self.namespaces.lock().map_err(|e| e.to_string())?;
        namespaces.remove(&ns_id);
        Ok(())
    }

    /// List all namespace IDs
    pub fn list_namespaces(&self) -> Result<Vec<NamespaceId>, String> {
        let namespaces = self.namespaces.lock().map_err(|e| e.to_string())?;
        Ok(namespaces.keys().copied().collect())
    }

    /// Get namespace count
    pub fn count(&self) -> Result<usize, String> {
        let namespaces = self.namespaces.lock().map_err(|e| e.to_string())?;
        Ok(namespaces.len())
    }
}

impl Default for UtsNamespaceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_namespace_creation() {
        let manager = UtsNamespaceManager::new();
        let ns_id = manager.create_namespace(None).expect("Failed to create namespace");
        assert_ne!(ns_id.raw(), 0);
    }

    #[test]
    fn test_namespace_hostname_isolation() {
        let manager = UtsNamespaceManager::new();
        let ns1 = manager.create_namespace(None).expect("Failed to create ns1");
        let ns2 = manager.create_namespace(None).expect("Failed to create ns2");

        manager.set_hostname(ns1, "host1".to_string()).expect("Failed to set hostname");
        manager.set_hostname(ns2, "host2".to_string()).expect("Failed to set hostname");

        let host1 = manager.get_hostname(ns1).expect("Failed to get hostname");
        let host2 = manager.get_hostname(ns2).expect("Failed to get hostname");

        assert_eq!(host1, "host1");
        assert_eq!(host2, "host2");
        assert_ne!(host1, host2);
    }

    #[test]
    fn test_hostname_max_length() {
        let manager = UtsNamespaceManager::new();
        let ns = manager.create_namespace(None).expect("Failed to create namespace");

        let long_hostname = "a".repeat(256);
        let result = manager.set_hostname(ns, long_hostname);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_hostname() {
        let manager = UtsNamespaceManager::new();
        let ns = manager.create_namespace(None).expect("Failed to create namespace");

        let result = manager.set_hostname(ns, "".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_namespace_listing() {
        let manager = UtsNamespaceManager::new();
        let ns1 = manager.create_namespace(None).expect("Failed to create ns1");
        let ns2 = manager.create_namespace(None).expect("Failed to create ns2");
        let ns3 = manager.create_namespace(None).expect("Failed to create ns3");

        let namespaces = manager.list_namespaces().expect("Failed to list namespaces");
        assert_eq!(namespaces.len(), 3);
        assert!(namespaces.contains(&ns1));
        assert!(namespaces.contains(&ns2));
        assert!(namespaces.contains(&ns3));
    }

    #[test]
    fn test_namespace_deletion() {
        let manager = UtsNamespaceManager::new();
        let ns = manager.create_namespace(None).expect("Failed to create namespace");
        
        manager.delete_namespace(ns).expect("Failed to delete namespace");
        
        let result = manager.get_namespace(ns);
        assert!(result.is_err());
    }

    #[test]
    fn test_domainname_isolation() {
        let manager = UtsNamespaceManager::new();
        let ns1 = manager.create_namespace(None).expect("Failed to create ns1");
        let ns2 = manager.create_namespace(None).expect("Failed to create ns2");

        manager.set_domainname(ns1, "domain1.local".to_string()).expect("Failed to set domainname");
        manager.set_domainname(ns2, "domain2.local".to_string()).expect("Failed to set domainname");

        let dom1 = manager.get_domainname(ns1).expect("Failed to get domainname");
        let dom2 = manager.get_domainname(ns2).expect("Failed to get domainname");

        assert_eq!(dom1, "domain1.local");
        assert_eq!(dom2, "domain2.local");
    }

    #[test]
    fn test_hierarchical_namespaces() {
        let manager = UtsNamespaceManager::new();
        let parent = manager.create_namespace(None).expect("Failed to create parent");
        let child = manager.create_namespace(Some(parent)).expect("Failed to create child");

        let ns_arc = manager.get_namespace(child).expect("Failed to get namespace");
        let ns = ns_arc.lock().expect("Failed to lock namespace");
        
        assert_eq!(ns.parent_id(), Some(parent));
    }

    #[test]
    fn test_namespace_count() {
        let manager = UtsNamespaceManager::new();
        manager.create_namespace(None).expect("Failed to create ns1");
        manager.create_namespace(None).expect("Failed to create ns2");
        
        let count = manager.count().expect("Failed to get count");
        assert_eq!(count, 2);
    }
}

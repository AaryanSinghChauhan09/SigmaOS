//! User Namespace Implementation
//! 
//! Provides user/group isolation per-namespace with UID/GID mapping,
//! capability sets, and thread-safe namespace management.

use std::sync::{Arc, Mutex, RwLock};
use std::collections::HashMap;
use std::fmt;

/// Unique identifier for a user namespace
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserNamespaceId(pub u64);

impl fmt::Display for UserNamespaceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UsrNs({})", self.0)
    }
}

/// Capability set with Linux capabilities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CapabilitySet {
    /// Change file ownership
    CapChown = 0,
    /// Bypass DAC permissions
    CapDacOverride = 1,
    /// Bypass DAC read search
    CapDacReadSearch = 2,
    /// Bypass file ownership checks
    CapFowner = 3,
    /// Don't clear setuid on exec
    CapFsetid = 4,
    /// Send signals to arbitrary processes
    CapKill = 5,
    /// Change group ID
    CapSetgid = 6,
    /// Change user ID
    CapSetuid = 7,
    /// Set file capability
    CapSetfcap = 8,
    /// Modify process capability
    CapSetpcap = 9,
    /// Bind to ports < 1024
    CapNetRaw = 10,
    /// Bind to ports < 1024
    CapNetBindService = 11,
    /// Use chroot
    CapSysChroot = 12,
    /// Use privileged operations (SYS_ADMIN)
    CapSysAdmin = 13,
    /// Bypass file read permission
    CapSysRawio = 14,
    /// Bypass permission checks on IPC
    CapIpcLock = 15,
    /// Load kernel modules
    CapSysModule = 16,
    /// Use ptrace
    CapSysPtrace = 17,
    /// Shutdown system
    CapSysboot = 18,
    /// Use renice/setpriority
    CapSysNice = 19,
    /// Use resource limits
    CapSysResource = 20,
    /// Set process accounting
    CapSysacct = 21,
    /// Network administration
    CapNetAdmin = 22,
    /// Monitor network
    CapNetRawMonitor = 23,
    /// Use MSG_PEEK
    CapIpcMsg = 24,
    /// Semaphore operations
    CapIpcSem = 25,
}

impl fmt::Display for CapabilitySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CapChown => write!(f, "CAP_CHOWN"),
            Self::CapDacOverride => write!(f, "CAP_DAC_OVERRIDE"),
            Self::CapDacReadSearch => write!(f, "CAP_DAC_READ_SEARCH"),
            Self::CapFowner => write!(f, "CAP_FOWNER"),
            Self::CapFsetid => write!(f, "CAP_FSETID"),
            Self::CapKill => write!(f, "CAP_KILL"),
            Self::CapSetgid => write!(f, "CAP_SETGID"),
            Self::CapSetuid => write!(f, "CAP_SETUID"),
            Self::CapSetfcap => write!(f, "CAP_SETFCAP"),
            Self::CapSetpcap => write!(f, "CAP_SETPCAP"),
            Self::CapNetRaw => write!(f, "CAP_NET_RAW"),
            Self::CapNetBindService => write!(f, "CAP_NET_BIND_SERVICE"),
            Self::CapSysChroot => write!(f, "CAP_SYS_CHROOT"),
            Self::CapSysAdmin => write!(f, "CAP_SYS_ADMIN"),
            Self::CapSysRawio => write!(f, "CAP_SYS_RAWIO"),
            Self::CapIpcLock => write!(f, "CAP_IPC_LOCK"),
            Self::CapSysModule => write!(f, "CAP_SYS_MODULE"),
            Self::CapSysPtrace => write!(f, "CAP_SYS_PTRACE"),
            Self::CapSysboot => write!(f, "CAP_SYS_BOOT"),
            Self::CapSysNice => write!(f, "CAP_SYS_NICE"),
            Self::CapSysResource => write!(f, "CAP_SYS_RESOURCE"),
            Self::CapSysacct => write!(f, "CAP_SYSACCT"),
            Self::CapNetAdmin => write!(f, "CAP_NET_ADMIN"),
            Self::CapNetRawMonitor => write!(f, "CAP_NET_RAW_MONITOR"),
            Self::CapIpcMsg => write!(f, "CAP_IPC_MSG"),
            Self::CapIpcSem => write!(f, "CAP_IPC_SEM"),
        }
    }
}

/// Maps UIDs or GIDs between host and namespace
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UidGidMapping {
    /// Starting UID/GID inside the namespace
    pub container_id: u32,
    /// Starting UID/GID on the host
    pub host_id: u32,
    /// Number of consecutive UIDs/GIDs in this range
    pub count: u32,
}

impl UidGidMapping {
    /// Create a new UID/GID mapping
    pub fn new(container_id: u32, host_id: u32, count: u32) -> Self {
        Self {
            container_id,
            host_id,
            count,
        }
    }

    /// Check if a container ID falls within this mapping
    pub fn contains_container_id(&self, container_id: u32) -> bool {
        container_id >= self.container_id && 
        container_id < self.container_id.saturating_add(self.count)
    }

    /// Check if a host ID falls within this mapping
    pub fn contains_host_id(&self, host_id: u32) -> bool {
        host_id >= self.host_id &&
        host_id < self.host_id.saturating_add(self.count)
    }

    /// Get the container ID for a given host ID
    pub fn get_container_id(&self, host_id: u32) -> Option<u32> {
        if self.contains_host_id(host_id) {
            let offset = host_id.saturating_sub(self.host_id);
            Some(self.container_id.saturating_add(offset))
        } else {
            None
        }
    }

    /// Get the host ID for a given container ID
    pub fn get_host_id(&self, container_id: u32) -> Option<u32> {
        if self.contains_container_id(container_id) {
            let offset = container_id.saturating_sub(self.container_id);
            Some(self.host_id.saturating_add(offset))
        } else {
            None
        }
    }
}

impl fmt::Display for UidGidMapping {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}",
            self.container_id, self.host_id, self.count
        )
    }
}

/// Entry from /etc/subuid file
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubuidEntry {
    /// Username
    pub user: String,
    /// Starting UID for this user
    pub start_uid: u32,
    /// Count of UIDs allocated
    pub count: u32,
}

impl SubuidEntry {
    /// Create a new subuid entry
    pub fn new(user: String, start_uid: u32, count: u32) -> Self {
        Self {
            user,
            start_uid,
            count,
        }
    }

    /// Check if this entry contains a UID
    pub fn contains_uid(&self, uid: u32) -> bool {
        uid >= self.start_uid && uid < self.start_uid.saturating_add(self.count)
    }
}

impl fmt::Display for SubuidEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.user, self.start_uid, self.count)
    }
}

/// Entry from /etc/subgid file
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubgidEntry {
    /// Username
    pub user: String,
    /// Starting GID for this user
    pub start_gid: u32,
    /// Count of GIDs allocated
    pub count: u32,
}

impl SubgidEntry {
    /// Create a new subgid entry
    pub fn new(user: String, start_gid: u32, count: u32) -> Self {
        Self {
            user,
            start_gid,
            count,
        }
    }

    /// Check if this entry contains a GID
    pub fn contains_gid(&self, gid: u32) -> bool {
        gid >= self.start_gid && gid < self.start_gid.saturating_add(self.count)
    }
}

impl fmt::Display for SubgidEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.user, self.start_gid, self.count)
    }
}

/// Tracks allocated subuid/subgid ranges per user
pub struct SubuidAllocationTracker {
    /// Allocated ranges: user -> list of (start, count)
    allocated: Arc<Mutex<HashMap<String, Vec<(u32, u32)>>>>,
}

impl SubuidAllocationTracker {
    /// Create a new allocation tracker
    pub fn new() -> Self {
        Self {
            allocated: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Check if a range conflicts with any existing allocation for a user
    fn check_conflict(&self, user: &str, start: u32, count: u32) -> Result<(), String> {
        let allocated = self
            .allocated
            .lock()
            .map_err(|e| format!("Failed to lock tracker: {}", e))?;

        if let Some(ranges) = allocated.get(user) {
            let new_end = start.saturating_add(count);
            for (existing_start, existing_count) in ranges {
                let existing_end = existing_start.saturating_add(*existing_count);
                // Check for overlap
                if (start < existing_end) && (existing_start < &new_end) {
                    return Err(format!(
                        "Range conflict for user {}: {}+{} overlaps with {}+{}",
                        user, start, count, existing_start, existing_count
                    ));
                }
            }
        }
        Ok(())
    }

    /// Allocate a range for a user
    pub fn allocate_range(
        &self,
        user: &str,
        start: u32,
        count: u32,
    ) -> Result<(u32, u32), String> {
        // Check for conflicts
        self.check_conflict(user, start, count)?;

        let mut allocated = self
            .allocated
            .lock()
            .map_err(|e| format!("Failed to lock tracker: {}", e))?;

        allocated
            .entry(user.to_string())
            .or_insert_with(Vec::new)
            .push((start, count));

        Ok((start, count))
    }

    /// Deallocate a range for a user
    pub fn deallocate_range(&self, user: &str, start: u32, count: u32) -> Result<(), String> {
        let mut allocated = self
            .allocated
            .lock()
            .map_err(|e| format!("Failed to lock tracker: {}", e))?;

        if let Some(ranges) = allocated.get_mut(user) {
            if let Some(pos) = ranges.iter().position(|&r| r == (start, count)) {
                ranges.remove(pos);
                if ranges.is_empty() {
                    allocated.remove(user);
                }
                return Ok(());
            }
        }

        Err(format!(
            "Range {}+{} not allocated for user {}",
            start, count, user
        ))
    }

    /// Get all allocated ranges for a user
    pub fn get_allocated_ranges(&self, user: &str) -> Result<Vec<(u32, u32)>, String> {
        let allocated = self
            .allocated
            .lock()
            .map_err(|e| format!("Failed to lock tracker: {}", e))?;

        Ok(allocated
            .get(user)
            .map(|ranges| ranges.clone())
            .unwrap_or_default())
    }
}

impl Default for SubuidAllocationTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse /etc/subuid file format
pub fn parse_subuid_file(content: &str) -> Result<Vec<SubuidEntry>, String> {
    let mut entries = Vec::new();

    for line in content.lines() {
        // Skip empty lines and comments
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 3 {
            return Err(format!("Invalid subuid line format: {}", line));
        }

        let user = parts[0].to_string();
        let start_uid = parts[1]
            .parse::<u32>()
            .map_err(|e| format!("Invalid start_uid in line '{}': {}", line, e))?;
        let count = parts[2]
            .parse::<u32>()
            .map_err(|e| format!("Invalid count in line '{}': {}", line, e))?;

        entries.push(SubuidEntry::new(user, start_uid, count));
    }

    Ok(entries)
}

/// Parse /etc/subgid file format
pub fn parse_subgid_file(content: &str) -> Result<Vec<SubgidEntry>, String> {
    let mut entries = Vec::new();

    for line in content.lines() {
        // Skip empty lines and comments
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 3 {
            return Err(format!("Invalid subgid line format: {}", line));
        }

        let user = parts[0].to_string();
        let start_gid = parts[1]
            .parse::<u32>()
            .map_err(|e| format!("Invalid start_gid in line '{}': {}", line, e))?;
        let count = parts[2]
            .parse::<u32>()
            .map_err(|e| format!("Invalid count in line '{}': {}", line, e))?;

        entries.push(SubgidEntry::new(user, start_gid, count));
    }

    Ok(entries)
}

/// User context within a namespace (UID, GID, supplementary groups)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserContext {
    /// Current UID in namespace
    pub uid: u32,
    /// Current GID in namespace
    pub gid: u32,
    /// Supplementary group IDs
    pub groups: Vec<u32>,
}

impl UserContext {
    /// Create new user context with UID and GID
    pub fn new(uid: u32, gid: u32) -> Self {
        Self {
            uid,
            gid,
            groups: Vec::new(),
        }
    }

    /// Add a supplementary group
    pub fn add_group(&mut self, gid: u32) -> Result<(), String> {
        if self.groups.contains(&gid) {
            return Err(format!("Group {} already in context", gid));
        }
        self.groups.push(gid);
        Ok(())
    }

    /// Remove a supplementary group
    pub fn remove_group(&mut self, gid: u32) -> Result<(), String> {
        if let Some(pos) = self.groups.iter().position(|&g| g == gid) {
            self.groups.remove(pos);
            Ok(())
        } else {
            Err(format!("Group {} not in context", gid))
        }
    }
}

/// A user namespace with isolation and capability sets
pub struct UserNamespace {
    /// Unique identifier for this namespace
    pub id: UserNamespaceId,
    /// UID mappings (namespace to host)
    pub uid_map: Vec<UidGidMapping>,
    /// GID mappings (namespace to host)
    pub gid_map: Vec<UidGidMapping>,
    /// Capabilities available in this namespace
    pub capabilities: Vec<CapabilitySet>,
    /// Owner UID on the host
    pub owner_uid: u32,
    /// Current user context
    pub user_context: UserContext,
    /// Parent namespace ID (if any)
    pub parent_id: Option<UserNamespaceId>,
}

impl UserNamespace {
    /// Create a new user namespace
    pub fn new(
        id: UserNamespaceId,
        owner_uid: u32,
        parent_id: Option<UserNamespaceId>,
    ) -> Self {
        Self {
            id,
            uid_map: Vec::new(),
            gid_map: Vec::new(),
            capabilities: Vec::new(),
            owner_uid,
            user_context: UserContext::new(0, 0),
            parent_id,
        }
    }

    /// Add a capability to this namespace
    pub fn grant_capability(&mut self, cap: CapabilitySet) -> Result<(), String> {
        if self.capabilities.contains(&cap) {
            return Err(format!("Capability {} already granted", cap));
        }
        self.capabilities.push(cap);
        Ok(())
    }

    /// Remove a capability from this namespace
    pub fn revoke_capability(&mut self, cap: CapabilitySet) -> Result<(), String> {
        if let Some(pos) = self.capabilities.iter().position(|&c| c == cap) {
            self.capabilities.remove(pos);
            Ok(())
        } else {
            Err(format!("Capability {} not granted", cap))
        }
    }

    /// Check if this namespace has a capability
    pub fn has_capability(&self, cap: CapabilitySet) -> bool {
        self.capabilities.contains(&cap)
    }

    /// Map a UID from namespace to host
    pub fn map_uid_ns_to_host(&self, ns_uid: u32) -> Result<u32, String> {
        for mapping in &self.uid_map {
            if let Some(host_uid) = mapping.get_host_id(ns_uid) {
                return Ok(host_uid);
            }
        }
        Err(format!("UID {} not mapped", ns_uid))
    }

    /// Map a UID from host to namespace
    pub fn map_uid_host_to_ns(&self, host_uid: u32) -> Result<u32, String> {
        for mapping in &self.uid_map {
            if let Some(ns_uid) = mapping.get_container_id(host_uid) {
                return Ok(ns_uid);
            }
        }
        Err(format!("UID {} not mapped", host_uid))
    }

    /// Map a GID from namespace to host
    pub fn map_gid_ns_to_host(&self, ns_gid: u32) -> Result<u32, String> {
        for mapping in &self.gid_map {
            if let Some(host_gid) = mapping.get_host_id(ns_gid) {
                return Ok(host_gid);
            }
        }
        Err(format!("GID {} not mapped", ns_gid))
    }

    /// Map a GID from host to namespace
    pub fn map_gid_host_to_ns(&self, host_gid: u32) -> Result<u32, String> {
        for mapping in &self.gid_map {
            if let Some(ns_gid) = mapping.get_container_id(host_gid) {
                return Ok(ns_gid);
            }
        }
        Err(format!("GID {} not mapped", host_gid))
    }
}

impl fmt::Debug for UserNamespace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UserNamespace")
            .field("id", &self.id)
            .field("uid_map", &self.uid_map)
            .field("gid_map", &self.gid_map)
            .field("capabilities", &self.capabilities)
            .field("owner_uid", &self.owner_uid)
            .field("user_context", &self.user_context)
            .field("parent_id", &self.parent_id)
            .finish()
    }
}

/// Manages all user namespaces in the system
pub struct UserNamespaceManager {
    namespaces: Arc<RwLock<HashMap<UserNamespaceId, Arc<Mutex<UserNamespace>>>>>,
    next_id: Arc<Mutex<u64>>,
}

impl UserNamespaceManager {
    /// Create a new namespace manager
    pub fn new() -> Self {
        Self {
            namespaces: Arc::new(RwLock::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Create a new user namespace
    pub fn create_namespace(
        &self,
        owner_uid: u32,
        parent_id: Option<UserNamespaceId>,
    ) -> Result<UserNamespaceId, String> {
        let mut id_gen = self
            .next_id
            .lock()
            .map_err(|e| format!("Failed to acquire ID lock: {}", e))?;
        let id = UserNamespaceId(*id_gen);
        *id_gen = id_gen.saturating_add(1);
        drop(id_gen);

        let namespace = UserNamespace::new(id, owner_uid, parent_id);
        let mut namespaces = self
            .namespaces
            .write()
            .map_err(|e| format!("Failed to write namespaces: {}", e))?;

        namespaces.insert(id, Arc::new(Mutex::new(namespace)));
        Ok(id)
    }

    /// Get a user namespace by ID
    pub fn get_namespace(&self, id: UserNamespaceId) -> Result<Arc<Mutex<UserNamespace>>, String> {
        let namespaces = self
            .namespaces
            .read()
            .map_err(|e| format!("Failed to read namespaces: {}", e))?;

        namespaces
            .get(&id)
            .cloned()
            .ok_or_else(|| format!("Namespace {} not found", id))
    }

    /// Delete a user namespace
    pub fn delete_namespace(&self, id: UserNamespaceId) -> Result<(), String> {
        let mut namespaces = self
            .namespaces
            .write()
            .map_err(|e| format!("Failed to write namespaces: {}", e))?;

        namespaces
            .remove(&id)
            .ok_or_else(|| format!("Namespace {} not found", id))?;

        Ok(())
    }

    /// List all namespace IDs
    pub fn list_namespaces(&self) -> Result<Vec<UserNamespaceId>, String> {
        let namespaces = self
            .namespaces
            .read()
            .map_err(|e| format!("Failed to read namespaces: {}", e))?;

        Ok(namespaces.keys().copied().collect())
    }

    /// Get the count of namespaces
    pub fn count(&self) -> Result<usize, String> {
        let namespaces = self
            .namespaces
            .read()
            .map_err(|e| format!("Failed to read namespaces: {}", e))?;

        Ok(namespaces.len())
    }
}

impl Default for UserNamespaceManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Validate a UID/GID mapping
pub fn validate_mapping(mapping: &UidGidMapping) -> Result<(), String> {
    // Check for invalid range (count = 0)
    if mapping.count == 0 {
        return Err("Mapping count must be > 0".to_string());
    }

    // Check for overflow in container range
    if mapping
        .container_id
        .checked_add(mapping.count - 1)
        .is_none()
    {
        return Err(format!(
            "Container range overflow: {}+{}",
            mapping.container_id, mapping.count
        ));
    }

    // Check for overflow in host range
    if mapping.host_id.checked_add(mapping.count - 1).is_none() {
        return Err(format!(
            "Host range overflow: {}+{}",
            mapping.host_id, mapping.count
        ));
    }

    Ok(())
}

/// Check if two mappings overlap in the container ID space
pub fn mappings_overlap(a: &UidGidMapping, b: &UidGidMapping) -> bool {
    let a_start = a.container_id;
    let a_end = a.container_id.saturating_add(a.count);
    let b_start = b.container_id;
    let b_end = b.container_id.saturating_add(b.count);

    // Check if ranges overlap
    (a_start < b_end) && (b_start < a_end)
}

impl UserNamespace {
    /// Set UID mappings for this namespace
    pub fn set_uid_map(&mut self, mappings: Vec<UidGidMapping>) -> Result<(), String> {
        // Validate each mapping
        for mapping in &mappings {
            validate_mapping(mapping)?;
        }

        // Check for overlaps
        for i in 0..mappings.len() {
            for j in (i + 1)..mappings.len() {
                if mappings_overlap(&mappings[i], &mappings[j]) {
                    return Err(format!(
                        "Overlapping UID mappings: {} and {}",
                        mappings[i], mappings[j]
                    ));
                }
            }
        }

        // Check for duplicates
        let mut seen = std::collections::HashSet::new();
        for mapping in &mappings {
            let key = (mapping.container_id, mapping.host_id);
            if !seen.insert(key) {
                return Err(format!("Duplicate UID mapping: {}", mapping));
            }
        }

        self.uid_map = mappings;
        Ok(())
    }

    /// Set GID mappings for this namespace
    pub fn set_gid_map(&mut self, mappings: Vec<UidGidMapping>) -> Result<(), String> {
        // Validate each mapping
        for mapping in &mappings {
            validate_mapping(mapping)?;
        }

        // Check for overlaps
        for i in 0..mappings.len() {
            for j in (i + 1)..mappings.len() {
                if mappings_overlap(&mappings[i], &mappings[j]) {
                    return Err(format!(
                        "Overlapping GID mappings: {} and {}",
                        mappings[i], mappings[j]
                    ));
                }
            }
        }

        // Check for duplicates
        let mut seen = std::collections::HashSet::new();
        for mapping in &mappings {
            let key = (mapping.container_id, mapping.host_id);
            if !seen.insert(key) {
                return Err(format!("Duplicate GID mapping: {}", mapping));
            }
        }

        self.gid_map = mappings;
        Ok(())
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user_namespace() {
        let manager = UserNamespaceManager::new();
        let result = manager.create_namespace(1000, None);
        assert!(result.is_ok());
        let ns_id = result.unwrap();
        assert_eq!(ns_id.0, 1);
    }

    #[test]
    fn test_get_user_namespace() {
        let manager = UserNamespaceManager::new();
        let ns_id = manager.create_namespace(1000, None).unwrap();
        let ns = manager.get_namespace(ns_id).unwrap();
        let locked = ns.lock().unwrap();
        assert_eq!(locked.owner_uid, 1000);
        assert_eq!(locked.id, ns_id);
    }

    #[test]
    fn test_delete_user_namespace() {
        let manager = UserNamespaceManager::new();
        let ns_id = manager.create_namespace(1000, None).unwrap();
        assert!(manager.delete_namespace(ns_id).is_ok());
        assert!(manager.get_namespace(ns_id).is_err());
    }

    #[test]
    fn test_list_user_namespaces() {
        let manager = UserNamespaceManager::new();
        let ns_id1 = manager.create_namespace(1000, None).unwrap();
        let ns_id2 = manager.create_namespace(2000, None).unwrap();
        let ns_id3 = manager.create_namespace(3000, None).unwrap();

        let list = manager.list_namespaces().unwrap();
        assert_eq!(list.len(), 3);
        assert!(list.contains(&ns_id1));
        assert!(list.contains(&ns_id2));
        assert!(list.contains(&ns_id3));
    }

    #[test]
    fn test_capability_set_operations() {
        let manager = UserNamespaceManager::new();
        let ns_id = manager.create_namespace(1000, None).unwrap();
        let ns = manager.get_namespace(ns_id).unwrap();

        {
            let mut locked = ns.lock().unwrap();
            assert!(!locked.has_capability(CapabilitySet::CapChown));
            
            let result = locked.grant_capability(CapabilitySet::CapChown);
            assert!(result.is_ok());
            assert!(locked.has_capability(CapabilitySet::CapChown));

            let result = locked.revoke_capability(CapabilitySet::CapChown);
            assert!(result.is_ok());
            assert!(!locked.has_capability(CapabilitySet::CapChown));
        }
    }

    #[test]
    fn test_uid_gid_mapping_contains() {
        let mapping = UidGidMapping::new(0, 100000, 65536);
        
        assert!(mapping.contains_container_id(0));
        assert!(mapping.contains_container_id(32768));
        assert!(mapping.contains_container_id(65535));
        assert!(!mapping.contains_container_id(65536));
        
        assert!(mapping.contains_host_id(100000));
        assert!(mapping.contains_host_id(132768));
        assert!(mapping.contains_host_id(165535));
        assert!(!mapping.contains_host_id(165536));
    }

    #[test]
    fn test_namespace_count() {
        let manager = UserNamespaceManager::new();
        assert_eq!(manager.count().unwrap(), 0);
        
        manager.create_namespace(1000, None).unwrap();
        assert_eq!(manager.count().unwrap(), 1);
        
        manager.create_namespace(2000, None).unwrap();
        assert_eq!(manager.count().unwrap(), 2);
    }

    // Task 9.3.2 Tests - UID/GID Mapping
    #[test]
    fn test_set_uid_map() {
        let manager = UserNamespaceManager::new();
        let ns_id = manager.create_namespace(1000, None).unwrap();
        let ns = manager.get_namespace(ns_id).unwrap();

        let mapping = UidGidMapping::new(0, 100000, 65536);
        {
            let mut locked = ns.lock().unwrap();
            let result = locked.set_uid_map(vec![mapping]);
            assert!(result.is_ok());
            assert_eq!(locked.uid_map.len(), 1);
        }
    }

    #[test]
    fn test_set_gid_map() {
        let manager = UserNamespaceManager::new();
        let ns_id = manager.create_namespace(1000, None).unwrap();
        let ns = manager.get_namespace(ns_id).unwrap();

        let mapping = UidGidMapping::new(0, 100000, 65536);
        {
            let mut locked = ns.lock().unwrap();
            let result = locked.set_gid_map(vec![mapping]);
            assert!(result.is_ok());
            assert_eq!(locked.gid_map.len(), 1);
        }
    }

    #[test]
    fn test_map_uid_host_to_ns_basic() {
        let manager = UserNamespaceManager::new();
        let ns_id = manager.create_namespace(1000, None).unwrap();
        let ns = manager.get_namespace(ns_id).unwrap();

        let mapping = UidGidMapping::new(0, 100000, 65536);
        {
            let mut locked = ns.lock().unwrap();
            locked.set_uid_map(vec![mapping]).unwrap();

            // Test mapping: host UID 100000 -> NS UID 0
            let result = locked.map_uid_host_to_ns(100000);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 0);

            // Test mapping: host UID 100500 -> NS UID 500
            let result = locked.map_uid_host_to_ns(100500);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 500);
        }
    }

    #[test]
    fn test_map_uid_ns_to_host_basic() {
        let manager = UserNamespaceManager::new();
        let ns_id = manager.create_namespace(1000, None).unwrap();
        let ns = manager.get_namespace(ns_id).unwrap();

        let mapping = UidGidMapping::new(0, 100000, 65536);
        {
            let mut locked = ns.lock().unwrap();
            locked.set_uid_map(vec![mapping]).unwrap();

            // Test mapping: NS UID 0 -> host UID 100000
            let result = locked.map_uid_ns_to_host(0);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 100000);

            // Test mapping: NS UID 500 -> host UID 100500
            let result = locked.map_uid_ns_to_host(500);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 100500);
        }
    }

    #[test]
    fn test_map_gid_host_to_ns_basic() {
        let manager = UserNamespaceManager::new();
        let ns_id = manager.create_namespace(1000, None).unwrap();
        let ns = manager.get_namespace(ns_id).unwrap();

        let mapping = UidGidMapping::new(0, 100000, 65536);
        {
            let mut locked = ns.lock().unwrap();
            locked.set_gid_map(vec![mapping]).unwrap();

            // Test mapping: host GID 100000 -> NS GID 0
            let result = locked.map_gid_host_to_ns(100000);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 0);

            // Test mapping: host GID 100500 -> NS GID 500
            let result = locked.map_gid_host_to_ns(100500);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 500);
        }
    }

    #[test]
    fn test_map_gid_ns_to_host_basic() {
        let manager = UserNamespaceManager::new();
        let ns_id = manager.create_namespace(1000, None).unwrap();
        let ns = manager.get_namespace(ns_id).unwrap();

        let mapping = UidGidMapping::new(0, 100000, 65536);
        {
            let mut locked = ns.lock().unwrap();
            locked.set_gid_map(vec![mapping]).unwrap();

            // Test mapping: NS GID 0 -> host GID 100000
            let result = locked.map_gid_ns_to_host(0);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 100000);

            // Test mapping: NS GID 500 -> host GID 100500
            let result = locked.map_gid_ns_to_host(500);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 100500);
        }
    }

    #[test]
    fn test_mapping_validation() {
        // Valid mapping
        let mapping = UidGidMapping::new(0, 100000, 65536);
        assert!(validate_mapping(&mapping).is_ok());

        // Invalid: count = 0
        let mapping = UidGidMapping::new(0, 100000, 0);
        assert!(validate_mapping(&mapping).is_err());

        // Invalid: overflow in container range
        let mapping = UidGidMapping::new(u32::MAX - 100, 100000, 200);
        assert!(validate_mapping(&mapping).is_err());

        // Invalid: overflow in host range
        let mapping = UidGidMapping::new(0, u32::MAX - 100, 200);
        assert!(validate_mapping(&mapping).is_err());
    }

    #[test]
    fn test_overlapping_ranges_rejected() {
        let manager = UserNamespaceManager::new();
        let ns_id = manager.create_namespace(1000, None).unwrap();
        let ns = manager.get_namespace(ns_id).unwrap();

        let mapping1 = UidGidMapping::new(0, 100000, 100);
        let mapping2 = UidGidMapping::new(50, 100050, 100); // Overlaps with mapping1

        {
            let mut locked = ns.lock().unwrap();
            let result = locked.set_uid_map(vec![mapping1, mapping2]);
            assert!(result.is_err());
            assert!(result.unwrap_err().contains("Overlapping"));
        }
    }

    #[test]
    fn test_invalid_range_rejected() {
        let manager = UserNamespaceManager::new();
        let ns_id = manager.create_namespace(1000, None).unwrap();
        let ns = manager.get_namespace(ns_id).unwrap();

        let invalid_mapping = UidGidMapping::new(0, 100000, 0); // count = 0

        {
            let mut locked = ns.lock().unwrap();
            let result = locked.set_uid_map(vec![invalid_mapping]);
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_multiple_mappings_coexist() {
        let manager = UserNamespaceManager::new();
        let ns_id = manager.create_namespace(1000, None).unwrap();
        let ns = manager.get_namespace(ns_id).unwrap();

        let mapping1 = UidGidMapping::new(0, 100000, 100);
        let mapping2 = UidGidMapping::new(100, 200000, 100);
        let mapping3 = UidGidMapping::new(200, 300000, 100);

        {
            let mut locked = ns.lock().unwrap();
            let result = locked.set_uid_map(vec![mapping1, mapping2, mapping3]);
            assert!(result.is_ok());
            assert_eq!(locked.uid_map.len(), 3);

            // Verify all three mappings work
            assert_eq!(locked.map_uid_ns_to_host(0).unwrap(), 100000);
            assert_eq!(locked.map_uid_ns_to_host(100).unwrap(), 200000);
            assert_eq!(locked.map_uid_ns_to_host(200).unwrap(), 300000);
        }
    }

    // Task 9.3.3 Tests - subuid/subgid Support
    #[test]
    fn test_parse_subuid_file_valid() {
        let content = "user1:100000:65536\nuser2:200000:32768\n";
        let result = parse_subuid_file(content);
        assert!(result.is_ok());
        let entries = result.unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].user, "user1");
        assert_eq!(entries[0].start_uid, 100000);
        assert_eq!(entries[0].count, 65536);
        assert_eq!(entries[1].user, "user2");
        assert_eq!(entries[1].start_uid, 200000);
        assert_eq!(entries[1].count, 32768);
    }

    #[test]
    fn test_parse_subgid_file_valid() {
        let content = "user1:100000:65536\nuser2:200000:32768\n";
        let result = parse_subgid_file(content);
        assert!(result.is_ok());
        let entries = result.unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].user, "user1");
        assert_eq!(entries[0].start_gid, 100000);
        assert_eq!(entries[0].count, 65536);
        assert_eq!(entries[1].user, "user2");
        assert_eq!(entries[1].start_gid, 200000);
        assert_eq!(entries[1].count, 32768);
    }

    #[test]
    fn test_parse_subuid_file_invalid_format() {
        let content = "user1:100000:invalid";
        let result = parse_subuid_file(content);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid"));
    }

    #[test]
    fn test_parse_subgid_file_invalid_format() {
        let content = "user1:100000:invalid";
        let result = parse_subgid_file(content);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid"));
    }

    #[test]
    fn test_allocate_subuid_range() {
        let tracker = SubuidAllocationTracker::new();
        let result = tracker.allocate_range("user1", 100000, 65536);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), (100000, 65536));

        let allocated = tracker.get_allocated_ranges("user1").unwrap();
        assert_eq!(allocated.len(), 1);
        assert_eq!(allocated[0], (100000, 65536));
    }

    #[test]
    fn test_allocate_subgid_range() {
        let tracker = SubuidAllocationTracker::new();
        let result = tracker.allocate_range("user1", 100000, 65536);
        assert!(result.is_ok());

        let allocated = tracker.get_allocated_ranges("user1").unwrap();
        assert_eq!(allocated.len(), 1);
    }

    #[test]
    fn test_conflict_detection() {
        let tracker = SubuidAllocationTracker::new();
        tracker.allocate_range("user1", 100000, 100).unwrap();

        // Try to allocate overlapping range
        let result = tracker.allocate_range("user1", 100050, 100);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("conflict"));
    }

    #[test]
    fn test_multiple_users_subuid() {
        let tracker = SubuidAllocationTracker::new();
        tracker.allocate_range("user1", 100000, 65536).unwrap();
        tracker.allocate_range("user2", 200000, 65536).unwrap();

        let allocated1 = tracker.get_allocated_ranges("user1").unwrap();
        let allocated2 = tracker.get_allocated_ranges("user2").unwrap();

        assert_eq!(allocated1.len(), 1);
        assert_eq!(allocated2.len(), 1);
        assert_eq!(allocated1[0], (100000, 65536));
        assert_eq!(allocated2[0], (200000, 65536));
    }

    #[test]
    fn test_subuid_file_parsing_edge_cases() {
        // Empty file
        let result = parse_subuid_file("");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);

        // File with comments and empty lines
        let content = "# Comment\n\nuser1:100000:65536\n# Another comment\n";
        let result = parse_subuid_file(content);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);

        // Missing field
        let content = "user1:100000";
        let result = parse_subuid_file(content);
        assert!(result.is_err());
    }

    #[test]
    fn test_allocation_prevents_overlaps() {
        let tracker = SubuidAllocationTracker::new();
        
        // Allocate range 1
        tracker.allocate_range("user1", 100000, 100).unwrap();
        
        // Try non-overlapping (should succeed)
        let result = tracker.allocate_range("user1", 100100, 100);
        assert!(result.is_ok());
        
        // Try overlapping (should fail)
        let result = tracker.allocate_range("user1", 100050, 100);
        assert!(result.is_err());
    }

    #[test]
    fn test_subuid_entry_contains() {
        let entry = SubuidEntry::new("user1".to_string(), 100000, 65536);
        
        assert!(entry.contains_uid(100000));
        assert!(entry.contains_uid(100500));
        assert!(entry.contains_uid(165535));
        assert!(!entry.contains_uid(165536));
        assert!(!entry.contains_uid(99999));
    }

    #[test]
    fn test_subgid_entry_contains() {
        let entry = SubgidEntry::new("user1".to_string(), 100000, 65536);
        
        assert!(entry.contains_gid(100000));
        assert!(entry.contains_gid(100500));
        assert!(entry.contains_gid(165535));
        assert!(!entry.contains_gid(165536));
        assert!(!entry.contains_gid(99999));
    }
}

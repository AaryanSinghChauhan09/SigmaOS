// Linux-inspired key management (keyrings)
// Secure key storage and management for SigmaOS

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

/// Key type (Linux keyctl.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyType {
    User,
    Session,
    Process,
    Thread,
    RequestKey,
}

/// Key permissions (Linux keyctl.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyPermissions {
    pub view: bool,
    pub read: bool,
    pub write: bool,
    pub search: bool,
    pub link: bool,
    pub setattr: bool,
}

impl KeyPermissions {
    pub fn new() -> Self {
        KeyPermissions {
            view: false,
            read: false,
            write: false,
            search: false,
            link: false,
            setattr: false,
        }
    }

    pub fn with_all() -> Self {
        KeyPermissions {
            view: true,
            read: true,
            write: true,
            search: true,
            link: true,
            setattr: true,
        }
    }
}

impl Default for KeyPermissions {
    fn default() -> Self {
        Self::new()
    }
}

/// Key payload
#[derive(Debug, Clone)]
pub enum KeyPayload {
    String(String),
    Binary(Vec<u8>),
}

/// Key entry
#[derive(Debug, Clone)]
pub struct Key {
    id: u64,
    key_type: KeyType,
    description: String,
    payload: KeyPayload,
    permissions: KeyPermissions,
    uid: u32,
    gid: u32,
}

impl Key {
    pub fn new(id: u64, key_type: KeyType, description: String, payload: KeyPayload, permissions: KeyPermissions, uid: u32, gid: u32) -> Self {
        Key {
            id,
            key_type,
            description,
            payload,
            permissions,
            uid,
            gid,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn key_type(&self) -> KeyType {
        self.key_type
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn payload(&self) -> &KeyPayload {
        &self.payload
    }

    pub fn permissions(&self) -> KeyPermissions {
        self.permissions
    }

    pub fn uid(&self) -> u32 {
        self.uid
    }

    pub fn gid(&self) -> u32 {
        self.gid
    }
}

/// Keyring (collection of keys)
#[derive(Debug, Clone)]
pub struct Keyring {
    id: u64,
    name: String,
    keys: BTreeMap<u64, Arc<Key>>,
    parent_keyring_id: Option<u64>,
}

impl Keyring {
    pub fn new(id: u64, name: String, parent_keyring_id: Option<u64>) -> Self {
        Keyring {
            id,
            name,
            keys: BTreeMap::new(),
            parent_keyring_id,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn add_key(&mut self, key: Arc<Key>) {
        self.keys.insert(key.id(), key);
    }

    pub fn remove_key(&mut self, key_id: u64) -> Option<Arc<Key>> {
        self.keys.remove(&key_id)
    }

    pub fn get_key(&self, key_id: u64) -> Option<Arc<Key>> {
        self.keys.get(&key_id).cloned()
    }

    pub fn search_key(&self, description: &str) -> Option<Arc<Key>> {
        for key in self.keys.values() {
            if key.description() == description {
                return Some(key.clone());
            }
        }
        None
    }

    pub fn key_count(&self) -> usize {
        self.keys.len()
    }

    pub fn parent_keyring_id(&self) -> Option<u64> {
        self.parent_keyring_id
    }
}

/// Key management manager
pub struct KeyManager {
    keyrings: BTreeMap<u64, Arc<Mutex<Keyring>>>,
    next_keyring_id: u64,
    next_key_id: u64,
}

impl KeyManager {
    pub fn new() -> Self {
        KeyManager {
            keyrings: BTreeMap::new(),
            next_keyring_id: 1,
            next_key_id: 1,
        }
    }

    /// Create a new keyring
    pub fn create_keyring(&mut self, name: String, parent_keyring_id: Option<u64>) -> u64 {
        let id = self.next_keyring_id;
        self.next_keyring_id += 1;

        let keyring = Arc::new(Mutex::new(Keyring::new(id, name, parent_keyring_id)));
        self.keyrings.insert(id, keyring);

        id
    }

    /// Get keyring
    pub fn get_keyring(&self, keyring_id: u64) -> Option<Arc<Mutex<Keyring>>> {
        self.keyrings.get(&keyring_id).cloned()
    }

    /// Add key to keyring
    pub fn add_key(&mut self, keyring_id: u64, key_type: KeyType, description: String, payload: KeyPayload, permissions: KeyPermissions, uid: u32, gid: u32) -> Result<u64, String> {
        let keyring = self.keyrings.get(&keyring_id)
            .ok_or_else(|| format!("Keyring not found: {}", keyring_id))?;

        let key_id = self.next_key_id;
        self.next_key_id += 1;

        let key = Arc::new(Key::new(key_id, key_type, description, payload, permissions, uid, gid));

        let mut keyring_guard = keyring.lock().unwrap();
        keyring_guard.add_key(key);

        Ok(key_id)
    }

    /// Remove key from keyring
    pub fn remove_key(&self, keyring_id: u64, key_id: u64) -> Result<(), String> {
        let keyring = self.keyrings.get(&keyring_id)
            .ok_or_else(|| format!("Keyring not found: {}", keyring_id))?;

        let mut keyring_guard = keyring.lock().unwrap();
        keyring_guard.remove_key(key_id)
            .ok_or_else(|| format!("Key not found: {}", key_id))?;

        Ok(())
    }

    /// Get key from keyring
    pub fn get_key(&self, keyring_id: u64, key_id: u64) -> Result<Arc<Key>, String> {
        let keyring = self.keyrings.get(&keyring_id)
            .ok_or_else(|| format!("Keyring not found: {}", keyring_id))?;

        let keyring_guard = keyring.lock().unwrap();
        keyring_guard.get_key(key_id)
            .ok_or_else(|| format!("Key not found: {}", key_id))
    }

    /// Search key by description
    pub fn search_key(&self, keyring_id: u64, description: &str) -> Result<Arc<Key>, String> {
        let keyring = self.keyrings.get(&keyring_id)
            .ok_or_else(|| format!("Keyring not found: {}", keyring_id))?;

        let keyring_guard = keyring.lock().unwrap();
        keyring_guard.search_key(description)
            .ok_or_else(|| format!("Key not found with description: {}", description))
    }

    /// Remove keyring
    pub fn remove_keyring(&mut self, keyring_id: u64) -> Result<(), String> {
        self.keyrings.remove(&keyring_id)
            .ok_or_else(|| format!("Keyring not found: {}", keyring_id))?;
        Ok(())
    }

    /// Get keyring count
    pub fn keyring_count(&self) -> usize {
        self.keyrings.len()
    }
}

impl Default for KeyManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_permissions_creation() {
        let perms = KeyPermissions::new();
        assert!(!perms.view);
        assert!(!perms.read);
    }

    #[test]
    fn test_key_permissions_all() {
        let perms = KeyPermissions::with_all();
        assert!(perms.view);
        assert!(perms.read);
        assert!(perms.write);
    }

    #[test]
    fn test_key_creation() {
        let key = Key::new(
            1,
            KeyType::User,
            "test_key".to_string(),
            KeyPayload::String("value".to_string()),
            KeyPermissions::with_all(),
            0,
            0
        );

        assert_eq!(key.id(), 1);
        assert_eq!(key.description(), "test_key");
    }

    #[test]
    fn test_keyring_creation() {
        let keyring = Keyring::new(1, "test_ring".to_string(), None);
        assert_eq!(keyring.id(), 1);
        assert_eq!(keyring.name(), "test_ring");
        assert_eq!(keyring.key_count(), 0);
    }

    #[test]
    fn test_keyring_add_key() {
        let mut keyring = Keyring::new(1, "test_ring".to_string(), None);
        let key = Arc::new(Key::new(
            1,
            KeyType::User,
            "test_key".to_string(),
            KeyPayload::String("value".to_string()),
            KeyPermissions::with_all(),
            0,
            0
        ));

        keyring.add_key(key);
        assert_eq!(keyring.key_count(), 1);
    }

    #[test]
    fn test_keyring_remove_key() {
        let mut keyring = Keyring::new(1, "test_ring".to_string(), None);
        let key = Arc::new(Key::new(
            1,
            KeyType::User,
            "test_key".to_string(),
            KeyPayload::String("value".to_string()),
            KeyPermissions::with_all(),
            0,
            0
        ));

        keyring.add_key(key.clone());
        let removed = keyring.remove_key(1);

        assert!(removed.is_some());
        assert_eq!(keyring.key_count(), 0);
    }

    #[test]
    fn test_keyring_search_key() {
        let mut keyring = Keyring::new(1, "test_ring".to_string(), None);
        let key = Arc::new(Key::new(
            1,
            KeyType::User,
            "test_key".to_string(),
            KeyPayload::String("value".to_string()),
            KeyPermissions::with_all(),
            0,
            0
        ));

        keyring.add_key(key);
        let found = keyring.search_key("test_key");

        assert!(found.is_some());
    }

    #[test]
    fn test_key_manager_creation() {
        let manager = KeyManager::new();
        assert_eq!(manager.keyring_count(), 0);
    }

    #[test]
    fn test_key_manager_create_keyring() {
        let mut manager = KeyManager::new();
        let id = manager.create_keyring("test_ring".to_string(), None);

        assert_eq!(id, 1);
        assert_eq!(manager.keyring_count(), 1);
    }

    #[test]
    fn test_key_manager_add_key() {
        let mut manager = KeyManager::new();
        let keyring_id = manager.create_keyring("test_ring".to_string(), None);

        let key_id = manager.add_key(
            keyring_id,
            KeyType::User,
            "test_key".to_string(),
            KeyPayload::String("value".to_string()),
            KeyPermissions::with_all(),
            0,
            0
        ).unwrap();

        assert_eq!(key_id, 1);
    }

    #[test]
    fn test_key_manager_get_key() {
        let mut manager = KeyManager::new();
        let keyring_id = manager.create_keyring("test_ring".to_string(), None);

        let key_id = manager.add_key(
            keyring_id,
            KeyType::User,
            "test_key".to_string(),
            KeyPayload::String("value".to_string()),
            KeyPermissions::with_all(),
            0,
            0
        ).unwrap();

        let key = manager.get_key(keyring_id, key_id).unwrap();
        assert_eq!(key.id(), key_id);
    }

    #[test]
    fn test_key_manager_search_key() {
        let mut manager = KeyManager::new();
        let keyring_id = manager.create_keyring("test_ring".to_string(), None);

        manager.add_key(
            keyring_id,
            KeyType::User,
            "test_key".to_string(),
            KeyPayload::String("value".to_string()),
            KeyPermissions::with_all(),
            0,
            0
        ).unwrap();

        let key = manager.search_key(keyring_id, "test_key").unwrap();
        assert_eq!(key.description(), "test_key");
    }

    #[test]
    fn test_key_manager_remove_key() {
        let mut manager = KeyManager::new();
        let keyring_id = manager.create_keyring("test_ring".to_string(), None);

        let key_id = manager.add_key(
            keyring_id,
            KeyType::User,
            "test_key".to_string(),
            KeyPayload::String("value".to_string()),
            KeyPermissions::with_all(),
            0,
            0
        ).unwrap();

        assert!(manager.remove_key(keyring_id, key_id).is_ok());
    }

    #[test]
    fn test_key_manager_remove_keyring() {
        let mut manager = KeyManager::new();
        let keyring_id = manager.create_keyring("test_ring".to_string(), None);

        assert!(manager.remove_keyring(keyring_id).is_ok());
        assert_eq!(manager.keyring_count(), 0);
    }
}

// Key Management for Crypto Keys
// Inspired by Linux keyrings for cryptographic key management

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

/// Key type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyType {
    User,
    Session,
    Process,
    Thread,
    RequestKey,
}

/// Key description
#[derive(Debug, Clone)]
pub struct KeyDescription {
    pub type_id: String,
    pub description: String,
    pub payload: Vec<u8>,
}

/// Key
#[derive(Debug, Clone)]
pub struct Key {
    pub serial: u64,
    pub key_type: KeyType,
    pub uid: u32,
    pub gid: u32,
    pub permissions: u32,
    pub description: KeyDescription,
    pub expiry: Option<u64>,
}

/// Keyring
pub struct Keyring {
    pub name: String,
    pub parent: Option<u64>,
    pub keys: HashMap<u64, Key>,
    next_serial: AtomicU64,
}

impl Keyring {
    pub fn new(name: String, parent: Option<u64>) -> Self {
        Self {
            name,
            parent,
            keys: HashMap::new(),
            next_serial: AtomicU64::new(1),
        }
    }

    /// Add a key to the keyring
    pub fn add_key(
        &mut self,
        key_type: KeyType,
        uid: u32,
        gid: u32,
        permissions: u32,
        description: KeyDescription,
        expiry: Option<u64>,
    ) -> u64 {
        let serial = self.next_serial.fetch_add(1, Ordering::SeqCst);

        let key = Key {
            serial,
            key_type,
            uid,
            gid,
            permissions,
            description,
            expiry,
        };

        self.keys.insert(serial, key);
        serial
    }

    /// Remove a key from the keyring
    pub fn remove_key(&mut self, serial: u64) -> Option<Key> {
        self.keys.remove(&serial)
    }

    /// Get a key by serial
    pub fn get_key(&self, serial: u64) -> Option<&Key> {
        self.keys.get(&serial)
    }

    /// Get mutable key by serial
    pub fn get_key_mut(&mut self, serial: u64) -> Option<&mut Key> {
        self.keys.get_mut(&serial)
    }

    /// Search keys by type
    pub fn search_by_type(&self, key_type: KeyType) -> Vec<&Key> {
        self.keys
            .values()
            .filter(|k| k.key_type == key_type)
            .collect()
    }

    /// Search keys by UID
    pub fn search_by_uid(&self, uid: u32) -> Vec<&Key> {
        self.keys.values().filter(|k| k.uid == uid).collect()
    }

    /// Check key expiry
    pub fn check_expiry(&self, serial: u64) -> bool {
        if let Some(key) = self.get_key(serial) {
            if let Some(expiry) = key.expiry {
                let current = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs() as u64;
                return current < expiry;
            }
            true
        }
        false
    }

    /// Get key count
    pub fn key_count(&self) -> usize {
        self.keys.len()
    }

    /// Get parent
    pub fn parent(&self) -> Option<u64> {
        self.parent
    }
}

/// Key management manager
pub struct KeyManager {
    keyrings: HashMap<u64, Keyring>,
    next_keyring_id: AtomicU64,
    root_keyring: u64,
}

impl KeyManager {
    pub fn new() -> Self {
        let mut manager = Self {
            keyrings: HashMap::new(),
            next_keyring_id: AtomicU64::new(1),
            root_keyring: 1,
        };

        // Create root keyring
        let root = Keyring::new("root".to_string(), None);
        manager.keyrings.insert(1, root);

        manager
    }

    /// Create a keyring
    pub fn create_keyring(&mut self, name: String, parent_id: Option<u64>) -> u64 {
        let id = self.next_keyring_id.fetch_add(1, Ordering::SeqCst);

        let keyring = Keyring::new(name, parent_id);
        self.keyrings.insert(id, keyring);

        id
    }

    /// Delete a keyring
    pub fn delete_keyring(&mut self, id: u64) -> Result<(), &'static str> {
        if id == self.root_keyring {
            return Err("Cannot delete root keyring");
        }

        if self.keyrings.remove(&id).is_some() {
            Ok(())
        } else {
            Err("Keyring not found")
        }
    }

    /// Get a keyring
    pub fn get_keyring(&self, id: u64) -> Option<&Keyring> {
        self.keyrings.get(&id)
    }

    /// Get mutable keyring
    pub fn get_keyring_mut(&mut self, id: u64) -> Option<&mut Keyring> {
        self.keyrings.get_mut(&id)
    }

    /// Add a key to a keyring
    pub fn add_key(
        &mut self,
        keyring_id: u64,
        key_type: KeyType,
        uid: u32,
        gid: u32,
        permissions: u32,
        description: KeyDescription,
        expiry: Option<u64>,
    ) -> Result<u64, &'static str> {
        let keyring = self
            .keyrings
            .get_mut(&keyring_id)
            .ok_or("Keyring not found")?;

        let serial = keyring.add_key(key_type, uid, gid, permissions, description, expiry);
        Ok(serial)
    }

    /// Remove a key from a keyring
    pub fn remove_key(&mut self, keyring_id: u64, serial: u64) -> Result<(), &'static str> {
        let keyring = self
            .keyrings
            .get_mut(&keyring_id)
            .ok_or("Keyring not found")?;

        if keyring.remove_key(serial).is_some() {
            Ok(())
        } else {
            Err("Key not found")
        }
    }

    /// Get root keyring
    pub fn root_keyring(&self) -> u64 {
        self.root_keyring
    }

    /// Get keyring count
    pub fn keyring_count(&self) -> usize {
        self.keyrings.len()
    }

    /// List all keyrings
    pub fn list_keyrings(&self) -> Vec<&Keyring> {
        self.keyrings.values().collect()
    }

    /// Create a session keyring for a process
    pub fn create_session_keyring(&mut self, process_id: u64) -> u64 {
        let name = format!("session_{}", process_id);
        self.create_keyring(name, Some(self.root_keyring))
    }

    /// Create a user keyring
    pub fn create_user_keyring(&mut self, uid: u32) -> u64 {
        let name = format!("user_{}", uid);
        self.create_keyring(name, Some(self.root_keyring))
    }

    /// Add a trusted key (e.g., for encryption)
    pub fn add_trusted_key(
        &mut self,
        keyring_id: u64,
        key_id: &str,
        key_data: Vec<u8>,
    ) -> Result<u64, &'static str> {
        let description = KeyDescription {
            type_id: "trusted".to_string(),
            description: format!("Trusted key: {}", key_id),
            payload: key_data,
        };

        self.add_key(keyring_id, KeyType::User, 0, 0, 0o600, description, None)
    }

    /// Add a session key
    pub fn add_session_key(
        &mut self,
        keyring_id: u64,
        key_data: Vec<u8>,
        expiry: u64,
    ) -> Result<u64, &'static str> {
        let description = KeyDescription {
            type_id: "session".to_string(),
            description: "Session key".to_string(),
            payload: key_data,
        };

        self.add_key(
            keyring_id,
            KeyType::Session,
            0,
            0,
            0o600,
            description,
            Some(expiry),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_keyring() {
        let mut manager = KeyManager::new();

        let id = manager.create_keyring("test".to_string(), Some(1));
        assert_eq!(id, 2);
        assert_eq!(manager.keyring_count(), 2);
    }

    #[test]
    fn test_delete_keyring() {
        let mut manager = KeyManager::new();

        let id = manager.create_keyring("test".to_string(), Some(1));
        assert!(manager.delete_keyring(id).is_ok());
        assert_eq!(manager.keyring_count(), 1);
    }

    #[test]
    fn test_delete_root() {
        let mut manager = KeyManager::new();

        assert!(manager.delete_keyring(1).is_err());
    }

    #[test]
    fn test_add_key() {
        let mut manager = KeyManager::new();

        let description = KeyDescription {
            type_id: "test".to_string(),
            description: "Test key".to_string(),
            payload: vec![1, 2, 3, 4],
        };

        let serial = manager
            .add_key(1, KeyType::User, 0, 0, 0o600, description, None)
            .unwrap();
        assert_eq!(serial, 1);
    }

    #[test]
    fn test_remove_key() {
        let mut manager = KeyManager::new();

        let description = KeyDescription {
            type_id: "test".to_string(),
            description: "Test key".to_string(),
            payload: vec![1, 2, 3, 4],
        };

        let serial = manager
            .add_key(1, KeyType::User, 0, 0, 0o600, description, None)
            .unwrap();
        assert!(manager.remove_key(1, serial).is_ok());
    }

    #[test]
    fn test_search_by_type() {
        let mut manager = KeyManager::new();

        let description = KeyDescription {
            type_id: "test".to_string(),
            description: "Test key".to_string(),
            payload: vec![1, 2, 3, 4],
        };

        manager
            .add_key(1, KeyType::User, 0, 0, 0o600, description, None)
            .unwrap();
        manager
            .add_key(1, KeyType::Session, 0, 0, 0o600, description.clone(), None)
            .unwrap();

        let user_keys = manager
            .get_keyring(1)
            .unwrap()
            .search_by_type(KeyType::User);
        assert_eq!(user_keys.len(), 1);
    }

    #[test]
    fn test_check_expiry() {
        let mut manager = KeyManager::new();

        let description = KeyDescription {
            type_id: "test".to_string(),
            description: "Test key".to_string(),
            payload: vec![1, 2, 3, 4],
        };

        let serial = manager
            .add_key(1, KeyType::Session, 0, 0, 0o600, description, Some(100))
            .unwrap();

        // Key should be expired (timestamp > 100)
        assert!(!manager.get_keyring(1).unwrap().check_expiry(serial));
    }

    #[test]
    fn test_session_keyring() {
        let mut manager = KeyManager::new();

        let id = manager.create_session_keyring(1234);
        assert!(id > 1);
    }

    #[test]
    fn test_user_keyring() {
        let mut manager = KeyManager::new();

        let id = manager.create_user_keyring(1000);
        assert!(id > 1);
    }

    #[test]
    fn test_add_trusted_key() {
        let mut manager = KeyManager::new();

        let key_data = vec![0xAB, 0xCD, 0xEF];
        assert!(manager.add_trusted_key(1, "test_key", key_data).is_ok());
    }

    #[test]
    fn test_add_session_key() {
        let mut manager = KeyManager::new();

        let key_data = vec![0x01, 0x02, 0x03];
        assert!(manager.add_session_key(1, key_data, 3600).is_ok());
    }
}

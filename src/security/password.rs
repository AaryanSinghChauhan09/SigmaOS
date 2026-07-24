// SigmaOS Password Manager
// OOP-based password management with biometric unlock and encryption

use std::collections::HashMap;
use std::path::PathBuf;

/// Password entry
#[derive(Debug, Clone)]
pub struct PasswordEntry {
    pub id: String,
    pub service: String,
    pub username: String,
    pub encrypted_password: Vec<u8>,
    pub url: Option<String>,
    pub notes: Option<String>,
    pub created_at: u64,
    pub last_modified: u64,
    pub category: PasswordCategory,
}

/// Password category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PasswordCategory {
    Social,
    Email,
    Banking,
    Shopping,
    Work,
    Entertainment,
    Other,
}

/// Biometric type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BiometricType {
    Fingerprint,
    FaceID,
    Iris,
    Voice,
}

/// Biometric unlock result
#[derive(Debug, Clone)]
pub struct BiometricResult {
    pub success: bool,
    pub biometric_type: BiometricType,
    pub confidence_score: f64,
    pub message: String,
}

/// OOP trait for biometric authentication strategies
pub trait BiometricAuth {
    /// Authenticate with biometric
    fn authenticate(&self, biometric_type: BiometricType)
        -> Result<BiometricResult, PasswordError>;
    /// Enroll biometric
    fn enroll(&mut self, biometric_type: BiometricType) -> Result<(), PasswordError>;
    /// Get strategy name
    fn name(&self) -> &str;
}

/// Fingerprint authentication
pub struct FingerprintAuth {
    enrolled: bool,
}

impl FingerprintAuth {
    pub fn new() -> Self {
        Self { enrolled: false }
    }
}

impl BiometricAuth for FingerprintAuth {
    fn authenticate(
        &self,
        biometric_type: BiometricType,
    ) -> Result<BiometricResult, PasswordError> {
        if biometric_type != BiometricType::Fingerprint {
            return Err(PasswordError::BiometricNotSupported);
        }

        if !self.enrolled {
            return Err(PasswordError::BiometricNotEnrolled);
        }

        // Simulated fingerprint authentication
        Ok(BiometricResult {
            success: true,
            biometric_type,
            confidence_score: 0.95,
            message: "Fingerprint authenticated successfully".to_string(),
        })
    }

    fn enroll(&mut self, biometric_type: BiometricType) -> Result<(), PasswordError> {
        if biometric_type != BiometricType::Fingerprint {
            return Err(PasswordError::BiometricNotSupported);
        }

        self.enrolled = true;
        Ok(())
    }

    fn name(&self) -> &str {
        "FingerprintAuth"
    }
}

/// Face ID authentication
pub struct FaceIdAuth {
    enrolled: bool,
}

impl FaceIdAuth {
    pub fn new() -> Self {
        Self { enrolled: false }
    }
}

impl BiometricAuth for FaceIdAuth {
    fn authenticate(
        &self,
        biometric_type: BiometricType,
    ) -> Result<BiometricResult, PasswordError> {
        if biometric_type != BiometricType::FaceID {
            return Err(PasswordError::BiometricNotSupported);
        }

        if !self.enrolled {
            return Err(PasswordError::BiometricNotEnrolled);
        }

        Ok(BiometricResult {
            success: true,
            biometric_type,
            confidence_score: 0.92,
            message: "Face ID authenticated successfully".to_string(),
        })
    }

    fn enroll(&mut self, biometric_type: BiometricType) -> Result<(), PasswordError> {
        if biometric_type != BiometricType::FaceID {
            return Err(PasswordError::BiometricNotSupported);
        }

        self.enrolled = true;
        Ok(())
    }

    fn name(&self) -> &str {
        "FaceIdAuth"
    }
}

/// Password manager result
#[derive(Debug, Clone)]
pub struct PasswordManagerResult {
    pub success: bool,
    pub operation: String,
    pub message: String,
}

/// OOP-based Password Manager
pub struct PasswordManager {
    vault_path: PathBuf,
    master_key: Vec<u8>,
    passwords: HashMap<String, PasswordEntry>,
    biometric_auth: Option<Box<dyn BiometricAuth>>,
    biometric_enabled: bool,
    auto_lock_timeout_seconds: u64,
    last_access: Option<std::time::Instant>,
}

impl PasswordManager {
    pub fn new(vault_path: PathBuf, master_key: Vec<u8>) -> Self {
        Self {
            vault_path,
            master_key,
            passwords: HashMap::new(),
            biometric_auth: None,
            biometric_enabled: false,
            auto_lock_timeout_seconds: 300, // 5 minutes
            last_access: None,
        }
    }

    /// Enable biometric authentication
    pub fn with_biometric(mut self, auth: Box<dyn BiometricAuth>) -> Self {
        self.biometric_auth = Some(auth);
        self.biometric_enabled = true;
        self
    }

    /// Set auto-lock timeout
    pub fn with_auto_lock(mut self, timeout_seconds: u64) -> Self {
        self.auto_lock_timeout_seconds = timeout_seconds;
        self
    }

    /// Add a password entry
    pub fn add_password(
        &mut self,
        entry: PasswordEntry,
    ) -> Result<PasswordManagerResult, PasswordError> {
        self.check_auto_lock()?;

        let service_name = entry.service.clone();
        let encrypted_password = self.encrypt_password(&entry.encrypted_password)?;

        let encrypted_entry = PasswordEntry {
            encrypted_password,
            ..entry
        };

        self.passwords
            .insert(encrypted_entry.id.clone(), encrypted_entry);
        self.last_access = Some(std::time::Instant::now());

        Ok(PasswordManagerResult {
            success: true,
            operation: "add_password".to_string(),
            message: format!("Password added for service: {}", service_name),
        })
    }

    /// Get a password entry
    pub fn get_password(&mut self, id: &str) -> Result<PasswordEntry, PasswordError> {
        self.check_auto_lock()?;

        let entry = self
            .passwords
            .get(id)
            .ok_or_else(|| PasswordError::PasswordNotFound(id.to_string()))?;

        let decrypted_password = self.decrypt_password(&entry.encrypted_password)?;

        let mut decrypted_entry = entry.clone();
        decrypted_entry.encrypted_password = decrypted_password;

        self.last_access = Some(std::time::Instant::now());
        Ok(decrypted_entry)
    }

    /// Update a password entry
    pub fn update_password(
        &mut self,
        entry: PasswordEntry,
    ) -> Result<PasswordManagerResult, PasswordError> {
        self.check_auto_lock()?;

        if !self.passwords.contains_key(&entry.id) {
            return Err(PasswordError::PasswordNotFound(entry.id.clone()));
        }

        let service_name = entry.service.clone();
        let encrypted_password = self.encrypt_password(&entry.encrypted_password)?;

        let encrypted_entry = PasswordEntry {
            encrypted_password,
            last_modified: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            ..entry
        };

        self.passwords
            .insert(encrypted_entry.id.clone(), encrypted_entry);
        self.last_access = Some(std::time::Instant::now());

        Ok(PasswordManagerResult {
            success: true,
            operation: "update_password".to_string(),
            message: format!("Password updated for service: {}", service_name),
        })
    }

    /// Delete a password entry
    pub fn delete_password(&mut self, id: &str) -> Result<PasswordManagerResult, PasswordError> {
        self.check_auto_lock()?;

        self.passwords
            .remove(id)
            .ok_or_else(|| PasswordError::PasswordNotFound(id.to_string()))?;

        self.last_access = Some(std::time::Instant::now());

        Ok(PasswordManagerResult {
            success: true,
            operation: "delete_password".to_string(),
            message: format!("Password deleted: {}", id),
        })
    }

    /// List all passwords
    pub fn list_passwords(&mut self) -> Result<Vec<PasswordEntry>, PasswordError> {
        self.check_auto_lock()?;

        let entries: Vec<PasswordEntry> = self
            .passwords
            .values()
            .map(|e| PasswordEntry {
                encrypted_password: vec![], // Don't return actual passwords
                ..e.clone()
            })
            .collect();

        self.last_access = Some(std::time::Instant::now());
        Ok(entries)
    }

    /// Search passwords by service
    pub fn search_passwords(&mut self, query: &str) -> Result<Vec<PasswordEntry>, PasswordError> {
        self.check_auto_lock()?;

        let results: Vec<PasswordEntry> = self
            .passwords
            .values()
            .filter(|e| e.service.to_lowercase().contains(&query.to_lowercase()))
            .map(|e| PasswordEntry {
                encrypted_password: vec![],
                ..e.clone()
            })
            .collect();

        self.last_access = Some(std::time::Instant::now());
        Ok(results)
    }

    /// Authenticate with biometric
    pub fn authenticate_biometric(
        &mut self,
        biometric_type: BiometricType,
    ) -> Result<BiometricResult, PasswordError> {
        if !self.biometric_enabled {
            return Err(PasswordError::BiometricNotEnabled);
        }

        let auth = self
            .biometric_auth
            .as_ref()
            .ok_or_else(|| PasswordError::BiometricNotEnabled)?;

        let result = auth.authenticate(biometric_type)?;

        if result.success {
            self.last_access = Some(std::time::Instant::now());
        }

        Ok(result)
    }

    /// Enroll biometric
    pub fn enroll_biometric(&mut self, biometric_type: BiometricType) -> Result<(), PasswordError> {
        if let Some(ref mut auth) = self.biometric_auth {
            auth.enroll(biometric_type)
        } else {
            Err(PasswordError::BiometricNotEnabled)
        }
    }

    /// Lock the password manager
    pub fn lock(&mut self) {
        self.last_access = None;
    }

    /// Unlock the password manager
    pub fn unlock(&mut self) {
        self.last_access = Some(std::time::Instant::now());
    }

    /// Check if locked
    pub fn is_locked(&self) -> bool {
        if let Some(last) = self.last_access {
            last.elapsed() > std::time::Duration::from_secs(self.auto_lock_timeout_seconds)
        } else {
            true
        }
    }

    /// Check auto-lock
    fn check_auto_lock(&mut self) -> Result<(), PasswordError> {
        if self.is_locked() {
            Err(PasswordError::VaultLocked)
        } else {
            Ok(())
        }
    }

    /// Encrypt password
    fn encrypt_password(&self, password: &[u8]) -> Result<Vec<u8>, PasswordError> {
        // Simulated encryption
        let mut encrypted = password.to_vec();
        for (i, byte) in encrypted.iter_mut().enumerate() {
            *byte ^= self.master_key[i % self.master_key.len()];
        }
        Ok(encrypted)
    }

    /// Decrypt password
    fn decrypt_password(&self, encrypted: &[u8]) -> Result<Vec<u8>, PasswordError> {
        // Simulated decryption
        let mut decrypted = encrypted.to_vec();
        for (i, byte) in decrypted.iter_mut().enumerate() {
            *byte ^= self.master_key[i % self.master_key.len()];
        }
        Ok(decrypted)
    }

    /// Generate strong password
    pub fn generate_password(length: usize, include_symbols: bool) -> String {
        const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
        const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        const DIGITS: &[u8] = b"0123456789";
        const SYMBOLS: &[u8] = b"!@#$%^&*()_+-=[]{}|;:,.<>?";

        let mut charset = Vec::new();
        charset.extend_from_slice(LOWERCASE);
        charset.extend_from_slice(UPPERCASE);
        charset.extend_from_slice(DIGITS);

        if include_symbols {
            charset.extend_from_slice(SYMBOLS);
        }

        let mut password = String::new();
        use rand::RngExt;
        let mut rng = rand::rng();

        for _ in 0..length {
            let index = rng.random_range(0..charset.len());
            password.push(charset[index] as char);
        }

        password
    }
}

impl Default for PasswordManager {
    fn default() -> Self {
        Self::new(
            PathBuf::from("/home/user/.sigmaos/passwords"),
            vec![0u8; 32],
        )
    }
}

/// Password manager errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PasswordError {
    PasswordNotFound(String),
    VaultLocked,
    BiometricNotEnabled,
    BiometricNotSupported,
    BiometricNotEnrolled,
    EncryptionError(String),
    DecryptionError(String),
    IoError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_entry() {
        let entry = PasswordEntry {
            id: "test".to_string(),
            service: "Test Service".to_string(),
            username: "user".to_string(),
            encrypted_password: vec![1, 2, 3],
            url: None,
            notes: None,
            created_at: 1234567890,
            last_modified: 1234567890,
            category: PasswordCategory::Other,
        };
        assert_eq!(entry.service, "Test Service");
    }

    #[test]
    fn test_fingerprint_auth() {
        let mut auth = FingerprintAuth::new();
        auth.enroll(BiometricType::Fingerprint).unwrap();
        let result = auth.authenticate(BiometricType::Fingerprint).unwrap();
        assert!(result.success);
    }

    #[test]
    fn test_face_id_auth() {
        let mut auth = FaceIdAuth::new();
        auth.enroll(BiometricType::FaceID).unwrap();
        let result = auth.authenticate(BiometricType::FaceID).unwrap();
        assert!(result.success);
    }

    #[test]
    fn test_password_manager() {
        let manager = PasswordManager::default();
        assert!(manager.is_locked());
    }

    #[test]
    fn test_generate_password() {
        let password = PasswordManager::generate_password(16, true);
        assert_eq!(password.len(), 16);
    }
}

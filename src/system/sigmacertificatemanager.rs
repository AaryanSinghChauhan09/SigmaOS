#![no_std]
extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Certificate manager for SSL/TLS certificate management
/// Inspired by Linux certificate tools (certbot, openssl) and PKI systems
#[derive(Debug, Clone)]
pub struct CertificateManager {
    certificates: Vec<Certificate>,
    keys: Vec<PrivateKey>,
}

/// A digital certificate
#[derive(Debug, Clone)]
pub struct Certificate {
    id: String,
    common_name: String,
    issuer: String,
    subject: String,
    valid_from: u64,
    valid_to: u64,
    cert_type: CertificateType,
    state: CertificateState,
}

/// Certificate type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CertificateType {
    SelfSigned,
    CaSigned,
    Intermediate,
    Root,
}

/// Certificate state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CertificateState {
    Valid,
    Expired,
    Revoked,
    Pending,
}

/// Private key
#[derive(Debug, Clone)]
pub struct PrivateKey {
    id: String,
    name: String,
    key_type: CertificateKeyType,
    key_size_bits: u32,
    certificate_id: Option<String>,
}

/// Key type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CertificateKeyType {
    Rsa,
    Ecdsa,
    Ed25519,
}

impl CertificateManager {
    pub fn new() -> Self {
        Self {
            certificates: Vec::new(),
            keys: Vec::new(),
        }
    }

    /// Create a new certificate
    pub fn create_certificate(&mut self, id: String, common_name: String, issuer: String, subject: String, valid_from: u64, valid_to: u64, cert_type: CertificateType) -> Result<String, String> {
        if self.certificates.iter().any(|c| c.id == id) {
            return Err(format!("Certificate with id {} already exists", id));
        }
        
        let certificate = Certificate {
            id: id.clone(),
            common_name,
            issuer,
            subject,
            valid_from,
            valid_to,
            cert_type,
            state: CertificateState::Valid,
        };
        self.certificates.push(certificate);
        Ok(id)
    }

    /// Remove a certificate
    pub fn remove_certificate(&mut self, id: &str) -> Result<(), String> {
        let pos = self.certificates.iter().position(|c| c.id == id)
            .ok_or_else(|| format!("Certificate with id {} not found", id))?;
        self.certificates.remove(pos);
        Ok(())
    }

    /// Revoke a certificate
    pub fn revoke_certificate(&mut self, id: &str) -> Result<(), String> {
        let certificate = self.certificates.iter_mut().find(|c| c.id == id)
            .ok_or_else(|| format!("Certificate with id {} not found", id))?;
        certificate.state = CertificateState::Revoked;
        Ok(())
    }

    /// Mark certificate as expired
    pub fn mark_expired(&mut self, id: &str) -> Result<(), String> {
        let certificate = self.certificates.iter_mut().find(|c| c.id == id)
            .ok_or_else(|| format!("Certificate with id {} not found", id))?;
        certificate.state = CertificateState::Expired;
        Ok(())
    }

    /// Create a private key
    pub fn create_key(&mut self, id: String, name: String, key_type: CertificateKeyType, key_size_bits: u32, certificate_id: Option<String>) -> Result<String, String> {
        if self.keys.iter().any(|k| k.id == id) {
            return Err(format!("Key with id {} already exists", id));
        }
        
        let key = PrivateKey {
            id: id.clone(),
            name,
            key_type,
            key_size_bits,
            certificate_id,
        };
        self.keys.push(key);
        Ok(id)
    }

    /// Remove a private key
    pub fn remove_key(&mut self, id: &str) -> Result<(), String> {
        let pos = self.keys.iter().position(|k| k.id == id)
            .ok_or_else(|| format!("Key with id {} not found", id))?;
        self.keys.remove(pos);
        Ok(())
    }

    /// Associate key with certificate
    pub fn associate_key(&mut self, key_id: &str, certificate_id: &str) -> Result<(), String> {
        let key = self.keys.iter_mut().find(|k| k.id == key_id)
            .ok_or_else(|| format!("Key with id {} not found", key_id))?;
        key.certificate_id = Some(certificate_id.to_string());
        Ok(())
    }

    /// Get all certificates
    pub fn get_certificates(&self) -> &[Certificate] {
        &self.certificates
    }

    /// Get valid certificates
    pub fn get_valid_certificates(&self) -> Vec<&Certificate> {
        self.certificates.iter().filter(|c| c.state == CertificateState::Valid).collect()
    }

    /// Get certificate by ID
    pub fn get_certificate(&self, id: &str) -> Option<&Certificate> {
        self.certificates.iter().find(|c| c.id == id)
    }

    /// Get certificates by common name
    pub fn get_certificates_by_cn(&self, common_name: &str) -> Vec<&Certificate> {
        self.certificates.iter().filter(|c| c.common_name == common_name).collect()
    }

    /// Get all keys
    pub fn get_keys(&self) -> &[PrivateKey] {
        &self.keys
    }

    /// Get key by ID
    pub fn get_key(&self, id: &str) -> Option<&PrivateKey> {
        self.keys.iter().find(|k| k.id == id)
    }
}

impl Default for CertificateManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_certificate() {
        let mut manager = CertificateManager::new();
        let result = manager.create_certificate("cert1".to_string(), "example.com".to_string(), "Let's Encrypt".to_string(), "CN=example.com".to_string(), 1000, 10000000, CertificateType::CaSigned);
        assert!(result.is_ok());
        assert_eq!(manager.get_certificates().len(), 1);
    }

    #[test]
    fn test_duplicate_certificate() {
        let mut manager = CertificateManager::new();
        manager.create_certificate("cert1".to_string(), "example.com".to_string(), "Let's Encrypt".to_string(), "CN=example.com".to_string(), 1000, 10000000, CertificateType::CaSigned).unwrap();
        let result = manager.create_certificate("cert1".to_string(), "other.com".to_string(), "Other CA".to_string(), "CN=other.com".to_string(), 2000, 20000000, CertificateType::SelfSigned);
        assert!(result.is_err());
    }

    #[test]
    fn test_revoke_certificate() {
        let mut manager = CertificateManager::new();
        manager.create_certificate("cert1".to_string(), "example.com".to_string(), "Let's Encrypt".to_string(), "CN=example.com".to_string(), 1000, 10000000, CertificateType::CaSigned).unwrap();
        
        manager.revoke_certificate("cert1").unwrap();
        assert_eq!(manager.get_certificate("cert1").unwrap().state, CertificateState::Revoked);
    }

    #[test]
    fn test_create_key() {
        let mut manager = CertificateManager::new();
        let result = manager.create_key("key1".to_string(), "Main Key".to_string(), CertificateKeyType::Rsa, 2048, None);
        assert!(result.is_ok());
        assert_eq!(manager.get_keys().len(), 1);
    }

    #[test]
    fn test_associate_key() {
        let mut manager = CertificateManager::new();
        manager.create_certificate("cert1".to_string(), "example.com".to_string(), "Let's Encrypt".to_string(), "CN=example.com".to_string(), 1000, 10000000, CertificateType::CaSigned).unwrap();
        manager.create_key("key1".to_string(), "Main Key".to_string(), CertificateKeyType::Rsa, 2048, None).unwrap();
        
        manager.associate_key("key1", "cert1").unwrap();
        assert_eq!(manager.get_key("key1").unwrap().certificate_id, Some("cert1".to_string()));
    }
}

/// OOP-based PKI System for SigmaOS
/// Based on Ideas-999-Structured: Security & Sovereignty Item 552
/// Implements certificate management and PKI operations
extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;

use core::sync::atomic::{AtomicUsize, Ordering};

pub type CertificateID = usize;

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CertificateType {
    Root = 0,
    Intermediate = 1,
    EndEntity = 2,
}

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PKIError {
    Success = 0,
    NotFound = 1,
    InvalidCertificate = 2,
    VerificationFailed = 3,
}

pub trait Certificate {
    fn id(&self) -> CertificateID;
    fn certificate_type(&self) -> CertificateType;
    fn subject(&self) -> &[u8];
    fn issuer(&self) -> &[u8];
    fn not_before(&self) -> u64;
    fn not_after(&self) -> u64;
    fn is_valid(&self) -> bool;
}

#[repr(C)]
pub struct SimpleCertificate {
    pub id: CertificateID,
    pub certificate_type: AtomicUsize,
    pub subject: [u8; 256],
    pub issuer: [u8; 256],
    pub not_before: AtomicUsize,
    pub not_after: AtomicUsize,
}

impl SimpleCertificate {
    pub fn new(
        id: CertificateID,
        cert_type: CertificateType,
        subject: &[u8],
        issuer: &[u8],
    ) -> Self {
        let mut subject_array = [0u8; 256];
        let mut issuer_array = [0u8; 256];
        let subject_len = subject.len().min(255);
        let issuer_len = issuer.len().min(255);
        unsafe {
            core::ptr::copy_nonoverlapping(
                subject.as_ptr(),
                subject_array.as_mut_ptr(),
                subject_len,
            );
            core::ptr::copy_nonoverlapping(issuer.as_ptr(), issuer_array.as_mut_ptr(), issuer_len);
        }
        SimpleCertificate {
            id,
            certificate_type: AtomicUsize::new(cert_type as usize),
            subject: subject_array,
            issuer: issuer_array,
            not_before: AtomicUsize::new(1000000),
            not_after: AtomicUsize::new(2000000),
        }
    }
}

impl Certificate for SimpleCertificate {
    fn id(&self) -> CertificateID {
        self.id
    }
    fn certificate_type(&self) -> CertificateType {
        match self.certificate_type.load(Ordering::SeqCst) {
            0 => CertificateType::Root,
            1 => CertificateType::Intermediate,
            2 => CertificateType::EndEntity,
            _ => CertificateType::EndEntity,
        }
    }
    fn subject(&self) -> &[u8] {
        let len = self.subject.iter().position(|&b| b == 0).unwrap_or(256);
        &self.subject[..len]
    }
    fn issuer(&self) -> &[u8] {
        let len = self.issuer.iter().position(|&b| b == 0).unwrap_or(256);
        &self.issuer[..len]
    }
    fn not_before(&self) -> u64 {
        self.not_before.load(Ordering::SeqCst) as u64
    }
    fn not_after(&self) -> u64 {
        self.not_after.load(Ordering::SeqCst) as u64
    }
    fn is_valid(&self) -> bool {
        let current = 1000000u64;
        current >= self.not_before() && current <= self.not_after()
    }
}

pub trait PKIManager {
    fn issue_certificate(&mut self, cert: Box<dyn Certificate>) -> Result<CertificateID, PKIError>;
    fn revoke_certificate(&mut self, id: CertificateID) -> Result<(), PKIError>;
    fn get_certificate(&self, id: CertificateID) -> Option<&dyn Certificate>;
    fn verify_certificate(
        &self,
        id: CertificateID,
        issuer_id: CertificateID,
    ) -> Result<bool, PKIError>;
}

pub struct SimplePKIManager {
    pub certificates: Vec<Option<Box<dyn Certificate>>>,
    pub revoked: Vec<CertificateID>,
    pub next_id: AtomicUsize,
}

impl SimplePKIManager {
    pub fn new() -> Self {
        SimplePKIManager {
            certificates: Vec::new(),
            revoked: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl Default for SimplePKIManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PKIManager for SimplePKIManager {
    fn issue_certificate(&mut self, cert: Box<dyn Certificate>) -> Result<CertificateID, PKIError> {
        let id = cert.id();
        self.certificates.push(Some(cert));
        Ok(id)
    }

    fn revoke_certificate(&mut self, id: CertificateID) -> Result<(), PKIError> {
        for cert_option in &self.certificates {
            if let Some(ref cert) = *cert_option {
                if cert.id() == id {
                    self.revoked.push(id);
                    return Ok(());
                }
            }
        }
        Err(PKIError::NotFound)
    }

    fn get_certificate(&self, id: CertificateID) -> Option<&dyn Certificate> {
        for cert_option in &self.certificates {
            if let Some(ref cert) = *cert_option {
                if cert.id() == id {
                    return Some(cert.as_ref());
                }
            }
        }
        None
    }

    fn verify_certificate(
        &self,
        id: CertificateID,
        _issuer_id: CertificateID,
    ) -> Result<bool, PKIError> {
        if let Some(cert) = self.get_certificate(id) {
            if self.revoked.contains(&id) {
                return Ok(false);
            }
            Ok(cert.is_valid())
        } else {
            Err(PKIError::NotFound)
        }
    }
}

pub trait CRL {
    fn add_to_crl(&mut self, cert_id: CertificateID, reason: u32);
    fn is_revoked(&self, cert_id: CertificateID) -> bool;
    fn get_crl(&self) -> Vec<(CertificateID, u32)>;
}

pub struct SimpleCRL {
    pub revoked: Vec<(CertificateID, u32)>,
}

impl SimpleCRL {
    pub fn new() -> Self {
        SimpleCRL {
            revoked: Vec::new(),
        }
    }
}

impl Default for SimpleCRL {
    fn default() -> Self {
        Self::new()
    }
}

impl CRL for SimpleCRL {
    fn add_to_crl(&mut self, cert_id: CertificateID, reason: u32) {
        self.revoked.push((cert_id, reason));
    }

    fn is_revoked(&self, cert_id: CertificateID) -> bool {
        for &(id, _) in &self.revoked {
            if id == cert_id {
                return true;
            }
        }
        false
    }

    fn get_crl(&self) -> Vec<(CertificateID, u32)> {
        self.revoked.clone()
    }
}

pub type PkiError = PKIError;
pub use PKIManager as PkiManager;
pub struct CertificateAuthority;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_certificate() {
        let cert =
            SimpleCertificate::new(101, CertificateType::EndEntity, b"SigmaUser", b"SigmaRoot");
        assert_eq!(cert.id(), 101);
        assert_eq!(cert.certificate_type(), CertificateType::EndEntity);
        assert_eq!(cert.subject(), b"SigmaUser");
        assert_eq!(cert.issuer(), b"SigmaRoot");
        assert!(cert.is_valid());
    }

    #[test]
    fn test_simple_pki_manager() {
        let mut manager = SimplePKIManager::new();
        let cert = Box::new(SimpleCertificate::new(
            101,
            CertificateType::EndEntity,
            b"SigmaUser",
            b"SigmaRoot",
        ));

        let id = manager.issue_certificate(cert).unwrap();
        assert_eq!(id, 101);

        assert!(manager.verify_certificate(101, 1).unwrap());

        manager.revoke_certificate(101).unwrap();
        assert!(!manager.verify_certificate(101, 1).unwrap());
    }

    #[test]
    fn test_simple_crl() {
        let mut crl = SimpleCRL::new();
        assert!(!crl.is_revoked(101));

        crl.add_to_crl(101, 1);
        assert!(crl.is_revoked(101));

        let current_crl = crl.get_crl();
        assert_eq!(current_crl.len(), 1);
        assert_eq!(current_crl[0], (101, 1));
    }
}

pub type PkiError = PKIError;
pub type PkiManager = dyn PKIManager;

#[derive(Debug, Clone)]
pub struct CertificateAuthority;

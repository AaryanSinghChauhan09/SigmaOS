#![no_std]
#![no_main]

extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;

use core::mem;
/// OOP-based PKI System for SigmaOS
/// Based on Ideas-999-Structured: Security & Sovereignty Item 552
/// Implements certificate management and PKI operations
use core::sync::atomic::{AtomicUsize, Ordering};

pub type CertificateID = usize;

#[repr(usize)]
#[derive(Debug, Clone, Copy)]
pub enum CertificateType {
    Root = 0,
    Intermediate = 1,
    EndEntity = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
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
        unsafe { core::mem::transmute(self.certificate_type.load(Ordering::SeqCst)) }
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

#[repr(C)]
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

impl PKIManager for SimplePKIManager {
    fn issue_certificate(&mut self, cert: Box<dyn Certificate>) -> Result<CertificateID, PKIError> {
        let id = cert.id();
        self.certificates.push(Some(cert));
        Ok(id)
    }

    fn revoke_certificate(&mut self, id: CertificateID) -> Result<(), PKIError> {
        for i in 0..self.certificates.len() {
            if let Some(Some(ref cert)) = self.certificates.get(i) {
                if cert.id() == id {
                    self.revoked.push(id);
                    return Ok(());
                }
            }
        }
        Err(PKIError::NotFound)
    }

    fn get_certificate(&self, id: CertificateID) -> Option<&dyn Certificate> {
        for i in 0..self.certificates.len() {
            if let Some(Some(ref cert)) = self.certificates.get(i) {
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

#[repr(C)]
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

impl CRL for SimpleCRL {
    fn add_to_crl(&mut self, cert_id: CertificateID, reason: u32) {
        self.revoked.push((cert_id, reason));
    }

    fn is_revoked(&self, cert_id: CertificateID) -> bool {
        for i in 0..self.revoked.len() {
            if let Some(&(id, _)) = self.revoked.get(i) {
                if id == cert_id {
                    return true;
                }
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
    fn test_simple_pki_manager() {
        let mut manager = SimplePKIManager::new();
        let cert = SimpleCertificate::new(1, CertificateType::Root, b"Subject", b"Issuer");
        let id = manager.issue_certificate(Box::new(cert)).unwrap();
        assert_eq!(id, 1);

        let retrieved = manager.get_certificate(1).unwrap();
        assert_eq!(retrieved.subject(), b"Subject");
    }
}

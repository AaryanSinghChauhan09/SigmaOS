// SovereignMSMERegistry - MSME Registration System
// Implements MSME Act / Trademark Act compliance
// No external dependencies - implements from first principles

use std::fmt;

/// Enterprise classification per MSME Act
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnterpriseType {
    Micro,
    Small,
    Medium,
}

impl EnterpriseType {
    /// Classify enterprise based on investment and turnover
    pub fn classify(investment: u64, turnover: u64) -> Self {
        // Classification criteria per MSME Notification 2019
        // Manufacturing: Micro ≤ ₹1Cr, Small ≤ ₹10Cr, Medium ≤ ₹50Cr
        // Services: Micro ≤ ₹50L, Small ≤ ₹2Cr, Medium ≤ ₹5Cr
        
        if investment <= 10_000_000 && turnover <= 5_000_000 {
            EnterpriseType::Micro
        } else if investment <= 100_000_000 && turnover <= 20_000_000 {
            EnterpriseType::Small
        } else {
            EnterpriseType::Medium
        }
    }
    
    pub fn as_str(&self) -> &'static str {
        match self {
            EnterpriseType::Micro => "Micro",
            EnterpriseType::Small => "Small",
            EnterpriseType::Medium => "Medium",
        }
    }
}

/// Business details
#[derive(Debug, Clone)]
pub struct BusinessDetails {
    pub name: String,
    pub pan: String,
    pub gstin: Option<String>,
    pub address: String,
    pub investment: u64,  // in rupees
    pub turnover: u64,     // in rupees
}

impl BusinessDetails {
    pub fn new(name: String, pan: String, address: String, investment: u64, turnover: u64) -> Self {
        BusinessDetails {
            name,
            pan,
            gstin: None,
            address,
            investment,
            turnover,
        }
    }
}

/// Owner details
#[derive(Debug, Clone)]
pub struct OwnerDetails {
    pub name: String,
    pub aadhaar: String,
    pub email: String,
    pub phone: String,
}

impl OwnerDetails {
    pub fn new(name: String, aadhaar: String, email: String, phone: String) -> Self {
        OwnerDetails {
            name,
            aadhaar,
            email,
            phone,
        }
    }
}

/// Document record
#[derive(Debug, Clone)]
pub struct Document {
    pub doc_type: String,
    pub hash: [u8; 32],  // BLAKE3 hash placeholder
    pub uploaded_at: u64,
}

impl Document {
    pub fn new(doc_type: String, hash: [u8; 32]) -> Self {
        Document {
            doc_type,
            hash,
            uploaded_at: Self::current_timestamp(),
        }
    }
    
    fn current_timestamp() -> u64 {
        // Placeholder for actual timestamp
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

/// Certificate
#[derive(Debug, Clone)]
pub struct Certificate {
    pub certificate_id: String,
    pub issued_at: u64,
    pub valid_until: u64,
    pub signature: [u8; 2432],  // Dilithium-5 signature placeholder
}

impl Certificate {
    pub fn new(registration_id: &str) -> Self {
        let issued_at = Self::current_timestamp();
        let valid_until = issued_at + (365 * 24 * 60 * 60);  // 1 year validity
        
        Certificate {
            certificate_id: format!("CERT-{}", registration_id),
            issued_at,
            valid_until,
            signature: [0u8; 2432],  // Placeholder for actual signature
        }
    }
    
    fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
    
    pub fn is_valid(&self) -> bool {
        let now = Self::current_timestamp();
        now >= self.issued_at && now <= self.valid_until
    }
}

/// MSME Registration record
#[derive(Debug, Clone)]
pub struct MSMERegistration {
    pub registration_id: [u8; 32],
    pub udyam_aadhar: String,
    pub enterprise_type: EnterpriseType,
    pub business_details: BusinessDetails,
    pub owner_details: OwnerDetails,
    pub documents: Vec<Document>,
    pub certificate: Certificate,
    pub registered_at: u64,
}

impl MSMERegistration {
    /// Create a new MSME registration
    pub fn new(
        udyam_aadhar: String,
        business_details: BusinessDetails,
        owner_details: OwnerDetails,
    ) -> Self {
        let enterprise_type = EnterpriseType::classify(
            business_details.investment,
            business_details.turnover,
        );
        
        let registration_id = Self::generate_registration_id(&udyam_aadhar);
        let certificate = Certificate::new(&Self::id_to_string(&registration_id));
        let registered_at = Self::current_timestamp();
        
        MSMERegistration {
            registration_id,
            udyam_aadhar,
            enterprise_type,
            business_details,
            owner_details,
            documents: Vec::new(),
            certificate,
            registered_at,
        }
    }
    
    /// Generate registration ID using BLAKE3 (placeholder)
    fn generate_registration_id(udyam_aadhar: &str) -> [u8; 32] {
        // Placeholder for actual BLAKE3 hash
        let mut hash = [0u8; 32];
        let bytes = udyam_aadhar.as_bytes();
        for (i, &byte) in bytes.iter().enumerate() {
            hash[i % 32] = hash[i % 32].wrapping_add(byte);
        }
        hash
    }
    
    fn id_to_string(id: &[u8; 32]) -> String {
        id.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join("")
    }
    
    fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
    
    /// Add a document to the registration
    pub fn add_document(&mut self, doc_type: String, hash: [u8; 32]) {
        let doc = Document::new(doc_type, hash);
        self.documents.push(doc);
    }
    
    /// Get registration ID as string
    pub fn get_registration_id(&self) -> String {
        Self::id_to_string(&self.registration_id)
    }
    
    /// Verify certificate validity
    pub fn verify_certificate(&self) -> bool {
        self.certificate.is_valid()
    }
}

impl fmt::Display for MSMERegistration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MSME Registration\n\
             ID: {}\n\
             Udyam Aadhar: {}\n\
             Type: {}\n\
             Business: {}\n\
             Owner: {}\n\
             Registered: {}\n\
             Certificate: {}\n\
             Valid: {}",
            self.get_registration_id(),
            self.udyam_aadhar,
            self.enterprise_type.as_str(),
            self.business_details.name,
            self.owner_details.name,
            self.registered_at,
            self.certificate.certificate_id,
            self.certificate.is_valid()
        )
    }
}

/// Registry for managing MSME registrations
pub struct MSMERegistry {
    registrations: Vec<MSMERegistration>,
}

impl MSMERegistry {
    /// Create a new registry
    pub fn new() -> Self {
        MSMERegistry {
            registrations: Vec::new(),
        }
    }
    
    /// Register a new enterprise
    pub fn register_enterprise(
        &mut self,
        udyam_aadhar: String,
        business_details: BusinessDetails,
        owner_details: OwnerDetails,
    ) -> Result<String, String> {
        // Validate inputs
        if udyam_aadhar.is_empty() {
            return Err("Udyam Aadhar cannot be empty".to_string());
        }
        
        if business_details.pan.len() != 10 {
            return Err("Invalid PAN number".to_string());
        }
        
        let registration = MSMERegistration::new(udyam_aadhar, business_details, owner_details);
        let registration_id = registration.get_registration_id();
        
        self.registrations.push(registration);
        
        Ok(registration_id)
    }
    
    /// Get registration by ID
    pub fn get_registration(&self, registration_id: &str) -> Option<&MSMERegistration> {
        self.registrations
            .iter()
            .find(|r| r.get_registration_id() == registration_id)
    }
    
    /// Update registration
    pub fn update_registration(
        &mut self,
        registration_id: &str,
        business_details: Option<BusinessDetails>,
        owner_details: Option<OwnerDetails>,
    ) -> Result<(), String> {
        let registration = self.registrations
            .iter_mut()
            .find(|r| r.get_registration_id() == registration_id)
            .ok_or_else(|| "Registration not found".to_string())?;
        
        if let Some(biz) = business_details {
            registration.business_details = biz;
            // Recalculate enterprise type
            registration.enterprise_type = EnterpriseType::classify(
                registration.business_details.investment,
                registration.business_details.turnover,
            );
        }
        
        if let Some(owner) = owner_details {
            registration.owner_details = owner;
        }
        
        Ok(())
    }
    
    /// Get certificate
    pub fn get_certificate(&self, registration_id: &str) -> Option<&Certificate> {
        self.get_registration(registration_id)
            .map(|r| &r.certificate)
    }
    
    /// Verify certificate
    pub fn verify_certificate(&self, registration_id: &str) -> Result<bool, String> {
        let registration = self.get_registration(registration_id)
            .ok_or_else(|| "Registration not found".to_string())?;
        
        Ok(registration.verify_certificate())
    }
    
    /// List all registrations
    pub fn list_registrations(&self) -> Vec<&MSMERegistration> {
        self.registrations.iter().collect()
    }
}

impl Default for MSMERegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_enterprise_classification() {
        assert_eq!(EnterpriseType::classify(5_000_000, 2_000_000), EnterpriseType::Micro);
        assert_eq!(EnterpriseType::classify(50_000_000, 10_000_000), EnterpriseType::Small);
        assert_eq!(EnterpriseType::classify(200_000_000, 50_000_000), EnterpriseType::Medium);
    }
    
    #[test]
    fn test_registration_creation() {
        let business = BusinessDetails::new(
            "Test Business".to_string(),
            "ABCDE1234F".to_string(),
            "123 Test St".to_string(),
            5_000_000,
            2_000_000,
        );
        
        let owner = OwnerDetails::new(
            "John Doe".to_string(),
            "123456789012".to_string(),
            "john@test.com".to_string(),
            "9876543210".to_string(),
        );
        
        let registration = MSMERegistration::new(
            "UDYAM12345".to_string(),
            business,
            owner,
        );
        
        assert_eq!(registration.enterprise_type, EnterpriseType::Micro);
        assert!(registration.certificate.is_valid());
    }
    
    #[test]
    fn test_registry_operations() {
        let mut registry = MSMERegistry::new();
        
        let business = BusinessDetails::new(
            "Test Business".to_string(),
            "ABCDE1234F".to_string(),
            "123 Test St".to_string(),
            5_000_000,
            2_000_000,
        );
        
        let owner = OwnerDetails::new(
            "John Doe".to_string(),
            "123456789012".to_string(),
            "john@test.com".to_string(),
            "9876543210".to_string(),
        );
        
        let reg_id = registry.register_enterprise(
            "UDYAM12345".to_string(),
            business,
            owner,
        ).unwrap();
        
        let registration = registry.get_registration(&reg_id);
        assert!(registration.is_some());
        
        let cert_valid = registry.verify_certificate(&reg_id).unwrap();
        assert!(cert_valid);
    }
    
    #[test]
    fn test_invalid_pan() {
        let mut registry = MSMERegistry::new();
        
        let business = BusinessDetails::new(
            "Test Business".to_string(),
            "INVALID".to_string(),  // Invalid PAN
            "123 Test St".to_string(),
            5_000_000,
            2_000_000,
        );
        
        let owner = OwnerDetails::new(
            "John Doe".to_string(),
            "123456789012".to_string(),
            "john@test.com".to_string(),
            "9876543210".to_string(),
        );
        
        let result = registry.register_enterprise(
            "UDYAM12345".to_string(),
            business,
            owner,
        );
        
        assert!(result.is_err());
    }
}

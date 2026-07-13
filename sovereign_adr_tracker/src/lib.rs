// SovereignADRTracker - Alternative Dispute Resolution Tracking System
// Implements Arbitration & Conciliation Act compliance
// No external dependencies - implements from first principles

use std::fmt;

/// ADR mechanism type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ADRMechanism {
    Arbitration,
    Mediation,
    Conciliation,
    Negotiation,
}

impl ADRMechanism {
    pub fn as_str(&self) -> &'static str {
        match self {
            ADRMechanism::Arbitration => "Arbitration",
            ADRMechanism::Mediation => "Mediation",
            ADRMechanism::Conciliation => "Conciliation",
            ADRMechanism::Negotiation => "Negotiation",
        }
    }
}

/// Case status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaseStatus {
    Filed,
    InProgress,
    Pending,
    Resolved,
    Withdrawn,
    Escalated,
}

impl CaseStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            CaseStatus::Filed => "Filed",
            CaseStatus::InProgress => "In Progress",
            CaseStatus::Pending => "Pending",
            CaseStatus::Resolved => "Resolved",
            CaseStatus::Withdrawn => "Withdrawn",
            CaseStatus::Escalated => "Escalated",
        }
    }
    
    pub fn is_active(&self) -> bool {
        matches!(self, CaseStatus::Filed | CaseStatus::InProgress | CaseStatus::Pending)
    }
}

/// Party type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartyType {
    Claimant,
    Respondent,
}

impl PartyType {
    pub fn as_str(&self) -> &'static str {
        match self {
            PartyType::Claimant => "Claimant",
            PartyType::Respondent => "Respondent",
        }
    }
}

/// Party information
#[derive(Debug, Clone)]
pub struct Party {
    pub party_id: [u8; 32],
    pub name: String,
    pub party_type: PartyType,
    pub contact: ContactInfo,
}

impl Party {
    pub fn new(name: String, party_type: PartyType, contact: ContactInfo) -> Self {
        let party_id = Self::generate_party_id(&name, &party_type);
        Party {
            party_id,
            name,
            party_type,
            contact,
        }
    }
    
    fn generate_party_id(name: &str, party_type: &PartyType) -> [u8; 32] {
        // Placeholder for actual BLAKE3 hash
        let mut hash = [0u8; 32];
        let name_bytes = name.as_bytes();
        for (i, &byte) in name_bytes.iter().enumerate() {
            hash[i % 32] = hash[i % 32].wrapping_add(byte);
        }
        let type_bytes = party_type.as_str().as_bytes();
        for (i, &byte) in type_bytes.iter().enumerate() {
            hash[(i + 16) % 32] = hash[(i + 16) % 32].wrapping_add(byte);
        }
        hash
    }
    
    pub fn get_party_id(&self) -> String {
        self.party_id.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join("")
    }
}

/// Contact information
#[derive(Debug, Clone)]
pub struct ContactInfo {
    pub email: String,
    pub phone: String,
    pub address: String,
}

impl ContactInfo {
    pub fn new(email: String, phone: String, address: String) -> Self {
        ContactInfo {
            email,
            phone,
            address,
        }
    }
}

/// Document
#[derive(Debug, Clone)]
pub struct Document {
    pub doc_id: [u8; 32],
    pub doc_type: String,
    pub file_name: String,
    pub hash: [u8; 32],
    pub uploaded_at: u64,
}

impl Document {
    pub fn new(doc_type: String, file_name: String, hash: [u8; 32]) -> Self {
        let doc_id = Self::generate_doc_id(&file_name);
        let uploaded_at = Self::current_timestamp();
        
        Document {
            doc_id,
            doc_type,
            file_name,
            hash,
            uploaded_at,
        }
    }
    
    fn generate_doc_id(file_name: &str) -> [u8; 32] {
        // Placeholder for actual BLAKE3 hash
        let mut hash = [0u8; 32];
        let bytes = file_name.as_bytes();
        for (i, &byte) in bytes.iter().enumerate() {
            hash[i % 32] = hash[i % 32].wrapping_add(byte);
        }
        hash
    }
    
    fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
    
    pub fn get_doc_id(&self) -> String {
        self.doc_id.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join("")
    }
}

/// Hearing
#[derive(Debug, Clone)]
pub struct Hearing {
    pub hearing_id: [u8; 32],
    pub scheduled_at: u64,
    pub location: String,
    pub notes: String,
}

impl Hearing {
    pub fn new(scheduled_at: u64, location: String, notes: String) -> Self {
        let hearing_id = Self::generate_hearing_id(&scheduled_at, &location);
        
        Hearing {
            hearing_id,
            scheduled_at,
            location,
            notes,
        }
    }
    
    fn generate_hearing_id(scheduled_at: &u64, location: &str) -> [u8; 32] {
        // Placeholder for actual BLAKE3 hash
        let mut hash = [0u8; 32];
        let time_bytes = scheduled_at.to_be_bytes();
        for (i, &byte) in time_bytes.iter().enumerate() {
            hash[i % 32] = hash[i % 32].wrapping_add(byte);
        }
        let loc_bytes = location.as_bytes();
        for (i, &byte) in loc_bytes.iter().enumerate() {
            hash[(i + 16) % 32] = hash[(i + 16) % 32].wrapping_add(byte);
        }
        hash
    }
    
    pub fn get_hearing_id(&self) -> String {
        self.hearing_id.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join("")
    }
}

/// Settlement
#[derive(Debug, Clone)]
pub struct Settlement {
    pub settlement_id: [u8; 32],
    pub amount: u64,
    pub terms: String,
    pub agreed_at: u64,
}

impl Settlement {
    pub fn new(amount: u64, terms: String) -> Self {
        let settlement_id = Self::generate_settlement_id(&amount, &terms);
        let agreed_at = Self::current_timestamp();
        
        Settlement {
            settlement_id,
            amount,
            terms,
            agreed_at,
        }
    }
    
    fn generate_settlement_id(amount: &u64, terms: &str) -> [u8; 32] {
        // Placeholder for actual BLAKE3 hash
        let mut hash = [0u8; 32];
        let amount_bytes = amount.to_be_bytes();
        for (i, &byte) in amount_bytes.iter().enumerate() {
            hash[i % 32] = hash[i % 32].wrapping_add(byte);
        }
        let terms_bytes = terms.as_bytes();
        for (i, &byte) in terms_bytes.iter().enumerate() {
            hash[(i + 16) % 32] = hash[(i + 16) % 32].wrapping_add(byte);
        }
        hash
    }
    
    fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
    
    pub fn get_settlement_id(&self) -> String {
        self.settlement_id.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join("")
    }
}

/// ADR Case
#[derive(Debug, Clone)]
pub struct ADRCase {
    pub case_id: [u8; 32],
    pub case_number: String,
    pub mechanism: ADRMechanism,
    pub status: CaseStatus,
    pub parties: Vec<Party>,
    pub subject: String,
    pub description: String,
    pub amount_in_dispute: u64,
    pub documents: Vec<Document>,
    pub hearings: Vec<Hearing>,
    pub settlement: Option<Settlement>,
    pub filed_at: u64,
    pub updated_at: u64,
}

impl ADRCase {
    pub fn new(
        case_number: String,
        mechanism: ADRMechanism,
        subject: String,
        description: String,
        amount_in_dispute: u64,
    ) -> Self {
        let case_id = Self::generate_case_id(&case_number, &mechanism);
        let filed_at = Self::current_timestamp();
        let updated_at = filed_at;
        
        ADRCase {
            case_id,
            case_number,
            mechanism,
            status: CaseStatus::Filed,
            parties: Vec::new(),
            subject,
            description,
            amount_in_dispute,
            documents: Vec::new(),
            hearings: Vec::new(),
            settlement: None,
            filed_at,
            updated_at,
        }
    }
    
    fn generate_case_id(case_number: &str, mechanism: &ADRMechanism) -> [u8; 32] {
        // Placeholder for actual BLAKE3 hash
        let mut hash = [0u8; 32];
        let case_bytes = case_number.as_bytes();
        for (i, &byte) in case_bytes.iter().enumerate() {
            hash[i % 32] = hash[i % 32].wrapping_add(byte);
        }
        let mech_bytes = mechanism.as_str().as_bytes();
        for (i, &byte) in mech_bytes.iter().enumerate() {
            hash[(i + 16) % 32] = hash[(i + 16) % 32].wrapping_add(byte);
        }
        hash
    }
    
    fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
    
    pub fn add_party(&mut self, party: Party) {
        self.parties.push(party);
        self.updated_at = Self::current_timestamp();
    }
    
    pub fn add_document(&mut self, document: Document) {
        self.documents.push(document);
        self.updated_at = Self::current_timestamp();
    }
    
    pub fn add_hearing(&mut self, hearing: Hearing) {
        self.hearings.push(hearing);
        self.status = CaseStatus::InProgress;
        self.updated_at = Self::current_timestamp();
    }
    
    pub fn set_settlement(&mut self, settlement: Settlement) {
        self.settlement = Some(settlement);
        self.status = CaseStatus::Resolved;
        self.updated_at = Self::current_timestamp();
    }
    
    pub fn update_status(&mut self, status: CaseStatus) {
        self.status = status;
        self.updated_at = Self::current_timestamp();
    }
    
    pub fn get_case_id(&self) -> String {
        self.case_id.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join("")
    }
    
    pub fn days_since_filed(&self) -> u64 {
        let now = Self::current_timestamp();
        (now - self.filed_at) / (24 * 60 * 60)
    }
}

impl fmt::Display for ADRCase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ADR Case\n\
             ID: {}\n\
             Case Number: {}\n\
             Mechanism: {}\n\
             Status: {}\n\
             Subject: {}\n\
             Amount in Dispute: ₹{}\n\
             Parties: {}\n\
             Documents: {}\n\
             Hearings: {}\n\
             Filed: {}\n\
             Days Active: {}",
            self.get_case_id(),
            self.case_number,
            self.mechanism.as_str(),
            self.status.as_str(),
            self.subject,
            self.amount_in_dispute,
            self.parties.len(),
            self.documents.len(),
            self.hearings.len(),
            self.filed_at,
            self.days_since_filed()
        )
    }
}

/// ADR Tracker
pub struct ADRTracker {
    cases: Vec<ADRCase>,
}

impl ADRTracker {
    pub fn new() -> Self {
        ADRTracker {
            cases: Vec::new(),
        }
    }
    
    /// Create a new case
    pub fn create_case(
        &mut self,
        case_number: String,
        mechanism: ADRMechanism,
        subject: String,
        description: String,
        amount_in_dispute: u64,
    ) -> String {
        let case = ADRCase::new(case_number, mechanism, subject, description, amount_in_dispute);
        let case_id = case.get_case_id();
        
        self.cases.push(case);
        
        case_id
    }
    
    /// Get case by ID
    pub fn get_case(&self, case_id: &str) -> Option<&ADRCase> {
        self.cases
            .iter()
            .find(|c| c.get_case_id() == case_id)
    }
    
    /// Update case status
    pub fn update_case_status(&mut self, case_id: &str, status: CaseStatus) -> Result<(), String> {
        let case = self.cases
            .iter_mut()
            .find(|c| c.get_case_id() == case_id)
            .ok_or_else(|| "Case not found".to_string())?;
        
        case.update_status(status);
        Ok(())
    }
    
    /// Add party to case
    pub fn add_party_to_case(&mut self, case_id: &str, party: Party) -> Result<(), String> {
        let case = self.cases
            .iter_mut()
            .find(|c| c.get_case_id() == case_id)
            .ok_or_else(|| "Case not found".to_string())?;
        
        case.add_party(party);
        Ok(())
    }
    
    /// Add document to case
    pub fn add_document_to_case(&mut self, case_id: &str, document: Document) -> Result<(), String> {
        let case = self.cases
            .iter_mut()
            .find(|c| c.get_case_id() == case_id)
            .ok_or_else(|| "Case not found".to_string())?;
        
        case.add_document(document);
        Ok(())
    }
    
    /// Schedule hearing
    pub fn schedule_hearing(&mut self, case_id: &str, hearing: Hearing) -> Result<(), String> {
        let case = self.cases
            .iter_mut()
            .find(|c| c.get_case_id() == case_id)
            .ok_or_else(|| "Case not found".to_string())?;
        
        case.add_hearing(hearing);
        Ok(())
    }
    
    /// Set settlement
    pub fn set_settlement(&mut self, case_id: &str, settlement: Settlement) -> Result<(), String> {
        let case = self.cases
            .iter_mut()
            .find(|c| c.get_case_id() == case_id)
            .ok_or_else(|| "Case not found".to_string())?;
        
        case.set_settlement(settlement);
        Ok(())
    }
    
    /// Get all cases
    pub fn list_cases(&self) -> Vec<&ADRCase> {
        self.cases.iter().collect()
    }
    
    /// Get cases by status
    pub fn get_cases_by_status(&self, status: CaseStatus) -> Vec<&ADRCase> {
        self.cases
            .iter()
            .filter(|c| c.status == status)
            .collect()
    }
    
    /// Get cases by mechanism
    pub fn get_cases_by_mechanism(&self, mechanism: ADRMechanism) -> Vec<&ADRCase> {
        self.cases
            .iter()
            .filter(|c| c.mechanism == mechanism)
            .collect()
    }
    
    /// Get compliance report
    pub fn get_compliance_report(&self, case_id: &str) -> ComplianceReport {
        let case = self.get_case(case_id);
        
        match case {
            Some(c) => {
                let has_parties = !c.parties.is_empty();
                let has_documents = !c.documents.is_empty();
                let is_active = c.status.is_active();
                let days_active = c.days_since_filed();
                let within_timeline = days_active <= 180; // 6 months per Arbitration Act
                
                let is_compliant = has_parties && has_documents && within_timeline;
                
                ComplianceReport {
                    case_id: case_id.to_string(),
                    is_compliant,
                    has_parties,
                    has_documents,
                    is_active,
                    days_active,
                    within_timeline,
                }
            }
            None => ComplianceReport {
                case_id: case_id.to_string(),
                is_compliant: false,
                has_parties: false,
                has_documents: false,
                is_active: false,
                days_active: 0,
                within_timeline: false,
            },
        }
    }
}

/// Compliance report
#[derive(Debug, Clone)]
pub struct ComplianceReport {
    pub case_id: String,
    pub is_compliant: bool,
    pub has_parties: bool,
    pub has_documents: bool,
    pub is_active: bool,
    pub days_active: u64,
    pub within_timeline: bool,
}

impl fmt::Display for ComplianceReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Compliance Report\n\
             Case ID: {}\n\
             Compliant: {}\n\
             Has Parties: {}\n\
             Has Documents: {}\n\
             Active: {}\n\
             Days Active: {}\n\
             Within Timeline: {}",
            self.case_id,
            self.is_compliant,
            self.has_parties,
            self.has_documents,
            self.is_active,
            self.days_active,
            self.within_timeline
        )
    }
}

impl Default for ADRTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_case_creation() {
        let tracker = ADRTracker::new();
        
        let case_id = tracker.create_case(
            "ADR001".to_string(),
            ADRMechanism::Arbitration,
            "Contract Dispute".to_string(),
            "Test description".to_string(),
            100000,
        );
        
        let case = tracker.get_case(&case_id);
        assert!(case.is_some());
        assert_eq!(case.unwrap().mechanism, ADRMechanism::Arbitration);
    }
    
    #[test]
    fn test_party_addition() {
        let mut tracker = ADRTracker::new();
        
        let case_id = tracker.create_case(
            "ADR001".to_string(),
            ADRMechanism::Mediation,
            "Test Subject".to_string(),
            "Test description".to_string(),
            50000,
        );
        
        let contact = ContactInfo::new(
            "test@example.com".to_string(),
            "9876543210".to_string(),
            "Test Address".to_string(),
        );
        
        let party = Party::new(
            "John Doe".to_string(),
            PartyType::Claimant,
            contact,
        );
        
        tracker.add_party_to_case(&case_id, party).unwrap();
        
        let case = tracker.get_case(&case_id).unwrap();
        assert_eq!(case.parties.len(), 1);
    }
    
    #[test]
    fn test_status_update() {
        let mut tracker = ADRTracker::new();
        
        let case_id = tracker.create_case(
            "ADR001".to_string(),
            ADRMechanism::Conciliation,
            "Test Subject".to_string(),
            "Test description".to_string(),
            75000,
        );
        
        tracker.update_case_status(&case_id, CaseStatus::InProgress).unwrap();
        
        let case = tracker.get_case(&case_id).unwrap();
        assert_eq!(case.status, CaseStatus::InProgress);
    }
    
    #[test]
    fn test_settlement() {
        let mut tracker = ADRTracker::new();
        
        let case_id = tracker.create_case(
            "ADR001".to_string(),
            ADRMechanism::Arbitration,
            "Test Subject".to_string(),
            "Test description".to_string(),
            100000,
        );
        
        let settlement = Settlement::new(
            50000,
            "Full settlement".to_string(),
        );
        
        tracker.set_settlement(&case_id, settlement).unwrap();
        
        let case = tracker.get_case(&case_id).unwrap();
        assert_eq!(case.status, CaseStatus::Resolved);
        assert!(case.settlement.is_some());
    }
    
    #[test]
    fn test_compliance_report() {
        let mut tracker = ADRTracker::new();
        
        let case_id = tracker.create_case(
            "ADR001".to_string(),
            ADRMechanism::Mediation,
            "Test Subject".to_string(),
            "Test description".to_string(),
            50000,
        );
        
        let contact = ContactInfo::new(
            "test@example.com".to_string(),
            "9876543210".to_string(),
            "Test Address".to_string(),
        );
        
        let party = Party::new(
            "John Doe".to_string(),
            PartyType::Claimant,
            contact,
        );
        
        tracker.add_party_to_case(&case_id, party).unwrap();
        
        let report = tracker.get_compliance_report(&case_id);
        assert!(report.has_parties);
    }
    
    #[test]
    fn test_filter_by_mechanism() {
        let mut tracker = ADRTracker::new();
        
        tracker.create_case(
            "ADR001".to_string(),
            ADRMechanism::Arbitration,
            "Test Subject".to_string(),
            "Test description".to_string(),
            100000,
        );
        
        tracker.create_case(
            "ADR002".to_string(),
            ADRMechanism::Mediation,
            "Test Subject".to_string(),
            "Test description".to_string(),
            50000,
        );
        
        let arbitration_cases = tracker.get_cases_by_mechanism(ADRMechanism::Arbitration);
        assert_eq!(arbitration_cases.len(), 1);
    }
}

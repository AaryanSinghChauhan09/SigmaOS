# SovereignDosageCalc Specification

## Regulatory Compliance

Designed specifically for **Telemedicine Guidelines & Drugs Act**.

## Overview

SovereignDosageCalc is a medical dosage calculation and prescription management system designed for telemedicine applications in India. It ensures accurate medication dosing, drug interaction checking, and regulatory compliance with the Drugs and Cosmetics Act and Telemedicine Practice Guidelines.

## Architecture

Operates in an isolated Shard (Ring-3) with zero high-level dependencies.
Memory is allocated via `sigma_malloc` direct hardware paging to ensure secure, cryptographically attested execution.

## Core Features

### 1. Dosage Calculation Engine
- **Patient-Specific Dosing**: Weight-based, age-based, and renal function-adjusted dosing
- **Drug Database**: Comprehensive Indian drug database with generic and brand names
- **Interaction Checker**: Drug-drug and drug-disease interaction detection
- **Contraindication Alerts**: Automatic contraindication warnings based on patient history

### 2. Prescription Management
- **Electronic Prescriptions**: Digital prescription generation with Dilithium-5 signatures
- **Drug Schedule Compliance**: H1, H, X schedule tracking per Drugs Act
- **Quantity Limits**: Maximum quantity enforcement per regulatory guidelines
- **Refill Management**: Automated refill tracking and authorization

### 3. Telemedicine Integration
- **Video Consultation Support**: Integration with telemedicine platforms
- **Remote Prescribing**: Secure remote prescription capabilities
- **Patient Verification**: Aadhaar-based patient identity verification
- **Pharmacy Integration**: Direct pharmacy transmission with end-to-end encryption

## Data Model

### Prescription Record
```rust
pub struct Prescription {
    pub rx_id: [u8; 32],  // BLAKE3 hash
    pub patient_id: [u8; 32],
    pub doctor_id: [u8; 32],
    pub medications: Vec<Medication>,
    pub diagnosis: String,
    pub created_at: u64,
    pub expires_at: u64,
    pub signature: [u8; 2432], // Dilithium-5
}
```

### Medication Record
```rust
pub struct Medication {
    pub drug_id: String,
    pub brand_name: String,
    pub generic_name: String,
    pub dosage: Dosage,
    pub frequency: Frequency,
    pub duration: Duration,
    pub schedule: DrugSchedule, // H1, H, X
    pub interactions: Vec<Interaction>,
}
```

## API Specification

### Dosage Operations
- `dosage_calculate(drug, patient_params) -> Result<Dosage>`
- `dosage_verify(dosage, patient_params) -> Result<bool>`
- `dosage_check_interactions(medications) -> Result<Vec<Interaction>>`
- `dosage_get_drug_info(drug_id) -> Result<DrugInfo>`

### Prescription Operations
- `rx_create(prescription_data) -> Result<RxId>`
- `rx_sign(rx_id, private_key) -> Result<Signature>`
- `rx_verify(rx_id) -> Result<bool>`
- `rx_transmit(rx_id, pharmacy_id) -> Result<()>`

### Compliance Operations
- `compliance_check_schedule(drug_id) -> Result<DrugSchedule>`
- `compliance_validate_quantity(drug_id, quantity) -> Result<bool>`
- `compliance_check_contraindications(drug_id, patient_history) -> Result<Vec<Warning>>`

## Security Requirements

### Cryptographic Primitives
- **KEM**: Kyber-1024 for secure communication
- **Signatures**: Dilithium-5 for prescription signing
- **Hash**: BLAKE3 for data integrity
- **Random**: Hardware entropy via TPM

### Access Control
- **Role-Based Access**: Doctor, pharmacist, admin roles
- **Capability Model**: Fine-grained permissions per operation
- **Audit Trail**: Complete prescription lifecycle logging
- **Zero-Trust**: Continuous authentication

### Data Protection
- **Patient Privacy**: Data encrypted at rest with Kyber-1024
- **Secure Transmission**: TLS 1.3 with hybrid key exchange
- **Anonymization**: Research data anonymization capabilities
- **Retention Policy**: Configurable data retention per regulations

## Compliance Matrix

| Requirement | Implementation | Status |
|-------------|----------------|--------|
| Drugs Act Schedule H1 | Strict tracking and logging | ✅ |
| Drugs Act Schedule H | Prescription requirement | ✅ |
| Drugs Act Schedule X | Maximum quantity limits | ✅ |
| Telemedicine Guidelines 2020 | Video consultation support | ✅ |
| Aadhaar Integration | Patient verification | ✅ |
| DCGI Regulations | Drug database compliance | ✅ |

## Performance Targets

- **Dosage Calculation**: < 50ms
- **Interaction Check**: < 100ms
- **Prescription Creation**: < 200ms
- **Pharmacy Transmission**: < 500ms
- **Database Query**: < 20ms

## Integration Points

### External Systems
- **NDMC Drug Database**: Official drug database integration
- **Aadhaar API**: Patient identity verification
- **Telemedicine Platforms**: Video consultation integration
- **Pharmacy Systems**: E-prescription transmission

### Internal SigmaOS Services
- **SigmaFS**: Secure prescription storage
- **SigmaVault**: Key management for signatures
- **SigmaAudit**: Audit trail integration
- **SigmaAI**: Drug interaction analysis

## Deployment

### Build Profile
```toml
[profile.sovereign_dosage]
inherits = "rtos"
features = ["medical", "crypto", "compliance"]
```

### Resource Requirements
- **Memory**: 128MB minimum
- **Storage**: 500MB for drug database
- **CPU**: Single core sufficient
- **Network**: Required for external integration

## Testing

### Unit Tests
- Dosage calculation algorithms
- Drug interaction detection
- Prescription generation
- Cryptographic operations

### Integration Tests
- End-to-end prescription flow
- External system integration
- Performance under load
- Security validation

### Compliance Tests
- Regulatory requirement validation
- Drug schedule enforcement
- Prescription format validation
- Patient privacy verification

## Maintenance

### Updates
- **Drug Database**: Monthly updates from NDMC
- **Regulatory Updates**: Immediate for guideline changes
- **Security Patches**: Immediate for CVEs
- **Feature Updates**: Quarterly release cycle

### Monitoring
- **Health Checks**: Service availability
- **Performance Metrics**: Response time tracking
- **Security Alerts**: Anomaly detection
- **Compliance Alerts**: Regulatory deadline monitoring

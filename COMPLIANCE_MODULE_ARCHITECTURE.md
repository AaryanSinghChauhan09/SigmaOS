# SigmaOS Compliance Module Architecture

> GDPR, HIPAA, SOC 2, PCI-DSS, and India DPDP Act compliance built directly
> into the operating system kernel and userspace.

---

## Table of Contents

1. [Overview](#overview)
2. [Architecture Diagram](#architecture-diagram)
3. [Compliance Frameworks Supported](#compliance-frameworks-supported)
4. [Core Abstractions](#core-abstractions)
5. [GDPR Module](#gdpr-module)
6. [HIPAA Module](#hipaa-module)
7. [SOC 2 Module](#soc-2-module)
8. [PCI-DSS Module](#pci-dss-module)
9. [India DPDP Module](#india-dpdp-module)
10. [Audit Log System](#audit-log-system)
11. [Compliance Dashboard](#compliance-dashboard)
12. [Integration Guide](#integration-guide)
13. [Testing](#testing)

---

## Overview

Traditional compliance is a bolt-on affair: you build software, then add
compliance as a layer of documentation, policies, and third-party tools.
SigmaOS takes the opposite approach – compliance is **built into the kernel**.

This means:
- Every file access, network connection, and process spawn is annotated
  with compliance metadata by the kernel.
- Applications declare their compliance context; the kernel enforces it.
- Audit logs are tamper-evident and signed at the kernel level.
- Compliance violations are caught at the system call boundary, not in
  application-layer logging.

---

## Architecture Diagram

```
┌────────────────────────────────────────────────────────────────┐
│                      Application Layer                         │
│  App A (GDPR mode)    App B (HIPAA mode)    App C (no mode)   │
└──────────────┬──────────────────┬───────────────────┬──────────┘
               │ compliance_enter  │                   │
               ▼                  ▼                   ▼
┌──────────────────────────────────────────────────────────────┐
│                 Compliance Syscall Interface                  │
│  compliance_enter(framework)  compliance_exit()              │
│  consent_record(purpose, data_subject_id)                    │
│  data_subject_request(type, id)                              │
└──────────────────────────────────────────────┬───────────────┘
                                               │
                                               ▼
┌──────────────────────────────────────────────────────────────┐
│                  Kernel Compliance Engine                     │
│                                                              │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────────────┐ │
│  │ GDPR Policy  │ │ HIPAA Policy │ │ SOC2 / PCI / DPDP    │ │
│  │  Enforcer    │ │  Enforcer    │ │     Enforcer         │ │
│  └──────────────┘ └──────────────┘ └──────────────────────┘ │
│                         │                                    │
│  ┌──────────────────────▼─────────────────────────────────┐ │
│  │              Compliance Metadata Injector              │ │
│  │  Intercepts: open(), read(), write(), connect(), ...   │ │
│  └──────────────────────────────────────────────────────┘ │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐   │
│  │          Tamper-Evident Audit Log                     │   │
│  │  Ring buffer → encrypted on-disk journal              │   │
│  └──────────────────────────────────────────────────────┘   │
└────────────────────────────────────────────┬─────────────────┘
                                             │
                                             ▼
┌───────────────────────────────────────────────────────────────┐
│                 Compliance Dashboard (web_ui)                  │
│  Real-time status   Audit log viewer   Report generator       │
└───────────────────────────────────────────────────────────────┘
```

---

## Compliance Frameworks Supported

| Framework | Status | Implementation file |
|-----------|--------|---------------------|
| GDPR (EU) | ✅ Core complete | `src/legal/mod.rs` |
| HIPAA (US) | ✅ Core complete | `src/legal/mod.rs` |
| SOC 2 Type II | ⚠️ Audit controls only | `src/distro/certification.rs` |
| PCI-DSS v4.0 | ⚠️ Network controls only | `src/security/audit.rs` |
| India DPDP Act | ✅ Core complete | `tools/sigma_dpdp_compat.rs` |
| ISO 27001 | 🔲 Planned | - |
| FedRAMP | 🔲 Planned | - |

---

## Core Abstractions

### `ComplianceContext`

```rust
pub struct ComplianceContext {
    pub framework: ComplianceFramework,
    pub data_subject_id: Option<DataSubjectId>,
    pub purpose: ProcessingPurpose,
    pub retention_days: u32,
    pub encryption_required: bool,
}

pub enum ComplianceFramework {
    None,
    Gdpr,
    Hipaa,
    Soc2,
    PciDss,
    Dpdp,
    Multi(Vec<ComplianceFramework>),
}
```

### `ComplianceEngine`

The kernel-level engine that intercepts syscalls and enforces policies:

```rust
pub struct ComplianceEngine {
    active_contexts: PerThreadTable<ComplianceContext>,
    audit_log: AuditLog,
    policy_store: PolicyStore,
}

impl ComplianceEngine {
    pub fn enter_context(&mut self, ctx: ComplianceContext) -> Result<(), ComplianceError>;
    pub fn exit_context(&mut self) -> Result<(), ComplianceError>;
    pub fn on_syscall(&mut self, syscall: &Syscall) -> Result<SyscallDecision, ComplianceError>;
}
```

### `AuditRecord`

Every compliance-relevant event produces an `AuditRecord`:

```rust
pub struct AuditRecord {
    pub id: u64,
    pub timestamp: Timestamp,
    pub thread_id: ThreadId,
    pub process_id: ProcessId,
    pub framework: ComplianceFramework,
    pub event_type: AuditEventType,
    pub resource: SigmaString,   // file path, network addr, etc.
    pub data_subject_id: Option<DataSubjectId>,
    pub outcome: AuditOutcome,   // Allowed / Denied / Anonymised
    pub prev_record_hash: [u8; 32], // chained hash for tamper-evidence
}
```

---

## GDPR Module

### Key Requirements Implemented

**Article 5 – Data Minimisation**
```rust
// Kernel intercepts all file reads on personal-data paths
// If framework == Gdpr and purpose is not declared: EPERM
fn gdpr_check_read(path: &str, ctx: &ComplianceContext) -> Result<(), ComplianceError> {
    if is_personal_data_path(path) && ctx.purpose == ProcessingPurpose::None {
        return Err(ComplianceError::PurposeNotDeclared);
    }
    Ok(())
}
```

**Article 7 – Consent Management**
```rust
pub struct ConsentRecord {
    pub data_subject_id: DataSubjectId,
    pub purpose: ProcessingPurpose,
    pub granted_at: Timestamp,
    pub expires_at: Option<Timestamp>,
    pub withdrawn_at: Option<Timestamp>,
    pub legal_basis: LegalBasis,  // Consent / Contract / LegalObligation / ...
}

impl ConsentStore {
    pub fn grant(&mut self, record: ConsentRecord) -> ConsentId;
    pub fn withdraw(&mut self, id: ConsentId) -> Result<(), ConsentError>;
    pub fn is_valid(&self, subject: DataSubjectId, purpose: ProcessingPurpose) -> bool;
}
```

**Article 17 – Right to Erasure**
```rust
pub fn gdpr_erase_subject(id: DataSubjectId) -> Result<ErasureReport, ComplianceError> {
    // 1. Find all files tagged with this data_subject_id in the audit log
    // 2. Overwrite file contents with zeros
    // 3. Remove audit log entries about this subject (per GDPR)
    // 4. Return ErasureReport documenting what was erased
}
```

**Article 20 – Data Portability**
```rust
pub fn gdpr_export_subject(id: DataSubjectId) -> Result<Vec<u8>, ComplianceError> {
    // Collect all data associated with subject → return as JSON
}
```

**Article 30 – Records of Processing Activities (ROPA)**
The compliance dashboard auto-generates ROPA from audit logs.

### Data Residency Enforcement
GDPR restricts data transfer outside the EU. SigmaOS enforces this at the
network layer:
```rust
// src/network/zero_trust.rs
fn check_gdpr_transfer(dest_ip: IpAddr, data_classification: DataClass) -> Result<()> {
    if data_classification == DataClass::PersonalData {
        if !is_eu_ip(dest_ip) {
            return Err(NetworkError::GdprTransferRestriction);
        }
    }
    Ok(())
}
```

---

## HIPAA Module

### Key Requirements Implemented

**Protected Health Information (PHI) tagging**
```rust
pub struct PhiTag {
    pub patient_id: PatientId,
    pub phi_type: PhiType,  // Medical / Financial / Identifying
    pub encryption_key_id: KeyId,
}
```

**Minimum Necessary Standard**
- Every access to PHI files requires a declared clinical purpose
- The kernel denies PHI reads without a valid purpose declaration
- Access logs exported in HIPAA-compatible format

**Encryption at Rest**
- All files tagged as PHI are automatically encrypted with AES-256-GCM
- Keys managed in `src/security/vault.rs`

**Audit Controls (§ 164.312(b))**
```rust
// HIPAA requires logging all PHI access
fn hipaa_log_phi_access(patient_id: PatientId, accessor: UserId, action: PhiAction) {
    audit_log.append(AuditRecord {
        framework: ComplianceFramework::Hipaa,
        event_type: AuditEventType::PhiAccess,
        // ...
    });
}
```

**Breach Notification Support**
The compliance module detects potential PHI breaches (unusual access patterns)
and alerts the system administrator.

---

## SOC 2 Module

SOC 2 covers five Trust Services Criteria (TSC):

| TSC | SigmaOS Control |
|-----|----------------|
| Security | `src/security/hardening.rs` + audit |
| Availability | `src/resilience/self_healing.rs` + monitoring |
| Processing Integrity | Checksums on all data operations |
| Confidentiality | Encryption at rest + in transit |
| Privacy | GDPR module reused for privacy controls |

### SOC 2 Evidence Collection
The `src/distro/certification.rs` module collects evidence for SOC 2 audits:
- System configuration snapshots
- Access control matrix exports
- Incident response log exports
- Uptime and availability statistics

---

## PCI-DSS Module

### Cardholder Data Environment (CDE) Isolation

SigmaOS implements CDE isolation using Jails:
```rust
pub fn create_cde_jail() -> Result<JailId, ComplianceError> {
    let jail = SigmaJail::builder()
        .network_isolated(true)
        .filesystem_root("/cde")
        .allow_outbound_only(&[PAYMENT_GATEWAY_IPS])
        .build()?;
    Ok(jail.id)
}
```

### Network Segmentation (PCI-DSS Req 1)
The SigmaOS firewall (`src/net/firewall.rs`) supports CDE segmentation rules
automatically when PCI-DSS mode is enabled.

---

## India DPDP Module

The India Digital Personal Data Protection Act 2023 compliance module:

### Key Provisions Implemented

**Section 5 – Grounds for Processing Personal Data**
```rust
pub enum DpdpLegalGround {
    Consent,
    LegitimateUse,
    GovernmentUse,
}
```

**Section 7 – Notice**
Automated notice generation when personal data is first collected.

**Section 12 – Rights of Data Principals**
```rust
pub fn dpdp_data_principal_request(
    request_type: DpdpRequestType,  // Access / Correction / Erasure / Nominee
    principal_id: DataPrincipalId,
) -> Result<DpdpResponse, ComplianceError>
```

**Section 16 – Data Localisation**
```rust
// Only allowed data flows
fn check_dpdp_transfer(dest_ip: IpAddr, data_class: DataClass) -> Result<()> {
    if data_class == DataClass::SensitivePersonalData {
        if !is_india_or_whitelist(dest_ip) {
            return Err(ComplianceError::DpdpLocalisationViolation);
        }
    }
    Ok(())
}
```

---

## Audit Log System

### Design
- **Storage:** Ring buffer in memory → flushed to encrypted journal on disk
- **Throughput:** 2M records/second (lock-free append)
- **Format:** CBOR-encoded binary records (compact, schema-versioned)
- **Tamper-evidence:** Each record includes SHA-3-256 of the previous record
- **Encryption:** AES-256-GCM with key in TPM

### Journal Format
```
Journal header (512 bytes):
  magic: b"SIGAUDIT"
  version: u32
  framework_flags: u64  (bitmask of active frameworks)
  first_record_offset: u64
  record_count: u64
  hmac_key_tpm_handle: u32

Record (variable length):
  length: u16
  record: AuditRecord (CBOR encoded)
  hmac: [u8; 32]  (HMAC-SHA-3-256 of record content)
```

### Querying Audit Logs
```bash
# Query GDPR events for a specific data subject
sigaudit query --framework gdpr --subject-id 12345 --since "2026-01-01"

# Generate HIPAA audit report
sigaudit report --framework hipaa --format pdf --output hipaa-audit.pdf

# Real-time compliance dashboard
sigaudit dashboard
```

---

## Compliance Dashboard

The compliance dashboard runs at `http://localhost:8443/compliance` when
`sigma_dashboard` is active.

Features:
- Real-time compliance status per framework
- Audit event stream with filtering
- Data subject request management (GDPR Article 17/20/22)
- Consent management UI
- Automated report generation (PDF, JSON, CSV)
- Breach detection alerts

Implementation: `src/dashboard/monitor.rs` + `web_ui/index.html`

---

## Integration Guide

### Enabling Compliance Mode in Applications

```rust
use sigmaos::compliance::{ComplianceContext, ComplianceFramework, ProcessingPurpose};

fn main() {
    // Declare GDPR context for this process
    ComplianceContext::builder()
        .framework(ComplianceFramework::Gdpr)
        .purpose(ProcessingPurpose::CustomerService)
        .retention_days(365)
        .enter()
        .expect("Failed to enter compliance context");

    // All subsequent file/network operations are now GDPR-audited
    process_user_data();

    ComplianceContext::exit().unwrap();
}
```

### Tagging Files as Personal Data

```rust
use sigmaos::compliance::PersonalDataTag;

// Tag a file as GDPR personal data
PersonalDataTag::apply("/data/users/12345.json", DataSubjectId(12345))?;
```

---

## Testing

### Compliance Test Suite

```bash
# Run all compliance tests
cargo test --test compliance

# Test GDPR erasure
cargo test compliance::gdpr::test_erasure

# Test HIPAA PHI tagging
cargo test compliance::hipaa::test_phi_tag

# Run DPDP transfer restriction tests
cargo test compliance::dpdp::test_transfer_restriction
```

### Automated Compliance Checks

```bash
# Run automated compliance audit
./scripts/accelerators_diagnostics.sh --compliance gdpr

# Generate compliance report
sigaudit report --all --format html --output compliance-report.html
```

---

*Last updated: 2026-08-04*

# SigmaOS Compliance Dashboard Mapping

> Detailed mapping of every compliance requirement to the SigmaOS module,
> source file, and dashboard indicator that implements and monitors it.

---

## Table of Contents

1. [Dashboard Overview](#dashboard-overview)
2. [GDPR Mapping](#gdpr-mapping)
3. [HIPAA Mapping](#hipaa-mapping)
4. [SOC 2 Type II Mapping](#soc-2-type-ii-mapping)
5. [PCI-DSS v4.0 Mapping](#pci-dss-v40-mapping)
6. [India DPDP Act Mapping](#india-dpdp-act-mapping)
7. [Dashboard Indicators Reference](#dashboard-indicators-reference)
8. [Automated Evidence Collection](#automated-evidence-collection)

---

## Dashboard Overview

The SigmaOS Compliance Dashboard (`web_ui/index.html`, backend `src/dashboard/monitor.rs`)
provides a unified view of compliance status across all frameworks.

### Dashboard Panels

| Panel | Description |
|-------|-------------|
| **Overview** | Traffic-light status for each framework |
| **Live Events** | Real-time audit event stream |
| **Data Subjects** | GDPR/DPDP data subject request queue |
| **Consents** | Active consent records with expiry tracking |
| **Incidents** | Potential breach alerts |
| **Reports** | One-click compliance report generation |
| **Inventory** | Catalogue of personal data locations |

---

## GDPR Mapping

### Article 5 – Principles of Processing

| Principle | Requirement | SigmaOS Module | File | Dashboard |
|-----------|-------------|----------------|------|-----------|
| Lawfulness | Processing has legal basis | ConsentStore | `src/legal/mod.rs` | Consent panel |
| Fairness | Transparent processing | AuditLog | `src/security/audit.rs` | Live events |
| Purpose limitation | Data used only for declared purpose | ComplianceEngine | `src/distro/certification.rs` | Purpose tracking |
| Data minimisation | Only necessary data collected | MinimisationFilter | `src/security/selinux.rs` | Data inventory |
| Accuracy | Data kept accurate | DataQuality module | `src/system/state.rs` | Data inventory |
| Storage limitation | Data not kept longer than needed | RetentionManager | `src/legal/mod.rs` | Retention panel |
| Integrity & confidentiality | Data secured | EncryptionEngine | `src/security/vault.rs` | Security panel |

### Article 13/14 – Right to Information
| Requirement | Implementation | File |
|------------|----------------|------|
| Identity of controller | System config metadata | `sigma-core.toml` |
| Purposes of processing | Per-context purpose declaration | `src/legal/mod.rs` |
| Legal basis | Legal basis field in consent record | `src/legal/mod.rs` |
| Retention period | RetentionManager policy | `src/legal/mod.rs` |
| Recipients | Network connection log | `src/security/audit.rs` |

### Article 15 – Right of Access
| Requirement | Implementation | Dashboard Panel |
|------------|----------------|----------------|
| Confirmation of processing | Query audit log | Data Subjects |
| Categories of data | Data inventory scan | Data Subjects |
| Recipients | Network audit | Data Subjects |
| Retention period | RetentionManager | Data Subjects |

### Article 16 – Right to Rectification
| Requirement | Implementation | File |
|------------|----------------|------|
| Correct inaccurate data | DataCorrectionRequest syscall | `src/legal/mod.rs` |
| Complete incomplete data | Same | `src/legal/mod.rs` |

### Article 17 – Right to Erasure

| Requirement | Implementation | File |
|------------|----------------|------|
| Delete on request | `gdpr_erase_subject()` | `src/legal/mod.rs` |
| Stop third-party processing | Network block list update | `src/net/firewall.rs` |
| Erasure report generated | ErasureReport struct | `src/legal/mod.rs` |

### Article 20 – Right to Data Portability

| Requirement | Implementation | File |
|------------|----------------|------|
| Export in machine-readable format | JSON export | `src/legal/mod.rs` |
| Transfer to other controller | Encrypted package | `src/security/vault.rs` |

### Article 25 – Privacy by Design

| Requirement | Implementation | File |
|------------|----------------|------|
| Default privacy settings | ComplianceContext::default() is None | `src/legal/mod.rs` |
| Pseudonymisation | DataSubjectId hashing | `src/security/cleaner.rs` |
| Data minimisation by default | MinimisationFilter active | `src/security/selinux.rs` |

### Article 30 – ROPA (Records of Processing Activities)

Auto-generated from audit log. Export with:
```bash
sigaudit report --framework gdpr --type ropa --format pdf
```

### Article 32 – Security of Processing

| Requirement | SigmaOS Control | File |
|------------|-----------------|------|
| Encryption at rest | AES-256-GCM for personal data | `src/security/vault.rs` |
| Encryption in transit | TLS 1.3 mandatory | `src/net/tls.rs` |
| Confidentiality | Pledge/unveil restrictions | `src/security/sigma_pledge.rs` |
| Integrity | File checksums | `src/filesystem/cow_snapshot.rs` |
| Availability | Self-healing + monitoring | `src/resilience/self_healing.rs` |
| Restore capability | CoW snapshots | `src/filesystem/cow_snapshot.rs` |
| Testing | Automated security tests | `tests/integration_test.rs` |

### Article 33/34 – Breach Notification

| Requirement | Implementation | File |
|------------|----------------|------|
| Detect breach within 72h | BreachDetector | `src/security/intrusion.rs` |
| Notify supervisory authority | Alert webhook | `src/integration/api.rs` |
| Notify data subjects | Email notification | `src/productivity/email.rs` |

---

## HIPAA Mapping

### Administrative Safeguards (§ 164.308)

| Standard | Implementation | File | Dashboard |
|---------|----------------|------|-----------|
| Security Officer | Role in governance config | `sigma-core.toml` | Admin panel |
| Workforce Training | Training completion log | `src/security/audit.rs` | Reports |
| Access Management | RBAC + capability tokens | `src/security/capability.rs` | Access panel |
| Incident Response | BreachDetector + alerts | `src/security/intrusion.rs` | Incidents |
| Contingency Plan | Backup + restore | `src/resilience/backup.rs` | Availability |
| Audit Controls | Audit log | `src/security/audit.rs` | Live events |
| Workforce Sanctions | Policy enforcement log | `src/security/audit.rs` | Reports |

### Physical Safeguards (§ 164.310)

| Standard | Implementation | File |
|---------|----------------|------|
| Facility Access | TPM attestation (device identity) | `src/tpm/mod.rs` |
| Workstation Use | Session lock + audit | `src/auth/access.rs` |
| Workstation Security | Secure boot | `src/boot/verified.rs` |
| Device Media Controls | Secure wipe on decommission | `src/security/cleaner.rs` |

### Technical Safeguards (§ 164.312)

| Standard | Implementation | File | Dashboard |
|---------|----------------|------|-----------|
| Access Control (§a) | RBAC + pledge | `src/security/sigma_pledge.rs` | Access panel |
| Unique User Identification | Per-user capabilities | `src/security/capability.rs` | Access panel |
| Automatic Logoff | Session timeout | `src/auth/access.rs` | Sessions |
| Encryption/Decryption | Vault AES-256-GCM | `src/security/vault.rs` | Security |
| Audit Controls (§b) | Audit log + tamper-evidence | `src/security/audit.rs` | Live events |
| Integrity Controls (§c) | File checksums | `src/filesystem/cow_snapshot.rs` | Integrity |
| Authentication (§d) | TPM + password + 2FA | `src/auth/access.rs` | Auth panel |
| Transmission Security (§e) | TLS 1.3 mandatory | `src/net/tls.rs` | Network |

### PHI Data Flow Map

```
PHI Tagged File ──► Kernel intercept ──► HIPAA audit record
                              │
                    PhiAccessCheck
                              │
                   Is purpose declared?  ──No──► EPERM + audit alert
                              │ Yes
                   Is user authorised?   ──No──► EPERM + audit alert
                              │ Yes
                   Allow access + log
```

---

## SOC 2 Type II Mapping

### Security (CC6 – Logical and Physical Access)

| Control | Implementation | File | Evidence |
|---------|----------------|------|----------|
| CC6.1 Access restrictions | RBAC + pledge | `src/security/capability.rs` | Quarterly access review |
| CC6.2 Account lifecycle | UserManager | `src/auth/access.rs` | Provisioning log |
| CC6.3 Remote access | VPN + TLS | `src/security/vpn.rs` | Connection log |
| CC6.6 Network segmentation | Firewall | `src/net/firewall.rs` | Config snapshot |
| CC6.7 Encryption | Vault | `src/security/vault.rs` | Key audit |
| CC6.8 Malware protection | Scanner | `src/security/scanner.rs` | Scan reports |

### Availability (A1)

| Control | Implementation | File | Dashboard |
|---------|----------------|------|-----------|
| A1.1 Capacity planning | Resource monitor | `src/resource/manager.rs` | Capacity panel |
| A1.2 Environmental threats | Self-healing | `src/resilience/self_healing.rs` | Availability |
| A1.3 Recovery | Backup + snapshot | `src/resilience/backup.rs` | Recovery |

### Processing Integrity (PI1)

| Control | Implementation | File |
|---------|----------------|------|
| PI1.1 Complete processing | Transaction log | `src/sigpkg/transaction.rs` |
| PI1.2 Accurate processing | Checksums | `src/filesystem/cow_snapshot.rs` |
| PI1.3 Authorised processing | Capability tokens | `src/security/capability_token.rs` |
| PI1.4 Timely processing | Performance monitor | `src/performance/profiler.rs` |

### Confidentiality (C1)

| Control | Implementation | File |
|---------|----------------|------|
| C1.1 Identify confidential info | Data classification tags | `src/legal/mod.rs` |
| C1.2 Protect confidential info | Vault + TLS | `src/security/vault.rs` |

### Privacy (P1–P8)

Maps 1:1 to GDPR articles (see GDPR mapping above).

---

## PCI-DSS v4.0 Mapping

### Requirement 1 – Network Security Controls

| Sub-req | Implementation | File |
|---------|----------------|------|
| 1.2 Network security controls | Firewall rules | `src/net/firewall.rs` |
| 1.3 Restrict inbound/outbound | CDE jail network | `src/virtualization/container.rs` |
| 1.4 Wireless networks | WiFi isolation | `src/network/wireless.rs` |

### Requirement 3 – Protect Stored Account Data

| Sub-req | Implementation | File |
|---------|----------------|------|
| 3.4 Render PAN unreadable | Tokenisation module | `src/security/vault.rs` |
| 3.5 Protect cryptographic keys | TPM key storage | `src/tpm/mod.rs` |

### Requirement 4 – Protect Cardholder Data in Transit

| Sub-req | Implementation | File |
|---------|----------------|------|
| 4.2 TLS for data in transit | TLS 1.3 mandatory | `src/net/tls.rs` |
| 4.2.1 Trusted certificates | PKI module | `src/security/pki.rs` |

### Requirement 6 – Secure Systems and Software

| Sub-req | Implementation | File |
|---------|----------------|------|
| 6.2 Vulnerability management | Scanner | `src/security/vulnerability.rs` |
| 6.3 Security vulnerabilities | CodeQL + Dependabot | `.github/workflows/` |
| 6.4 Web application protection | WAF (planned) | - |

### Requirement 8 – Identify Users and Authenticate Access

| Sub-req | Implementation | File |
|---------|----------------|------|
| 8.2 Unique IDs | Per-user capability tokens | `src/security/capability_token.rs` |
| 8.3 MFA | TPM + password + TOTP | `src/auth/access.rs` |
| 8.4 Password policies | Password strength enforcer | `src/security/password.rs` |

### Requirement 10 – Log and Monitor All Access

| Sub-req | Implementation | File |
|---------|----------------|------|
| 10.2 Audit logging | Audit log | `src/security/audit.rs` |
| 10.3 Protect audit logs | Tamper-evident journal | `src/security/audit.rs` |
| 10.4 Log review | Dashboard | `src/dashboard/monitor.rs` |

---

## India DPDP Act Mapping

### Chapter II – Grounds for Processing

| Section | Requirement | Implementation | File |
|---------|------------|----------------|------|
| § 4 | Lawful processing | ConsentStore | `tools/sigma_dpdp_compat.rs` |
| § 5 | Purpose limitation | Purpose declaration | `tools/sigma_dpdp_compat.rs` |
| § 6 | Notice to data principal | Automated notice | `tools/sigma_dpdp_compat.rs` |
| § 7 | Consent | ConsentRecord | `tools/sigma_dpdp_compat.rs` |
| § 8 | Legitimate use | LegitimateUseChecker | `tools/sigma_dpdp_compat.rs` |

### Chapter III – Rights and Duties

| Section | Right | Implementation | Dashboard |
|---------|-------|----------------|-----------|
| § 11 | Access to personal data | DataSubjectRequest | Data Subjects |
| § 12 | Correction and erasure | dpdp_erase / dpdp_correct | Data Subjects |
| § 13 | Grievance redressal | GrievanceQueue | Requests |
| § 14 | Nominee designation | NomineeRecord | Data Subjects |

### Chapter IV – Obligations of Data Fiduciaries

| Section | Obligation | Implementation | File |
|---------|-----------|----------------|------|
| § 17 | Localisation | Network geo-filter | `src/net/firewall.rs` |
| § 18 | Significant data fiduciary | SDF config | `sigma-core.toml` |
| § 19 | DPIA | Impact assessment module | `tools/sigma_dpdp_compat.rs` |
| § 20 | Data audits | DPDP audit report | `src/security/audit.rs` |

---

## Dashboard Indicators Reference

### Status Colours

| Colour | Meaning |
|--------|---------|
| 🟢 Green | Compliant – all controls passing |
| 🟡 Yellow | Warning – minor issue or nearing expiry |
| 🔴 Red | Non-compliant – immediate action required |
| ⚪ Grey | Framework not enabled |

### KPIs per Framework

| KPI | Description | Target |
|-----|-------------|--------|
| `gdpr.consent_coverage` | % of personal data with consent | 100% |
| `gdpr.retention_violations` | Data held past retention period | 0 |
| `hipaa.phi_unencrypted` | PHI files not encrypted | 0 |
| `hipaa.audit_gaps` | Hours without audit records | 0 |
| `soc2.access_reviews_overdue` | Reviews not done in 90 days | 0 |
| `pcidss.tls_version` | Minimum TLS version in use | TLS 1.3 |
| `dpdp.localisation_violations` | Data sent outside allowed regions | 0 |

---

## Automated Evidence Collection

```bash
# Collect all compliance evidence (runs monthly in CI)
./scripts/accelerators_diagnostics.sh --collect-evidence

# Output:
# evidence/gdpr/ropa-2026-08.pdf
# evidence/hipaa/security-assessment-2026-08.pdf
# evidence/soc2/cc6-evidence-2026-08.zip
# evidence/pcidss/saq-2026-08.pdf
# evidence/dpdp/processing-register-2026-08.pdf
```

---

*Last updated: 2026-08-04*

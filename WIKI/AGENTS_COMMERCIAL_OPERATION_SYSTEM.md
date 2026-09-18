# AI Agent Guidelines: Commercial Operating System Management in SigmaOS

## 📌 1. Executive Summary & Strategy

**SigmaOS** operates on an **Open-Core Commercial Ecosystem Model** inspired by enterprise operating systems (Red Hat Enterprise Linux, SUSE Linux Enterprise, Ubuntu Pro, and macOS Enterprise).

While the microkernel core, memory management, POSIX/Win32 translation layers, and basic package management remain **100% free, open-source, and unrestricted**, commercial enterprise capabilities are managed through modular, capability-gated enterprise extensions.

As an AI agent, your directive when implementing commercial subsystems is to maintain open-source core purity while building enterprise-grade reliability, statutory compliance, and certification infrastructure.

---

## 🏛️ 2. Commercial Subscription & Licensing Tiers

```
+-----------------------------------------------------------------------------------+
|                        SIGMAOS COMMERCIAL ARCHITECTURE                            |
+-----------------------------------------------------------------------------------+
|  🏢 Enterprise Sovereign Tier: PQC Vault, Air-Gapped SLA, Statutory Compliance    |
|  💼 Professional Tier: Multi-Device Sync, Tax/GST Engines, AI Assistant Pro        |
|  🐧 Open-Source Core: Microkernel, PMM/VMM, HAL, POSIX/Win32, SigmaPkg            |
+-----------------------------------------------------------------------------------+
```

### Tier Specifications & Feature Gates:

| Feature Dimension | 🆓 Community Tier | 💼 Professional Tier | 🏢 Enterprise Sovereign Tier |
| :--- | :--- | :--- | :--- |
| **Target Audience** | Individual Developers & Hobbyists | SMBs, Accountants, Freelancers | Enterprise, Defense, Finance, Gov |
| **Microkernel & Subsystems** | Full Access (`#![no_std]`) | Full Access | Full Access |
| **Support & SLA** | Community Forums & Wiki | 24/7 AI Code Assistant Pro | 99.999% SLA + Autonomous Sysadmin |
| **Compliance & Audit** | Basic Audit Logging | Tax & BOQ Estimators (`india_professional_tools`) | Statutory Governance & Penalty Breach Notifier |
| **Security & Cryptography** | Capability Tokens, Pledge/Unveil | Hardware Token Auth | Quantum-Resilient PQC Vault & Amnesic RAM |
| **Module Locations** | `src/kernel/`, `src/sigpkg/` | `src/compatibility/`, `src/productivity/` | `src/dashboard/statutory_compliance.rs`, `src/distro/certification.rs` |

---

## ⚙️ 3. Core Commercial Subsystems & Implementation

### 3.1 Statutory Governance & Penalty Breach Notifier
* **Module Location:** `src/dashboard/statutory_compliance.rs`
* **Functionality:** Audits operational compliance against international standards (GDPR, HIPAA, SOC2, PCI-DSS, GST/Tax regulations). Automatically generates dispute rollback audit chains and breach notifications.

### 3.2 Software Certification & Hardware Compliance Programs
* **Module Location:** `src/distro/certification.rs`
* **Functionality:** Implements the `SoftwareCertificationProgram` and `HardwareCertificationProgram` to issue verified compatibility signatures (`HardwareCertificate`, `AppManifest`). Staged QA release pipelines ensure high-reliability enterprise updates.

### 3.3 License Manifest & Feature Gate Verification
* **Manifest Standard:** Enterprise features check cryptographically signed `.sigmalicense` manifests containing Dilithium-5 signatures.
* **Gating Pattern:**
  ```rust
  pub fn check_enterprise_capability(feature: &str, license: &SignedLicense) -> Result<bool, CommercialError> {
      if !license.verify_pqc_signature() {
          return Err(CommercialError::InvalidLicense);
      }
      Ok(license.has_feature(feature))
  }
  ```

---

## 🛡️ 4. AI Agent Guidelines for Commercial Subsystems

1. **Open-Source Core Invariant:**
   * Never lock microkernel primitives, hardware drivers, memory allocators, or basic command-line utilities behind commercial flags.
2. **Graceful Degraded Mode:**
   * If an enterprise license expires or fails verification, the subsystem must fail open into Community Tier mode rather than crashing the operating system.
3. **Statutory Tamper-Evident Audit Trails:**
   * All billing, compliance, and license state modifications must append immutable entries to the `ChainedAuditTrailLedger`.

---

## 🧪 5. Standalone Testing Procedures

AI agents can verify statutory compliance, hardware certification, and commercial enterprise modules via standalone unit compilation:

```bash
# Test statutory compliance & breach notification layer
rustc --test --edition=2021 src/dashboard/statutory_compliance.rs -o build/compliance_tests && ./build/compliance_tests && rm build/compliance_tests

# Test hardware & software certification programs
rustc --test --edition=2021 src/distro/certification.rs -o build/cert_tests && ./build/cert_tests && rm build/cert_tests
```

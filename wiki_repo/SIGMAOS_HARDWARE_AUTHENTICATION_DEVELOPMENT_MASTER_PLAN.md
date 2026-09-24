# SigmaOS Hardware Authentication Development Master Plan: Linux & BSD Inspired Architecture

## Executive Summary

The **SigmaOS Hardware Authentication Subsystem** (`SovereignFirmitasAttestationEngine` in `src/security/pqc_measurement.rs`, `src/security/password.rs`, `src/drivers/kernel_releases.rs`, `src/init/emergency_gate.rs`) is engineered as a unified zero-trust authentication framework bridging Linux hardware security mechanisms (TPM 2.0 PCR sealing via `systemd-cryptenroll`, YubiKey HMAC-SHA1 challenge-response `pam_yubico`, FIDO2/WebAuthn CTAP2 USB/NFC security keys `pam_u2f`, Biometric PAM `pam_fprintd`) with BSD security key standards (FreeBSD `pam_fido2`, OpenBSD `bioctl` biometric tokens & FIDO2 `fido2-cred`).

This document defines the master development plan for the SigmaOS hardware authentication engine across architectural pillars, subsystem specifications, a 4-phase chronological development roadmap, and verification benchmark metrics.

---

## 1. Architectural Philosophy & Cross-Distro Inspirations

```
                          ┌──────────────────────────────────────────────────────────┐
                          │         SigmaOS Universal Hardware Authentication        │
                          └────────────────────────────┬─────────────────────────────┘
                                                       │
      ┌────────────────────────────────────────────────┼────────────────────────────────────────────────┐
      ▼                                                ▼                                                ▼
┌───────────────────────────┐            ┌───────────────────────────┐            ┌───────────────────────────┐
│       Linux Stack         │            │     FreeBSD & OpenBSD     │            │    Post-Quantum Firmware   │
│ • TPM 2.0 PCR Sealing     │            │ • FreeBSD pam_fido2 Key   │            │ • Dilithium-5 Attestation │
│ • FIDO2/CTAP2 pam_u2f     │            │ • OpenBSD bioctl Tokens   │            │ • PQC Enclave PCR Measurement│
│ • YubiKey pam_yubico      │            │ • FIDO2 fido2-cred Tools  │            │ • TPM2 NVRAM Panic Logging│
│ • Biometric pam_fprintd   │            │ • SmartCard PKCS#11 CCID  │            │ • Hardware Emergency Shell│
└───────────────────────────┘            └───────────────────────────┘            └───────────────────────────┘
```

---

## 2. Six Core Hardware Authentication Development Pillars

### Pillar 1: TPM 2.0 Cryptoprocessor PCR Sealing & Key Unlocking
* **TPM 2.0 PCR Measurement & Sealing (`Tpm2PcrBank` in `src/security/pqc_measurement.rs`)**:
  - Extend PCR registers (`PCR[0]` bootloader, `PCR[4]` kernel image, `PCR[7]` Secure Boot state).
  - Seal LUKS2/SigmaFS master volume keys to TPM 2.0 PCR banks, auto-unlocking root partitions during boot if boot integrity measurement matches verified baselines (`systemd-cryptenroll` model).

### Pillar 2: FIDO2 / CTAP2 USB & NFC Security Key PAM Integration (`pam_u2f`)
* **WebAuthn / FIDO2 CTAP2 Protocol Parser**:
  - Parse USB HID and NFC CTAP2 CBOR frames directly in userland PAM modules (`pam_u2f`).
  - Require physical user presence verification (touch button / PIN prompt) for sudo, SSH, and Zenith desktop login authentication.

### Pillar 3: YubiKey HMAC-SHA1 Challenge-Response Authentication (`pam_yubico`)
* **Slot-Based HMAC-SHA1 Challenge-Response**:
  - Issue 64-byte HMAC-SHA1 challenges to YubiKey Slot 2, validating secret responses against salt hashes stored in `/etc/yubico/`.

### Pillar 4: Biometric Sensor Protocol Engine (`fprintd` / IR Face Unlock)
* **Fingerprint & IR Face ID (`FingerprintAuth`, `FaceIdAuth` in `src/security/password.rs`)**:
  - Hardware-isolated fingerprint sensor image matching and IR camera facial feature vector verification.
  - Zero-memory-leak credential scrubbing upon session completion.

### Pillar 5: SmartCard / PIV (PKCS#11 / CCID) Certificate Authentication
* **ISO/IEC 7816 CCID SmartCard Driver**:
  - Interface with PIV/CAC government smartcards over USB CCID, validating X.509 client certificates against the system CA bundle via PKCS#11 modules.

### Pillar 6: Post-Quantum Hardware Token Signature Verification (`Dilithium-5`)
* **Post-Quantum Firmness Attestation (`SovereignFirmitasAttestationEngine`)**:
  - Verify post-quantum Dilithium-5 hardware token signatures for high-security administrator elevation and emergency shell access (`src/init/emergency_gate.rs`).

---

## 3. Four-Phase Chronological Roadmap

```
  Phase 1: TPM 2.0 PCR Sealing & FIDO2 CTAP2 Parser (Months 1–3)
  ├── TPM 2.0 PCR Sealing & Auto-Unlock Root Volume Pipeline
  ├── USB HID / NFC CTAP2 Protocol CBOR Frame Parser
  └── `pam_u2f` Security Key User Presence Verification

  Phase 2: YubiKey Challenge-Response & PAM Hardware Module (Months 3–6)
  ├── YubiKey Slot 2 HMAC-SHA1 Challenge-Response Driver
  ├── PAM Hardware Integration Module (`/etc/pam.d/system-auth`)
  └── Fallback Recovery PIN Verification with Rate Limiting

  Phase 3: Biometric Sensor Protocol & SmartCard PIV Integration (Months 6–9)
  ├── `fprintd` Fingerprint Sensor & IR Face ID Engine
  ├── USB CCID SmartCard / PIV (ISO/IEC 7816) PKCS#11 Driver
  └── Hardware Token Disconnect Watchdog Session Locker

  Phase 4: PQC Token Signing & Unified Zenith Hardware Login UI (Months 9–12)
  ├── Post-Quantum Dilithium-5 Hardware Token Signature Engine
  ├── Zenith Wayland Compositor Hardware Touch/PIN Graphic Prompts
  └── Sub-200ms Hardware Auth Benchmark & Hardened Lockout Test Suite
```

---

## 4. Verification and Benchmark Metrics

| Subsystem Target | Benchmark Framework | Target Performance Metric |
| :--- | :--- | :--- |
| **TPM 2.0 Key Unsealing** | Boot Bench Test | < 50ms PCR measurement check and master key release |
| **FIDO2 CTAP2 Auth Latency** | WebAuthn Response Test | < 150ms touch-to-session authentication response |
| **YubiKey Challenge-Response** | HMAC-SHA1 Timing Test | < 100ms challenge challenge-response cycle |
| **Hardware Disconnect Safety** | Hotplug Token Removal Test | Instant < 10ms session lock upon hardware key removal |

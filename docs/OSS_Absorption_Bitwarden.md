# 🧩 Cleanroom Absorption: Bitwarden Credentials Vault

SigmaOS features a native credentials manager, **SigmaVault**, which inherits zero-knowledge security principles from Bitwarden while introducing hardware-level compartmentalization.

---

## 🎯 Target Architecture: Bitwarden

Bitwarden provides end-to-end AES-256 bit encryption of secrets, cross-device secure synchronization, open-source auditing, and biometric authentication unlocks.

### Gaps in Legacy Bitwarden:
- Vulnerable to RAM-scraping attacks where master keys can be recovered from user-space memory dumps.
- Relies on web-based or heavy electron clients.

---

## 🔑 SigmaOS Sovereign Features

### 1. Active RAM scrubbing
- Leverages our `SecureCleaner` module to overwrite and purge decrypted credentials immediately from memory registers, mitigating advanced forensic memory-scraping attacks.

### 2. Hardware Enveloping
- Secrets are bound directly to TPM 2.0 modules with Kyber-1024 wrapping, ensuring keys cannot be extracted even with physical host access.

### 3. Native CLI & API Integration
- Embedded cleanly into `sigma-sh` and capability tokens, so programs can request isolated passwords securely without leaking the master database.

---

## 📊 Absorption Matrix

| Capability | Bitwarden | SigmaVault |
|------------|-----------|------------|
| End-to-End Encryption | ✅ | ✅ |
| Biometric Unlock | ✅ | ✅ Native FaceID/Fingerprint |
| RAM Scraping Protection | ❌ | ✅ Secure Memory Overwrite |
| TPM 2.0 Hardware Bind | ❌ | ✅ Kyber-1024 KEM |
| Zero-knowledge Sync | ✅ | ✅ |
| Footprint | Heavy (Electron) | Zero-overhead Rust binary |

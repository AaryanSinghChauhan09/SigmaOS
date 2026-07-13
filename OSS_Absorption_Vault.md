# OSS Absorption: Vault (HashiCorp) — Secrets Management

> **Status**: 📋 Planned | **Source Project**: HashiCorp Vault | **Target Shard**: `SigmaOS Sovereign Secrets Layer`

---

## 1. Executive Summary

HashiCorp Vault is an identity-based secrets and encryption management system. It provides secure, audited, short-lived credentials for databases, cloud providers, and custom applications.

SigmaOS absorbs the **dynamic secrets** and **seal/unseal TPM model** of Vault, embedding them into the `sigma-vault` subsystem for cryptographic key management and short-lived credential injection.

---

## 2. Key Features Absorbed

### 2.1 Dynamic Credential Injection

Instead of hard-coding database passwords in application configuration files, `sigma-vault` generates unique, short-lived credentials on demand for each requesting application, tied to its capability token identity.

```bash
$ sigma-vault secrets read db/app-backend
Σ [VAULT] Generating dynamic credentials for app-backend:
  Username: v-sigma-app-backend-7d3a
  Password: ******* (expires in 1h)
```

### 2.2 TPM-Sealed Master Key

The `sigma-vault` master encryption key is sealed to the machine's Trusted Platform Module (TPM). The system cannot unseal the secrets store unless it boots with the exact expected PCR measurement chain (i.e., an unmodified trusted boot chain).

---

## 3. References & Standards

- HashiCorp Vault — `vaultproject.io` (MPL-2.0 / BSL-1.1)

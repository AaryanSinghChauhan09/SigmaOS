# sigma-vault — Secrets Manager Specification

**Status:** Draft · Target: v0.2
**Owner:** security team
**Canonical source:** `userland/sigma-vault/`

---

## Overview

sigma-vault is the centralised secrets manager for SigmaOS. It stores secrets in an AES-256-GCM encrypted on-disk store, seals the master key to the TPM2 chip, exposes a PKCS#11 interface for TLS consumers, and gates all access via sigma_pledge capabilities and a mandatory audit log.

## Goals

- Master key never stored in cleartext on disk — sealed to TPM2 PCR state

- AES-256-GCM with unique 96-bit IV per encryption operation

- Audit log: every access (read, write, delete, rotate) recorded with process identity

- PKCS#11 interface: drop-in for OpenSSL engines and TLS stacks

- Zero in-memory secret retention after use (mlock + explicit_bzero)

- Daemon model: vaultd runs at Ring-3 with minimal pledge; clients use IPC

---

## Architecture

```
┌───────────────────────────────────────────┐
│  Client process (sigma_pledge includes     │
│  "vault" capability token)                │
│  sigma-vault CLI  OR  PKCS#11 dlopen()    │
└────────────────┬──────────────────────────┘
                 │ IPC (sigma-bus UNIX socket)
                 ▼
┌───────────────────────────────────────────┐
│  vaultd (Ring-3, pledge: stdio rpath      │
│  wpath cpath tpm2 audit)                  │
│  ┌──────────────┐  ┌────────────────────┐ │
│  │ Key Unsealing│  │  Secret Store      │ │
│  │ TPM2 PCR seal│  │  AES-256-GCM       │ │
│  │ /dev/tpm0    │  │  ~/.sigma-vault.db │ │
│  └──────────────┘  └────────────────────┘ │
│  ┌────────────────────────────────────────┐│
│  │ Audit Logger → /var/log/vault.audit    ││
│  └────────────────────────────────────────┘│
└───────────────────────────────────────────┘
```

---

## TPM2 Key Sealing

1. At vault init: generate 256-bit master key in TPM2 (`TPM2_Create` under parent key)

2. Seal to PCR set: PCR0 (firmware) + PCR7 (Secure Boot state) + PCR11 (OS state)

3. Sealed blob stored at `/etc/sigma-vault/master.seal`

4. On vaultd startup: `TPM2_Unseal` — fails if PCR values changed (firmware tampered)

5. Master key loaded into mlock'd memory region; used to derive per-secret keys via HKDF-SHA3-256

6. Master key zeroed from memory immediately after deriving all session keys

---

## Encrypted Store Format

File: `~/.sigma-vault.db` (or `/etc/sigma-vault/system.db` for system secrets)

```
Header (64 bytes):
  magic[8]     = "SIGVAULT"
  version[2]   = 0x0001
  kdf_algo[2]  = 0x0001 (HKDF-SHA3-256)
  reserved[52]

Entry (per secret):
  name_len[2] + name[name_len]   (UTF-8, max 255 bytes)
  iv[12]                         (random 96-bit IV)
  tag[16]                        (GCM auth tag)
  ciphertext_len[4] + ciphertext (AES-256-GCM encrypted value)
  created_at[8]                  (Unix timestamp)
  rotated_at[8]                  (Unix timestamp, 0 if never)
```

Entries appended; compaction on `vault compact` command.

---

## CLI

```
sigma-vault get    <name>              # print plaintext to stdout

sigma-vault set    <name> [value]      # value from arg or stdin (interactive)

sigma-vault delete <name>
sigma-vault list   [--names-only]
sigma-vault rotate <name>             # re-encrypt with new IV

sigma-vault init   [--tpm2 | --passphrase]   # initialise new store

sigma-vault export <name> --format=env  # emit as KEY=VALUE for shell sourcing

```

---

## PKCS#11 Interface

Library: `libsigma-vault-pkcs11.so` — implements PKCS#11 v3.0 subset:

- `C_Initialize`, `C_Finalize`

- `C_OpenSession`, `C_CloseSession`

- `C_Login` (PIN = capability token)

- `C_Sign`, `C_Verify` (Dilithium-5 + ECDSA P-256)

- `C_Encrypt`, `C_Decrypt` (AES-256-GCM)

- `C_GenerateKeyPair`, `C_DestroyObject`

Used by: sigma-sh TLS, sigma-net TLS 1.3 handshake, sigma-pkg signature verification.

---

## Capability Gating

Processes must hold `"vault"` in their sigma_pledge capability set to open IPC connection to vaultd. Privilege escalation rejected with `EPERM` + audit entry.

Audit log entry format (JSON Lines):
```json
{"ts":1700000000,"pid":412,"comm":"sigma-sh","op":"get","name":"db_password","result":"ok"}
```

---

## Implementation Plan

- [ ] 1. TPM2 seal/unseal wrapper (`security/tpm2_vault.c`)

- [ ] 2. AES-256-GCM encrypt/decrypt (`crypto/aes_gcm.c`)

- [ ] 3. HKDF-SHA3-256 key derivation (`crypto/hkdf.c`)

- [ ] 4. On-disk store reader/writer (`src/store.c`)

- [ ] 5. vaultd daemon + IPC socket listener

- [ ] 6. sigma_pledge self-restriction for vaultd

- [ ] 7. CLI commands: get/set/delete/list/rotate/init

- [ ] 8. Audit logger (append-only, fsync after each entry)

- [ ] 9. PKCS#11 shared library skeleton

- [ ] 10. mlock + explicit_bzero for all in-memory secrets

- [ ] 11. Tests: encrypt/decrypt round-trip, TPM2 mock seal, audit log entries

---

## Status

| Feature | State |
|---------|-------|
| TPM2 seal/unseal | ⬜ Not started |
| AES-256-GCM store | ⬜ Not started |
| vaultd daemon | ⬜ Not started |
| CLI | ⬜ Not started |
| PKCS#11 library | ⬜ Not started |
| Audit log | ⬜ Not started |

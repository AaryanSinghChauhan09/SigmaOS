# Security Subsystems

> SigmaOS v15.0 "Zenith" — Security Reference

## Overview

SigmaOS is designed with a security-first architecture. Defense-in-depth is achieved through layered controls at every level: hardware, kernel, system, and application.

---

## Layer 1 — Hardware Security

### UEFI Secure Boot (`kernel/security/sigma_secboot.rs`)

| Feature | Details |
|---|---|
| DB validation | Verifies image hash against UEFI Allowed Database |
| DBX blocklist | Rejects images matching revoked hashes/signers |
| MOK support | Machine Owner Keys for user-managed signing |
| TPM 2.0 | Measures boot images into PCRs 4 & 8 |
| Audit Mode | Logs but does not block; for gradual rollout |

Secure Boot state is queried at boot and stored globally. All executable images loaded after boot services exit are validated via `secboot_verify_image()`.

---

## Layer 2 — Memory Protection

- **W^X Enforcement**: Pages are either writable OR executable, never both. Enforced by the VMM (`sigma_vmm.rs`) at the page-table level.
- **ASLR**: Code, heap, stack, and mmap regions are randomized at process creation.
- **Stack Canaries**: All kernel stacks include a guard page.
- **SMEP/SMAP**: Supervisor Mode Execution/Access Prevention enabled at boot.

---

## Layer 3 — Kernel Security

### Mandatory Access Control (`kernel/security/sigma_mac.rs`)

Implements a label-based MAC policy (inspired by SELinux/AppArmor) enforced at all VFS and IPC call sites.

### Syscall Filtering (`kernel/core/sigma_syscall_dispatch.rs`)

Two mechanisms are layered:

1. **Pledge** — Process declares upfront which syscall categories it will use. Violations result in `SIGKILL`.
2. **Seccomp-BPF** — Dynamic BPF filter program allows/rejects individual syscalls and argument ranges.

### IDS — Intrusion Detection System (`kernel/security/sigma_ids.rs`)

- Suricata-compatible rule format loaded from `/etc/sigma/ids/rules/`
- Hooks into the network stack before packets reach userland sockets
- Generates alerts to `sigma_journal` on suspicious patterns

### Fail2Ban (`kernel/security/sigma_fail2ban.rs`)

- Tracks per-IP authentication failures in a kernel-resident table
- Automatically adds block entries to the network filter (nftables-compatible)
- Configurable threshold: default 5 failures → 10-minute ban

---

## Layer 4 — Secrets Management

### Sigma Vault (`kernel/security/sigma_vault.rs`)

A Vault-compatible secrets engine built into the kernel:

| Feature | Details |
|---|---|
| KVv2 | Versioned key-value secret store |
| Transit | Encryption-as-a-service (AES-256-GCM) |
| Key Derivation | Argon2id (time=3, memory=64MB, parallelism=4) |
| Signing | Dilithium-5 post-quantum signatures |
| Access Policy | Capability-token based access |

```rust
// Example usage
let token = vault_authenticate("service_account");
let secret = vault_kv_get(token, "db/credentials");
let ciphertext = vault_transit_encrypt(token, "app-key", plaintext);
```

---

## Security Checklist for Developers

- [ ] Use `pledge()` syscall at application init to declare required capabilities
- [ ] Never `mmap` pages as `PROT_WRITE | PROT_EXEC` simultaneously
- [ ] Store secrets only via Sigma Vault, never in plain files
- [ ] Validate all user-space pointer arguments in syscall handlers
- [ ] Register service unit with mandatory MAC label in `.service` file

---

## CVE Response Process

Security reports: `security@sigmaos.dev`

Response SLA:
- Critical (CVSS ≥ 9.0): 24-hour acknowledgment, 7-day patch
- High (7.0–8.9): 72-hour acknowledgment, 30-day patch
- Medium/Low: Next scheduled release

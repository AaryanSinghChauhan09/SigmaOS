# SigmaOS MicroVM Compartmentalization

> QubesOS-inspired domain isolation with Firecracker MicroVMs.

## Overview

SigmaOS implements **hardware-enforced security domains** using lightweight MicroVMs. Each domain (Vault, Personal, Work, Untrusted) runs in an isolated virtual machine with strictly controlled inter-domain communication.

## Domain Trust Hierarchy

```
┌─────────────────────────────────┐
│          sigma-vault            │  ← Highest trust, air-gapped
│   Encrypted secrets, keys      │     No network, no USB
├─────────────────────────────────┤
│        sigma-personal           │  ← High trust
│   Banking, health records       │     Network OK, clipboard OK
├─────────────────────────────────┤
│          sigma-work             │  ← Standard trust
│   Daily work, browsing          │     Network OK, clipboard OK
├─────────────────────────────────┤
│        sigma-untrusted          │  ← Lowest trust
│   Downloads, unknown apps       │     No network, no clipboard
└─────────────────────────────────┘
```

## Default Domains

| Domain           | Trust     | Network | USB | Clipboard | RAM    | vCPUs |
|------------------|-----------|---------|-----|-----------|--------|-------|
| sigma-vault      | Vault     | ❌      | ❌  | ❌        | 512 MB | 1     |
| sigma-personal   | Trusted   | ✅      | ❌  | ✅        | 2 GB   | 2     |
| sigma-work       | Standard  | ✅      | ❌  | ✅        | 4 GB   | 4     |
| sigma-untrusted  | Untrusted | ❌      | ❌  | ❌        | 1 GB   | 1     |

## Inter-Domain Transfer Policy

Data can only flow **downward** (from less trusted to more trusted contexts):

- **Vault** → Never exports data
- **Untrusted** → Can only share with other Untrusted domains
- **Standard** → Can share with Standard and Trusted
- **Trusted** → Can share with Trusted only

## Implementation

- **Source**: `security/sigma_microvm_isolation.rs`
- **VMM Backend**: Firecracker (Amazon's microVM hypervisor)
- **Key APIs**:
  - `boot_domain_vm(domain)` — launch an isolated MicroVM
  - `is_transfer_allowed(from, to)` — enforce transfer policy
  - `list_active_domains()` — enumerate running domains

## Integration Points

| Component        | Integration                                    |
|------------------|------------------------------------------------|
| Zenith Desktop   | Color-coded window borders per domain          |
| File Manager     | Domain-aware file picker with transfer prompts |
| Clipboard        | Filtered through domain policy engine          |
| Network Stack    | Per-domain network namespace isolation         |

# Distro Absorption: RHEL (Red Hat Enterprise Linux)

## Overview

RHEL is the industry standard for enterprise Linux deployments, prized for its 10-year support lifecycle, strict ABI stability guarantees, and certified hardware/software ecosystem.

## Key Principles Absorbed

### Long-Term Support & ABI Stability

- SigmaOS commits to a stable kernel ABI across major releases.
- Internal crate versioning follows semantic versioning strictly.
- The `sigpkg` transaction manager ensures deterministic rollback, matching RHEL's `rpm-ostree` model.

### Enterprise Security (SELinux)

- RHEL's SELinux Mandatory Access Control has been absorbed into `sigma_security`.
- `SigmaContext` (`user:role:type:level`) replaces SELinux's text-based policy language with native Rust types.
- Context transition rules are enforced at the `ProfileSystem` level.

### Subscription & Entitlement Model

- SigmaOS replaces RHEL's subscription-manager with a sovereign attestation model.
- Device identity is established via TPM-backed keys rather than external entitlement servers.

### Content Delivery (OSTree / rpm-ostree)

- RHEL's image-based deployment model (Fedora CoreOS, Silverblue) is absorbed via `ContentAddressedStorage` in `sigpkg`.
- Atomic deployments and rollbacks are native to the package manager.

## Displaced Technologies

| RHEL Component | SigmaOS Replacement |
| --- | --- |
| SELinux | `sigma_security::SigmaContext` |
| rpm-ostree | `sigpkg::ContentAddressedStorage` |
| subscription-manager | Sovereign TPM attestation |
| systemd | `sigma_init` (planned) |
| yum/dnf | `sigpkg` declarative resolver |

## Status

**Core Absorbed** — SELinux context model and OSTree deployment model are implemented. DNF compatibility layer exists in `tools/sigma_dnf_compat.rs`.

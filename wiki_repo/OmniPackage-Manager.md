# 📦 OmniPackage Manager (OmniPkg)

> **"Abolish dependency hell. Every package runs in its own sovereign container, cryptographically attested from install to execution."**

OmniPkg is SigmaOS's answer to APT, Nix, and Flatpak — built from scratch to match our zero-trust, zero-dependency kernel philosophy.

---

## 🆚 Comparison with Linux Package Managers

| Feature | APT (Debian) | Nix | OmniPkg (SigmaOS) |
|:--|:--|:--|:--|
| Isolation | None (global `/usr`) | Nix store (shared) | **Per-package sandbox container** |
| Reproducibility | No | Yes (hash-based) | **Yes (Dilithium-5 signed manifests)** |
| Rollback | No | Yes (generations) | **Yes (snapshot-integrated)** |
| Supply-chain trust | GPG key | Hash closure | **TPM-attested + PQC-signed** |
| Conflict resolution | libX vs libY hell | Functional isolation | **Zero conflict (no shared globals)** |

---

## 1. Package Format: `.spk` (Sovereign Package)

An `.spk` file is a self-contained, statically-linked archive:
```
my_app.spk
├── manifest.sigma     # Declarative metadata (name, version, curation level)
├── binary             # Statically compiled native binary (no libc dependency)
├── signature.dilithium  # Post-quantum Dilithium-5 signature
└── sandbox.policy     # SovereignSandbox container policy (IPC, net, fs rules)
```

---

## 2. Curation Levels

Every package on OmniPkg carries an enforced curation level:

| Level | Description | Sandbox Policy |
|:--|:--|:--|
| `CURATION_OFFICIAL` | Built and signed by the SigmaOS team | Full access (TPM boot required) |
| `CURATION_COMMUNITY` | Community-contributed, code-reviewed | POSIX Compat Shim + restricted net |
| `CURATION_UNVERIFIED` | Third-party / untrusted source | Full airgap — no net, no IPC |

---

## 3. POSIX Compatibility Shim (OmniPkg for Linux Binaries)

SigmaOS bridges the software availability gap via `SovereignCompatShim.cpp`:

```
Linux Binary
     │
     ▼
CompatShim (intercepts sys_read, sys_write, sys_mmap, etc.)
     │
     ▼
SigmaOS Syscall Dispatcher (256-slot sovereign table)
     │
     ▼
Hardware
```

- Unsupported syscalls **abort hard** (zero-trust posture).
- All COMMUNITY-level Linux binaries are automatically wrapped in the shim.
- Shim execution is **logged and auditable** via `sigma_compliance_cli`.

---

## 4. Installation Flow

```bash
omnikg install nginx          # Fetches + verifies signature
omnikg install ./my_app.spk   # Install local package
omnikg list                   # List installed packages + curation level
omnikg rollback nginx         # Atomic rollback via Emergency Lattice Sync
omnikg audit nginx            # Print compliance attestation for package
```

---

## 5. Transactional Installs & Rollback

OmniPkg never writes to a global `/usr`. Each install creates an isolated path:
```
/sigma/pkgs/[AppName]/[version]/
```
Rolling back is atomic — OmniPkg calls `recovery_atomic_sync()` before any install and restores the previous state on failure.

---

## 6. Roadmap

| Phase | Milestone |
|:--|:--|
| **Q1** | Native `.spk` packaging spec + signing toolchain |
| **Q2** | Official repository (50+ core packages) |
| **Q3** | POSIX compat shim for 200+ common Linux binaries |
| **Q4** | Community submission portal + bounty program |

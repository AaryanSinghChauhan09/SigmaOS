# SigmaOS Security Policy

> Security is a foundational commitment of SigmaOS, not an afterthought. This document defines the threat model, secure defaults, privilege model, update signing, CVE process, and responsible disclosure policy.

---

## Table of Contents

1. [Threat Model](#threat-model)
2. [Security Design Principles](#security-design-principles)
3. [Secure Defaults](#secure-defaults)
4. [Privilege Model](#privilege-model)
5. [Update Signing & Integrity](#update-signing--integrity)
6. [Boot Integrity](#boot-integrity)
7. [Process Isolation & Sandboxing](#process-isolation--sandboxing)
8. [Network Security](#network-security)
9. [Cryptography Policy](#cryptography-policy)
10. [CVE Response Process](#cve-response-process)
11. [Responsible Disclosure](#responsible-disclosure)
12. [Security Audit Policy](#security-audit-policy)

---

## Threat Model

SigmaOS's security is designed against the following threat actors and attack vectors:

### Threat Actors

| Actor | Capability | Goal |
|-------|-----------|------|
| **Opportunistic attacker** | Automated exploit scanners, off-the-shelf tools | Mass exploitation of known vulnerabilities |
| **Targeted attacker** | Custom exploits, social engineering | Compromise specific user's data or system |
| **Malicious app** | App-level code execution | Escape sandbox, access sensitive data |
| **Supply chain attacker** | Compromise upstream packages or mirrors | Deliver malicious code to users via "trusted" updates |
| **Physical attacker** | Physical device access | Access data on unattended or stolen device |
| **Network attacker** | MITM, network interception | Intercept update traffic, serve malicious packages |

### Out of Scope

The following are explicitly out of scope for SigmaOS's threat model:

- Nation-state attackers with kernel-level 0-days against up-to-date systems
- Hardware implants or supply chain compromise of physical components
- Attacks requiring prolonged unsupervised physical access (full disk encryption handles this)
- Social engineering of the user (beyond user education)

---

## Security Design Principles

SigmaOS security is built on these core principles:

1. **Defense in Depth**: Multiple independent security layers. Compromise one and you still face others.
2. **Least Privilege**: Every process has only the minimum capabilities required.
3. **Fail-Safe Defaults**: The secure option is always the default. Insecure options require explicit user action.
4. **Open Design**: Security mechanisms are publicly documented and auditable.
5. **Separation of Privilege**: High-privilege operations require multiple conditions (key + password, not just one).
6. **Least Common Mechanism**: Minimize shared state between security domains.
7. **Psychological Acceptability**: Security mechanisms that are too inconvenient will be bypassed.

---

## Secure Defaults

The following are enabled by default on every SigmaOS installation:

### System

| Default | Setting | Notes |
|---------|---------|-------|
| Firewall | Enabled, block all inbound | User can add exceptions |
| Full disk encryption | Offered in installer | Off by default, but prominently recommended |
| Automatic security updates | Enabled | Security channel updates without user action |
| DNS-over-HTTPS | Enabled | Via sigma-net, configurable |
| SSH daemon | Disabled | Must be explicitly enabled by user |
| Root account | Locked | `sudo` required for privilege escalation |
| Sudo timeout | 15 minutes | Credential cached for 15 minutes only |
| Core dumps | Disabled | Enabled by developer mode flag |
| Unprivileged user namespaces | Restricted | Only Flatpak/Bubblewrap paths allowed |
| Kernel lockdown | Integrity mode | Prevents unsigned kernel modules |

### Network

| Default | Setting |
|---------|---------|
| IPv6 privacy extensions | Enabled |
| mDNS/Avahi | Disabled (no network service discovery) |
| NFS client | Disabled |
| SMB/CIFS client | Disabled by default (install via sigma-store) |
| Bluetooth | Disabled until user enables |

### Desktop

| Default | Setting |
|---------|---------|
| Screen lock | After 5 minutes idle |
| Screen blank | After 3 minutes idle |
| Lock on suspend | Enabled |
| HTTPS-only mode (browser) | Enabled |
| Clipboard access for apps | Restricted (portal required) |

---

## Privilege Model

SigmaOS uses a **capability-based privilege model** with no SUID binaries (with specific documented exceptions).

### Privilege Levels

```
Root (uid=0)         — System init, package management only
System services      — Dedicated service accounts (uid 100-499)
                       e.g., sigma-audio (uid 105), sigma-net (uid 106)
Session services     — Run as user, in user session
                       e.g., sigma-shell, sigma-panel
User                 — Normal user account (uid 1000+)
```

### Capability Grants

System services drop all capabilities at startup, then add back only what they need:

| Service | Capabilities Retained | Reason |
|---------|-----------------------|--------|
| sigma-net | CAP_NET_ADMIN, CAP_NET_RAW | Network configuration |
| sigma-display | CAP_SYS_NICE | DRM/KMS priority |
| sigma-audio | none | All via user session |
| sigma-pkg | CAP_FSETID, CAP_CHOWN | Package installation |
| sigma-init | ALL (PID 1) | System management |

### Privilege Escalation

No `sudo` for desktop users — only **polkit** for privilege escalation in GUI workflows:
- Installing packages: polkit dialog
- Changing network settings: polkit dialog
- Hardware configuration: polkit dialog
- Shell `sudo` still available for developer/power users

---

## Update Signing & Integrity

Every update artifact is signed before delivery and verified before installation.

### Signature Chain

```
1. Source code commit → signed with developer GPG key
2. Build system → reproducible build in isolated environment
3. Package → signed with sigma-pkg-signing-key (Ed25519)
4. Package metadata → signed with sigma-repo-key (Ed25519)
5. ISO → signed with sigma-release-key (Ed25519)
6. All signing events → logged to Sigstore Rekor transparency log
```

### Verification on Client

`sigma-update` verifies:
1. Repository metadata signature (trust-on-first-use + pinned key)
2. Package manifest signature
3. Individual package signature
4. Package content hash (SHA256)
5. Package version ordering (prevents downgrade attacks)

If any verification step fails, the update is rejected and the user is notified. The system remains in its current state.

### Key Pinning

SigmaOS uses **TOFU (Trust On First Use)** for the initial install, then **key pinning**:
- On first boot, the current release key fingerprint is recorded in `/etc/sigma/trusted-keys`
- Subsequent updates must be signed by a trusted key
- Key rotation requires a signed key rotation manifest (new key signed by old key)

---

## Boot Integrity

### Secure Boot

SigmaOS implements a complete Secure Boot chain:

```
UEFI Firmware
└── SigmaOS shim (signed by Microsoft for hardware compatibility)
    └── GRUB2 (signed by SigmaOS Release Key)
        └── Linux kernel (signed by SigmaOS Kernel Key)
            └── initrd (integrity checked by kernel)
                └── Root filesystem (verified by dm-verity)
```

### dm-verity

The root filesystem is protected by `dm-verity`:
- A Merkle tree hash of the entire root partition is computed at release time
- The hash root is embedded in the kernel or initrd
- Any modification to the root filesystem is detected at mount time
- Detected tampering causes an emergency boot failure (not silent corruption)

**Note:** dm-verity applies to the read-only system partition. The `/home` and user data partitions are writable and are not dm-verity protected.

### TPM2 Integration (Optional)

For systems with TPM2:
- Measured boot: PCR values reflect the boot chain state
- System disk encryption key is sealed to TPM2 PCRs 0-7 (firmware, SHIM, bootloader, kernel)
- Key is automatically released only if the boot chain is unmodified
- If boot chain changes (kernel update, GRUB update): re-enrollment required

---

## Process Isolation & Sandboxing

### Flatpak Applications

All GUI apps in sigma-store run via Flatpak:
- Separate mount namespace (read-only system view)
- Separate PID and IPC namespaces
- Network access via portal (user-controlled)
- Filesystem access via portal (user-controlled)
- Webcam/microphone via portal (per-access user prompt)

### System Daemons

System daemons are sandboxed via systemd unit hardening:

```ini
[Service]
# Typical system daemon security settings
NoNewPrivileges=yes
PrivateTmp=yes
ProtectSystem=strict
ProtectHome=yes
ReadWritePaths=/var/lib/sigma-<component>
CapabilityBoundingSet=<minimal set>
SystemCallFilter=@system-service
RestrictNamespaces=yes
RestrictRealtime=yes
LockPersonality=yes
MemoryDenyWriteExecute=yes
```

### seccomp-bpf Filtering

All system processes have seccomp-bpf allowlists:

| Process Class | Syscall Allowlist |
|---------------|------------------|
| GUI app | ~180 syscalls (no network admin, no module load) |
| System daemon | ~120 syscalls (role-specific) |
| Kernel module helper | Minimal (only what the module needs) |

---

## Network Security

### Default Firewall (nftables)

```
# Default inbound policy: DROP all
# Default outbound policy: ACCEPT all
# Established connections: ACCEPT

Allowed inbound (loopback): all
Allowed inbound (external): none by default

User exceptions (configured via sigma-settings):
- SSH (disabled by default, opt-in)
- Custom ports (user-configured)
```

### DNS Security

- DNS-over-HTTPS enabled by default (using Cloudflare or configurable)
- DNSSEC validation enabled
- DNS queries do not leak to local network (all go through DoH)
- Local hostname resolution uses mDNS only when enabled

### TLS Configuration

- TLS 1.3 preferred, TLS 1.2 minimum — TLS 1.0/1.1 disabled
- Certificate validation is strict — no user bypass for invalid certs in system tools
- Certificate pinning for SigmaOS update infrastructure

---

## Cryptography Policy

SigmaOS uses modern cryptographic primitives:

| Use Case | Algorithm | Notes |
|----------|-----------|-------|
| Package signing | Ed25519 | Fast, small signatures |
| ISO signing | Ed25519 | |
| TLS | TLS 1.3 with X25519 + AES-256-GCM | |
| Disk encryption | AES-256-XTS (LUKS2) | |
| Password hashing | Argon2id | |
| Key derivation | HKDF-SHA256 | |
| Random numbers | `/dev/urandom` (CSPRNG) | Never /dev/random |
| SSH keys | Ed25519 | RSA keys deprecated |

**Forbidden:** MD5, SHA-1, DES, 3DES, RC4, RSA < 2048 bits, TLS < 1.2

---

## CVE Response Process

### Severity Classification

SigmaOS uses CVSS v3.1 for CVE scoring:

| CVSS Score | Severity | Response SLA |
|------------|----------|-------------|
| 9.0 - 10.0 | Critical | 24 hours |
| 7.0 - 8.9 | High | 72 hours |
| 4.0 - 6.9 | Medium | 7 days |
| 0.1 - 3.9 | Low | Next release |

### CVE Workflow

```
1. CVE reported (internally or via disclosure)
2. Acknowledge within 24 hours
3. Assess impact on SigmaOS
4. Develop fix (coordinated with upstream if applicable)
5. Test fix in staging environment
6. Build and sign hotfix packages
7. Publish to security update channel
8. Publish security advisory
9. Request CVE assignment (if original vulnerability)
```

### Security Advisory Format

Security advisories are published to:
- GitHub Security Advisories
- `https://sigmaos.dev/security`
- OSV (Open Source Vulnerability) database

Advisory format:
```markdown
## SigmaOS Security Advisory SIGMA-YYYY-NNNN

**CVE:** CVE-YYYY-NNNNN
**Severity:** Critical/High/Medium/Low (CVSS N.N)
**Affected versions:** < vX.Y
**Fixed in:** vX.Y+1 / hotfix vX.Y.Z

### Description
[What the vulnerability is]

### Impact
[What an attacker can do]

### Mitigation
[Workaround if patch not yet available]

### Fix
[What was changed]

### Credits
[Reporter, if consented to disclosure]
```

---

## Responsible Disclosure

SigmaOS follows a **coordinated vulnerability disclosure** policy.

### How to Report

**Email:** `security@sigmaos.dev`
**PGP Key:** Published at `https://sigmaos.dev/security/pgp-key.asc`

Use PGP encryption for all vulnerability reports. Reports sent without encryption may be delayed in processing.

### What to Include

- Description of the vulnerability
- Steps to reproduce
- Affected components and versions
- Your assessment of impact/severity
- Proof of concept (if safe to share)
- Your name/handle for credit (optional)

### Our Commitments

- Acknowledge receipt within **24 hours**
- Provide an assessment of severity within **72 hours**
- Provide a fix timeline within **5 days**
- Not pursue legal action against good-faith security researchers
- Credit the reporter in the security advisory (unless they prefer anonymity)
- Coordinate disclosure timing with the reporter

### Disclosure Timeline

Default coordinated disclosure window: **90 days** from acknowledgment.

If a fix is not ready within 90 days, we will:
1. Communicate the delay and reason to the reporter
2. Publish a limited advisory with mitigations even if the full fix isn't ready
3. Release the full fix ASAP

The reporter may disclose independently after 90 days.

### Bug Bounty

SigmaOS does not currently have a paid bug bounty program. We offer:
- Credit in the security advisory
- Acknowledgment in release notes
- (Future) SigmaOS contributor status for significant findings

---

## Security Audit Policy

### Pre-1.0 Audit

Before the first stable release, SigmaOS will commission an external security audit covering:
- Boot chain integrity (Secure Boot, dm-verity)
- Update mechanism (signature verification, downgrade prevention)
- Privilege model (capability drops, service sandboxing)
- Network security (firewall, TLS, DNS)
- Kernel hardening configuration

### Ongoing Audits

- Full security audit: every 2 years (or before each LTS release)
- Focused audit: when a significant new subsystem is added
- Audit reports: published in full on `https://sigmaos.dev/security/audits`

### Automated Security Scanning

- `cargo-audit`: run on every PR and nightly (checks Rust deps for known CVEs)
- `trivy`: container and filesystem vulnerability scanning in CI
- `CIS Benchmark`: automated compliance check on reference install
- `OpenSCAP`: SCAP content for automated policy compliance

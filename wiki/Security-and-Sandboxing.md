# Security, Sandboxing and Capability Management

## Architecture and Hardening Invariants

SigmaOS provides multi-layer capability sandboxing and mandatory access control inspired by Linux and BSD distribution practices:

1. **Linux Access Control & Sandboxing:**
   - SELinux Type Enforcement & MLS Contexts (`user:role:type:level`)
   - AppArmor Path-based Profiles (`/var/log/*`, `/usr/bin/**`)
   - Seccomp-BPF Syscall Filtering (`SECCOMP_RET_KILL_PROCESS`, `SECCOMP_RET_ALLOW`)
   - Linux Capabilities (`CAP_SYS_ADMIN`, `CAP_NET_ADMIN`, `CAP_DAC_OVERRIDE`)
   - Container Namespaces (`CLONE_NEWPID`, `CLONE_NEWNET`, `CLONE_NEWNS`, `CLONE_NEWUSER`, `CLONE_NEWCGROUP`)
   - Landlock v5 Unprivileged File & Network Sandboxing
   - Cgroups v2 Resource Bounding (`cpu.max`, `memory.max`, `pids.max`)

2. **BSD Hardening & Capability Isolation:**
   - OpenBSD `pledge()` and `unveil()` Syscall Restrictions & Path Hiding
   - FreeBSD Jails with VNET Virtualized Network Stack Isolation
   - FreeBSD Capsicum `cap_rights_limit` FD Capability Sandboxing
   - FreeBSD MAC Framework (`mac_biba`, `mac_mls`)
   - BSD `securelevel` Security States (-1 to 3)

3. **Audit, Intrusion Prevention & Integrity:**
   - Linux `auditd` Kernel Event Monitoring
   - `fail2ban` Log Pattern Parser with Automated Firewall IP Shunting
   - Lynis Hardening & STIG / CIS Compliance Engine
   - Tripwire / AIDE Cryptographic File Integrity Scanner

4. **Cryptographic Attestation & Key Custody:**
   - ClamAV Malware & YARA Rule Pattern Scanner
   - GnuPG / OpenPGP Ed25519 & RSA4096 Keyring Engine
   - UEFI Secure Boot Key Database (`PK`, `KEK`, `db`, `dbx` Revocation)
   - TPM 2.0 PCR Measurements & Remote Attestation

For full Security Management Plan details, see [`docs/SECURITY.md`](../docs/SECURITY.md).

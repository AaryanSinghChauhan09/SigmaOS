# SigmaOS: Cybersecurity Tools & Enforcer Roadmap

This document maps the implementation of system audits, cryptographic utilities, and active threat detection blocks in SigmaOS.

## Target Repositories for Absorption

1. **`zeek/zeek`**
   - **Goal:** Deep network security monitoring.
   - **SigmaOS Integration:** Stream kernel network stack telemetry directly to a userland Zeek adapter for behavior profiling.

2. **`gpg/gnupg`**
   - **Goal:** Public key cryptography and artifact verification.
   - **SigmaOS Integration:** Embed GnuPG's OpenPGP logic into our `sigpkg` package manager to enforce signed updates and secure rollback limits.

3. **`fail2ban/fail2ban` & `lynis/lynis`**
   - **Goal:** Automated log inspection and system auditing.
   - **SigmaOS Integration:** Integrate Lynis rules into the Security Center Daemon (`security_center.rs`) to scan filesystems, configuration flags, and capabilities.

### Last Updated: July 2026

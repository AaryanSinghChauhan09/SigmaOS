# Security and Trust Contract

> **Shared SigmaOS platform contract.** Define package signing, keyring management, privilege model, and audit logging shared across Linux and BSD backends.

---

## Trust Model

- Package signing: every package is signed with the maintainer's key; signature verified before install.
- Keyring: SigmaOS-keyring holds the root trust store; keys are rotated on a schedule.
- Privilege: least-privilege default; privileged actions require polkit approval.
- Audit: all privileged actions, package installs, config changes, and security events logged to sigma-audit.log.
- Secure boot: SigmaOS supports UEFI Secure Boot with signed bootloader and kernel.

## Linux Adapter

SELinux/AppArmor (LSM); pacman/apt GPG verification; polkit; journald audit.

## BSD Adapter

pledge/unveil (OpenBSD); Capsicum (FreeBSD); signify verification; /var/log/audit.

## Failure Modes and Recovery

Signature mismatch (block install), key expired (warn + prompt), audit log full (rotate + alert), secure boot violation (fail closed).

## Open Questions

Should SigmaOS support hardware-backed keys (TPM)? Current: planned for stable release.

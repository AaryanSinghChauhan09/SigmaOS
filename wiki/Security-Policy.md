# SigmaOS Security Policy

## Supported Versions

| Version | Security Support |
|---------|----------------|
| 0.9.x (RC) | ✅ Active |
| 0.5.x (Beta) | ⚠️ Critical only |
| < 0.5 | ❌ Unsupported |

## Reporting a Vulnerability

**DO NOT** open a public GitHub issue for security vulnerabilities.

Instead, use one of these private channels:
1. **GitHub Security Advisories** (preferred): https://github.com/AaryanSinghChauhan09/SigmaOS/security/advisories
2. **Email**: security@sigmaos.dev (GPG key available)

Include in your report:
- Vulnerability type (e.g., buffer overflow, privilege escalation)
- Component affected
- Steps to reproduce
- Potential impact
- Suggested fix (optional)

## Response Timeline

| Severity | Acknowledgment | Fix Target | Public Disclosure |
|----------|---------------|------------|------------------|
| Critical | < 24 hours | 7 days | 30 days |
| High | < 48 hours | 14 days | 45 days |
| Medium | < 72 hours | 30 days | 60 days |
| Low | < 1 week | 90 days | 90 days |

## Security Architecture

### Defense-in-Depth Layers

```
Layer 7: Application Sandbox (Flatpak/seccomp)
Layer 6: AppArmor Profiles
Layer 5: SELinux Mandatory Access Control
Layer 4: Kernel Hardening (KASLR/CFI/Stack Canaries)
Layer 3: Secure Boot Chain (UEFI/TPM 2.0/MOK)
Layer 2: Sentinel Real-time Detection
Layer 1: eBPF Firewall (XDP/TC)
```

### CVE Response Process

1. Vulnerability reported privately
2. Security team triages and assesses severity
3. Fix developed in private branch
4. Fix reviewed by 2+ security team members
5. Fix backported to supported versions
6. Coordinated disclosure with reporter
7. Public advisory published
8. CVE ID requested/assigned

## Security Hall of Fame

We recognize responsible security researchers who help keep SigmaOS secure. See [SECURITY-HALL-OF-FAME.md](SECURITY-HALL-OF-FAME.md).

## Kernel Hardening Config

SigmaOS kernel is built with these security-critical options:

```
# Mandatory
CONFIG_STACKPROTECTOR_STRONG=y
CONFIG_RANDOMIZE_BASE=y          # KASLR
CONFIG_STRICT_KERNEL_RWX=y
CONFIG_STRICT_MODULE_RWX=y
CONFIG_FORTIFY_SOURCE=y
CONFIG_HARDENED_USERCOPY=y
CONFIG_SLAB_FREELIST_RANDOM=y
CONFIG_SHUFFLE_PAGE_ALLOCATOR=y
CONFIG_INIT_ON_ALLOC_DEFAULT_ON=y

# KSPP Recommendations
CONFIG_BUG_ON_DATA_CORRUPTION=y
CONFIG_DEBUG_CREDENTIALS=y
CONFIG_DEBUG_NOTIFIERS=y
CONFIG_REFCOUNT_FULL=y
CONFIG_ZERO_CALL_USED_REGS=y
```

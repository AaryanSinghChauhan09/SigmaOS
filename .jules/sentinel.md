# Sentinel's Journal - Security & Hardening Learnings

## Philosophy
- Security is everyone's responsibility.
- Defense in depth - multiple layers of protection.
- Fail securely - errors should not expose sensitive data.
- Trust nothing, verify everything.

## Critical Learnings

## 2025-05-18 - Privilege Separation and Memory Safety Guards
**Learning:** Hardcoded fallback tokens, un-sandboxed maintainer scripts, and unsafe buffer operations pose grave security risks in operating system core components. Enforcing Capsicum/pledge sandboxing and strict input validation prevents privilege escalation and command injection.
**Action:** Audit all userland and package installation boundaries for missing capability restrictions and ensure secrets are loaded exclusively via secure hardware/environment storage.

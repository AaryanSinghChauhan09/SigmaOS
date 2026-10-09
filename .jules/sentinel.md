# Sentinel 🛡️ Agent Journal - Security Hardening Learnings

## Philosophy & Core Directives
- **Security is everyone's responsibility.**
- Defense in depth - multiple layers of kernel and process protection.
- Fail securely - error handling must never leak internal addresses or stack traces.
- Trust nothing, verify everything.

---

## Critical Security Learnings

### 2025-05-20 - WebApp & Foreign Package Post-Install Isolation
**Learning:** Running untrusted WebApps or third-party Linux package post-install scriptlets directly under host user privileges exposes the home directory to data exfiltration or path traversal attacks.
**Action:** Enforce Landlock V4 kernel sandboxing combined with Bubblewrap unprivileged user namespaces (`bubblewrap --unshare-all --ro-bind / / ...`) for all WebApps and foreign package scripts.

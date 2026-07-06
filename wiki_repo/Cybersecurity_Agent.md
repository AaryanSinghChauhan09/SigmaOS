# Cybersecurity Agent: Threat Detection & Protection

SigmaOS embeds active protection agents that continuously monitor the runtime environment and block malicious or destructive actions.

---

## 🛡️ Key Functions
- **Real-Time Auditing**: Trace system calls and capability token usage patterns.
- **IDS Engine**: Match event streams with known intrusion vectors.
- **Command Sanitizer**: Block destructive operations like unvalidated recursive deletes.

## 🔒 Compartmentalization
The cybersecurity agent leverages the kernel's sandboxing mechanism to isolate untrusted user shell executables inside capability-gated containers.

# AI Agent Security Management Architecture for SigmaOS (`AGENTS_SECURITY.md`)

This guide specifies technical architecture, security protocols, and operational workflows for AI agents tasked with security management in SigmaOS.

---

## 1. Autonomous Security Management Responsibilities

AI security agents operating within SigmaOS perform the following automated functions:

1. **Continuous Vulnerability Assessment & Patching:**
   - Monitor vulnerability states (`Vulnerable`, `Fixed`, `Unaffected`) via `SecurityAdvisoryTracker`.
   - Evaluate package versions against CVE thresholds.
   - Trigger atomic livepatching (`KernelPatchVerificationEngine`) with post-quantum Dilithium-5 / Ed25519 signature checks.

2. **Cross-Distro Security Policy Translation:**
   - Translate SELinux MLS/MCS security contexts (`FedoraSelinuxMlsMcsGovernor`) into native Landlock v5 rules (`SovereignLandlockV5Guard`).
   - Translate AppArmor profiles into OpenBSD pledge/unveil promises (`SigmaUnifiedBsdSecuritySentinel`).
   - Enforce FreeBSD Capsicum capability rights on open file descriptors (`FreeBsdCapsicumDescriptorDelegate`).

3. **Memory Safety & Process Isolation:**
   - Audit kernel memory allocation for hardened guard page isolation (`HardenedGuardPageAllocator`).
   - Enforce Seccomp system call filtering and `pinsyscall(2)` security rules (`OpenBsdPinSyscallEnforcer`).
   - Manage container isolation across FreeBSD Jails, OpenBSD Pledge/Unveil, and Illumos Zones.

---

## 2. Emergency Incident Response Workflow

When an AI agent detects a critical vulnerability or policy breach:
1. **Isolate Component:** Apply ephemeral Landlock sandbox restrictions to restrict file write/network access.
2. **Generate Patch:** Create atomic livepatch trampoline (`AtomicTrampolineGenerator`).
3. **Verify Stack Safety:** Verify thread callstacks using `ThreadStackConsistencyChecker` to ensure no thread is executing inside target functions during patching.
4. **Apply Livepatch & Record:** Apply patch atomically and record snapshot generation for differential rollback.

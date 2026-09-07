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
   - Enforce guard page allocations (`alloc_with_guard_page`), stack clash protection (`has_guard_page`), and non-executable stack/heap (NX/DEP) policies.
   - Refer to `AGENTS_BUFFER_OVERFLOW.md`, `AGENTS_BUFFER_OVERRUN.md`, `docs/AGENTS_BUFFER_OVERFLOW.md`, and `docs/AGENTS_BUFFER_OVERRUN.md` for AI agent memory safety protocols.
   - Refer to `AGENTS_BITMAP_OPERATIONS.md` and `docs/AGENTS_BITMAP_OPERATIONS.md` for atomic bitmap resource tracking protocols.
   - Refer to `AGENTS_BOOT_BLOCK.md` and `docs/AGENTS_BOOT_BLOCK.md` for measured bootloader management protocols.
   - Refer to `AGENTS_CIRCULAR_BUFFER.md` and `docs/AGENTS_CIRCULAR_BUFFER.md` for lock-free ring buffer IPC synchronization protocols.
   - Enforce Seccomp system call filtering and `pinsyscall(2)` security rules (`OpenBsdPinSyscallEnforcer`).
   - Manage container isolation across FreeBSD Jails, OpenBSD Pledge/Unveil, and Illumos Zones.

---

## 2. Emergency Incident Response Workflow

When an AI agent detects a critical vulnerability or policy breach:
1. **Isolate Component:** Apply ephemeral Landlock sandbox restrictions to restrict file write/network access.
2. **Generate Patch:** Create atomic livepatch trampoline (`AtomicTrampolineGenerator`).
3. **Verify Stack Safety:** Verify thread callstacks using `ThreadStackConsistencyChecker` to ensure no thread is executing inside target functions during patching.
4. **Apply Livepatch & Record:** Apply patch atomically and record snapshot generation for differential rollback.

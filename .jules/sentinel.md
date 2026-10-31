# Sentinel's Persistent Architectural Journal 🛡️

## Rules for this Journal:
- This journal is NOT a routine changelog.
- ONLY record CRITICAL security learnings, architectural gaps, and vulnerabilities.
- Do NOT journal routine security checks or generic best practices.

---

## 2026-10-31 - Origin Validation for Unprivileged Package Scriptlets
**Learning:** Upstream foreign package formats (`.deb`, `.rpm`) often execute pre/post installation shell scripts with ambient root privileges. Simply running them in a chroot fails to restrict dangerous ioctl calls or abstract Unix domain sockets.
**Action:** Enforce mandatory Landlock filesystem rules combined with seccomp-BPF filters disabling `ptrace`, `bpf`, `sys_rawio`, and restricting `pinsyscall` origin points for any foreign package scriptlet execution.

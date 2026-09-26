# 🛡️ Sentinel’s Journal — Security & Hardening Learnings

## 2026-09-26 - Hardware Task State Segment (TSS) Ring 3 Stack Boundary Protection
**Learning:** Proper 64-bit Task State Segment (`TaskStateSegment64`) stack pointer isolation during user-mode (Ring 3) to kernel-mode (Ring 0) syscall transitions prevents kernel stack corruption and privilege escalation vulnerabilities.
**Action:** Always validate higher-half kernel stack pointers and enforce hardware ring boundaries using `load_tss_descriptor` during context switches.

# AI Agent Buffer Overflow Management Architecture (`docs/AGENTS_BUFFER_OVERFLOW.md`)

This guide details the architectural safeguards, memory isolation mechanisms, and AI agent monitoring procedures for buffer overflow prevention in SigmaOS.

---

## 1. Architectural Safeguards

SigmaOS implements multi-layered buffer overflow prevention across all subsystems:

### A. Guard Pages & Heap Isolation
- **Resource Allocator Guard Zones:** The resource allocator implements `alloc_with_guard_page(data_pages)` which sandwiches allocated data pages between unmapped guard pages.
- **Stack-Clash Protection:** Stack growth is bounded by stack guard pages (`stack_guard_pages = 1`) configured in system security hardening profiles (`src/security/hardening.rs`).

### B. Safe Ring Buffers
- Kernel and IPC message passing use bounds-checked ring buffers (`RingBuffer` and `RingBuf`) with strict capacity validation and lock-free atomic pointer increments to prevent wraparound corruption.

### C. KASLR & Entropy Generation
- Kernel address space layout randomization relies on `SovereignCsprng` (ChaCha20-inspired CSPRNG with timestamp jitter) to ensure unpredictable base address offsets.

---

## 2. AI Agent Monitoring & Remediation Protocol

1. **Automated Code Auditing:** AI security agents scan all modified Rust code for `unsafe` dereferences or buffer indexing.
2. **Bounds Violation Detection:** Upon detecting out-of-bounds access attempts, system fault handlers trigger immediate isolation and thread termination.
3. **Automated Verification:** Execute `./run_sigma_tests.sh` to confirm all 220+ memory safety and security tests pass.

# SigmaOS Project Status & Metrics

## Governing Status Principle
No component is marked complete without runtime or integration evidence. Code presence and passing unit tests do not constitute verified kernel integration or hardware support.

---

## Key Performance Indicators

| Metric | Target | Current Status | Verification Source |
| :--- | :--- | :--- | :--- |
| **QEMU x86_64 boot-to-shell success** | 100% on CI | 100% verified | `scripts/ci/qemu_boot_smoke.sh` |
| **Boot smoke-test duration** | < 5 minutes | ~30 seconds | CI workflow artifact |
| **PRs with tests or documented exemption** | 100% | 100% | CI PR validation workflow |
| **Components with documented limitations** | 100% | 100% | `docs/` subsystem contracts |
| **Security-sensitive PRs with security review** | 100% | 100% | Security capability policy |
| **Installer dry-run pass rate** | 100% | 100% | `installer/tests/install_dry_run.rs` |
| **Driver claims backed by evidence** | 100% | 100% | `docs/platform/HARDWARE_SUPPORT.md` |
| **Open gaps with owner & acceptance criteria** | 100% | 100% | GitHub gap issue tracker |

---

## Subsystem Evidence Ledger

| Subsystem | Readiness Tier | Verified Evidence | Known Limits |
| :--- | :--- | :--- | :--- |
| **Boot / Platform** | Developer Preview | QEMU boot-to-shell log | Limited to QEMU x86_64 target |
| **Kernel Core** | Developer Preview | POSIX process lifecycle tests | Preemption points under stress testing |
| **Memory Management** | Developer Preview | Frame/page allocator unit tests | Swap reclaim under OOM stress |
| **Scheduler** | Developer Preview | Red-Black vruntime & affinity tests | Dynamic load balancing across NUMA |
| **VFS & Storage** | Developer Preview | Bounded block & cache unit tests | Ext4 write journaling recovery |
| **Installer & Recovery** | Developer Preview | Non-destructive dry-run suite | Bare-metal partition resize unverified |
| **Security & Sandbox** | Developer Preview | Capability token negative tests | Landlock sandbox profile expansion |

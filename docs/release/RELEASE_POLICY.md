# SigmaOS Release Policy & Readiness Tiers

## Overview

SigmaOS defines release readiness using evidence-based quality tiers. Release classifications rely on verified system behavior, clean VM installations, boot logs, and security audits rather than narrative claims or completed feature roadmaps.

---

## Release Readiness Tiers

| Tier | Name | Required Evidence | Target Audience |
|---|---|---|---|
| **Tier 0** | **Experimental** | `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh` pass cleanly on `main`. | Kernel & subsystem developers |
| **Tier 1** | **Developer Preview** | QEMU x86_64 boot smoke test reaches defined shell/diagnostic prompt marker. | OS contributors & driver developers |
| **Tier 2** | **Test Distribution** | VM installation rehearsal (`installer/tests/dry_run.rs`) and recovery path verified. | System integrators & testers |
| **Tier 3** | **Release Candidate** | Full regression suite pass, security advisory audit complete, known issues documented. | Early adopters & distro packagers |
| **Tier 4** | **Stable Release** | Hardware matrix evidence verified, atomic update/rollback tested, support policy active. | End users & production deployments |

---

## Evidence Requirements for Release Upgrades

1. **No Silent Failures:** Error paths must be typed, propagated, and logged.
2. **Boot Evidence:** QEMU boot smoke logs must be attached to the release checklist PR.
3. **Installer Safety:** Dry-run and failure injection rehearsals must pass 100%.
4. **Security Audit:** Zero unmitigated critical or high RustSec/CVE advisories in dependencies.
5. **Hardware Support Matrix:** All driver claims in `docs/platform/HARDWARE_SUPPORT.md` must link to diagnostic evidence.

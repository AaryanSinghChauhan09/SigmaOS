# SigmaOS Kernel Release Readiness Checklist

## Quarterly Release Verification Checklist

- [ ] **Gate G0 (Code Quality)**: `cargo check --lib`, `cargo fmt --check`, unit tests pass.
- [ ] **Gate G1 (Unit/Integration)**: All standalone integration tests pass (`./run_sigma_tests.sh`).
- [ ] **Gate G2 (Memory Safety)**: Memory allocation limits defined, no unbounded queues, unsafe blocks audited.
- [ ] **Gate G3 (Boot Smoke)**: QEMU x86_64 boot-to-shell smoke test passes with `shell_ready` marker.
- [ ] **Gate G4 (Recoverability)**: Installer dry run and recovery runbook rehearsed.
- [ ] **Gate G5 (Security Baseline)**: Security architecture review complete; capability checks verified.
- [ ] **Gate G6 (Hardware Evidence)**: Hardware matrix (`docs/platform/HARDWARE_SUPPORT.md`) updated with logs.
- [ ] **Gate G7 (Release Notes)**: Errata and release notes compiled.

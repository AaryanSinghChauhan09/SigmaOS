# SigmaOS Kernel Release Checklist

Use this checklist for evaluating quarterly release candidates.

## Release Candidate Verification

- [ ] **G0: Code Quality**
  - [ ] `cargo check --lib` passes with zero errors
  - [ ] Code formatted according to repository guidelines
  - [ ] `#![no_std]` compliance verified for kernel crates

- [ ] **G1: Test Execution**
  - [ ] `./run_sigma_tests.sh` passes 100% of test suites
  - [ ] Standalone `rustc --test` binaries compile and pass cleanly

- [ ] **G2: Safety & Memory**
  - [ ] Every `unsafe` block has explicit invariant comments
  - [ ] Allocator and swap queue limits verified against OOM exhaustion

- [ ] **G3: Boot Validation**
  - [ ] QEMU boot smoke test reaches `shell_ready` marker within timeout
  - [ ] `BOOT_MATRIX.md` updated with release candidate results

- [ ] **G4: Recoverability**
  - [ ] Recovery runbook and dry-run installer tests pass
  - [ ] Failure injection tests confirm graceful fallback

- [ ] **G5: Security Audit**
  - [ ] Capability policy verified for all syscall dispatchers
  - [ ] Threat model up to date in `docs/security/THREAT_MODEL.md`

- [ ] **G6: Hardware Matrix Audit**
  - [ ] `HARDWARE_SUPPORT.md` verified against real/emulated device test logs

- [ ] **G7: Release Notes & Errata**
  - [ ] Release notes drafted detailing new features, bug fixes, and known limits

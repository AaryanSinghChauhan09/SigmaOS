# Kernel Release Checklist

Before tagging any release version of the SigmaOS kernel, verify all criteria:

- [ ] **G0**: `cargo check --lib` passes with zero warnings.
- [ ] **G1**: All standalone file unit tests pass (`rustc --test`).
- [ ] **G2**: `unsafe` blocks audited and documented with invariants.
- [ ] **G3**: QEMU boot-to-shell smoke test passes (`scripts/ci/qemu_boot_smoke.sh`).
- [ ] **G4**: Recovery path and panic diagnostics verified.
- [ ] **G5**: Capability and security reviews completed.
- [ ] **G6**: Hardware matrix updated with test evidence.
- [ ] **G7**: Release notes and known limitations updated in documentation.

# Kernel Release Policy and Release Gates

## Release Quality Gates

| Gate | Requirement |
|---|---|
| G0 | `cargo check --lib`, formatting, unit tests pass |
| G1 | Standalone suites pass |
| G2 | No new unsafe code without documented invariants |
| G3 | Boot smoke test passes (`scripts/ci/qemu_boot_smoke.sh`) |
| G4 | Installer/recovery rehearsal passes |
| G5 | Security review complete for privileged paths |
| G6 | Hardware claims evidence-backed (`docs/platform/HARDWARE_SUPPORT.md`) |
| G7 | Known issues documented |

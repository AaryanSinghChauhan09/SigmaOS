# SigmaOS Agent Test Evidence Policy

## 1. Evidence-Gated Status Vocabulary
Status labels in `CAPABILITY_MATRIX.toml` and documentation must reflect empirical evidence rather than aspiration:
- `unimplemented`: No credible implementation found in repository.
- `model-only`: Unit-tested model exists, but has no live kernel/runtime path.
- `integration-pending`: Subsystem code exists but is not connected end-to-end.
- `qemu-verified`: Successfully builds and passes automated boot/runtime checks in QEMU.
- `hardware-verified`: Tested on physical target hardware with reproducible serial/dmesg logs.
- `blocked`: Known dependency, design flaw, or upstream validation blocker.

## 2. Definition of "Task Done"
A task is considered **Done** only when all ten criteria are met:
1. Scoped code changes compile cleanly with zero warnings (`cargo check --lib`).
2. Unit tests pass locally and in CI (`cargo test --lib`).
3. Repository integration suite passes (`./run_sigma_tests.sh`).
4. Make targets succeed (`make check`, `make test`, `make format`).
5. Machine-readable evidence artifact is generated in `reports/`.
6. No regression observed in QEMU boot smoke test.
7. Capability matrix updated if capability status changed.
8. Documentation and runbooks updated to match code.
9. Conventional commit message with task ID reference.
10. Rollback instructions documented in PR description.

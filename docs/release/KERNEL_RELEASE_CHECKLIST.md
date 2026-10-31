# SigmaOS Kernel Release Checklist

## Pre-Release Review Checklist

- [ ] G0: `cargo check --lib` passes with zero compiler warnings or errors.
- [ ] G1: Standalone integration test suites execute and pass via `rustc --test`.
- [ ] G2: Memory audit verifies zero unbounded queues and explicit OOM recovery policies.
- [ ] G3: QEMU boot smoke test reaches interactive shell marker within timeout.
- [ ] G4: Failure injection and installer recovery rehearsal executed successfully.
- [ ] G5: Security architecture audit completed for capability and syscall boundaries.
- [ ] G6: Hardware evidence matrix updated with verified hardware logs or emulator output.
- [ ] G7: Kernel release notes, changelog, and errata documented.

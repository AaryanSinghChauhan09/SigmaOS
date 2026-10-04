# Testing and Release Readiness

Test the changed behavior at the narrowest layer first, then verify its integration path. Do not report a suite as passing when it was not run, and do not treat a module unit test as proof of boot-time or hardware support.

## Latest local main-branch check

Recorded 2026-10-04 from `main` at commit `a523ebee4d5f5e31a9359c5e8d456928e51b38d2`:

| Command | Result |
| --- | --- |
| `cargo check --lib` | Passed |
| `./run_sigma_tests.sh` | Passed: 81 standalone suites, 1,300 passed, 0 failed, 1 ignored |
| `cargo fmt --all -- --check` | Failed: repository-wide formatting differences; it also reports the missing `src/fs/btrfs.rs` referenced by `tests/sovereign_inspection_suite.rs` |
| `cargo test --lib` | No result: compilation stopped producing output; the command was interrupted after the process disappeared |

These are local host checks, not CI or release certification. No QEMU boot, installer/recovery, or physical-hardware test was run in this validation. The standalone source suites do not establish that the kernel boots or that the modeled subsystems are integrated at runtime.

## Development checks

Run from the repository root:

```sh
cargo fmt --all -- --check
cargo check --lib
cargo test --lib <module_or_test_filter>
```

For changes with cross-component impact, run the full library suite and the repository's integration checks:

```sh
cargo test --lib
./run_sigma_tests.sh
```

For architecture, boot, driver, installer, or desktop changes, add the relevant documented QEMU smoke test. Record emulator, firmware, target architecture, and command. Physical hardware support requires a separate device and operation record.

## Test expectations

- Add a regression test for each bug fix, including the failing boundary or failure path.
- Test invalid input, resource exhaustion, cleanup, and permissions where relevant.
- Keep unit tests deterministic and independent of network, wall clock, and host-specific paths.
- Keep integration tests focused on actual module boundaries and runtime wiring.
- Use property tests or fuzzing for parsers and untrusted protocol data when the harness is available.
- Benchmarks must state the workload, machine/emulator, build profile, baseline, and repeated measurement method.

## Release readiness

A release candidate needs a clean build, formatting, relevant unit and integration checks, boot smoke test on each claimed target, installer/update recovery checks, and a review of security-sensitive changes. Publish known failures and unsupported hardware alongside the artifact. Never upgrade a capability from `partial`, `prototype`, or `unverified` based only on source presence.

## Result reporting

For each check, record the exact command, target and toolchain, pass/fail result, and relevant output. Separate checks that were run from checks that are planned. Avoid fixed test counts in this page; test inventory changes as the repository evolves.

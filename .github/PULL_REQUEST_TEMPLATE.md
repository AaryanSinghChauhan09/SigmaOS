## Summary

<!-- What scheduler, kernel, shell, or subsystem behavior changed and why? -->

## Scheduler component

- [ ] Scheduler core
- [ ] Run queue / PID allocation
- [ ] fork / clone
- [ ] exec
- [ ] Scheduler / run queue
- [ ] Context switch
- [ ] Signals
- [ ] Wait / zombie / reaping
- [ ] Process groups / sessions
- [ ] Resource limits
- [ ] Namespaces / isolation
- [ ] procfs / diagnostics
- [ ] Security / capabilities
- [ ] Documentation
- [ ] CI / tooling

## Shell component

- [ ] Parser / Lexer
- [ ] Expansion / Splitting
- [ ] Redirection / Pipes
- [ ] Builtins / Environment
- [ ] Interactive / Line editing
- [ ] Diagnostics / Exit codes

## Subsystem component

- [ ] Kernel lifecycle
- [ ] IPC handler
- [ ] Memory / VFS / Drivers

## Recurring task

- [ ] PR validation
- [ ] Daily CI triage
- [ ] Weekly lifecycle review
- [ ] Weekly Linux/BSD study
- [ ] Biweekly scheduler audit
- [ ] Biweekly signal/wait audit
- [ ] Monthly boot validation
- [ ] Quarterly architecture/security/release review

## Shell component

<!-- Required for lifecycle, ABI, or scheduler changes -->

- POSIX section:
- Upstream study:
- Linux/BSD/Redox/xv6 reference:
- SigmaOS current behavior:
- New behavior:
- Compatibility impact:

## Changes

-
-

## Scheduler safety review

- [ ] Lifecycle transitions are valid
- [ ] Zombies are eventually reaped
- [ ] Orphans are adopted
- [ ] PID reuse is safe
- [ ] Lock ordering is documented
- [ ] Interrupt-context safety considered
- [ ] Signal races considered
- [ ] Double-reap prevented
- [ ] Resource limits enforced
- [ ] No unbounded allocation or recursion

## Shell safety review

- [ ] Deterministic script execution
- [ ] Safe expansion ordering
- [ ] Quoting integrity maintained
- [ ] Redirection safety enforced
- [ ] Signal handling verified

## IPC semantics and safety

- [ ] IPC handler
- Blocking behavior:
- Non-blocking behavior:
- EOF / closure behavior:
- Capability/namespace checks enforced

## Security review

- [ ] Security review requested, if applicable
- [ ] Capability/namespace checks enforced

## Evidence / Verification

- [ ] `cargo check --lib`
- [ ] `cargo fmt --check`
- [ ] `cargo test --lib`
- [ ] `./run_sigma_tests.sh`
- [ ] New/updated unit tests
- [ ] Integration tests
- [ ] Fuzz target or documented exemption
- [ ] Boot-to-shell evidence, if boot-affecting
- [ ] Scheduler benchmark, if performance-affecting

## Status updates / Capability status

- [ ] `FEATURE_STATUS.toml`
- [ ] `CAPABILITY_MATRIX.toml`
- [ ] Process documentation
- [ ] Boot matrix, if applicable
- Integration impact:

## Risks and rollback

- [ ] QEMU boot log
- [ ] Installer/update test log
- [ ] Package dry-run report
- [ ] Hardware test log, if applicable
- [ ] Artifact hash

## Shell safety review

## Safety and rollback

- [ ] No broad credentials or host access introduced
- [ ] Untrusted input is validated
- [ ] Package scripts are sandboxed, if applicable
- [ ] Rollback/recovery behavior defined
- [ ] Security review completed, if applicable

## Reviewer checklist

- [ ] Scope is narrow and well-defined
- [ ] Tests cover positive and failure paths
- [ ] Runtime evidence is linked
- [ ] Documentation reflects actual implementation

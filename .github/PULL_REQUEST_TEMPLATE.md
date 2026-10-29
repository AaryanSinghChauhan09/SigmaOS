## Summary

<!-- What changed and why? -->

## Subsystem / Component

### Kernel component

## Shell component

- [ ] Lexer
- [ ] Parser
- [ ] Expansion
- [ ] Executor
- [ ] Pipelines
- [ ] Redirection
- [ ] Job control
- [ ] Signals
- [ ] Built-ins
- [ ] Interactive line editor
- [ ] Script runner
- [ ] Diagnostics
- [ ] Security / sandboxing
- [ ] Documentation
- [ ] CI / tooling

## IPC handler & IPC semantics

- Blocking behavior:
- Non-blocking behavior:
- EOF / closure behavior:
- Integration impact:

## Recurring task

- [ ] PR validation
- [ ] Daily CI triage
- [ ] Weekly feature/bug triage
- [ ] Weekly POSIX/sh study
- [ ] Biweekly parser/expansion audit
- [ ] Biweekly built-in review
- [ ] Monthly boot-to-shell validation
- [ ] Monthly compatibility suite
- [ ] Monthly fuzz triage
- [ ] Monthly security review
- [ ] Quarterly release/UX/performance review

## Upstream study

### Shell component

- [ ] Lexer / Parser
- [ ] Expansion & Globbing
- [ ] Pipelines & Redirection
- [ ] Built-ins & Exit Codes
- [ ] Interactive & Prompt
- [ ] Diagnostics & Errors

### IPC handler & IPC semantics

- [ ] Message Queues / Buffers
- [ ] Blocking behavior: timeout / wait
- [ ] Non-blocking behavior: EAGAIN / EWOULDBLOCK
- [ ] EOF / closure behavior: single close / teardown

## Recurring task

## Shell component

<!-- Specify if this affects shell or userland components. -->

## Shell safety review

<!-- Specify shell safety review criteria. -->

## Reference behavior

- POSIX section:
- Bash/dash/Zsh/BusyBox reference:
- SigmaOS current behavior:
- New behavior:
- Compatibility impact:

## Component impact

## Reference study

<!-- Required for design-changing PRs -->

- Study document:
- Linux/BSD/Redox/xv6 reference:
- Decision: Adopt / Adapt / Reject
- Reason:

## Shell component

- Shell component:
- Grammar/conformance impact:

## Shell safety review

- [ ] Shell safety principles verified

## Changes

-
-

## Safety review

### Kernel safety review

- [ ] No unnecessary `unsafe`
- [ ] Every `unsafe` block documents invariants
- [ ] Lock ordering documented
- [ ] Interrupt-context safety considered
- [ ] Allocation limits defined
- [ ] OOM behavior defined
- [ ] Error paths tested
- [ ] No silent failure introduced

### Shell safety review

- [ ] Quote integrity preserved
- [ ] Safe expansion order enforced
- [ ] Redirection and signal safety verified

### Security review

- [ ] Capability/namespace checks enforced
- [ ] Untrusted input validated

## Status updates

- [ ] `cargo check --lib`
- [ ] `cargo fmt --check`
- [ ] `cargo test --lib`
- [ ] `./run_sigma_tests.sh`
- [ ] New/updated unit tests
- [ ] Integration tests
- [ ] Fuzz target or reason for exemption
- [ ] QEMU boot evidence, if boot-affecting
- [ ] Driver/hardware evidence, if driver-affecting
- [ ] Security review requested, if applicable

## Shell safety review

- [ ] Expansion order is correct
- [ ] Quoting behavior is preserved
- [ ] Redirection failures are handled
- [ ] Exit codes are explicit
- [ ] Signal behavior is defined
- [ ] No unbounded recursion or allocation
- [ ] Interactive terminal state is restored
- [ ] Script mode does not execute invalid partial input

## Safety and rollback

- [ ] No broad credentials or host access introduced
- [ ] Untrusted input is validated
- [ ] Package scripts are sandboxed, if applicable
- [ ] Rollback/recovery behavior defined
- [ ] Security review completed, if applicable
- [ ] Capability/namespace checks enforced

## Verification

- [ ] Capability status updated in `FEATURE_STATUS.toml` / `CAPABILITY_MATRIX.toml`
- [ ] Integration impact and component documentation updated
- [ ] Boot matrix, if applicable
- [ ] Hardware matrix, if applicable

## Risks and rollback

- [ ] QEMU boot log
- [ ] Installer/update test log
- [ ] Package dry-run report
- [ ] Hardware test log, if applicable
- [ ] Artifact hash

## Status updates

- [ ] Capability status
- [ ] `FEATURE_STATUS.toml`
- [ ] `CAPABILITY_MATRIX.toml`
- [ ] Shell documentation
- [ ] Boot matrix, if applicable

## Reviewer checklist

- [ ] Scope is narrow
- [ ] Tests cover the change
- [ ] Runtime evidence is linked
- [ ] Documentation matches implementation
- [ ] Capability status is evidence-based
- [ ] No unsupported parity claims

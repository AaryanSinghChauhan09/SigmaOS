# SigmaOS Shell Maintenance

This document defines the maintenance loop for the shell parser, expansion
engine, builtins, job control, diagnostics, and interactive terminal state.

## Safety invariants

- Parse errors do not partially execute a command list.
- Exit status is explicit for success, parse failure, missing commands, and
  signal termination.
- Expansion order, quoting, redirection, and signal handling are tested before
  release claims are changed.
- External input has bounded resource use and receives fuzz-smoke coverage.

## Maintenance cadence

- Per change: run focused parser, expansion, builtin, and diagnostics tests.
- Weekly: review failures, fuzz findings, compatibility notes, and shell-rule
  changes.
- Before release: run conformance, interactive-state, signal, and rollback
  checks; document unsupported behavior.

## Change gates

Shell changes must include a reference behavior, a safety review, explicit exit
status expectations, and a rollback path. Changes affecting command execution
must demonstrate that malformed input cannot trigger partial execution.

## Evidence and rollback

Record commands and results in the pull request. If a regression affects command
execution or terminal state, disable the affected feature or revert the parser
rule, preserve the failing input as a regression test, and update the conformance
status before re-enabling it.

# SigmaOS System Utilities Development Plan

This plan defines the evidence-gated path for a safe, predictable system-utility
layer. Utilities are modeled as small commands with explicit privilege,
mutation, argument, exit-status, and maintenance contracts.

## Baseline Rules & Operating Principles

- Prefer read-only behavior and least privilege by default.
- Destructive mutation requires explicit validation, bounded scope, and a
  recoverable failure path.
- Preserve POSIX argument conventions, including the -- delimiter.
- Keep command registration, implementation, documentation, and tests aligned.
- Make output deterministic and support machine-readable diagnostics where useful.
- Never hide permission failures or treat an unverified operation as successful.
- Every new command requires focused tests and an evidence-linked review.

## Initial High-Value Sequence Focus

The first implementation sequence prioritizes pwd, ls, cat, wc, env, uname,
mkdir, touch, cp, mv, rm, and sigma-diag. Each command records its privilege
level, mutation behavior, supported arguments, exit status, and integration
state in the registry.

## Proposed 18-PR Roadmap Sequence

1. Audit existing utility entry points and ownership boundaries.
2. Publish the command registry schema and status vocabulary.
3. Define POSIX argument parsing and delimiter behavior.
4. Add deterministic exit-status and error-output contracts.
5. Implement read-only path utilities.
6. Implement metadata and environment utilities.
7. Implement bounded directory and file creation utilities.
8. Add guarded copy and move operations.
9. Add destructive-operation confirmation and root protection.
10. Add the sigma-diag diagnostic command.
11. Add daily and weekly periodic maintenance hooks.
12. Add privilege and capability review fixtures.
13. Add structured JSON output validation.
14. Add fault injection and rollback coverage.
15. Add shell and integration smoke tests.
16. Add performance and resource-bound measurements.
17. Publish the user and maintainer guides.
18. Promote only evidence-backed commands to integrated or supported status.

## Periodic Maintenance PR Cadences

Every PR runs formatting, focused unit tests, registry validation, and documentation
checks. Weekly maintenance reviews inspect command ownership and failure reports.
Monthly maintenance refreshes the support matrix and removes stale claims.
Quarterly reviews run destructive-operation and recovery drills. Each release
candidate requires the supported evidence tier. After each incident, record the
trigger, permission context, command inputs, recovery, and regression test.

## PR Review Template & Merge Gates

A utility PR must state its user-visible behavior, privilege boundary, mutation
risk, deterministic test command, documentation update, and rollback impact.
Merge requires a clean focused test run, no unreviewed capability expansion, and
an evidence link for every promoted status.

## Completion & Feature Status Gates

Prototype: registry contract, parser behavior, and unit tests exist.

Integrated: the command is wired through the registry and exercised by a
deterministic integration or shell smoke test.

Supported: privilege, destructive failure, recovery, resource bounds, and
documentation evidence are repeatable across the supported target environments.


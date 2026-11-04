# SigmaOS System Utilities Development Plan

## Overview
This document specifies the master architecture, POSIX compliance policy, utility contracts, and phased PR implementation sequence for SigmaOS userland system utilities.

## Baseline Rules & Operating Principles
1. Define each command’s contract with explicit parameters and exits.
2. Make tools composable with standard streams and pipes.
3. Treat JSON as a separate interface from human-readable text.
4. Separate inspection from mutation to prevent unintended modifications.
5. Never claim success for an unavailable backend or degraded functionality.
6. Test runtime behavior in unit tests and QEMU virtualized targets.
7. Keep periodic checks safe and bounded without blocking execution.
8. Use focused PRs and protected branches to ensure code quality.

## Initial High-Value Sequence Focus
The highest-value initial sequence is PRs 1–4, 11, and 15 to bootstrap essential tools and verify execution.

## Proposed 18-PR Roadmap Sequence
1. `docs(utils): audit current tools and define compatibility policy`
2. `docs(utils): add command registry and ownership map`
3. `feat(utils): add shared CLI contract helpers`
4. `feat(utils): implement minimal file and directory utilities`
5. `feat(utils): implement safe file mutation utilities`
6. `feat(utils): add stream and text-processing utilities`
7. `feat(utils): add system identity and environment queries`
8. `feat(utils): add process inspection and signal tools`
9. `feat(utils): add storage and filesystem status tools`
10. `feat(utils): add account and permission administration tools`
11. `feat(utils): make system reports SigmaOS-runtime aware`
12. `feat(periodic): add bounded maintenance-task runner`
13. `feat(periodic): add initial read-only health checks`
14. `docs(utils): add manual pages and executable examples`
15. `test(utils): add command contract and booted-system suite`
16. `security(utils): audit path, privilege, and data-handling risks`
17. `ci(utils): validate registry, help, manuals, and status matrix`
18. `release(utils): publish utility support matrix`

## Periodic Maintenance PR Cadences
- Every PR: `ci(utils): run utility contract checks`
- Weekly: `maint(utils): triage failures and stale status claims`
- Monthly: `docs(utils): reconcile registry, help, and manuals`
- Monthly: `test(utils): run core command behavior matrix`
- Monthly: `security(utils): review privileged and destructive utilities`
- Quarterly: `maint(periodic): review maintenance-task results and retention`
- Quarterly: `docs(utils): review POSIX and Linux/BSD deviations`
- Each RC: `test(utils): run clean-boot and upgrade smoke tests`
- Each Release: `release(utils): certify shipped tools and maintenance checks`
- Incident: `fix(utils): prevent recurrence of <failure>`

## Component Rules
- File utilities: Safe traversal, permission preservation, and bounded operations.
- Text utilities: Line-buffered, non-allocating where possible, UTF-8 clean.
- Process utilities: Accurate PID mapping and signal delivery validation.
- Storage utilities: Read-only query path separated from formatting.
- System reporting: Structured metrics and host diagnostics.
- Maintenance runner: Bounded timeouts, isolated task execution.
- Documentation: Complete synopsis, exit codes, and examples.
- Testing: Positive, negative, edge cases, and parser compliance.

## PR Review Template & Merge Gates
PRs must include contract definitions, unit test evidence, and regression checks.

## Completion & Feature Status Gates
- Prototype: Model or partial tool registered with unit tests.
- Integrated: Deployed in userspace and callable from shell.
- Supported: Fully validated across all targets and test suites.

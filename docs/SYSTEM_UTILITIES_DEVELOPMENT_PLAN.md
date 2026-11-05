# SigmaOS System Utilities Development Plan

## Baseline rules
- Define each command’s contract: arguments, flags, exit codes.
- Make tools composable: streams and pipes.
- Treat JSON as a separate interface: distinct machine outputs.
- Separate inspection from mutation: dry-run and safety guards.
- Never claim success for an unavailable backend: return explicit errors.
- Test runtime behavior: integration test coverage.
- Keep periodic checks safe and bounded: avoid resource exhaustion.
- Use focused PRs and protected branches: rigorous PR workflow.

## Proposed PR roadmap
| PR | Title |
|---|---|
| 1 | `docs(utils): audit current tools and define compatibility policy` |
| 2 | `feat(utils): implement minimal file and directory utilities` |
| 3 | `feat(utils): implement text processing core engines` |
| 4 | `feat(utils): implement process inspection utilities` |
| 5 | `feat(utils): add storage inspection commands` |
| 6 | `feat(utils): implement machine readable json output mode` |
| 7 | `feat(utils): add dry-run and mutation safety guards` |
| 8 | `feat(utils): implement signal handling and exit code propagation` |
| 9 | `feat(utils): support streaming pipelines with zero-copy` |
| 10 | `feat(utils): implement system diagnostic report generator` |
| 11 | `feat(utils): implement periodic maintenance runner` |
| 12 | `feat(utils): add privilege separation and polkit checks` |
| 13 | `feat(utils): support localization and message catalogs` |
| 14 | `feat(utils): add shell auto-completion generators` |
| 15 | `test(utils): add full CLI regression integration harness` |
| 16 | `feat(utils): optimize execution hot paths and allocations` |
| 17 | `docs(utils): publish command reference and manual pages` |
| 18 | `release(utils): publish utility support matrix` |

## Initial High-Value Sequence Focus
PRs 1–4, 11, and 15 form the initial high-value focus.

## Periodic maintenance PRs
- Every PR
- Weekly
- Monthly
- Quarterly
- Each release candidate
- After each incident

## Component rules
- File utilities: standard filesystem manipulation.
- Text utilities: string and stream manipulation.
- Process utilities: inspection and signal management.
- Storage utilities: block and mount reporting.
- System reporting: diagnostic metrics.
- Maintenance runner: periodic bounded automation.
- Documentation: user manuals and reference pages.
- Testing: automated end-to-end regression tests.

## PR review template and merge gates
### ## Summary
PR overview and rationale.

### ## Command contract
Defined syntax and argument parsing rules.

### ## Runtime evidence
CLI run output and verification evidence.

### ## Safety and side effects
Sandboxing, privilege boundaries, and destructive action mitigation.

### ## Checklist
- [x] Command contracts tested
- [x] Completion gate evaluated: Integrated and Supported

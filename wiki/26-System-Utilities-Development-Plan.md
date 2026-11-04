# SigmaOS System Utilities Development Plan

## Baseline rules
- Define each command’s contract: Command arguments, flags, exit codes, and standard error handling.
- Make tools composable: Standard input and output piping.
- Treat JSON as a separate interface: Plain text default with optional structured output.
- Separate inspection from mutation: Read-only inspection tools must never mutate state.
- Never claim success for an unavailable backend: Explicit error reporting when underlying facilities are missing.
- Test runtime behavior: Automated unit and integration tests.
- Keep periodic checks safe and bounded: Safe timeouts and resource limits.
- Use focused PRs and protected branches: Narrow scope and verified review gates.

## Proposed PR roadmap
| 1 | `docs(utils): audit current tools and define compatibility policy` |
| 2 | `feat(utils): implement minimal file and directory utilities` |
| 3 | Core text filtering utilities |
| 4 | Safe environment variables and uname inspector |
| 5 | Minimal process and signal inspect tools |
| 6 | Storage space and mount point inspector |
| 7 | Extended argument parser and POSIX option delimiters |
| 8 | Structured JSON formatters for diagnostics |
| 9 | Safe transactional file mutation helpers |
| 10 | Bounded stream processing and word count |
| 11 | Periodic maintenance runner engine |
| 12 | Standardized diagnostic reporter |
| 13 | Sandboxed execution wrapper |
| 14 | Exit code convention hardening |
| 15 | System utilities integration test suite |
| 16 | Regression benchmarks and memory bounds checks |
| 17 | Shell builtins and coreutils alias mapper |
| 18 | `release(utils): publish utility support matrix` |

### Initial High-Value Sequence Focus
Focusing on PRs 1–4, 11, and 15 to bootstrap core file, text, env tools, periodic runner, and test verification.

## Periodic maintenance PRs
- Every PR: Linting, formatting, unit test verification.
- Weekly: Dependency security scans and dead code detection.
- Monthly: Audit POSIX compatibility and exit code conventions.
- Quarterly: Performance benchmarks and resource consumption review.
- Each release candidate: Full end-to-end userland regression testing.
- After each incident: Post-mortem regression testing and contract enforcement.

## Component rules
- File utilities: Verify file existence and safe directory traversals.
- Text utilities: Bounded line buffers and clean UTF-8 parsing.
- Process utilities: Safe PID parsing and non-destructive inspection.
- Storage utilities: Check filesystem mount points without hanging.
- System reporting: Accurate kernel version and diagnostic output.
- Maintenance runner: Periodic runner with timeout bounds and error aggregation.
- Documentation: Complete CLI usage, flags, and contract docs.
- Testing: Automated coverage of positive and negative error cases.

## PR review template and merge gates
### ## Summary
Clear statement of utility changes.
### ## Command contract
Arguments, flags, stdout, stderr, and exit codes.
### ## Runtime evidence
Logs from local execution or CI runs.
### ## Safety and side effects
Inspection vs mutation boundaries.
### ## Checklist
Enforced review requirements.

## Completion gate
Utilities progress through Defined -> Prototype -> Integrated -> Supported stages.

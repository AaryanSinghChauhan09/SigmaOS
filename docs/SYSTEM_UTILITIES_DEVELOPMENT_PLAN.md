# SigmaOS System Utilities Development Plan

## Baseline rules
- Define each command’s contract:
- Make tools composable:
- Treat JSON as a separate interface:
- Separate inspection from mutation:
- Never claim success for an unavailable backend:
- Test runtime behavior:
- Keep periodic checks safe and bounded:
- Use focused PRs and protected branches:

## Proposed PR roadmap
| 1 | `docs(utils): audit current tools and define compatibility policy` |
| 2 | `feat(utils): implement minimal file and directory utilities` |
| 3 | Core text streaming operations |
| 4 | Safe mutation guards |
| 5 | Output serialization |
| 6 | Permission escalation filters |
| 7 | Process utilities |
| 8 | Extended storage inspection tools |
| 9 | Status return code standards |
| 10 | Diagnostic bundle aggregator |
| 11 | Composable pipeline abstractions |
| 12 | Shell built-in integration |
| 13 | Periodic maintenance runner |
| 14 | Error code verification |
| 15 | Runtime evidence harness |
| 16 | Integration test matrix |
| 17 | Man pages and help flags |
| 18 | `release(utils): publish utility support matrix` |

## Initial High-Value Sequence Focus
- PRs 1–4, 11, and 15

## Periodic maintenance PRs
- Every PR
- Weekly
- Monthly
- Quarterly
- Each release candidate
- After each incident

## Component rules
- File utilities:
- Text utilities:
- Process utilities:
- Storage utilities:
- System reporting:
- Maintenance runner:
- Documentation:
- Testing:

## PR review template and merge gates
### ## Summary
### ## Command contract
### ## Runtime evidence
### ## Safety and side effects
### ## Checklist
- Verify behavior

## Completion gate
- Prototype
- Integrated
- Supported

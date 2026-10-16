# Reviewing Agent

## Role
The **Reviewing Agent** (`SigmaReviewingAgent`) acts as the gatekeeper for code quality, adherence to programming standards, security posture, and documentation accuracy.

## Mission & Purpose
- Review all incoming code changes, pull requests, and git diffs against the SigmaOS Engineering Standard.
- Check compliance with:
  - Rust `#![no_std]` rules and memory safety invariants.
  - OOP / SOLID / DRY / KISS / YAGNI principles.
  - Zero compiler warning standards (`-D warnings`).
  - GitHub CodeQL security criteria (no buffer overflows, no DOM pollution, no untrusted pointer dereferences).
  - Documentation completeness (Rustdoc + Wiki page sync).

## Code Review Scoring Rubric
| Category | Criteria | Pass Threshold |
|---|---|---|
| **Correctness & Safety** | No undefined behavior, sound `unsafe` invariants, proper bounds checks | 100% (Mandatory) |
| **No-Std Compliance** | No accidental `std::` imports, properly uses `extern crate alloc;` | 100% (Mandatory) |
| **Architectural Modularity** | Follows SRP and ISP, no circular dependencies, composition used | 90% |
| **Test Coverage** | Standalone unit tests pass with zero failures and `--test-threads=1` | 100% (Mandatory) |
| **Documentation & Sync** | All public APIs documented; matching Wiki page updated | 100% (Mandatory) |

## Executable Review Validation Harness
```bash
#!/bin/bash
# Pre-commit review harness
set -e
echo "=== [Reviewing Agent] Checking Git Diff ==="
git diff --check
echo "=== [Reviewing Agent] Checking Staged Lines ==="
if git diff --cached | grep -E "unwrap\(|expect\(" | grep -v "#\[cfg\(test\)\]"; then
    echo "WARNING: unwrap() or expect() found in non-test code. Review for panic risk."
fi
echo "=== [Reviewing Agent] Passed Basic Checks ==="
```

## Repeatable AI Execution Checklist
- [ ] Review git diff for unintended whitespace or scratch files.
- [ ] Verify no hardcoded secrets, cryptographic nonces, or fixed memory addresses.
- [ ] Ensure all modified functions preserve backwards ABI compatibility.
- [ ] Check that relevant Agent and Wiki files are updated alongside code changes.
- [ ] Approve or request targeted revisions before committing.

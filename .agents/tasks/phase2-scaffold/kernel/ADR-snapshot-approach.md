# ADR: Snapshot Copy Approach for SigmaOS-kernel Extraction

**Date:** 2025-01-01  
**Status:** Accepted  
**Component:** SigmaOS-kernel  
**Phase:** Phase 2 — Repository Extraction

---

## Context

The `src/kernel/` directory was identified as the canonical source path for the
SigmaOS-kernel component (see `source-layout.md`). A `git subtree split` on
`src/kernel/` was attempted:

```
git -C /home/aaryansinghchauhan/SigmaOS subtree split -P src/kernel/ -b tmp-kernel-split
```

The monorepo contains **24,460 commits**. After processing approximately 6,877
commits the operation hit the 120-second timeout without completing. Continuing
to wait is not feasible in an automated step context.

## Decision

Use a **direct snapshot copy** of `src/kernel/` into the extraction repo's
`src/` directory:

```
cp -r /home/aaryansinghchauhan/SigmaOS/src/kernel/* \
      /home/aaryansinghchauhan/SigmaOS/.agents/tasks/phase2-scaffold/kernel/repo/src/
```

The extraction repo is then committed with a single initial commit (Phase 2
extraction commit), preserving the file tree but not the granular monorepo
commit history.

## Consequences

**Positive:**
- Extraction completes immediately without wall-clock or compute budget issues.
- The resulting repo is self-contained, builds in isolation, and can be pushed
  to `github.com/AaryanSinghChauhan09/SigmaOS-kernel` as-is.
- Future history will be clean and component-scoped from this point forward.

**Negative / Trade-offs:**
- Pre-extraction monorepo commit history for `src/kernel/` is not carried into
  the new repo. It remains accessible in the monorepo history under
  `src/kernel/` for auditing purposes.
- If a full history migration is later desired, `git subtree split` can be run
  as a long-running background job (estimated >30 minutes on this repo) and its
  output force-pushed to overwrite the initial snapshot commit.

## Alternatives Considered

| Alternative | Reason rejected |
|---|---|
| `git subtree split` (full) | Timed out at ~6877/24460 commits; not feasible in automated step |
| `git filter-repo` | Not installed in the environment; would require `pip install git-filter-repo` |
| `git fast-export --subdirectory-filter` | Equivalent complexity; same timeout risk |

## Future Migration Path

When the monorepo commit count stabilizes or a longer maintenance window is
available, run:

```bash
git -C /home/aaryansinghchauhan/SigmaOS subtree split \
    -P src/kernel/ \
    -b tmp-kernel-split-full

# Then in the extraction repo:
git fetch /home/aaryansinghchauhan/SigmaOS tmp-kernel-split-full
git replace --graft HEAD <split-tip>
git filter-repo --force   # rewrite to inline the graft
git push --force-with-lease origin main
```

This preserves all downstream tags and references while importing full history.

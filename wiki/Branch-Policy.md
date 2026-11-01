# SigmaOS Branch Policy

> This document defines the official branch policy for all SigmaOS component repositories. All contributors and maintainers must follow this policy. PRs that violate this policy are rejected.

---

## The Fundamental Rule

**`main` contains only code. No `.md`, `.txt`, `.rst`, or other documentation files in `main`.**

Documentation lives on the `docs` branch, in the GitHub Wiki, or in the project's dedicated documentation repository.

This rule exists because:
1. Documentation changes should not trigger code CI pipelines
2. Code changes should not require documentation updates to merge
3. History of code and docs are often unrelated — separate branches keep history clean
4. Multi-repo automation that processes `main` should never encounter non-code files

---

## Branch Reference

### `main`

- **Purpose:** Production-ready, reviewed code
- **Allowed files:** Source code (`.rs`, `.c`, `.h`, `.py`, `.sh`, etc.), `Cargo.toml`, `Makefile`, `.github/` CI config, build scripts
- **Forbidden files:** `.md`, `.txt`, `.rst`, `.adoc`, changelogs, READMEs
- **Who can push:** No one directly. All changes via PR only.
- **Protection rules:**
  - Require PR with 1+ approvals
  - Require CI to pass
  - No force pushes
  - No branch deletion

### `docs`

- **Purpose:** All documentation for this component
- **Allowed files:** `.md`, `.txt`, `.rst`, `CHANGELOG`, `README`, API docs, spec files
- **Forbidden files:** Source code (never mix code into `docs`)
- **Who can push:** Maintainers directly, contributors via PR
- **Sync model:** The `docs` branch is synced from the `docs/` directory that is staged in the feature branch (moved to docs branch by CI after merge, never landing in main itself)

### `feature/*`

- **Pattern:** `feature/<short-description>` (e.g., `feature/delta-updates`, `feature/wayland-fractional-scaling`)
- **Purpose:** New feature development
- **Branch from:** `main`
- **Merges to:** `main` via PR
- **Lifetime:** Deleted after merge

### `fix/*`

- **Pattern:** `fix/<short-description>` (e.g., `fix/crash-on-empty-package-list`, `fix/panel-memory-leak`)
- **Purpose:** Bug fixes
- **Branch from:** `main` (or `release/*` for hotfixes)
- **Merges to:** `main` via PR (or `release/*` for hotfixes)
- **Lifetime:** Deleted after merge

### `refactor/*`

- **Pattern:** `refactor/<short-description>` (e.g., `refactor/extract-transaction-engine`)
- **Purpose:** Code refactoring without behavior changes
- **Branch from:** `main`
- **Merges to:** `main` via PR
- **Lifetime:** Deleted after merge

### `security/*`

- **Pattern:** `security/<cve-id-or-description>` (e.g., `security/cve-2027-12345`, `security/fix-signature-bypass`)
- **Purpose:** Security fixes, may be kept private until disclosure
- **Branch from:** `main` or `release/*`
- **Merges to:** `main` and relevant `release/*` branches
- **Lifetime:** Deleted after public disclosure + merge

### `release/*`

- **Pattern:** `release/v<version>` (e.g., `release/v1.0-rc1`, `release/v1.0`)
- **Purpose:** Release preparation — version bumps, changelog, final testing
- **Branch from:** `main` (at release cutoff)
- **Merges to:** `main` (via PR after release)
- **Lifetime:** Kept indefinitely as a release marker (do not delete release branches)
- **Protected:** Same rules as main (no direct pushes)

### `experiment/*`

- **Pattern:** `experiment/<description>` (e.g., `experiment/io-uring-backend`)
- **Purpose:** Experimental work not ready for review, proof of concepts
- **Branch from:** `main`
- **Merges to:** Never auto-merged. Requires an RFC and a clean rewrite as a `feature/*` PR.
- **Lifetime:** 90-day maximum. Automatically closed by bot if unmerged after 90 days.

---

## Branch Naming Rules

- Use **lowercase only** (no `Feature/MyFeature`)
- Use **hyphens**, not underscores (`feature/my-feature`, not `feature/my_feature`)
- Keep names **concise** (< 50 characters)
- Avoid redundant prefixes like `sigma-` in the branch name (the repo is already `sigma-pkg`)
- Be **descriptive**: `fix/crash-on-empty-list` is good; `fix/crash` is not

---

## Merge Strategy

### Squash Merge (default for `feature/*` and `fix/*`)

Use squash merge when:
- The PR contains many "work in progress" commits that should be cleaned up
- The feature is a single logical unit

```bash
# GitHub UI: "Squash and merge"
# CLI:
git checkout main
git merge --squash feature/my-feature
git commit -m "feat(pkg): add delta update support"
```

### Rebase Merge (for carefully structured multi-commit PRs)

Use rebase merge when:
- The PR has multiple logical commits, each with a meaningful message
- The contributor has kept a clean history

```bash
# GitHub UI: "Rebase and merge"
```

### No merge commits on `main`

**Merge commits are never used on `main`.** The `main` branch history is a clean, linear sequence of meaningful commits.

---

## Documentation File Policy (Detailed)

### Files ALLOWED in `main`

```
*.rs         Rust source
*.c / *.h    C source
*.py         Python scripts
*.sh         Shell scripts
Makefile     Build files
Cargo.toml   Rust manifest
Cargo.lock   Rust lockfile (for binaries, not libraries)
*.toml       Configuration schemas (not changelogs)
*.yaml/*.yml CI/CD configuration
*.json       JSON schemas and configuration
.gitignore   Git ignore rules
.editorconfig Editor configuration
```

### Files FORBIDDEN in `main`

```
*.md         Markdown (README, CHANGELOG, docs)
*.txt        Plain text
*.rst        ReStructuredText
CHANGELOG    Changelog file
README       Readme
CONTRIBUTING Contributing guide (lives in wiki)
LICENSE      License file (exception — see below)
```

**Exception:** `LICENSE` file (required by GitHub) is permitted in `main`. This is the only documentation-like file allowed.

### Where Documentation Goes

| Document Type | Location |
|---------------|----------|
| Component README | `docs` branch, `README.md` |
| API documentation | Generated from rustdoc, published to docs site |
| Changelog | `docs` branch, `CHANGELOG.md` |
| Spec/design docs | `docs` branch, `docs/` directory |
| Wiki pages | GitHub Wiki |
| Contributing guide | GitHub Wiki |
| Architecture | GitHub Wiki |

---

## Cleanup Policy

### Automatic Branch Deletion

- Feature/fix/refactor/security branches are **automatically deleted after merge**
- This is enforced by GitHub's "Automatically delete head branches" setting (enabled on all repos)
- Do not re-create merged branches — start a new branch if follow-up work is needed

### Stale Branch Policy

- Branches with no commits for **60 days** receive a warning bot comment
- Branches with no commits for **90 days** are flagged for deletion
- Maintainers review flagged branches monthly and delete stale ones
- Exception: `release/*` branches are kept indefinitely

### Rebasing Stale Feature Branches

If your feature branch is > 2 weeks old and `main` has moved significantly:

```bash
git fetch origin
git rebase origin/main
# resolve conflicts
git push --force-with-lease origin feature/your-branch
```

Use `--force-with-lease` not `--force`. It refuses to overwrite if someone else has pushed to your branch.

---

## The Single-Main-Branch Rule

SigmaOS uses **one and only one `main` branch** per repository. There is no `develop`, `staging`, `integration`, or `trunk` branch.

Rationale:
- Multiple integration branches create confusion about "what's the current truth"
- Feature branches are short-lived; they don't need a persistent integration target
- CI validates every PR against `main` before merge — that is the integration step
- If `main` breaks, the build is red and nothing merges until it's fixed

### What This Means in Practice

- Work on `feature/*` branches
- Test locally, then open a PR to `main`
- CI must pass before merge
- Main is always green

---

## Force Push Policy

**Force push is banned on `main`, `docs`, and `release/*` branches.**

Force push is allowed on:
- Your own feature/fix branches (but use `--force-with-lease`)
- Never after sharing your branch with others

### How to Fix a Bad Commit on `main`

You cannot force-push. Instead:

```bash
# Revert the bad commit
git revert <bad-commit-hash>
git push origin main  # via PR

# Or, if you need to remove sensitive data:
# Open a security incident — this requires special handling
```

---

## Branch Policy Enforcement

### Pre-receive Hook (CI)

A CI check runs on every PR and verifies:
1. Branch name matches an allowed pattern
2. No `.md` or `.txt` files are added to `main`
3. Commit messages follow conventional commit format

### Violations

PRs that violate branch policy are:
1. Automatically labeled `policy-violation`
2. Blocked from merge by CI
3. The contributor is notified with a link to this document

### Reporting Issues

If you believe this policy is causing a problem, open a Discussion with the `governance` label. Policy changes require an RFC.

---

## Quick Reference Card

```
Working on a feature?     git checkout -b feature/your-feature
Fixing a bug?             git checkout -b fix/what-is-fixed
Security fix?             git checkout -b security/cve-or-description
Experimenting?            git checkout -b experiment/what-youre-trying
Writing docs?             Make sure they go to docs branch, not main
Adding a README?          docs branch only — never main
Merging?                  Squash or rebase — never merge commit
Done?                     PR auto-deletes your branch after merge
```

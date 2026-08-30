# Pull Requests Integration Guide

This page documents the integration process for SigmaOS pull requests and branch management.

## Branch Strategy

SigmaOS uses a **single-main-branch** strategy:

*   All development happens in feature branches
*   Feature branches are merged into `main` and deleted
*   Only `main` exists as a long-lived branch

## Merge Policy

When merging branches:

1.  **Prefer improvement** - take the version that adds more capability
2.  **Keep custom implementations** - never reintroduce libc/std dependencies
3.  **Security first** - if both versions exist, prefer the more secure one
4.  **No conflicts in main** - resolve all conflicts before merging

## Integrated Branches (History)

| Branch | Topic | Commits |
|--------|-------|------|
| `improve-installer-script-*` | Reliability & Observability | Multiple |
| `improve-package-manager-*` | Package manager architecture | Multiple |
| `improve-sigmaos-systemd-*` | Syntax fixes | Multiple |
| `improve-sshd-*` | Dynamic crypto salting | 1 |
| `jules-*-7e7b3d2e` | FreeBSD/OpenBSD/Gentoo/NixOS improvements | 1 |
| `jules-*-73ce6847` | Memory safety verification | 1 |
| `jules-*-03d7127e` | AI Integration Phase 4 | 1 |
| `jules-*-ccefedb8` | Sandboxie/Firejail sandboxing | 1 |
| `jules-*-82aa0a51` | CLI REPL improvements | 1 |
| `AaryanSinghChauhan09-patch-*` | CI/CD workflow additions | 14+ |

## Creating a Pull Request

1.  Create a feature branch:

```bash
git checkout -b feat/my-feature
```

2.  Make changes with custom implementations (no std deps):

```bash
# Implement feature
git add -A
git commit -m "feat: describe your feature"
```

3.  Push and create PR:

```bash
git push origin feat/my-feature
gh pr create --title "feat: My Feature" --body "Description"
```

4.  After merge, delete the branch:

```bash
git push origin --delete feat/my-feature
```

## Code Review Checklist

*   \[ ] No `use std::` imports (use `klib` equivalents)
*   \[ ] No `unwrap()` without SAFETY comment
*   \[ ] All `unsafe` blocks have `// SAFETY:` comment
*   \[ ] New modules added to `src/lib.rs`
*   \[ ] Documentation added to `docs/` and `wiki_repo/`
*   \[ ] Tests added to `tests/`
*   \[ ] Security implications considered

# PULL REQUEST TEMPLATE

---
name: Pull Request
about: Submit a new Sovereign Shard or bug fix
---

# Pull Request

## Summary

<!-- One-line description of what this PR does -->

## Type

- [ ] New Shard

- [ ] Bug Fix

- [ ] Documentation Update

- [ ] GitHub Actions / CI improvement

## Closes

<!-- Reference the ROADMAP.md milestone, IDEAS_BACKLOG.md item, or issue number -->

Closes #

## Shard Checklist (New Shards Only)

- [ ] C++ OOP Singleton with `getInstance()`

- [ ] `extern "C"` wrappers for all public functions

- [ ] Registered in `SovereignUSR` via `usr_register_shard()`

- [ ] `cppcheck` passes with zero warnings

- [ ] Wiki page created or updated in `SigmaOS.wiki/`

- [ ] `IDEAS_BACKLOG.md` or `MISSING_COMPONENTS.md` updated

## Bug Fix Checklist

- [ ] Root cause identified and documented

- [ ] Regression test described

- [ ] No new `cppcheck` warnings introduced

## Testing

<!-- Describe how you tested this change -->

## Screenshots / Serial Output

<!-- Paste sigma_log output or screenshots if applicable -->

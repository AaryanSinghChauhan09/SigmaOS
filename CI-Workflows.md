# SigmaOS CI/CD Workflows Reference

All automation lives under `.github/workflows/`. Each workflow is scoped to a single concern.

## Workflow Summary

| File | Trigger | Purpose |
|------|---------|---------|
| `ci.yml` | PR / push to `main` | Cross-arch CMake + kernel build matrix |
| `pr_quality_gate.yml` | PR to `main` | Format, lint, license, unit-tests, security, size-budget |
| `pr_labeler.yml` | PR opened/updated | Area labels, size labels, first-contributor welcome |
| `release.yml` | Tag push / cron / manual | Reproducible images, signed release, channel promotion |
| `stale.yml` | Daily cron | Mark and close inactive issues/PRs |
| `security.yml` | Push, PR, weekly cron | CodeQL, cargo-audit, Trivy, Gitleaks |
| `benchmarks.yml` | Daily cron / manual | Microbenchmarks, flaky-test detector, fuzzing |

## PR Quality Gate Jobs

```
commit-lint ──→ (all jobs)
format-check ──→ build-matrix ──→ unit-tests
license-check
clippy
clang-tidy
security
size-budget
```

## Release Pipeline

```
git tag v1.2.3
    └─→ build-images (x86_64, aarch64, riscv64gc)
            └─→ sign & attach SHA-256 + CycloneDX SBOM
                    └─→ git-cliff CHANGELOG
                            └─→ GitHub Release (softprops)
```

Channel promotion (manual `workflow_dispatch`):
```
nightly → beta → stable
(each requires environment approval in GitHub Settings)
```

## Labels

Labels created by `pr_labeler.yml` and `.github/labeler.yml`:

| Label | Trigger |
|-------|---------|
| `area/kernel` | changes under `kernel/` |
| `area/drivers` | changes under `drivers/` |
| `area/tools-cli` | changes under `tools/` |
| `area/docs` | `.md` files, `docs/`, `wiki_repo/` |
| `area/ci` | `.github/` |
| `area/web-ui` | HTML/CSS/JS and `web_ui/` |
| `area/security` | kernel security, armor, policy |
| `area/rust` | any `.rs` file |
| `size/XS` | < 30 added lines |
| `size/S` | 30–99 lines |
| `size/M` | 100–299 lines |
| `size/L` | 300–999 lines |
| `size/XL` | 1000+ lines |

## Secrets Required

| Secret | Used by |
|--------|---------|
| `GITHUB_TOKEN` | All workflows (auto-provided) |

No third-party tokens required for the base setup.

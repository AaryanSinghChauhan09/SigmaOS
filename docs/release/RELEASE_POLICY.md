# SigmaOS Release Engineering Policy

## Overview

SigmaOS classifies releases into strict maturity tiers based on observable runtime evidence rather than narrative roadmap claims.

## Release Maturity Tiers

| Release Tier | Required Observable Evidence |
|---|---|
| **Experimental** | Kernel compiles cleanly (`cargo check --lib`), standalone unit tests pass (`./run_sigma_tests.sh`). |
| **Developer Preview** | QEMU x86_64 boot validation reaches shell/diagnostic state (`tests/boot/boot_to_shell_test.py`). |
| **Test Distribution** | Clean VM ISO installation and recovery rollback verified (`installer/tests/dry_run.rs`). |
| **Release Candidate** | Full test suite pass, security advisory audit clean, zero critical regressions. |
| **Stable** | Hardware matrix evidence verified, automated update/rollback tested, security policy active. |

## Release Engineering Workflows

Releases are prepared using `./scripts/release/prepare_release.sh` and orchestrated via `.github/workflows/release.yml`.

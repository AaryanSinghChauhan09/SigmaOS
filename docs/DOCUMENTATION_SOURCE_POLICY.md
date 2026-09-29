# SigmaOS Documentation Source Policy

## Purpose
This document establishes the official source of truth and management rules for all SigmaOS documentation, wiki pages, and architecture guides.

## Canonical Source of Truth
1. The `docs/` directory in the primary SigmaOS Git repository is the **single canonical source of truth** for all documentation.
2. Mirror directories (e.g., `wiki/`, `wiki_content/`, `WIKI/`, `wiki_repo/`) are generated or synced from `docs/` and must not be manually edited directly.
3. Any changes to documentation or architecture specifications must be submitted as PRs targeting markdown files within `docs/`.

## Enforcement
- CI workflows automatically verify that manual edits are not made directly to `wiki/` or legacy mirror directories.
- Automated link checkers validate hyper-links across all documentation files during pull request builds.

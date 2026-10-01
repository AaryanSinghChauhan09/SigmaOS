# SigmaOS Documentation Source Policy

## Overview
This document defines the single-source-of-truth policy and governance rules for documentation within the SigmaOS repository.

## Canonical Source of Documentation
1. **Primary Documentation (`docs/`)**: All architecture specifications, operational guides, subsystem technical designs, and master roadmaps reside in the `docs/` directory.
2. **GitHub Wiki (`wiki/` and `wiki_repo/`)**: The `wiki/` directory contains topic-focused documentation formatted for GitHub Wiki deployment (Arch Linux-style topic organization).
3. **Synchronization**: Changes to core documentation must be mirrored or synchronized across `docs/`, `wiki/`, and `wiki_repo/` using `scripts/sync_wiki.sh` to maintain consistency.

## Rules for AI Agents and Contributors
- Do not make untracked manual edits in generated wiki output locations without updating the corresponding source documentation in `docs/` or `wiki/`.
- Maintain single-branch (`main`) cleanliness and clean git tree state before submitting PRs.
- All documentation files must be formatted in Markdown (`.md`) and pass `markdown-link-check` and internal structure validation checks.

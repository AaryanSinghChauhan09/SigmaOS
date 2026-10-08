# SIGMAOS DOCUMENTATION SOURCE POLICY

## Single Source of Truth
The `docs/` directory serves as the primary single source of truth for all official SigmaOS documentation, architectural specifications, and strategic roadmaps.

## Synchronization Rules
1. All documentation edits must take place in `docs/` or the root repository specification files.
2. Mirror directories (`wiki/`, `WIKI/`, `wiki_repo/`, `wiki_content/`) are automatically updated via `./scripts/sync_wiki.sh` and CI automation pipelines.
3. Direct manual edits to auto-generated wiki mirrors are prohibited and enforced via GitHub CI check gates.

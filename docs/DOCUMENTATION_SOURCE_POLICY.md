# Documentation Source Policy

## Overview
This policy defines the single source of truth for documentation in the SigmaOS repository.

## Rules
1. **`docs/` is the Single Source of Truth**: All user documentation, architecture decision records, guides, and manuals must be authored and maintained within the `docs/` directory.
2. **Auto-generated Mirrors**: The `wiki/` and `wiki_content/` directories are synchronized or auto-generated from `docs/`. Manual edits directly to `wiki/` or `wiki_content/` are prohibited and caught by CI documentation check workflows.
3. **Deprecated Mirrors**: Legacy directories (`WIKI/`, `wiki_repo/`) are deprecated and will be removed.

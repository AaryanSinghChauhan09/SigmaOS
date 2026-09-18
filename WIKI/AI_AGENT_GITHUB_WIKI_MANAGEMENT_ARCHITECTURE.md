# AI Agent GitHub Wiki Management Architecture

## Executive Overview

SigmaOS maintains a dual-repository documentation model for its GitHub Wiki. Technical documentation, architecture guides, and developer references are co-located in the main repository under `wiki/` and mirrored in `wiki_repo/` (the submodule or standalone Git repository used for GitHub Wiki publishing). Automated CI workflows (`.github/workflows/`) ensure zero documentation drift between the main source tree and the published GitHub Wiki pages.

This document serves as the architectural reference for AI coding agents creating, synchronizing, or maintaining GitHub Wiki pages for SigmaOS.

---

## Dual-Repository Wiki Architecture & Synchronization Flow

```
                                +-----------------------------------+
                                |     AI Agent / Developer Edit     |
                                +-----------------------------------+
                                                  |
                                                  v
                                +-----------------------------------+
                                |      wiki/ Documentation Root      |
                                |     (Main Repository Source)      |
                                +-----------------------------------+
                                                  |
                                                  | File Copy / Synchronization
                                                  v
                                +-----------------------------------+
                                |     wiki_repo/ Publishing Submodule|
                                |       (GitHub Wiki Upstream)      |
                                +-----------------------------------+
                                                  |
                                                  | Git Push via GitHub Workflow
                                                  v
                                +-----------------------------------+
                                |    GitHub Wiki Web Interface      |
                                | (https://github.com/.../wiki)     |
                                +-----------------------------------+
```

### Core Components & Conventions

1. **Dual Directory Structure**:
   - `wiki/`: In-repo primary source of truth for all wiki Markdown files.
   - `wiki_repo/`: Mirrored copy configured for GitHub Wiki deployment.

2. **Landing Page Indices (`Home.md`)**:
   - Both `wiki/Home.md` and `wiki_repo/Home.md` serve as the master wiki index page.
   - Every newly created guide or architectural document MUST be listed on `Home.md` using double-bracket wiki links (e.g. `[[AI_AGENT_GITHUB_WIKI_MANAGEMENT]]`) or standard Markdown links.

3. **Markdown Standards**:
   - Use standard CommonMark Markdown syntax.
   - Section headers must be properly hierarchy-formatted (`#`, `##`, `###`).
   - Relative links to architectural documents in `docs/` must be explicit.

---

## Zero-Drift Guardrails

AI agents modifying documentation must adhere to these rules:
1. **Atomic Dual Edits**: Every document created or updated in `wiki/` MUST be simultaneously created or updated in `wiki_repo/`.
2. **Landing Page Index Maintenance**: Whenever a new guide is created, `wiki/Home.md` and `wiki_repo/Home.md` MUST be updated with an entry under the appropriate section.
3. **No External Broken Links**: Hyperlinks to repository code must reference valid relative file paths.

---

## Related Architectural References
- `wiki/Home.md` - Master wiki landing index.
- `wiki_repo/Home.md` - Mirrored wiki landing index.
- `.github/workflows/` - CI/CD deployment pipelines.

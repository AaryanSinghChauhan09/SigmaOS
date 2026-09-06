# AI Agent GitHub Wiki Management Guide

## Overview
This wiki guide details GitHub Wiki documentation management protocols for AI coding agents operating on SigmaOS. It covers dual-repository synchronization between `wiki/` and `wiki_repo/`, `Home.md` landing page index updates, Markdown hyperlink standards, and zero-drift documentation rules.

## Synchronization Protocol
1. **Source of Truth**: All wiki documents originate in `wiki/`.
2. **Mirror Deployment**: Documents are copied to `wiki_repo/` for GitHub Wiki publishing.
3. **Landing Index**: Every new page must be registered in `wiki/Home.md` and `wiki_repo/Home.md`.

## Example Sync
```bash
cp wiki/AI_AGENT_GITHUB_WIKI_MANAGEMENT.md wiki_repo/AI_AGENT_GITHUB_WIKI_MANAGEMENT.md
```

## Related Documents
- `docs/AI_AGENT_GITHUB_WIKI_MANAGEMENT_ARCHITECTURE.md`
- `docs/AI_AGENT_GITHUB_WIKI_MANAGEMENT_GUIDELINES.md`
- `wiki/Home.md`

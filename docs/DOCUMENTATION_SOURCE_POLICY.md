# Documentation Source of Truth Policy

**Version:** 1.0
**Status:** Draft
**Last Updated:** 2025-01-22
**Purpose:** Define the canonical documentation hierarchy and synchronization process

---

## 1. Documentation Hierarchy

SigmaOS uses a single-source-of-truth documentation model:

```
docs/                    # Canonical authoritative source (EDIT ONLY HERE)
├── AGENTS.md           # Architecture guide for AI agents
├── ARCHITECTURE_DECISIONS.md  # ADR records
├── PROJECT_STATUS.md   # Implementation status
├── ROADMAP.md          # Execution roadmap
├── RELEASE_CRITERIA.md # Release gate
└── *.md                # All other documentation

wiki_content/           # Generated/synchronized content (DO NOT EDIT)
├── *.md                # Auto-generated from docs/

wiki/                   # Publication mirror (DO NOT EDIT)
├── *.md                # Synchronized to GitHub Wiki

WIKI/                   # Legacy mirror (PENDING REMOVAL)
└── *.md                # Stale copy, do not use

wiki_repo/              # Legacy export mirror (PENDING REMOVAL)
└── *.md                # Stale copy, do not use
```

---

## 2. Editing Rules

### 2.1 DO

- **Edit only files in `docs/`**
- All documentation changes must start in `docs/`
- Use markdown with clear headings and structure
- Link to other docs using relative paths
- Include AI agent maintenance instructions where relevant

### 2.2 DO NOT

- **Do not edit `wiki_content/` directly** - it will be overwritten
- **Do not edit `wiki/` directly** - it will be overwritten
- **Do not edit `WIKI/`** - it is deprecated
- **Do not edit `wiki_repo/`** - it is deprecated
- **Do not maintain multiple independent copies** of the same document

---

## 3. Synchronization Process

### 3.1 Automatic Synchronization

A CI job will:

1. Detect changes in `docs/`
2. Generate or update `wiki_content/` from `docs/`
3. Sync `wiki/` to GitHub Wiki
4. Check for stale copies in legacy directories
5. Validate all internal links
6. Fail CI on any synchronization error

### 3.2 Manual Synchronization (Temporary)

Until CI automation is implemented:

```bash
# Sync docs to wiki_content
./scripts/sync_docs_to_wiki.sh

# Sync wiki_content to GitHub Wiki
gh wiki-create wiki_content/*.md

# Check for stale copies
./scripts/check_stale_docs.sh
```

---

## 4. Link Validation

### 4.1 Internal Links

All internal links should use relative paths:

```markdown
See [Architecture Decisions](ARCHITECTURE_DECISIONS.md) for details.
```

### 4.2 External Links

External links should be validated periodically:

```bash
# Check for broken links
./scripts/check_broken_links.sh
```

---

## 5. Migration Timeline

### Phase 1: Current (2025-01-22)
- Establish `docs/` as canonical source
- Document policy
- Begin consolidating duplicates

### Phase 2: Near Future
- Implement CI synchronization
- Remove stale legacy copies
- Add broken-link checks to CI

### Phase 3: Future
- Remove `WIKI/` directory
- Remove `wiki_repo/` directory
- Full automation of documentation pipeline

---

## 6. Validation

### 6.1 Pre-Commit Checklist

Before committing documentation changes:

1. [ ] Edited only files in `docs/`
2. [ ] All internal links are relative
3. [ ] No duplicate content across multiple files
4. [ ] AI agent instructions included where relevant
5. [ ] Status classifications updated in PROJECT_STATUS.md

### 6.2 CI Validation

CI will verify:

- No manual edits to `wiki_content/`
- No manual edits to `wiki/`
- All internal links resolve
- No broken external links
- No stale copies in legacy directories

---

## 7. References

- [ADR-003: Documentation Source of Truth](ARCHITECTURE_DECISIONS.md#adr-003-documentation-source-of-truth)
- [PROJECT_STATUS.md](PROJECT_STATUS.md)
- [GitHub Wiki Documentation](https://docs.github.com/en/wikis)

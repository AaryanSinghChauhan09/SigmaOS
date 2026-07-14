# Contributing to SigmaOS — Docs

This document covers documentation-specific contribution guidelines for the SigmaOS `docs/` directory.

For general code contributions, see the top-level [CONTRIBUTING.md](../CONTRIBUTING.md).

---

## Table of Contents

- [Documentation Philosophy](#documentation-philosophy)
- [File Naming Conventions](#file-naming-conventions)
- [Markdown Style Guide](#markdown-style-guide)
- [Adding a New Document](#adding-a-new-document)
- [Reviewing Existing Docs](#reviewing-existing-docs)
- [Syncing to the Wiki](#syncing-to-the-wiki)

---

## Documentation Philosophy

SigmaOS documentation aims to be:

- **Accurate**: Reflect the current implementation state, not aspirational claims
- **Concise**: Prefer clear, short explanations over verbose prose
- **Structured**: Use headings, tables, and code blocks consistently
- **Sovereign**: All docs must work without external CDN, Google Fonts, or analytics embeds

---

## File Naming Conventions

| Type | Convention | Example |
| ---- | ---------- | ------- |
| Module README | `README.md` in the module directory | `modules/core/kernel/README.md` |
| Design documents | `Title-With-Kebab-Case.md` | `Kernel-Architecture.md` |
| Specs | `Title_With_Snake_Case.md` | `Networking_Stack_Spec.md` |
| API references | `API_Reference.md` | `API_Reference.md` |
| Internal docs | Uppercase with underscores | `SECURITY.md`, `CONTRIBUTING.md` |

---

## Markdown Style Guide

- Use ATX-style headings (`#`, `##`, `###`) — no underline style
- Surround headings with blank lines (MD022)
- Surround fenced code blocks with blank lines (MD031)
- Specify language for all fenced code blocks (MD040)
- Use relative links for in-repo documents
- Use table-of-contents sections for documents > 200 lines
- Maximum line length: 120 characters
- End all files with a single newline character

### Code Block Rules

```rust
// Good: specify language always
fn main() {}
```

---

## Adding a New Document

1. Choose the appropriate location (`docs/`, `modules/*/`, root)
2. Follow the file naming conventions above
3. Start with a single `# H1` heading (the document title)
4. Include a brief description paragraph under the title
5. Use a `## Table of Contents` for documents > 200 lines
6. Run `py scripts/maintenance/fix_markdown_lint.py` before committing

---

## Reviewing Existing Docs

When reviewing documentation PRs:

- Verify all code examples compile/run (mark as `<!-- compile-tested -->` if verified)
- Check all internal links are not broken
- Ensure content matches the current implementation in source files
- Flag aspirational/future content with an `> [!NOTE] Planned Feature` callout

---

## Syncing to the Wiki

Premium documentation pages are mirrored to the GitHub Wiki at `wiki_repo/`. To sync a `docs/` page to the wiki:

1. Copy the file to `wiki_repo/<Title>.md`
2. Update any relative links to wiki-relative links
3. Commit both files together:
   ```bash
   git add docs/My-Doc.md wiki_repo/My-Doc.md
   git commit -m "docs: add My-Doc to docs and sync to wiki"
   ```

---

## Related

- [CONTRIBUTING.md](../CONTRIBUTING.md) — Root contribution guide
- [Coding-Standards.md](Coding-Standards.md) — Code style
- [Building-from-Source.md](Building-from-Source.md) — Build setup

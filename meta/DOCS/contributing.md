# Contributing to SigmaOS — Meta Documentation

This document explains how to contribute to the internal metadata under the `meta/` directory.

The `meta/` directory contains internal governance, maintainer-facing documentation, and structured metadata consumed by tooling. It is distinct from:

- `docs/` — Engineering specifications and user-facing technical documentation
- `wiki_repo/` — GitHub Wiki content synchronized to the public wiki


---

## What Belongs in `meta/DOCS/`

| File | Purpose |
| ---- | ------- |
| `SECURITY.md` | Internal security policy, incident response procedures |
| `MAINTAINERS` | List of module owners and their areas of responsibility |
| `contributing.md` | Internal contributing policy (this file) |

Only content that is **maintainer-facing** or **tooling-consumed** should live here.

---

## File Format Requirements

All markdown files in `meta/DOCS/` must:

- Use UTF-8 encoding
- Use LF (`\n`) line endings
- Pass `markdownlint` with the repo's `.markdownlint.json` config
- Not contain external image embeds or CDN dependencies


---

## Adding New Metadata Files

1. Create the file in `meta/DOCS/`
2. Add the file to this index (update the table above)
3. If the file is tooling-consumed, document the tool that reads it in a comment at the top of the file
4. Commit with prefix `meta: add <filename> — <purpose>`


---

## Security Metadata

Security-related metadata files (`SECURITY.md`) contain:

- Internal incident response contacts
- CVE triage procedures
- Private disclosure channels


These files should not duplicate the public-facing [SECURITY.md](../../SECURITY.md) in the repository root, but may cross-reference it.

---

## Related

- [Root CONTRIBUTING.md](../../CONTRIBUTING.md)
- [docs/CONTRIBUTING.md](../../docs/CONTRIBUTING.md)
- [SECURITY.md](SECURITY.md)

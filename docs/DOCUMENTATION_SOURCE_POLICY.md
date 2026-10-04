# Documentation Source and Sync Policy

## Where documentation lives

- `docs/` is the source of truth for repository engineering policies, project status, architecture decisions, and cross-component plans.
- The GitHub Wiki is the user-facing reference for component behavior and procedures. Each OS component has one canonical page containing current status, design references, validation evidence, limitations, and its future roadmap.
- `wiki/` is the checked-in mirror of the GitHub Wiki. `SigmaOS.wiki/` is the local Git checkout used to publish wiki commits. There is no automatic sync between these locations.
- Every tracked repository Markdown file is also represented in the Wiki. Non-Wiki source documents are imported as `Repository-*` reference pages with a link to their original path; [17-Repository-Documents](../wiki/17-Repository-Documents.md) provides the full inventory. Imported pages preserve source material, while canonical component pages remain the maintained source for current OS behavior and roadmaps.

## Editing and publication

1. Update the repository source document or component page first.
2. Keep its GitHub Wiki page and checked-in `wiki/` mirror in sync when either is changed.
3. Update navigation and internal links in the same change; remove obsolete duplicate pages after migrating useful material to the canonical page.
4. Check changed links, `git diff --check`, and relevant tests before pushing.
5. Do not mark proposed or model-only work as integrated or supported. Add exact test commands, target/configuration, and limitations.

Avoid maintaining multiple independent roadmaps for one component. The top-level roadmap should link to component pages instead of repeating their work lists.

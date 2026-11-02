# Governance: SigmaOS-docs

## 1. Scope and Responsibility

- **Primary responsibility:** SigmaOS-docs owns the documentation structure and publishing pipeline for SigmaOS: the user and admin handbook, wiki parity with the implementation, API documentation generation, and the contribution guides for documentation writers. It defines the documentation taxonomy that all other SigmaOS repos must follow when documenting their components.
- **Runtime layer:** documentation
- **Scope boundary with adjacent repos:** Every other SigmaOS repo owns the source-level documentation in its own repo; this repo owns the aggregation, cross-repo navigation, published website, and the contribution guide that standardizes how all repos document themselves.

## 2. Public API / Contract

| Interface | Type | Stability |
|-----------|------|-----------|
| `docs.sigmaos.org` website | Website | stable |
| Documentation taxonomy schema | Schema | stable |
| `sigmaos-docs` build CLI | CLI | stable |
| Markdown frontmatter schema | Schema | stable |
| API docs generation pipeline | Build pipeline | unstable |
| Internal search index | internal | internal-only |

- **Breaking-change policy:** Taxonomy and frontmatter schema stable within a MAJOR version; URL structure stable to preserve links.
- **API documentation location:** This repo IS the documentation location; per-repo API docs link in here.

## 3. Dependencies and Upstream Interface

| Name | Min Version | Reason | Patched? |
|------|-------------|--------|----------|
| mdbook | 0.4 | handbook builder | No |
| mkdocs | 1.5 | wiki/site builder (alternative) | No |
| rustdoc / sphinx / typedoc | varies | API doc generators per language | No |
| markdownlint | latest | doc linting | No |
| vale | latest | prose linting | No |

- **Optional/soft dependencies:** `algolia`/`lunr` for search; `crowdin` for translation.
- **Upstream patch inventory:** None currently.
- **Dependency update policy:** Build tools reviewed quarterly; site regenerated on every doc merge.

## 4. Ownership and Maintainer

- **Primary maintainer:** @sigmaos-docs-team
- **Co-maintainers:** @core-lead, @ux-lead
- **Review quorum:** 2 maintainer approvals required; taxonomy changes require @core-lead approval.
- **Escalation path:** Documentation gaps that mislead users are `priority:high`.
- **Last ownership review date:** 2025-01-15

## 5. Test Status and CI Status

- **Unit test coverage:** N/A (documentation repo) — link checking and lint are the "tests".
- **Integration tests:** Present — doc build, link check, and frontmatter validation run in CI.
- **CI pipeline file:** `.github/workflows/docs-ci.yml`
- **Required passing checks before merge:** `markdownlint`, `link-check`, `build-site`, `frontmatter-validate`
- **Known flaky tests:** `link-check` occasionally flags external sites that are down (issue #3).
- **CI build badge:** `![docs CI](https://github.com/SigmaOS/SigmaOS-docs/actions/workflows/docs-ci.yml/badge.svg)`

## 6. Support Matrix

| Arch | Status |
|------|--------|
| N/A (documentation) | supported (arch-independent) |

| OS/kernel Target | Min Version | Status |
|------------------|-------------|--------|
| Build host (Linux) | any | supported |

- **Hardware classes:** N/A.
- **Backend variants:** Built on Linux CI; output is static HTML.

## 7. Release Maturity

- **Current maturity label:** `beta`
- **Last label change date:** 2025-01-01
- **Planned next label:** `beta → stable` at v1.0
- **Criteria for promotion:** Handbook complete for user/admin/dev paths, wiki parity confirmed with all component repos, API docs auto-generated for all repos with public APIs, search functional.

## 8. What This Repo Does NOT Do

- Does not own source-level API docs for individual components → owned by each repo (this aggregates)
- Does not implement SigmaOS features → handled by code repos
- Does not own code comments or inline docs → owned by each repo
- Does not define architecture decisions → owned by SigmaOS-core and ADRs
- Does not provide man pages source → owned by each repo (this links them)

## 9. Config Schema and Runtime Contract

- **Config file path(s):** `book.toml` (mdbook), `mkdocs.yml` (mkdocs), `frontmatter` schema
- **Schema file location:** `schemas/frontmatter.schema.json`, `schemas/taxonomy.schema.json`
- **Schema format:** TOML (book config), JSON (frontmatter), YAML (taxonomy)
- **Runtime environment variables:** N/A (build-time only)
- **Breaking config change policy:** Frontmatter and taxonomy schema additive within a MAJOR version.

## 10. Failure Modes and Recovery Path

| Failure | Cause | Symptom | Recovery step |
|---------|-------|---------|---------------|
| Build failure | Broken markdown or syntax | Site not published | Fix syntax; CI reports line numbers |
| Broken link | Moved or removed page | 404 on published site | `link-check` catches in CI; fix redirect or link |
| Stale API docs | Component repo didn't update docs | API docs out of sync | Enforce doc-parity policy in component repo PR |
| Search index corrupt | Bad build | Search returns no results | Rebuild site; reindex |
| Frontmatter invalid | Missing required fields | Page excluded from build | Validate frontmatter; add required fields |

- **Fallback behavior:** A broken page does not fail the whole build (skipped with error); previous published site retained until a successful build.
- **Operator escalation path:** Check CI logs; file issue with failing build link.
- **Crash dump artifacts:** Build log in CI artifacts.

## 11. Update / Rollback Behavior

- **Update mechanism:** Docs published on every merge to main; previous version retained and versioned.
- **Rollback mechanism:** Site is redeployed from previous commit; per-page rollback via git history.
- **Expected downtime on update:** None — static site redeployed atomically.
- **Pre-update state checks:** Build passes CI.
- **Post-update validation:** Link check and smoke test on published site.
- **Atomic update guarantee:** Yes — static site deploy is atomic.

## 12. Security Boundary

- **Privilege level required:** Build runs in CI (no runtime privileges); published site is static.
- **Trust boundary:** Trusts markdown from SigmaOS repos; external links marked `rel="noopener"`; does not embed untrusted scripts.
- **Attack surface:** Published website (static, low risk); build pipeline
- **Hardening measures:** CSP headers on published site; external links sandboxed; no user input processing.
- **Threat model reference:** `docs/threat-model.md` (low-risk component)
- **Known CVEs / mitigations:** None.
- **Security audit status:** Not applicable (static content).

## 13. Compatibility Matrix

| Repo | Version Range | Status |
|------|---------------|--------|
| All SigmaOS repos | current | tested (frontmatter compatibility) |

| Package | Version Range | Status |
|---------|---------------|--------|
| mdbook | 0.4+ | tested |
| markdownlint | latest | tested |
| vale | latest | tested |

- **Known incompatibilities:** None.
- **Forward-compatibility pledge:** Frontmatter and taxonomy schema stable for v1.x; URLs preserved.

## 14. Documentation Parity

- **User documentation status:** up-to-date (this repo IS the docs)
- **Developer/contributor docs status:** up-to-date | `CONTRIBUTING.md`
- **Man page or in-app help status:** N/A (this links man pages)
- **Documentation parity policy:** This repo defines the parity policy all other repos follow: every API or behavior change must include a doc update in the same PR.
- **Last documentation review date:** 2025-01-12

## 15. Public Issue Labels and Roadmap

All required labels apply. Repo-specific extra labels:

| Label | Meaning |
|-------|---------|
| `component:handbook` | User/admin handbook issues |
| `component:api-docs` | API documentation generation issues |
| `component:taxonomy` | Documentation taxonomy issues |
| `component:search` | Site search issues |
| `area:user` | User-facing docs |
| `area:admin` | Admin-facing docs |
| `area:dev` | Developer-facing docs |

- **Public roadmap / milestone link:** `https://github.com/SigmaOS/SigmaOS-docs/milestones`
- **Issue triage cadence:** Every Wednesday

## 16. Versioning Policy

- **Versioning scheme:** Semantic Versioning (`MAJOR.MINOR.PATCH`) — tracks the documentation structure, not content.
- **Pre-release suffix rules:** `-alpha.N`, `-beta.N`, `-rc.N`
- **Version tagging convention:** `vMAJOR.MINOR.PATCH` signed tags
- **Changelog format:** Conventional Commits; CHANGELOG auto-generated
- **Breaking change signal:** MAJOR bump for taxonomy or frontmatter schema incompatibility.
- **Version pinning in dependents:** Component repos reference the taxonomy version their frontmatter targets.

## 17. Interaction with SigmaOS Core Repo

- **Relationship type:** peer — SigmaOS-docs documents SigmaOS-core's APIs and architecture; core's API changes must be reflected here.
- **IPC / integration mechanism:** None at runtime; documentation is generated from core's D-Bus introspection and source; aggregation via build pipeline.
- **Core API version it targets:** `SigmaOS-core >= 0.4` (documented version)
- **Integration tested in CI:** yes — docs build pulls core's API introspection to verify doc parity.
- **Change coordination policy:** Core API changes require a doc PR in this repo (or in core's repo linked here); doc-parity CI check enforces this.
- **Core team review required for cross-boundary changes:** yes

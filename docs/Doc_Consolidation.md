# SigmaOS Documentation Consolidation Plan

> Addresses doc sprawl: multiple overlapping roadmaps, architecture files, and
> competitive analysis docs that make the project harder to trust at first glance.
> Based on recommendation from external technical review, July 2026.

---

## The Problem

The repo accumulated overlapping docs through rapid iteration:

```
Architecture duplicates:
  ARCHITECTURE.md          ← canonical ✅
  Architecture.md          ← duplicate (same content, different case)
  docs/Design.md           ← design philosophy (keep, different scope)
  wiki_repo/Architecture.md, Architecture_Overview.md,
  Architecture_Philosophy.md, ARCHITECTURE_ROADMAP.md,
  ARCHITECTURE_WHITEPAPER.md, Sovereign-Architecture.md,
  SigmaOS_Architecture.md ...  ← wiki mirrors, mostly OK

Roadmap duplicates:
  ROADMAP.md               ← canonical ✅ (new, phase-based)
  roadmap.md               ← legacy (lowercase, older)
  DEVELOPMENT_ROADMAP.md   ← branch-centric (branches gone, stale)
  PHASE_A_EXECUTION_CHECKLIST.md
  PHASE_B_EXECUTION_CHECKLIST.md
  PHASE_C_EXECUTION_CHECKLIST.md
  PHASE_G_ROADMAP.md
  wiki_repo/BRANCH_ROADMAP.md, ROADMAP_SEQUENCE.md,
  APEX_INFINITY_ROADMAP.md, EXPANSION_ROADMAP.md ...

Competitive analysis duplicates:
  docs/Competitive_Analysis.md  ← canonical ✅ (new)
  wiki_repo/COMPETITIVE_ANALYSIS.md
  wiki_repo/Competitive_Gaps.md, Competitive_Gaps_Analysis.md,
  COMPETITIVE_GAPS_PHASE_2.md, Competitive-Analysis.md,
  Competitive-Gap-Matrix.md, Competitive-Gaps.md,
  Competitive-USPs.md, Competitor_Analysis.md,
  COMPETITOR_COMPARISON.md, Competitor-Matrix.md,
  Gap-Analysis.md, Gap-Analysis-vs-Competitors.md,
  GAP_ANALYSIS_INDUSTRIAL_COMPETITORS.md ...

CI Pipeline duplicates:
  .github/workflows/*.yml  ← canonical ✅
  wiki_repo/CI_Pipeline.md, CI_Pipeline_Guide.md,
  CI-Pipeline.md, CI-Workflows.md ...
```

---

## Canonical Files (Single Source of Truth)

| Topic | Canonical File | Supersedes |
|---|---|---|
| Architecture | `ARCHITECTURE.md` | `Architecture.md` (case dup) |
| Growth Roadmap | `ROADMAP.md` | `roadmap.md`, `DEVELOPMENT_ROADMAP.md`, `PHASE_*` |
| Competitive Analysis | `docs/Competitive_Analysis.md` | All wiki_repo/Competitive_* |
| Ideas Backlog | `docs/IDEAS_1000.md` | `wiki_repo/IDEAS_BACKLOG.md`, `SigmaOS_100_ITEM_BACKLOG.md`, `SigmaOS_1000_ITEM_BACKLOG.md` |
| OSS Reference | `docs/OSS_Reference_Map.md` | Scattered absorption docs |
| Minimal v0.1 Spec | `docs/Minimal_SigmaOS_v0.1.md` | Phase checklists |
| Download Guide | `DOWNLOAD.md` + `download.html` | Scattered release pages |
| Driver Strategy | `docs/Open_Source_Drivers.md` | `DRIVER_ECOSYSTEM.md`, `DRIVER_PORTING_PIPELINE.md` |
| Strategic Vision | `STRATEGIC_VISION.md` | `wiki_repo/SOVEREIGN_CONSTITUTION.md` (conceptually) |
| Contributing | `CONTRIBUTING.md` | `wiki_repo/CONTRIBUTING.md`, `Contribution_Guide.md` |
| Quick Start | `QUICKSTART.md` | `Getting-Started.md`, `Onboarding_Guide.md` |

---

## Action Plan

### Phase A — Root-level (do now, low risk)
- [x] Create `ROADMAP.md` as canonical phase-based roadmap
- [x] Create `DOWNLOAD.md` and `download.html`
- [x] Create `QUICKSTART.md`
- [ ] Add deprecation notice to `roadmap.md`: "See ROADMAP.md"
- [ ] Add deprecation notice to `DEVELOPMENT_ROADMAP.md`
- [ ] Delete `Architecture.md` (exact duplicate of `ARCHITECTURE.md`)

### Phase B — Wiki repo cleanup (batch PR)
The wiki has 500+ pages — many are valuable reference. The ones to consolidate:
- Remove or merge the ~15 overlapping Competitive_* pages into one
  `Competitive-Analysis.md` that points to `docs/Competitive_Analysis.md`
- Remove or merge the ~8 overlapping Architecture_* pages into one
  `Architecture-Overview.md` that points to `ARCHITECTURE.md`
- Archive `APEX_INFINITY_ROADMAP.md`, `EXPANSION_ROADMAP.md`,
  `ROADMAP_SEQUENCE.md` — redirect to `ROADMAP.md`
- Keep all profession-specific pages (they're not duplicates)
- Keep all shard manifest pages (they're not duplicates)
- Keep all security / PQC pages (distinct content)

### Phase C — Contributor guidelines
- Add to `CONTRIBUTING.md`: "Before creating a new doc, check this list
  of canonical files. New docs go in `docs/`. Wiki pages go in `wiki_repo/`."
- Add `docs/README.md` with a table of what's in `docs/`

---

## Why This Matters for Contributors

Doc sprawl is a trust signal. When a contributor opens the repo and finds
20 files claiming to be the architecture or roadmap, they ask:
- "Which one is current?"
- "Has this project been abandoned and restarted multiple times?"
- "Can I trust the documentation?"

A clean, single-source-of-truth structure answers: *this project is organised,
intentional, and ready for contributors.*

---

## What NOT to Remove

- `CHANGELOG.md` — unique history
- `FEATURE_MATRIX.md` — unique status table
- `CURRENT_PROBLEMS_MANIFEST.md` — unique bug tracker
- `GOVERNANCE.md` — unique governance policy
- `SECURITY_POLICY.md` + `SECURITY.md` — unique security disclosures
- `PROFILES.md` — unique profession profiles
- All `Profession-*.md` wiki pages — unique per-profession content
- All `Shard-*.md` wiki pages — unique shard reference
- `LANGUAGE_POLICY.md` — unique ABI guide
- `STRATEGIC_VISION.md` — unique high-level vision

---

*Created: July 2026 · Review after v0.1 release*

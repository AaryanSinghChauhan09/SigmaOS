# docs/ — SigmaOS Documentation Index

> Before creating a new doc, check this table.
> New documentation goes in `docs/`. Wiki pages go in `wiki_repo/`.
> Never create duplicates of the canonical files below.

---

## Canonical Files in docs/

| File | What it covers | Supersedes |
|------|----------------|------------|
| [Architecture.md](../Architecture.md) ← root | System layers, subsystems, profiles, directory map | `Architecture.md` (case dup in root) |
| [Competitive_Analysis.md](Competitive_Analysis.md) | Honest gap analysis vs Alpine/Arch/Ubuntu | All `wiki_repo/Competitive_*.md` |
| [Design.md](Design.md) | Core design philosophy and architectural decisions | — |
| [Doc_Consolidation.md](Doc_Consolidation.md) | Doc sprawl audit, canonical file table, cleanup plan | — |
| [Hardware_CI_Matrix.md](Hardware_CI_Matrix.md) | QEMU test matrix, real HW CI plan, driver smoke tests, benchmarks | — |
| [IDEAS_2000.md](IDEAS_2000.md) | Ideas 1001–2000 across 21 new categories | — |
| [IDEAS_1000.md](IDEAS_1000.md) | Ideas 1–1000 across 22 categories | — |
| [License_Map.md](License_Map.md) | Per-directory SPDX licensing, firmware blob policy, cleanroom statement | — |
| [Minimal_SigmaOS_v0.1.md](Minimal_SigmaOS_v0.1.md) | v0.1 bootable ISO component checklist, build commands, test matrix | `PHASE_*` checklists |
| [Open_Source_Drivers.md](Open_Source_Drivers.md) | Driver strategy: open/proprietary, SDF guide, roadmap v15→v17 | `DRIVER_ECOSYSTEM.md`, `DRIVER_PORTING_PIPELINE.md` |
| [OSS_Reference_Map.md](OSS_Reference_Map.md) | Subsystem-by-subsystem OSS reference map (cleanroom only) | Scattered `*_ABSORPTION_*.md` docs |

---

## Root-Level Canonical Files (not in docs/)

| File | What it covers |
|------|----------------|
| [README.md](../README.md) | Project overview, download links, quick build |
| [QUICKSTART.md](../QUICKSTART.md) | New user/contributor entry point |
| [ROADMAP.md](../ROADMAP.md) | Phase-based growth plan (v0.1 → v3.0) |
| [DOWNLOAD.md](../DOWNLOAD.md) | All 50+ format tables with build flags |
| [ARCHITECTURE.md](../ARCHITECTURE.md) | Canonical architecture reference |
| [INSTALL.md](../INSTALL.md) | QEMU demo, build profiles, troubleshooting |
| [CONTRIBUTING.md](../CONTRIBUTING.md) | Technical mandates, PR process, CI requirements |
| [STRATEGIC_VISION.md](../STRATEGIC_VISION.md) | Long-term vision and positioning |
| [FEATURE_MATRIX.md](../FEATURE_MATRIX.md) | Per-feature status (✅/🔄/⬜) across profiles |
| [CHANGELOG.md](../CHANGELOG.md) | Release history |
| [SECURITY_POLICY.md](../SECURITY_POLICY.md) | CVE disclosure and response policy |
| [GOVERNANCE.md](../GOVERNANCE.md) | Governance model |
| [LANGUAGE_POLICY.md](../LANGUAGE_POLICY.md) | Language domains, FFI rules, ABI guidelines |

---

## What Goes Where

| Content type | Location |
|---|---|
| Technical spec for a new subsystem | `docs/<SubsystemName>.md` |
| Wiki page for a feature/format | `wiki_repo/<Feature-Name>.md` |
| Root project document (contributing, security, etc.) | Root `*.md` |
| Phase checklist | `docs/Minimal_SigmaOS_v0.1.md` (or a new `docs/Phase_vX.Y.md`) |
| Competitive analysis update | Edit `docs/Competitive_Analysis.md` |
| New idea | Add to `docs/IDEAS_1000.md` |
| New OSS reference | Add to `docs/OSS_Reference_Map.md` |
| Driver guide | Edit `docs/Open_Source_Drivers.md` |
| License question | Edit `docs/License_Map.md` |

---

### If in doubt, open a GitHub Discussion before creating a new file.

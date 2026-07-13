# Documentation Migration Log

> **Status**: 🔄 Active | **Scope**: `Wiki Synchronization & Cleanup`

## 1. Executive Summary

To reduce duplicacy and redundancy across the SigmaOS ecosystem, all conceptual, architectural, and design documentation is being migrated to the GitHub Wiki. 

The main repository will strictly retain developer-centric files:
- `README.md` (Simplified project introduction)
- `CONTRIBUTING.md` (Build instructions, PR guidelines)
- `SECURITY.md` (Vulnerability reporting policy)
- `LICENSE`

---

## 2. Migration Log

| File Category | Origin | Action Taken | Result |
| :--- | :--- | :--- | :--- |
| **Architectural Specifications** | Main Repo (`/docs/architecture/`) | Migrated to Wiki (`Sigma_Kernel_Runtime.md`, etc.) | Duplicates deleted from main repo. Single source of truth established. |
| **OS Concept Roadmaps** | Main Repo (`COMPREHENSIVE_*.md`) | Migrated to Wiki | Roadmaps centralized in Wiki. |
| **Feature Summaries** | Main Repo (`README.md`) | Consolidated and moved to Wiki (`Feature_Implementation_Roadmap.md`) | `README.md` drastically simplified to focus on build instructions. |
| **Package Scripts** | Main Repo (`/scripts/packages/`) | Migrated to `sigma-pkg` binary documentation | Cleaner package handling, script bloat removed. |
| **Visual Assets** | Main Repo (`/docs/diagrams/`) | Fixed links & centralized in Wiki | Broken diagrams fixed. Images hosted once. |

---

## 3. Pending Cleanups

- [ ] Scan and flag partially implemented placeholder files in the main repo.
- [ ] Migrate `INSTALL.md` edge cases to the Wiki's Troubleshooting section.
- [ ] Implement CI/CD automation to detect `.md` duplication between repo and Wiki.

*All contributors: Please place new conceptual design documents directly in the Wiki, rather than opening a PR with a new `.md` file in the main repository.*

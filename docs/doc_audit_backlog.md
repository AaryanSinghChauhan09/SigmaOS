# SigmaOS Documentation Audit and Backlog

## 1. Repository Markdown Files Audit

| File Path | Status | Duplicated in Wiki | Priority | Action Plan |
| :--- | :--- | :--- | :--- | :--- |
| `README.md` | Implemented (Needs Update) | No | High | Implement comprehensive structural updates, setup detailed build/install commands, and update current release status. |
| `CODE_OF_CONDUCT.md` | Implemented | Yes | Low | Retain in repository root for GitHub integration; ensure matching wiki page exists. |
| `COMMUNITY.md` | Implemented | Yes | Medium | Clean up/consolidate repository-level community guide to Wiki; keep brief pointers in repo. |
| `LICENSE.md` / `LICENSES.md` | Implemented | Yes | Low | Keep licensing files in repository root as canonical legal files. |
| `SUPPORT.md` | Implemented | Yes | Medium | Consolidate conceptual support steps to Wiki; keep standard GitHub Support page in repo. |
| `THIRD-PARTY-NOTICES.md` | Implemented | No | Low | Keep in repository root as legal notice. |
| `sigma-build/Readme.md` | Partial | No | Medium | Expand build instructions, document compilation prerequisites and options. |
| `.kiro/specs/sigmaos-roadmap/design.md` | Partial | Yes | High | Move conceptual roadmap design to Wiki and simplify repo specification. |
| `.kiro/specs/sigmaos-roadmap/requirements.md` | Partial | Yes | High | Move conceptual requirements to Wiki and link under Master Roadmap. |
| `.kiro/specs/sigmaos-roadmap/tasks.md` | Partial | Yes | High | Consolidate tasks to Wiki Task list. |

---

## 2. GitHub Wiki Pages Audit

| Wiki Page Name | Status | Original File Source | Action Plan |
| :--- | :--- | :--- | :--- |
| `Master_Strategic_Roadmap` | Implemented | `Roadmap.md` | Keep updated with latest milestone shifts. |
| `Maturity_Parity_Roadmap` | Implemented | User plan | Reference in all strategic guides. |
| `SigmaFS_Innovations` | Implemented | User plan | Core feature documentation. |
| `SigmaMedia_Frameworks` | Implemented | User plan | Core feature documentation. |
| `Sigma_AI_Agents` | Implemented | User plan | Core feature documentation. |
| `Sigma_Kernel_Runtime` | Implemented | User plan | Core feature documentation. |
| `Advanced_Absorption` | Implemented | Master Index | Maintain as the single index page for all distro and OSS absorption pages. |

---

## 3. Prioritized Implementation Backlog

1. **README.md improvements**: Enhance quickstart options, add detailed sub-commands, render structural architecture diagram, and integrate new absorption milestones.
2. **sigma-build/Readme.md**: Complete missing build-chain troubleshooting steps and details on profile configurations.
3. **Audit/Deduplication**: Align repo community/support files with the corresponding Wiki pages, leaving only essential pointers in the repo.

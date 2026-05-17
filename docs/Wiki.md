# Wiki & Repo Improvements (Complete Architecture)

This document defines the documentation, repository, and community ecosystem improvements for SigmaOS.

## Documentation Structure
1. **Consolidated /docs/ Directory**: All subsystem specifications (Kernel.md, HAL.md, Storage.md, etc.) in a single authoritative location.
2. **Automated Wiki Sync**: `wiki_repo/` mirrors the docs directory; changes committed to main are auto-propagated to the GitHub Wiki via `tools/sync_all_branches.js`.
3. **Logic.md Architecture Map**: Comprehensive file relationship graph connecting every subsystem shard.
4. **Problems.md Bug Ledger**: Living document tracking all known bugs, resolutions, and audit statuses.

## GitHub Wiki Structure
5. **Home.md**: Project overview, quick-start guide, and navigation hub.
6. **Architecture Overview**: Mermaid diagrams showing kernel-to-userland shard dispatch flows.
7. **Subsystem Pages**: Dedicated pages for Kernel, HAL, Syscalls, IPC, Memory, Storage, Desktop, and Tools.
8. **Branch Guide**: Per-branch compilation targets and optimization profiles.
9. **Onboarding Guide**: Developer contribution pipeline with code standards and build instructions.
10. **RFC Template**: Structured proposal format for new subsystem expansions.

## Community & Ecosystem
11. **sigma-build Repo**: Community-contributed build scripts in the SlackBuilds tradition.
12. **Contributor Guidelines**: CONTRIBUTING.md, CODE_OF_CONDUCT.md, and PR templates.
13. **RFC Process**: Structured Request for Comments with review stages: Draft → Under Review → Approved → Implemented.
14. **Bug Bounty Program**: Defined severity tiers and responsible disclosure process.
15. **gh-pages Portal**: Interactive desktop simulator, documentation demos, and live installer guides served from the gh-pages branch.

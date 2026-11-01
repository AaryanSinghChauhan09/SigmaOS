# SigmaOS Main Branch Cleanup Plan

**Created**: 2024-11-01  
**Target Branch**: `prepare/main-cleanup`  
**Status**: Planning phase - not yet executed

---

## Overview

This plan outlines the cleanup of the main branch to establish a clean, focused mono-repo structure. The goal is to:
1. Keep essential kernel, build tooling, and CI infrastructure in main
2. Move all design, planning, and architectural documentation to a dedicated `SigmaOS-docs` repository
3. Simplify the root directory by removing scattered documentation and design artifacts
4. Establish main as a production-ready, build-focused repository

---

## Phase 1: Files/Directories to Delete from Main

These items should be completely removed from the main branch (they are not being moved anywhere—they are temporary artifacts or will be replaced):

### Build Artifacts & Temporary Files
- `build/` — Cargo build output (ephemeral, regenerated on build)
- `target/` — Rust target directory (ephemeral, regenerated on build)
- `test_*.html` / `test_*.* files (test harnesses, not needed in main)

### Executable/Binary Artifacts (not source)
- `sovereign_security_and_hardening_advancements_pr` — Large binary file (~7.3 MB)
- `test_dfbsd_v62`, `test_dfbsd_v63`, `test_omarchy` — Test binaries
- All executable test binaries in the repository root

### Rationale
- Build outputs are generated during CI/build and should not be committed
- Binary test artifacts are stale and not source code
- These reduce repository bloat and prevent accidentally committing generated content

---

## Phase 2: Files/Directories to Move to SigmaOS-docs Repository

All design, planning, and architectural documentation should migrate to `SigmaOS-docs/` (new repo). This includes:

### Design Documents & Architecture
**From root level:**
- `ARCHITECTURE.md` (if exists at root)
- `PRINCIPLES.md` (if exists at root)
- `COMPONENTS.md` (if exists at root)
- `CAPABILITY_MATRIX.md` (if exists at root)

**From `docs/` directory (all .md files):**
- `docs/ARCHITECTURE_OVERVIEW.md`
- `docs/DISPATCHER_PLAN.md`
- `docs/HARDWARE_TARGETS.md`
- `docs/ImprovementPlan.md`
- `docs/INTERRUPT_HANDLER_PLAN.md`
- `docs/ipc-handler.md`
- `docs/NEXT_STEPS_GUIDELINES.md`
- `docs/PHYSICAL_MEMORY_MANAGER_PLAN.md`
- `docs/PROCESS_MANAGER_PLAN.md`
- `docs/PROCESS_SYNCHRONIZATION_PLAN.md`
- `docs/PRODUCT_BRIEF.md`
- `docs/PR_PROPOSAL_REPEATABLE_AGENT_OPERATING_LOOP.md`
- `docs/PR_PROPOSAL_ROLE_BASED_GOVERNANCE_AND_OPERATING_PLAN.md`
- `docs/PR_PROPOSAL_RUST_OSDEV_ECOSYSTEM_SYNTHESIS.md`
- `docs/README.md`
- `docs/RELEASE_CRITERIA.md`
- `docs/SCHEDULER_DEVELOPMENT_PLAN.md`
- `docs/SCHEDULER_STATUS_AND_INVARIANTS.md`
- `docs/SECURITY_ARCHITECTURE.md`
- `docs/SELF_SUFFICIENCY_GUIDE.md`
- `docs/SIGMAOS_GUI_DEVELOPMENT_PLAN.md`
- `docs/SIGNAL_HANDLER_AUDIT_AND_ABI_PROFILE.md`
- `docs/SIGNAL_HANDLER_PLAN.md`
- `docs/SOFTWARE-CATEGORY-MATRIX.md`
- `docs/SOVEREIGN_OS_ABSOLUTE_OMNIPRESENT_SELF_SUFFICIENCY_ULTRA_ENCYCLOPEDIA_V61.md`
- `docs/SYSTEM_UTILITIES_DEVELOPMENT_PLAN.md`
- `docs/thread-manager.md`
- `docs/V1_SCOPE.md`
- `docs/VISION.md`
- `docs/WHAT_IS_WORKING_AND_NOT_WORKING.md`

### Wiki Directories
- `wiki/` — GitHub wiki content
- `WIKI/` — Alternate wiki directory
- `wiki_repo/` — Local wiki repository mirror

### Rules & Matrix Documentation at Root Level
- `CAPABILITY_MATRIX.toml` — Feature/capability matrix
- `COMPONENT_RULES.toml` — Component definition rules
- `COMPONENTS.toml` — Component registry
- `FEATURE_STATUS.toml` — Feature tracking status
- `IO_RULES.toml` — I/O subsystem rules
- `MEMORY_RULES.toml` — Memory management rules
- `PROCESS_RULES.toml` — Process management rules
- `SCHEDULER_RULES.toml` — Scheduler rules
- `SHELL_RULES.toml` — Shell rules
- `STORAGE_RULES.toml` — Storage rules
- `VFS_RULES.toml` — Virtual filesystem rules

### Research & Planning
- `research/` — Research directory with explorations
- `.jules/` — Planning/notes directory (palette.md, bolt.md, sentinel.md)
- `reports/` — Analysis and reporting documents
- `manual/` — Manual pages and documentation
- `images/` — Documentation images (not code assets)
- `index.html` — Old web UI/documentation index

### Rationale
- These are all documentation, planning, and reference materials—not production source
- Separating them into a dedicated docs repo keeps main focused on code and CI
- Users can still access all planning docs via the docs repository link
- Rules and matrices are development-time references, not runtime dependencies

---

## Phase 3: Items to Keep in Main Branch

### Cargo & Rust Configuration
- `Cargo.toml` — Workspace manifest (kept as is)
- `Cargo.lock` — Lock file
- `rust-toolchain.toml` — Rust version pinning

### Source Code
- `src/` — All kernel source code
- `tests/` — All test suites
- `kernel/` — Kernel-specific code

### Build Tooling & Scripts
- `tools/` — Build tools and utilities
- `.cargo/` — Cargo configuration
- `Makefile` — Build orchestration
- `run_sigma_tests.sh` — Test runner script

### System Components (non-documentation)
- `bin/` — Binaries/executables
- `boot/` — Boot loader code
- `shell/` — Shell implementation
- `etc/` — System configuration
- `usr/` — User-level utilities
- `install/` — Installation scripts
- `installer/` — Installer code
- `iso_root/` — ISO building infrastructure
- `pkgbuild/` — Package building
- `templates/` — Code/config templates
- `components/` — Component implementations
- `zenith_desktop/` — Desktop environment (source)
- `web_ui/` — Web UI source
- `fuzz/` — Fuzzing harnesses
- `default/` — Default configuration
- `thread/` — Threading implementation

### Git & CI/CD
- `.git/` — Version control metadata
- `.github/` — GitHub workflows, issue templates, and CI configuration
- `.github/workflows/` — All CI workflows (kept as is)
- `.github/CODEOWNERS`, `ISSUE_TEMPLATE/`, etc.
- `.gitignore` — Git ignore rules
- `.devcontainer/` — Development container config

### Kiro & Development Environment
- `.kiro/` — Kiro configuration
- `.meta.json` — Metadata file

### Scripts (build/maintenance related)
- `add_alloc_import.sh`
- `complete_remaining_tasks.sh`
- `FIX_TESTS.sh`

### Rationale
- These are essential for building, testing, and maintaining the kernel
- They are source code, not documentation
- CI/workflows are needed for main branch operations
- Configuration is runtime-critical

---

## Phase 4: Files Requiring Review/Decision

The following files exist and their status is ambiguous:

- `zenith_desktop.css` — Desktop CSS (keep in main if desktop is a primary feature, otherwise move to docs)
- `PULL_REQUEST_TEMPLATE.md` — GitHub template (keep in main; it's infrastructure, not documentation)
- `.meta.json` — Metadata (keep; used by build system)

**Decision**: Keeping these for now, as they support development infrastructure or are CSS for a shipped component.

---

## Expected Main Branch Structure After Cleanup

```
SigmaOS/
├── .github/
│   ├── workflows/              # CI/CD orchestration
│   ├── CODEOWNERS
│   ├── ISSUE_TEMPLATE/
│   ├── PULL_REQUEST_TEMPLATE.md
│   └── labeler.yml
├── .kiro/                       # Kiro dev config
├── .cargo/                      # Cargo config
├── .devcontainer/               # Dev environment
├── .gitignore
├── .meta.json
├── bin/                         # Executables
├── boot/                        # Boot code
├── components/                  # Component source
├── default/                     # Default configs
├── etc/                         # System config
├── fuzz/                        # Fuzzing harnesses
├── install/                     # Installation scripts
├── installer/                   # Installer source
├── iso_root/                    # ISO infrastructure
├── kernel/                      # Kernel code
├── manual/                      # Runtime manual pages (NOT docs)
├── pkgbuild/                    # Packaging
├── scripts/                     # Build scripts
├── shell/                       # Shell implementation
├── src/                         # Core kernel source
├── templates/                   # Code templates
├── tests/                       # Test suites
├── thread/                      # Threading
├── tools/                       # Build tooling
├── usr/                         # User utilities
├── web_ui/                      # Web UI source
├── zenith_desktop/              # Desktop source
├── Cargo.lock
├── Cargo.toml
├── Makefile
├── PULL_REQUEST_TEMPLATE.md     # (in .github/)
├── rust-toolchain.toml
├── run_sigma_tests.sh
├── add_alloc_import.sh
├── complete_remaining_tasks.sh
├── FIX_TESTS.sh
└── zenith_desktop.css           # Desktop stylesheet
```

**Deletions (not moved, not kept):**
- `build/`, `target/` (ephemeral)
- All binary test artifacts (stale)
- `sovereign_security_and_hardening_advancements_pr` (artifact)

**Moved to SigmaOS-docs:**
- `docs/` → SigmaOS-docs/docs/
- `wiki/`, `WIKI/`, `wiki_repo/` → SigmaOS-docs/wiki/
- `*.toml` (CAPABILITY_MATRIX, COMPONENT_RULES, etc.) → SigmaOS-docs/reference/
- `research/`, `.jules/`, `reports/`, `manual/` (docs), `images/` → SigmaOS-docs/
- `index.html` → SigmaOS-docs/

---

## Implementation Timeline

### When to Execute
This cleanup should be executed **after** the SigmaOS-docs repository is created and populated with all moved content.

### Steps (to be executed in a later phase)
1. Ensure SigmaOS-docs repo exists and has all documentation content committed
2. Remove all files in "Phase 1: Delete" list
3. Remove all files in "Phase 2: Move" list
4. Verify main builds and tests still pass
5. Create a single commit summarizing the cleanup
6. Push to prepare/main-cleanup branch
7. Open PR for review and merge to main

---

## Rollback Considerations

- All moved content will be preserved in SigmaOS-docs (retrievable via git history there)
- Build artifacts in Phase 1 are ephemeral and will be auto-regenerated
- Git history in main will preserve all deleted content via commits (git log/git show)
- No data loss; this is a structural reorganization only

---

## Success Criteria

After cleanup, main branch should:
- ✅ Have only source code, build configuration, and CI/CD infrastructure
- ✅ Build and test successfully (`cargo build`, `cargo test`)
- ✅ Have a cleaner, more focused root directory
- ✅ Contain no orphaned binary artifacts or generated files
- ✅ Have all documentation accessible via SigmaOS-docs repository
- ✅ Pass CI/CD workflows unchanged
- ✅ Have a clear separation of concerns: code in main, docs in docs repo

---

## Notes

- This plan is prepared but **not yet executed**
- Execution requires confirmation from maintainers
- The SigmaOS-docs repository must be created and populated before cleanup proceeds
- This is a one-time structural reorganization to establish long-term maintainability

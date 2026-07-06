# Phase B Execution Checklist — Unified Engine (Automation, CLI, GUI)

Status: `[x]` done · `[~]` partial · `[ ]` not started
Repo: [SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS) · Wiki: [Phase B](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Phase-B-Execution-Checklist)

---

## Automation

| Task | Status | Files |
| ------ | -------- | ------- |
| `sigma_automation.sh` (backup, update, recovery, wiki-sync) | [x] | `scripts/sigma_automation.sh` |
| `sigma_git_sync.sh` (commit/push + wiki mirror) | [x] | `scripts/sigma_git_sync.sh` |
| Branch parity CI | [x] | `scripts/ci_branch_check.sh`, `.github/workflows/branch-parity.yml` |
| Multi-branch status report | [x] | `scripts/sigma_branch_sync.sh` |
| Doxygen → wiki index | [x] | `scripts/doxygen_wiki_export.sh` |

**Test:** `./scripts/sigma_automation.sh update` · `./scripts/ci_branch_check.sh`

---

## CLI & customization

| Task | Status | Files |
| ------ | -------- | ------- |
| Modular commands (`update`, `backup`, `sync`, `branch-check`) | [x] | `userland/tools/sigma_cli.cpp`, `scripts/sigma_cli_host.sh` |
| Aliases + profiles | [x] | `userland/tools/sigma_cli.cpp` |
| Declarative `~/.sigma_profile` template | [x] | `docs/examples/sigma_profile.example` |
| VFS profile load | [~] | `zenith_desktop/personalization/sigma_profile_engine.cpp` |
| Profile ↔ Zenith WM/theme apply | [x] | `zenith_desktop/zenith_unified_init.cpp` |

**Test:** `./scripts/sigma_cli_host.sh update` · `sigma-cli profile list`

---

## GUI (Zenith Toolkit)

| Task | Status | Files |
| ------ | -------- | ------- |
| Compositor loop (framebuffer + input poll) | [~] | `zenith_desktop/compositor/sigma_compositor.cpp` |
| Auto-tiling WM (`auto_tile`, BSP/master-stack) | [~] | `zenith_desktop/wm/sigma_tiling_wm.cpp` |
| Theme engine (light/dark, accent) | [~] | `zenith_desktop/theme/sigma_theme_engine.cpp` |
| Unified Zenith boot (`zenith_subsystem_init`) | [x] | `zenith_desktop/zenith_unified_init.cpp` |
| Tiling smoke test | [x] | `tools/zenith/sigma_tiling_test.cpp` |

**Test:** `./tools/zenith/build_tiling_test.sh`

---

## Branch parity & docs

| Task | Status | Files |
| ------ | -------- | ------- |
| `FEATURE_MATRIX.md` | [x] | `FEATURE_MATRIX.md` |
| Phase 7–8 roadmap | [x] | `docs/PHASE_7_8_ROADMAP.md` |
| Wiki subsystem guides | [x] | `wiki_repo/*.md` |
| Wiki CI on push | [x] | `.github/workflows/wiki-sync.yml` |

---

## Release gate

1. `./scripts/ci_branch_check.sh` passes on `main`.

2. `./scripts/sigma_automation.sh wiki-sync` before push.

3. Wiki Action green after `wiki_repo/**` merge.

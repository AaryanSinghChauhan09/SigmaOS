# Phase 7–8 Roadmap — Unified Automation, CLI, GUI & Branch Parity

Goal: leapfrog SteamOS, Clear Linux, NixOS, Fedora CoreOS, Flatcar, Solus, Rescuezilla, and RancherOS by **owning the full stack** and shipping one coherent automation + personalization surface across all branches.

Repo: [github.com/AaryanSinghChauhan09/SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS)

---

## 1. Core automation & CLI

| Deliverable | Status | Location |
| ------------- | -------- | ---------- |
| Automation engine (backup, update, recovery, wiki) | Done | `scripts/sigma_automation.sh` |
| GitHub sync (commit/push + wiki mirror) | Done | `scripts/sigma_git_sync.sh` |
| Modular CLI (profiles, aliases, automation bridge) | Done | `userland/tools/sigma_cli.cpp` |
| `sigma-cli update` → automation update-check | Done | `sigma_cli.cpp` |
| Host `sigma-cli` parity script | Done | `scripts/sigma_cli_host.sh` |

### Test

```bash
./scripts/sigma_cli_host.sh update
./scripts/sigma_automation.sh backup
./scripts/sigma_git_sync.sh --dry-run
```

---

## 2. GUI & personalization (Zenith Toolkit)

| Deliverable | Status | Location |
| ------------- | -------- | ---------- |
| Compositor loop (framebuffer + input poll) | In progress | `zenith_desktop/compositor/sigma_compositor.cpp` |
| Auto-tiling WM (`auto_tile`, BSP/master-stack) | In progress | `zenith_desktop/wm/sigma_tiling_wm.cpp` |
| Theme engine (light/dark, accent) | Partial | `zenith_desktop/theme/sigma_theme_engine.cpp` |
| `~/.sigma_profile` keys | Partial | `zenith_desktop/personalization/sigma_profile_engine.cpp` |
| Example profile | Done | `docs/examples/sigma_profile.example` |
| Tiling smoke test | Done | `tools/zenith/sigma_tiling_test.cpp` |

### Test

```bash
./tools/zenith/build_tiling_test.sh    # host smoke when gcc available

# In-guest: zenith_compositor_init(); zenith_compositor_run_loop();

```

---

## 3. Branch consistency

| Deliverable | Status | Location |
| ------------- | -------- | ---------- |
| Feature matrix | Done | `FEATURE_MATRIX.md` |
| Branch parity CI script | Done | `scripts/ci_branch_check.sh` |
| GitHub Actions workflow | Done | `.github/workflows/branch-parity.yml` |

### Test

```bash
./scripts/ci_branch_check.sh
```

---

## 4. GitHub Wiki integration

| Deliverable | Status | Location |
| ------------- | -------- | ---------- |
| Wiki mirror directory | Done | `wiki_repo/` |
| Auto-sync on push | Done | `.github/workflows/wiki-sync.yml` |
| Doxygen API export | Configured | `Doxyfile` → `docs/api/html/` |
| Doxygen → wiki stub export | Done | `scripts/doxygen_wiki_export.sh` |
| Subsystem guides | Done | `wiki_repo/*.md` (Networking, Containers, Boot, Zenith) |
| Contributing | Done | `CONTRIBUTING.md` |

### Test

```bash
./scripts/sigma_automation.sh wiki-sync
doxygen Doxyfile
./scripts/doxygen_wiki_export.sh
```

---

## 5. Competitive differentiation (execution order)

1. **Immutable base + Safe Mode** — match CoreOS/Flatcar; exceed with Fix-it menu (`sigma_boot_recovery_menu.c`).

2. **Sovereign net + pods** — match RancherOS; exceed with no containerd/docker dependency.

3. **Zenith desktop** — match Solus polish; exceed with auto-tiling + declarative profile.

4. **Build registry + git sync** — match NixOS reproducibility narrative via signed `.spkg` + CI provenance.

5. **Recovery automation** — match Rescuezilla via `sigma_automation.sh recovery-check` + future GUI assistant.

6. **Performance** — match Clear Linux via scheduler + PGO release profiles.

---

## 6. Phase 7–8 checklist

- [x] `FEATURE_MATRIX.md` + `ci_branch_check.sh`

- [x] Extend `sigma_automation.sh` (`update`, `wiki-sync` mirrors Phase 7–8 docs)

- [x] Extend `sigma_cli` / host wrapper (`update`, `branch-check`)

- [~] Compositor input event loop hardening

- [~] VFS-backed `~/.sigma_profile` load

- [x] Wiki pages: Phase 7–8, Feature Matrix, subsystem guides

- [~] Push to GitHub + wiki sync (run `./scripts/sigma_git_sync.sh` on maintainer machine)

See also: [PHASE_A_EXECUTION_CHECKLIST.md](../PHASE_A_EXECUTION_CHECKLIST.md), [COMPETITOR_COMPARISON.md](COMPETITOR_COMPARISON.md).

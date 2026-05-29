# Phase C Execution Checklist — Meta-Distro Integration

Every competitor distro becomes a **subsystem** under `sigma_meta_distro_init()`.

Status: `[x]` done · `[~]` partial · `[ ]` not started

---

### Integration goals (meta-platform)

- [~] `sigma_game_layer.c` → Proton/Wine compatibility (`sigma_game_launch_with_proton`, gamemode → sched)
- [~] `sigma_sched.c` + `sigma_sched_profiles.c` → silicon-aware scheduler bridge
- [~] `sigma_pkg_registry/` → sovereign build recipes + community `recipes/COMMUNITY.md`
- [~] `sigma_boot.c` + `sigma_immutable_root.c` → immutable root + Safe Mode
- [~] `sigma_pod_cli.cpp` → container orchestration (namespaces/cgroups)
- [~] `sigma_recovery.c` + `sigma_recovery_gui.c` → recovery wizard UI
- [~] `sigma_compositor.cpp` + `sigma_tiling_wm.cpp` → Zenith GUI + auto-tiling
- [x] `sigma_automation.sh` → automation engine
- [x] `sigma_cli.cpp` → modular CLI (update/backup/sync/profiles)
- [~] `~/.sigma_profile` → personalization (`sigma_profile_engine.cpp` parser)
- [x] `sigma_git_sync.sh` → GitHub sync
- [x] `FEATURE_MATRIX.md` + `ci_branch_check.sh` → branch parity
- [x] `sigma_meta_distro.c` → unified subsystem registry
- [x] `init/sigma_meta_boot.c` → profile-aware feature mask at boot
- [~] `zenith_desktop/wm/sigma_tiling.c` → C ABI shim over `sigma_tiling_wm.cpp`
- [~] `sigma_pkg_registry/sigma_pkg_recipe.c` → declarative `.spkg` recipe loader

### One-call bootstrap

```c
sigma_meta_distro_init(SIGMA_META_ALL_FEATURES);
/* or profile-driven: */
sigma_meta_boot_for_profile(PROFILE_DESKTOP);
```

---

### Unified engine

| Surface | Status | Entry |
|---------|--------|-------|
| Meta-distro init | [x] | `sigma_meta_distro_init(SIGMA_META_ALL_FEATURES)` |
| Automation | [x] | `scripts/sigma_automation.sh` |
| CLI | [x] | `sigma-cli` / `scripts/sigma_cli_host.sh` |
| Wiki | [x] | `wiki_repo/` + `.github/workflows/wiki-sync.yml` |

---

### Tests

```bash
./scripts/sigma_automation.sh meta-check
./scripts/ci_branch_check.sh
./scripts/sigma_branch_sync.sh --report
./scripts/sigma_git_sync.sh --dry-run
```

---

### Remaining milestones

1. Live Proton/Wine process spawn inside `sigma-pod` GPU profile.
2. PGO release targets wiring Clear Linux flags in top-level `Makefile`.
3. VFS read of real `~/.sigma_profile` on guest boot.
4. Graphical recovery wizard (framebuffer) atop `sigma_recovery_gui.c`.

See [docs/META_DISTRO_UNIFIED_ENGINE.md](docs/META_DISTRO_UNIFIED_ENGINE.md).

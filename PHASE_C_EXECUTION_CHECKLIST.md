# Phase C Execution Checklist — Meta-Distro Integration

Absorb competitor strengths as **sovereign subsystems** under one engine.  
Status: `[x]` scaffold · `[~]` partial · `[ ]` not started

---

## Integration goals (competitor → subsystem)

| Competitor | Subsystem | Status | Canonical path |
|------------|-----------|--------|----------------|
| SteamOS | Gaming / Proton layer | [~] | `kernel/subsystems/sigma_game_layer.c` |
| Clear Linux | Silicon-aware scheduler profiles | [~] | `kernel/scheduler/sigma_sched_profiles.c` |
| NixOS | Sovereign build registry (`.spkg`) | [~] | `sigma_pkg_registry/`, `include/security/sigma_pkg_registry.h` |
| Fedora CoreOS / Flatcar | Immutable root + Safe Mode | [~] | `kernel/core/boot/sigma_boot.c`, rollback shards |
| RancherOS | Native pod orchestration | [~] | `userland/tools/sigma_pod_cli.cpp` |
| Rescuezilla / SystemRescue | GUI recovery + snapshots | [~] | `kernel/recovery/sigma_recovery.c` |
| Solus / Ubuntu | Zenith GUI + personalization | [~] | `zenith_desktop/` |
| SlackBuilds | Community `.spkg` recipes | [~] | `sigma_pkg_registry/recipes/` |

---

## Unified engine surfaces

| Surface | Status | Notes |
|---------|--------|-------|
| Automation | [x] | `scripts/sigma_automation.sh` |
| CLI | [x] | `sigma_cli` + host wrapper |
| GUI / UX | [~] | Compositor + tiling + theme |
| Personalization | [~] | `~/.sigma_profile` |
| GitHub sync | [x] | `scripts/sigma_git_sync.sh` |
| Branch parity | [x] | `FEATURE_MATRIX.md` + CI |
| Wiki auto-sync | [x] | `wiki_repo/` + workflow |

---

## Tests

```bash
./scripts/ci_branch_check.sh
./scripts/sigma_branch_sync.sh --report
./scripts/sigma_automation.sh recovery-check
```

---

## Next milestones

1. Wire `sigma_game_layer` to container GPU passthrough policy.
2. Enable PGO + `sigma_sched_profiles` in release `Makefile` targets.
3. Signed recipe verification in `SovereignPkg_Register`.
4. VFS-backed `~/.sigma_profile` and recovery GUI wizard.

See [docs/META_DISTRO_UNIFIED_ENGINE.md](docs/META_DISTRO_UNIFIED_ENGINE.md).

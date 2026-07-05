# Meta-Distro Unified Engine

SigmaOS absorbs competitor distro strengths as **modular subsystems** under one sovereign engine.

## Bootstrap

```c
#include "sigma_meta_distro.h"

sigma_meta_distro_init(SIGMA_META_ALL_FEATURES);
/* Profile boot (Minimal / Developer / Desktop / Cloud / Mobile): */
sigma_meta_boot_for_profile(2); /* Desktop */
```

## Subsystem map

| Competitor | Module | Feature |
|------------|--------|---------|
| SteamOS | `sigma_game_layer.c` | Gaming + Proton hooks |
| Clear Linux | `sigma_sched.c` | Performance profiles |
| NixOS | `sigma_pkg_registry/` | Reproducible `.spkg` recipes |
| CoreOS / Flatcar | `sigma_immutable_root.c` + `sigma_boot.c` | Immutable root + Safe Mode |
| RancherOS | `sigma_pod_cli.cpp` | Native namespaces/cgroups |
| Rescuezilla | `sigma_recovery.c` + GUI | Rollback wizard |
| Solus | Zenith + `~/.sigma_profile` | Theme, tiling, personalization |

## Automation

```bash
./scripts/sigma_automation.sh meta-check
./scripts/ci_branch_check.sh
./scripts/sigma_git_sync.sh --dry-run
```

Canonical docs: [META_DISTRO_UNIFIED_ENGINE.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/META_DISTRO_UNIFIED_ENGINE.md)

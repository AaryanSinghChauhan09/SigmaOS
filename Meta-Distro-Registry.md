# Meta-Distro Registry

SigmaOS loads competitor-inspired features through **`sigma_meta_distro_init()`**.

```c
sigma_meta_distro_init(SIGMA_META_ALL_FEATURES);
```

## Subsystem map

| Distro | Module |
|--------|--------|
| SteamOS | `kernel/subsystems/sigma_game_layer.c` |
| Clear Linux | `kernel/scheduler/sigma_sched.c` |
| NixOS | `sigma_pkg_registry/` |
| CoreOS / Flatcar | `kernel/core/boot/sigma_immutable_root.c` |
| RancherOS | `userland/tools/sigma_pod_cli.cpp` |
| Rescuezilla | `kernel/recovery/sigma_recovery_gui.c` |
| Solus | `zenith_desktop/` |

## Verify

```bash
./scripts/sigma_automation.sh meta-check
```

Checklist: [Phase-C-Execution-Checklist](Phase-C-Execution-Checklist)

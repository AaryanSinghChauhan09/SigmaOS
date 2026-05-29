# SigmaOS Feature Matrix (Branch Parity)

Tracks **required subsystems** across `main`, `release/*`, and specialty branches.  
CI: `scripts/ci_branch_check.sh` · Wiki mirror: [Feature-Matrix](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Feature-Matrix)

Status: `✓` present · `~` partial · `—` not required for branch profile

## Core subsystems (all active branches)

| Subsystem | Canonical paths | main | release/standalone | release/cloud | release/microkernel | kernel-exp | drivers-dev |
|-----------|-----------------|------|--------------------|---------------|---------------------|------------|-------------|
| Networking stack | `kernel/net/sigma_net.c`, `kernel/net/sigma_net_socket.cpp` | ✓ | ✓ | ✓ | ✓ | ✓ | ~ |
| Socket ABI | `kernel/include/sigma_socket_abi.h` | ✓ | ✓ | ✓ | ✓ | ✓ | — |
| Container CLI | `userland/tools/sigma_pod_cli.cpp`, `include/sigma_pod_spec.h` | ✓ | ✓ | ✓ | ~ | ✓ | — |
| Orchestrator | `kernel/core/orchestrator/sigma_orchestrator.cpp` | ✓ | ✓ | ✓ | ✓ | ✓ | — |
| Boot / Safe Mode | `kernel/core/boot/sigma_boot.c`, `kernel/core/boot/sigma_boot_recovery_menu.c` | ✓ | ✓ | ✓ | ✓ | ✓ | — |
| Resilience | `kernel/resilience/sigma_micro_fallback.cpp`, `kernel/resilience/sigma_rollback.cpp` | ✓ | ✓ | ✓ | ✓ | ✓ | — |
| Automation | `scripts/sigma_automation.sh`, `scripts/sigma_git_sync.sh` | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Modular CLI | `userland/tools/sigma_cli.cpp` | ✓ | ✓ | ✓ | ~ | ✓ | ✓ |
| Cgroups | `kernel/core/process/sigma_cgroup.c` | ~ | ~ | ✓ | ~ | ✓ | — |
| Docs / matrix | `FEATURE_MATRIX.md`, `PHASE_A_EXECUTION_CHECKLIST.md` | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |

## Desktop / Zenith (standalone + desktop profiles)

| Subsystem | Canonical paths | main | release/standalone | release/cloud |
|-----------|-----------------|------|--------------------|---------------|
| Compositor | `zenith_desktop/compositor/sigma_compositor.cpp` | ✓ | ✓ | ~ |
| Auto-tiling WM | `zenith_desktop/wm/sigma_tiling_wm.cpp` | ~ | ✓ | — |
| Theme engine | `zenith_desktop/theme/sigma_theme_engine.cpp` | ~ | ✓ | — |
| Personalization | `zenith_desktop/personalization/sigma_profile_engine.cpp`, `~/.sigma_profile` | ~ | ✓ | — |
| Profile selector | `init/sigma_profile_selector.cpp` | ✓ | ✓ | ✓ |

## Competitive differentiation map

| Competitor | SigmaOS answer | Primary files |
|------------|----------------|---------------|
| SteamOS | Sovereign compositor + auto-tiling, no X11/Wayland debt | `zenith_desktop/` |
| Clear Linux | Silicon-aware scheduler + PGO builds | `kernel/core/scheduler/`, `Makefile` |
| NixOS | Signed `.spkg` registry + `sigma_git_sync.sh` provenance | `suites/S10_Registry/`, `scripts/` |
| Fedora CoreOS / Flatcar | Immutable base + Safe Mode + rollback | `kernel/resilience/`, `sigma_boot.c` |
| Solus / Ubuntu | Zenith theme + `~/.sigma_profile` | `zenith_desktop/personalization/` |
| Rescuezilla / SystemRescue | `sigma_automation.sh recovery-check` + Fix-it menu | `scripts/`, `sigma_boot_recovery_menu.c` |
| RancherOS | `sigma-pod run-native` namespaces/cgroups | `sigma_pod_cli.cpp`, `sigma_cgroup.c` |

## Branch profiles (CI)

| Git branch pattern | Profile name | Extra requirements |
|--------------------|--------------|-------------------|
| `main` | `core` | Full matrix rows above |
| `release/standalone` | `desktop` | Zenith compositor + WM + profile engine |
| `release/cloud` | `cloud` | Orchestrator + net stack; no WM required |
| `release/microkernel` | `microkernel` | Boot + syscalls; minimal userland |
| `kernel-exp` | `kernel` | All `kernel/` net + boot + cgroup |
| `drivers-dev` | `drivers` | `kernel/core/drivers/`, HAL |
| `*` (default) | `core` | Core subsystem files only |

## Verification

```bash
./scripts/ci_branch_check.sh
./scripts/ci_branch_check.sh --json
./scripts/sigma_automation.sh wiki-sync
```

## Phase C meta-distro (main branch)

| Module | Path | Status |
|--------|------|--------|
| **Registry hub** | `kernel/subsystems/sigma_meta_distro.c` | wired |
| Gaming layer | `kernel/subsystems/sigma_game_layer.c` | partial |
| Scheduler bridge | `kernel/scheduler/sigma_sched.c` | partial |
| Immutable root | `kernel/core/boot/sigma_immutable_root.c` | partial |
| Package registry | `sigma_pkg_registry/` | partial |
| Recovery + GUI | `kernel/recovery/sigma_recovery.c`, `sigma_recovery_gui.c` | partial |
| Zenith unified init | `zenith_desktop/zenith_unified_init.cpp` | wired |

See `PHASE_C_EXECUTION_CHECKLIST.md`.

Last updated: Phase B/C unified engine + branch parity.

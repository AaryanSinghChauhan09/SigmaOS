# Phase A Execution Checklist

Trackable file-level tasks for sovereignty + UX differentiation.
Status key: `[x]` done · `[~]` partial · `[ ]` not started

Full wiki version: https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Phase-A-Execution-Checklist

---

## Status Legend

| Symbol | Meaning |
|--------|---------|
| `[x]` | Implemented and committed |
| `[~]` | Header/stub present — full implementation pending |
| `[ ]` | Not yet started |

---

## Team Quick Reference

| Area | Primary Files | Verify |
|------|---------------|--------|
| Networking | `kernel/net/sigma_net.c`, `kernel/net/sigma_net_socket.cpp` | `gcc -DSIGMA_NET_HOST_TEST kernel/net/sigma_net.c -o sigma_net_test && ./sigma_net_test` |
| Pods | `userland/tools/sigma_pod_cli.cpp` | `sigma-pod run-native demo.spkg --all-ns --cpu=250 --mem=128` |
| Boot | `kernel/core/boot/sigma_boot.c`, `kernel/core/boot/sigma_boot_recovery_menu.c` | QEMU boot + rollback path |
| Zenith UI | `zenith_desktop/compositor/`, `zenith_desktop/wm/sigma_tiling_wm.cpp` | `sigma_wm_auto_tile()` after profile load |
| DevOps | `scripts/sigma_automation.sh`, `scripts/sigma_git_sync.sh` | `./scripts/sigma_git_sync.sh --dry-run` |
| Docs | `docs/`, `wiki_repo/`, `CONTRIBUTING.md` | `doxygen Doxyfile` (when configured) |

---

## 1. Networking Sovereignty (`kernel/net/`, `kernel/core/drivers/`)

| Task | Status | Owner | Files |
|------|--------|-------|-------|
| NIC TX hook (`nic_tx_packet`) wired to e1000 driver | `[x]` | Networking | `kernel/net/sigma_net.c`, `kernel/core/drivers/SovereignE1000.cpp` |
| RX delivery hook (`nic_rx_deliver` → `sigma_net_receive_frame`) | `[x]` | Networking | `kernel/core/network/SovereignNICDriver.cpp` |
| ICMP echo reply transmit enabled | `[x]` | Networking | `kernel/net/sigma_net.c` |
| TCP state machine + socket send/recv data path | `[~]` | Networking | `kernel/net/sigma_net_tcp.cpp`, `kernel/net/sigma_net_socket.cpp` |
| ARP resolution (replace stub) | `[~]` | Networking | `kernel/net/sigma_net_arp.cpp` |
| RX/TX buffer pools + `nic_init()` | `[x]` | Networking | `kernel/net/sigma_net.c` |
| `sigma_socket_open/send/recv` ABI | `[x]` | Networking | `kernel/include/sigma_socket_abi.h`, `kernel/net/sigma_net_socket.cpp` |
| Single socket ABI authority (remove duplicate APIs) | `[~]` | Networking | `kernel/net/sigma_net_socket.cpp`, `net/sockets/SovereignSocketAPI.cpp` |
| `SIGMA_SYS_SOCKET` syscall allocation | `[x]` | Kernel | `kernel/core/syscall/SovereignSyscall.cpp` |

**Tests**
```bash
# Host/dev (when cross-toolchain available)
make PROFILE=standalone iso
./qemu-boot.sh standalone
# In-guest: ping gateway, verify ICMP reply path logs
```

---

## 2. Container Orchestrator (`userland/tools/`, `kernel/core/orchestrator/`)

| Task | Status | Owner | Files |
|------|--------|-------|-------|
| `sigma-pod run` IPC spawn message | `[x]` | Orchestration | `userland/tools/sigma_pod_cli.cpp` |
| `sigma-pod run-native` namespace/cgroup spec | `[x]` | Orchestration | `userland/tools/sigma_pod_cli.cpp`, `include/sigma_pod_spec.h` |
| Orchestrator native spawn handler | `[~]` | Orchestration | `kernel/core/orchestrator/sigma_orchestrator.cpp` |
| Kernel cgroup CPU/mem/io enforcement | `[~]` | Orchestration | `kernel/core/process/sigma_cgroup.c` |
| Pod lifecycle logging (`sigma_pod.log`) | `[~]` | Orchestration | `userland/tools/sigma_pod_cli.cpp` (ring + VFS path) |

**Tests**
```bash
sigma-pod run-native demo.spkg --all-ns --cpu=250 --mem=128
sigma-pod list
sigma-pod stop 0
```

---

## 3. Bootloader Resilience (`kernel/core/`, `kernel/resilience/`)

| Task | Status | Owner | Files |
|------|--------|-------|-------|
| Rollback gate at early boot | `[x]` | Kernel/Boot | `kernel/core/sigma_kernel_main.c` |
| Resilient fallback shell entry | `[x]` | Kernel/Boot | `kernel/resilience/sigma_micro_fallback.cpp` |
| `SIGMA_MINIMAL_MODE` reduced boot path | `[x]` | Kernel/Boot | `kernel/core/sigma_kernel_main.c` |
| Boot stage API + safe-mode selector | `[~]` | Kernel/Boot | `kernel/core/system/SovereignBoot.cpp`, `kernel/core/boot/sigma_boot.c` |
| Bootloader UI "Fix it" menu | `[~]` | Kernel/Boot | `kernel/core/boot/sigma_boot_recovery_menu.c` |

**Tests**
```bash
# Simulate repeated failed boots (rollback counter path)
qemu-system-x86_64 -cdrom build/sigmaos.iso -serial stdio
```

---

## 4. Zenith Toolkit + Auto-Tiling (`zenith_desktop/`)

| Task | Status | Owner | Files |
|------|--------|-------|-------|
| Compositor init + fallback mode | `[x]` | GUI/UX | `zenith_desktop/compositor/sigma_compositor.cpp` |
| Auto-tiling WM (`auto_tile()` / BSP/columns/grid) | `[~]` | GUI/UX | `zenith_desktop/wm/sigma_tiling_wm.cpp` |
| Theme/widget engine | `[~]` | GUI/UX | `zenith_desktop/theme/sigma_theme_engine.cpp` |
| Profile engine (`~/.sigma_profile`) | `[~]` | GUI/UX | `zenith_desktop/personalization/sigma_profile_engine.cpp` |
| Input → compositor event loop hardening | `[ ]` | GUI/UX | `zenith_desktop/compositor/` |

**Tests**
```bash
make iso-secure   # or PROFILE=standalone
# Validate tiling layout transitions and theme apply
```

---

## 5. CLI, Customization & Automation (`userland/`, `scripts/`)

| Task | Status | Owner | Files |
|------|--------|-------|-------|
| Sovereign shell aliases/history | `[~]` | CLI | `userland/shell/sigma_shell.cpp` |
| Modular CLI profiles/aliases tool | `[x]` | CLI | `userland/tools/sigma_cli.cpp` |
| Automation engine (update/backup/recovery) | `[x]` | DevOps | `scripts/sigma_automation.sh` |
| GitHub sync helper | `[x]` | DevOps | `scripts/sigma_git_sync.sh` |
| Boot profile selector (Minimal/Desktop/Cloud) | `[x]` | Init | `init/sigma_profile_selector.cpp` |

**Tests**
```bash
./scripts/sigma_automation.sh backup
./scripts/sigma_automation.sh recovery-check
./scripts/sigma_git_sync.sh --dry-run
sigma-cli profile show
```

---

## 6. Documentation & Wiki

| Task | Status | Owner | Files |
|------|--------|-------|-------|
| Competitor comparison table | `[x]` | Docs | `docs/COMPETITOR_COMPARISON.md` |
| Differentiation blueprint | `[x]` | Docs | `docs/SIGMAOS_DIFFERENTIATION_BLUEPRINT.md` |
| Stability playbook (net/boot/pods) | `[x]` | Docs/Wiki | `docs/`, wiki `Stability-Playbook.md` |
| Phase A checklist (this file) | `[x]` | Docs | `PHASE_A_EXECUTION_CHECKLIST.md` |
| CURRENT_PROBLEMS manifest refresh | `[x]` | Docs | `CURRENT_PROBLEMS_MANIFEST.md` |
| Wiki sync via `wiki_repo/` CI | `[~]` | Docs | `.github/workflows/wiki-sync.yml`, `wiki_repo/` |
| Subsystem guides | `[x]` | Docs | `wiki_repo/*.md` |
| Doxygen export script | `[x]` | Docs | `scripts/doxygen_wiki_export.sh` |

---

## Phase 7–8 Summary (Unified Automation + GUI + Branch Parity)

Goal: leapfrog SteamOS, Clear Linux, NixOS, Fedora CoreOS, Flatcar, Solus, Rescuezilla, and RancherOS
by owning the full stack. See [docs/PHASE_7_8_ROADMAP.md](PHASE_7_8_ROADMAP.md) and [FEATURE_MATRIX.md](../FEATURE_MATRIX.md).

| Task | Status |
|------|--------|
| Feature matrix + `ci_branch_check.sh` | `[x]` |
| `sigma-cli update` / host wrapper | `[x]` |
| Compositor input event loop hardening | `[~]` |
| Wiki subsystem guides | `[x]` |
| Doxygen wiki export script | `[x]` |
| Push to GitHub + wiki sync | `[x]` |

### Core Automation & CLI

| Deliverable | Status | Location |
|-------------|--------|----------|
| Automation engine (backup, update, recovery, wiki) | Done | `scripts/sigma_automation.sh` |
| GitHub sync (commit/push + wiki mirror) | Done | `scripts/sigma_git_sync.sh` |
| Modular CLI (profiles, aliases, automation bridge) | Done | `userland/tools/sigma_cli.cpp` |
| `sigma-cli update` → automation update-check | Done | `sigma_cli.cpp` |
| Host sigma-cli parity script | Done | `scripts/sigma_cli_host.sh` |

```bash
./scripts/sigma_cli_host.sh update
./scripts/sigma_automation.sh backup
./scripts/sigma_git_sync.sh --dry-run
```

### GUI & Personalization (Zenith Toolkit)

| Deliverable | Status | Location |
|-------------|--------|----------|
| Compositor loop (framebuffer + input poll) | In progress | `zenith_desktop/compositor/sigma_compositor.cpp` |
| Auto-tiling WM (BSP/master-stack) | In progress | `zenith_desktop/wm/sigma_tiling_wm.cpp` |
| Theme engine (light/dark, accent) | Partial | `zenith_desktop/theme/sigma_theme_engine.cpp` |
| `~/.sigma_profile` keys | Partial | `zenith_desktop/personalization/sigma_profile_engine.cpp` |
| Example profile | Done | `docs/examples/sigma_profile.example` |
| Tiling smoke test | Done | `tools/zenith/sigma_tiling_test.cpp` |

```bash
./tools/zenith/build_tiling_test.sh    # host smoke when gcc available
# In-guest: zenith_compositor_init(); zenith_compositor_run_loop();
```

### Branch Consistency

| Deliverable | Status | Location |
|-------------|--------|----------|
| Feature matrix | Done | `FEATURE_MATRIX.md` |
| Branch parity CI script | Done | `scripts/ci_branch_check.sh` |
| GitHub Actions workflow | Done | `.github/workflows/branch-parity.yml` |

```bash
./scripts/ci_branch_check.sh
```

---

## Competitive Differentiation Execution Order

1. **Immutable base + Safe Mode** — match CoreOS/Flatcar; exceed with Fix-it menu (`sigma_boot_recovery_menu.c`).
2. **Sovereign net + pods** — match RancherOS; exceed with no containerd/docker dependency.
3. **Zenith desktop** — match Solus polish; exceed with auto-tiling + declarative profile.
4. **Build registry + git sync** — match NixOS reproducibility via signed `.spkg` + CI provenance.
5. **Recovery automation** — match Rescuezilla via `sigma_automation.sh recovery-check` + GUI assistant.
6. **Performance** — match Clear Linux via scheduler + PGO release profiles.

---

## Release Gates (All Branches)

Before merging to `main` or promoting a `release/*` branch:

1. Kernel/network/boot smoke path passes in CI (`sigma_ci.yml`)
2. Docs updated for every subsystem touched
3. `CURRENT_PROBLEMS_MANIFEST.md` reflects new status
4. Wiki page updated in `wiki_repo/` (auto-sync on push)

---

*See also: [SIGMAOS_DIFFERENTIATION_BLUEPRINT.md](SIGMAOS_DIFFERENTIATION_BLUEPRINT.md) · [COMPETITOR_COMPARISON.md](COMPETITOR_COMPARISON.md) · [PHASE_7_8_ROADMAP.md](PHASE_7_8_ROADMAP.md)*

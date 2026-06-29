# Phase A Execution Checklist

Trackable file-level tasks for sovereignty + UX differentiation.
Status key: `[x]` done · `[~]` partial · `[ ]` not started

Repo: [SigmaOS on GitHub](https://github.com/AaryanSinghChauhan09/SigmaOS)

---

## Quick Reference — Area → Files → Verify

| Area | Primary Files | Verify Command | 
| --- | --- | --- | 
| Networking | `kernel/net/sigma_net.c`, `kernel/net/sigma_net_socket.cpp` | `gcc -DSIGMA_NET_HOST_TEST kernel/net/sigma_net.c -o test && ./test` | 
| Pods | `userland/tools/sigma_pod_cli.cpp` | `sigma-pod run-native demo.spkg --all-ns --cpu=250 --mem=128` | 
| Boot | `kernel/core/boot/sigma_boot.c`, `sigma_boot_recovery_menu.c` | QEMU boot + rollback path | 
| Zenith UI | `zenith_desktop/compositor/`, `zenith_desktop/wm/sigma_tiling_wm.cpp` | `sigma_wm_auto_tile()` after profile load | 
| DevOps | `scripts/sigma_automation.sh`, `scripts/sigma_git_sync.sh` | `./scripts/sigma_git_sync.sh --dry-run` | 
| Docs | `docs/`, `wiki_repo/`, `CONTRIBUTING.md` | `doxygen Doxyfile` | 

---

## 1. Networking Sovereignty

| Task | Status | Files | 
| --- | --- | --- | 
| NIC TX hook wired to e1000 driver | `[x]` | `kernel/net/sigma_net.c`, `SovereignE1000.cpp` | 
| RX delivery hook → `sigma_net_receive_frame` | `[x]` | `kernel/core/network/SovereignNICDriver.cpp` | 
| ICMP echo reply transmit enabled | `[x]` | `kernel/net/sigma_net.c` | 
| TCP state machine + socket send/recv | `[~]` | `kernel/net/sigma_net_tcp.cpp`, `sigma_net_socket.cpp` | 
| ARP resolution (replace stub) | `[~]` | `kernel/net/sigma_net_arp.cpp` | 
| RX/TX buffer pools + `nic_init()` | `[x]` | `kernel/net/sigma_net.c` | 
| `sigma_socket_open/send/recv` ABI | `[x]` | `kernel/include/sigma_socket_abi.h` | 
| Single socket ABI authority (remove duplicates) | `[~]` | `kernel/net/sigma_net_socket.cpp`, `net/sockets/SovereignSocketAPI.cpp` | 
| `SIGMA_SYS_SOCKET` syscall allocation | `[x]` | `kernel/core/syscall/SovereignSyscall.cpp` | 

**Test:**
```bash
make PROFILE=standalone iso
./qemu-boot.sh standalone
# In-guest: ping gateway, verify ICMP reply path logs
```

---

## 2. Container Orchestrator

| Task | Status | Files | 
| --- | --- | --- | 
| `sigma-pod run` IPC spawn message | `[x]` | `userland/tools/sigma_pod_cli.cpp` | 
| `sigma-pod run-native` namespace/cgroup spec | `[x]` | `sigma_pod_cli.cpp`, `include/sigma_pod_spec.h` | 
| Orchestrator native spawn handler | `[~]` | `kernel/core/orchestrator/sigma_orchestrator.cpp` | 
| Kernel cgroup CPU/mem/io enforcement | `[~]` | `kernel/core/process/sigma_cgroup.c` | 
| Pod lifecycle logging (`sigma_pod.log`) | `[~]` | `userland/tools/sigma_pod_cli.cpp` | 

**Test:**
```bash
sigma-pod run-native demo.spkg --all-ns --cpu=250 --mem=128
sigma-pod list
sigma-pod stop 0
```

---

## 3. Bootloader Resilience

| Task | Status | Files | 
| --- | --- | --- | 
| Rollback gate at early boot | `[x]` | `kernel/core/sigma_kernel_main.c` | 
| Resilient fallback shell entry | `[x]` | `kernel/resilience/sigma_micro_fallback.cpp` | 
| `SIGMA_MINIMAL_MODE` reduced boot path | `[x]` | `kernel/core/sigma_kernel_main.c` | 
| Boot stage API + safe-mode selector | `[~]` | `kernel/core/system/SovereignBoot.cpp`, `sigma_boot.c` | 
| Bootloader UI "Fix it" menu | `[~]` | `kernel/core/boot/sigma_boot_recovery_menu.c` | 

**Test:**
```bash
# Simulate repeated failed boots
qemu-system-x86_64 -cdrom build/sigmaos.iso -serial stdio
```

---

## 4. Zenith Toolkit + Auto-Tiling

| Task | Status | Files | 
| --- | --- | --- | 
| Compositor init + fallback mode | `[x]` | `zenith_desktop/compositor/sigma_compositor.cpp` | 
| Auto-tiling WM (BSP/columns/grid) | `[~]` | `zenith_desktop/wm/sigma_tiling_wm.cpp` | 
| Theme/widget engine | `[~]` | `zenith_desktop/theme/sigma_theme_engine.cpp` | 
| Profile engine (`~/.sigma_profile`) | `[~]` | `zenith_desktop/personalization/sigma_profile_engine.cpp` | 
| Input → compositor event loop hardening | `[ ]` | `zenith_desktop/compositor/` | 

**Test:**
```bash
make iso-secure
# Validate tiling layout transitions and theme apply
```

---

## 5. CLI, Customization & Automation

| Task | Status | Files | 
| --- | --- | --- | 
| Sovereign shell aliases/history | `[~]` | `userland/shell/sigma_shell.cpp` | 
| Modular CLI profiles/aliases tool | `[x]` | `userland/tools/sigma_cli.cpp` | 
| Automation engine (update/backup/recovery) | `[x]` | `scripts/sigma_automation.sh` | 
| GitHub sync helper | `[x]` | `scripts/sigma_git_sync.sh` | 
| Boot profile selector (Minimal/Desktop/Cloud) | `[x]` | `init/sigma_profile_selector.cpp` | 

**Test:**
```bash
./scripts/sigma_automation.sh backup
./scripts/sigma_automation.sh recovery-check
./scripts/sigma_git_sync.sh --dry-run
sigma-cli profile show
```

---

## 6. Documentation & Wiki

| Task | Status | Files | 
| --- | --- | --- | 
| Competitor comparison table | `[x]` | `docs/COMPETITOR_COMPARISON.md` | 
| Differentiation blueprint | `[x]` | `docs/SIGMAOS_DIFFERENTIATION_BLUEPRINT.md` | 
| Phase A checklist | `[x]` | `PHASE_A_EXECUTION_CHECKLIST.md` | 
| CURRENT_PROBLEMS manifest | `[x]` | `CURRENT_PROBLEMS_MANIFEST.md` | 
| Wiki sync via CI | `[~]` | `.github/workflows/wiki-sync.yml`, `wiki_repo/` | 
| Subsystem guides | `[x]` | `wiki_repo/*.md` | 
| Doxygen export | `[x]` | `scripts/doxygen_wiki_export.sh` | 

---

## Phase 7–8 Summary (Unified Automation + GUI + Branch Parity)

Goal: leapfrog SteamOS, Clear Linux, NixOS, Fedora CoreOS, Flatcar, Solus, Rescuezilla, and RancherOS by owning the full stack.

| Task | Status | 
| --- | --- | 
| Feature matrix + `ci_branch_check.sh` | `[x]` | 
| `sigma-cli update` / host wrapper | `[x]` | 
| Compositor input event loop | `[~]` | 
| Wiki subsystem guides | `[x]` | 
| Doxygen wiki export | `[x]` | 
| Push to GitHub + wiki sync | `[x]` | 

### Core Automation & CLI

| Deliverable | Status | Location | 
| --- | --- | --- | 
| Automation engine (backup, update, recovery, wiki) | Done | `scripts/sigma_automation.sh` | 
| GitHub sync (commit/push + wiki mirror) | Done | `scripts/sigma_git_sync.sh` | 
| Modular CLI (profiles, aliases, automation bridge) | Done | `userland/tools/sigma_cli.cpp` | 
| `sigma-cli update` → automation `update-check` | Done | `sigma_cli.cpp` | 
| Host sigma-cli parity script | Done | `scripts/sigma_cli_host.sh` | 

```bash
./scripts/sigma_cli_host.sh update
./scripts/sigma_automation.sh backup
./scripts/sigma_git_sync.sh --dry-run
```

### GUI & Personalization (Zenith Toolkit)

| Deliverable | Status | Location | 
| --- | --- | --- | 
| Compositor loop (framebuffer + input poll) | In progress | `zenith_desktop/compositor/sigma_compositor.cpp` | 
| Auto-tiling WM (BSP/master-stack) | In progress | `zenith_desktop/wm/sigma_tiling_wm.cpp` | 
| Theme engine (light/dark, accent) | Partial | `zenith_desktop/theme/sigma_theme_engine.cpp` | 
| `~/.sigma_profile` keys | Partial | `zenith_desktop/personalization/sigma_profile_engine.cpp` | 
| Example profile | Done | `docs/examples/sigma_profile.example` | 
| Tiling smoke test | Done | `tools/zenith/sigma_tiling_test.cpp` | 

```bash
./tools/zenith/build_tiling_test.sh
# In-guest:
# zenith_compositor_init();
# zenith_compositor_run_loop();
```

### Branch Consistency

| Deliverable | Status | Location | 
| --- | --- | --- | 
| Feature matrix | Done | `FEATURE_MATRIX.md` | 
| Branch parity CI script | Done | `scripts/ci_branch_check.sh` | 
| GitHub Actions workflow | Done | `.github/workflows/branch-parity.yml` | 

```bash
./scripts/ci_branch_check.sh
```

### GitHub Wiki Integration

| Deliverable | Status | Location | 
| --- | --- | --- | 
| Wiki mirror directory | Done | `wiki_repo/` | 
| Auto-sync on push | Done | `.github/workflows/wiki-sync.yml` | 
| Doxygen API export | Configured | `Doxyfile → docs/api/html/` | 
| Doxygen → wiki stub export | Done | `scripts/doxygen_wiki_export.sh` | 
| Subsystem guides | Done | `wiki_repo/*.md` | 
| CONTRIBUTING | Done | `CONTRIBUTING.md` | 

```bash
./scripts/sigma_automation.sh wiki-sync
doxygen Doxyfile
./scripts/doxygen_wiki_export.sh
```

---

## Release Gates (all branches)

Before merging to `main` or promoting a `release/*` branch:

1. Kernel/network/boot smoke path passes in CI (`sigma_ci.yml`)
2. Docs updated for every subsystem touched
3. `CURRENT_PROBLEMS_MANIFEST.md` reflects new status
4. Wiki page updated in `wiki_repo/` (auto-sync on push)

---

*See also: [Differentiation Blueprint](Differentiation-Blueprint) · [Development Roadmap](Development-Roadmap) · [Gap Analysis](Gap-Analysis) · [Competitive Gap Matrix](Competitive-Gap-Matrix)*

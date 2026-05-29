# Phase A Execution Checklist

Trackable milestones for sovereignty + UX. Status: `[x]` done, `[~]` partial, `[ ]` open.

## Networking
- [x] TX hook `nic_tx_packet` → e1000
- [x] RX hook `nic_rx_deliver` → `sigma_net_receive_frame`
- [x] `SIGMA_SYS_SOCKET` syscall
- [ ] TCP state machine + ARP

## Containers
- [x] `sigma-pod run-native` spec
- [~] Orchestrator `spawnNativeContainer`
- [ ] Kernel cgroup enforcement

## Boot resilience
- [x] Rollback gate + resilient fallback
- [x] `SIGMA_MINIMAL_MODE`
- [~] `sigma_boot.c` coordinator
- [ ] Bootloader “Fix it” UI

## Zenith UX
- [x] Compositor scaffold + fallback
- [~] Auto-tiling WM
- [~] Theme engine + profile engine
- [ ] Input/compositor hardening

## Automation & CLI
- [x] `scripts/sigma_automation.sh`
- [x] `scripts/sigma_git_sync.sh`
- [x] `userland/tools/sigma_cli.cpp`

Canonical full checklist: [PHASE_A_EXECUTION_CHECKLIST.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/PHASE_A_EXECUTION_CHECKLIST.md)

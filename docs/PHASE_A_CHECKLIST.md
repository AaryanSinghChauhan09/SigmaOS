# Phase A Execution Checklist — Quick Reference

See full wiki: https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Phase-A-Execution-Checklist

## Status Legend
- [x] Done
- [~] Partial / In Progress
- [ ] Not Started

## 1. Networking
- [x] NIC TX/RX hooks (e1000)
- [x] ICMP echo reply
- [~] TCP state machine + socket ABI
- [~] ARP resolution
- [~] Single socket ABI (remove duplicates)

## 2. Containers
- [x] sigma-pod IPC spawn
- [x] sigma-pod run-native namespace/cgroup spec
- [~] Kernel orchestrator native spawn handler
- [~] Kernel cgroup CPU/mem/io enforcement

## 3. Boot Resilience
- [x] Rollback gate at early boot
- [x] Resilient fallback shell
- [x] SIGMA_MINIMAL_MODE boot path
- [~] Boot stage safe-mode selector
- [~] Bootloader "Fix it" menu

## 4. Zenith Desktop
- [x] Compositor init + fallback
- [~] Auto-tiling WM (BSP/grid)
- [~] Theme engine
- [~] ~/.sigma_profile engine
- [ ] Input event loop hardening

## 5. CLI & Automation
- [x] sigma-cli modular profiles
- [x] sigma_automation.sh
- [x] sigma_git_sync.sh
- [x] Boot profile selector
- [~] Shell aliases/history

## Release Gates
Every branch merge requires:
1. CI smoke tests passing (sigma_ci.yml)
2. Docs updated
3. CURRENT_PROBLEMS_MANIFEST.md updated
4. Wiki page synced

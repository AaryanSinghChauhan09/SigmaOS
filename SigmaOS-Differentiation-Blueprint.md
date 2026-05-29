# SigmaOS Differentiation Blueprint

This page defines how SigmaOS surpasses SteamOS, Clear Linux, NixOS, Fedora CoreOS, Flatcar, Solus, Rescuezilla, and RancherOS by combining:

- **Core sovereignty** (boot, net, containers, libc, drivers)
- **User-facing polish** (Zenith desktop, tiling, personalization, automation)

## Priority Execution Order

1. **Core hardening first**
   - complete NIC <-> stack RX/TX integration
   - complete native namespace/cgroup enforcement in orchestrator
   - keep rollback-driven safe mode as default resilience behavior
2. **Desktop and recovery polish**
   - Zenith compositing reliability, input determinism, tiling defaults
   - profile-driven personalization (`~/.sigma_profile`)
   - guided recovery workflow (rollback/snapshot/diagnostics)
3. **Ecosystem and transparency**
   - deterministic registry recipes and provenance checks
   - CI gating for kernel, drivers, network, and desktop paths
   - docs updated together with subsystem code changes

## Current Differentiators Already Landed

- Kernel networking TX hooked to sovereign e1000 path.
- RX delivery entry (`nic_rx_deliver`) routes frames into stack parser.
- Syscall supports `SIGMA_SYS_SOCKET` allocation path.
- Boot path checks rollback state and enters resilient fallback mode when required.
- `sigma-pod run-native` can send explicit namespace/cgroup intent to orchestrator.

## Biggest Remaining Gaps

- Broad hardware coverage (GPU/Wi-Fi/Bluetooth + platform matrix).
- Full orchestrator-side enforcement for native pod spec fields.
- End-to-end package registry and reproducible recipe lifecycle.
- Recovery UX maturity (guided tools vs. low-level fallback only).
- Zenith production hardening and cohesion.

## Success Metrics

- Boot success >= 99% across profile test matrix.
- Repeated boot failures always recover to known-good path.
- Native pod launch applies namespace/cgroup limits in kernel path.
- Desktop smoke tests pass on all release profiles.
- Every release ships with matched docs + tests + rollback evidence.


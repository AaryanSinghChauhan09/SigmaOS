# Roadmap

## Short-Term: Core Enhancements (Q3 2026)

- [x] Universal package wrapper (`sigma-pkg`).
- [ ] Kernel interrupts and basic timer handlers.
- [ ] Implement Zenith UI Wayland compositor rendering pipeline.
- [ ] Hardware Attestation (TPM 2.0 handshake) driver.
- [ ] Full FIPS-140 compliance tests.

## Mid-Term: Zenith UI & Security (Q4 2026)

- [x] Integrate `liboqs` (Kyber, Dilithium) into `SovereignPQC`.
- [ ] Zenith UI: Adaptive layouts, EGL/Vulkan integration.
- [ ] Capability-based process isolation.
- [ ] Implement global hotkeys daemon (Alt+Space, Alt+A).

## Long-Term: AI-Native & Self-Healing (Q1 2027)

- [ ] Build a functioning AI Assistant (Alt+A) daemon hooked into `sysfs`.
- [ ] BTRFS-based Self-Healing OS snapshot and rollback scripts.
- [ ] Kubernetes-native node layer out-of-the-box.
- [ ] Sigma Profiles: pre-configured UI/Kernel settings for gamers & devs.

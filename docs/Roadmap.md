# Roadmap

## Short-Term: Kernel & Core UI (Q3 2026)
- [x] Kernel interrupts and basic timer handlers.
- [x] Universal package wrapper (`sigma-pkg`).
- [ ] Implement Zenith UI Wayland compositor rendering pipeline.
- [ ] Hardware Attestation (TPM 2.0 handshake) driver.
- [ ] Full FIPS-140 compliance tests.

## Mid-Term: Security & AI Integration (Q4 2026)
- [x] Integrate `liboqs` (Kyber, Dilithium) into `SovereignPQC`.
- [ ] Build a functioning AI Assistant (Alt+A) daemon hooked into `sysfs`.
- [ ] Implement capability-based process isolation.
- [ ] BTRFS-based Self-Healing OS snapshot and rollback scripts.

## Long-Term: Cloud & Gaming (Q1 2027)
- [ ] Kubernetes-native node layer out-of-the-box.
- [ ] Native Vulkan rendering pipeline acceleration.
- [ ] Sigma Profiles: pre-configured UI/Kernel settings for gamers & devs.

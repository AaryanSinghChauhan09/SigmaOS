# SigmaOS Development Roadmap

## Current Progress

```
Phase A: Foundation          ████████████████████ 100% ✅
Phase B: Core Subsystems     ████████████████████ 100% ✅
Phase C: Security Layer      ████████████████████ 100% ✅
Phase D: AI Integration      ████████████████░░░░  80% 🚧
Phase E: Desktop Environment ████████████████░░░░  80% 🚧
Phase F: Ecosystem           ████████████░░░░░░░░  60% 🚧
Phase G: Bootable ISO        ░░░░░░░░░░░░░░░░░░░░   0% 📋 (Q1 2027)
Phase H: Enterprise          ░░░░░░░░░░░░░░░░░░░░   0% 📋 (2027)
```

## Q3 2026 — Performance & Ecosystem

### Kernel
- [ ] RISC-V 64-bit full support
- [ ] ARM64 hardware bring-up improvements
- [ ] io_uring v3 async I/O
- [ ] eBPF CO-RE support
- [ ] Kernel live patching

### Packages
- [ ] AUR compatibility (stable)
- [ ] dpkg/APT bridge
- [ ] DNF/RPM bridge
- [ ] Nix expression evaluator (stable)

### Security
- [ ] AppArmor profile engine
- [ ] TPM 2.0 TOTP integration
- [ ] HSM support

## Q4 2026 — Desktop & S-AI

### Desktop
- [ ] Zenith compositor (production)
- [ ] sigma-control-center
- [ ] WCAG 2.1 AA accessibility
- [ ] Fractional scaling
- [ ] HDR display support

### S-AI
- [ ] Orchestrator v2
- [ ] Local speech-to-text (Whisper)
- [ ] Neural power manager (stable)
- [ ] AI crash analyzer

## 2027 Q1 — Bootable ISO

- [ ] sigma-install graphical installer
- [ ] Hardware auto-detection
- [ ] sigma-welcome onboarding
- [ ] 95% Arch package compatibility
- [ ] Full POSIX compliance

## 2027 Q2 — Ecosystem Growth

- [ ] sigma-store (app marketplace)
- [ ] Hardware certification program
- [ ] sigma-sdk v1.0

## 2027 Q3 — Enterprise

- [ ] Active Directory / LDAP
- [ ] Centralized logging (sigma-siem)
- [ ] SOC2/ISO27001 compliance toolkit

## Milestones

| Milestone | Target | Status |
|-----------|--------|--------|
| v0.1 Foundation | Q2 2026 | ✅ |
| v0.2 Security | Q3 2026 | ✅ |
| v0.3 AI Integration | Q4 2026 | 🚧 |
| v0.4 Desktop Alpha | Q4 2026 | 🚧 |
| v0.5 Bootable ISO | Q1 2027 | 📋 |
| v1.0 Stable | Q3 2027 | 📋 |
| v1.1 LTS | Q1 2028 | 📋 |

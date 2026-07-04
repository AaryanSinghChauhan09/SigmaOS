# SigmaOS Roadmap

> Quick navigation hub for all roadmap documents.

## Roadmap Documents

| Document | Purpose |
|----------|---------|
| [Development-Roadmap](Development-Roadmap) | Master roadmap with phases, versions, and branch priorities |
| [Branch-Development-Roadmap](Branch-Development-Roadmap) | Per-branch file-level task lists |
| [Feature-Roadmap](Feature-Roadmap) | Implemented vs planned features |
| [Version-Timeline](Version-Timeline) | Release history and upcoming versions |
| [Release-Profiles](Release-Profiles) | All 8 deployment profiles explained |
| [PHASE_G_ROADMAP](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/PHASE_G_ROADMAP.md) | Phase G kernel completion tasks |
| [Competitive-Analysis](Competitive-Analysis) | How SigmaOS compares to Linux distros |
| [Gap-Analysis](Gap-Analysis) | Remaining gaps vs competitors |

## Current Status at a Glance

```
Phase F  ████████████████████  100% ✅  (KMS, cgroup, pkg registry)
Phase G  ████████████████████  100% ✅  (kernel boot — COMPLETE)
Phase H  ████████████████░░░░  50% 🔄  (India Stack — ACTIVE)
  - sigma-health (ABDM FHIR): ✅
  - sigma-accounts (GST IRN): ✅
  - sigma-pay (UPI/NPCI): ✅
  - sigma-aadhaar (QR Auth): ✅
```

## The Critical Path

Everything depends on `kernel-exp` shipping Phase 0:

1. `kernel-exp` → bootable kernel
2. `drivers-dev` → GPU + Wi-Fi drivers
3. `fs-dev` → VFS + SigmaFS
4. All `release/*` profiles become functional

## Quick Links

- [CURRENT_PROBLEMS_MANIFEST.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/CURRENT_PROBLEMS_MANIFEST.md)
- [FEATURE_MATRIX.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/FEATURE_MATRIX.md)
- [CONTRIBUTOR_ROADMAP.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/CONTRIBUTOR_ROADMAP.md)
- [GitHub Issues](https://github.com/AaryanSinghChauhan09/SigmaOS/issues)

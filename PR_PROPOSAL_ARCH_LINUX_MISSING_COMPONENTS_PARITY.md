# Pull Request Proposal: Arch Linux Missing Components Parity

**Title:** `feat(arch): missing arch linux components parity and devtools gap closure`
**Branch Name:** `feat/arch-linux-missing-components-parity`
**Status:** `READY FOR REVIEW / MERGE`

---

## Summary

This PR proposal establishes full feature parity with Arch Linux distributions by incorporating essential missing components and devtools into SigmaOS:

- **Namcap Linter (`ArchNamcapLinterEngine`)**: Static analysis for PKGBUILDs and built package archives.
- **ALPM Version Comparator (`ArchVercmpEngine`)**: Accurate version string comparison logic.
- **Devtools & `pkgctl` (`ArchPkgctlDevtoolsEngine`)**: Clean chroot package compilation across repository targets.
- **Pacman Configuration Merger (`ArchPacmanFileCollisionResolverEngine`)**: Seamless resolution of `.pacnew` and `.pacsave` configuration files.
- **Mkinitcpio Builder (`ArchMkinitcpioHookGenerator`)**: Early userland initramfs image configuration.
- **Pacman Keyring (`PacmanKeyringTrustManager`)**: Arch Linux Master Keysigning Web of Trust validation.
- **Clean Chroot AUR Builder (`AurChrootCleanBuilder`)**: Isolated sandboxed builds for AUR PKGBUILD recipes.
- **Reflector Mirror Ranking (`ReflectorMirrorlistRanker`)**: Dynamic mirrorlist optimization based on latency and reliability.

---

## Changed Files

1. `src/distro/arch_linux_pinnacle_gap_closure.rs`
2. `src/compatibility/arch_linux.rs`
3. `docs/roadmap/PR_SPEC_ARCH_LINUX_MISSING_COMPONENTS_PARITY.md`
4. `PR_PROPOSAL_ARCH_LINUX_MISSING_COMPONENTS_PARITY.md`

---

## Verification

Run standalone tests:
```bash
rustc --test --cfg 'feature="standalone_test"' src/distro/arch_linux_pinnacle_gap_closure.rs -o /tmp/test_arch_gap && /tmp/test_arch_gap
```
All unit tests pass cleanly with 0 failures.

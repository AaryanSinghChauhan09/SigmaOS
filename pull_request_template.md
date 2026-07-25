<!-- SigmaOS Pull Request Template
     Fill in each section. Delete irrelevant items.
     Inspired by Linux kernel patch submission guidelines. -->

## Description

<!-- One paragraph: what does this PR do and why? -->

## Subsystem(s) affected

<!-- Check all that apply -->
- [ ] kernel/core (scheduler, MM, syscalls, IRQ)
- [ ] kernel/fs (VFS, tmpfs, sigmafs, ext4)
- [ ] kernel/net (TCP, UDP, sockets, Wi-Fi)
- [ ] kernel/security (pledge, unveil, seccomp, PQC)
- [ ] drivers (NVMe, USB, GPU, Wi-Fi, audio, input)
- [ ] boot (UEFI, multiboot, bootloader)
- [ ] sigma-sh (shell)
- [ ] sigma-pkg (package manager)
- [ ] zenith_desktop (compositor, WM)
- [ ] userland/ai (sigma-ai, GGUF, NL-CLI)
- [ ] sdk (driver DDK, app SDK)
- [ ] ci / build system
- [ ] docs / wiki
- [ ] other: ___

## Type of change

- [ ] `feat` — new feature
- [ ] `fix` — bug fix
- [ ] `impl` — implements a roadmap item from Ideas-999-Structured
- [ ] `perf` — performance improvement
- [ ] `security` — security fix/hardening
- [ ] `refactor` — code refactoring (no behaviour change)
- [ ] `driver` — new or updated hardware driver
- [ ] `docs` — documentation only
- [ ] `ci` — CI/workflow changes
- [ ] `test` — tests only

## Roadmap item (if applicable)

<!-- Reference the Ideas-999-Structured.md idea number, e.g. "Idea #86 Wi-Fi DDK" -->
Ideas: #___

## Testing done

<!-- Required — describe what you tested and how -->

**Build targets tested:**
- [ ] x86_64-unknown-none (kernel no_std)
- [ ] aarch64-unknown-none
- [ ] riscv64gc-unknown-none-elf
- [ ] x86_64-unknown-linux-gnu (host tools / sigma-sh)

**Tests run:**
- [ ] `cargo test` passes
- [ ] `cargo clippy -- -D warnings` clean
- [ ] `cargo fmt --check` clean
- [ ] QEMU boot smoke test (`make qemu` or `./qemu-boot.sh smoke`)
- [ ] Manual testing: describe below

```
# Paste your test session here
$ sigma-sh
$ sigma-pkg install ...
```

**Hardware tested on (if driver change):**
- [ ] QEMU virtio
- [ ] Physical hardware: ___

## Safety / Security considerations

<!-- Required if touching kernel/, security/, drivers/, or kabi/ -->

- [ ] All `unsafe` blocks have `// SAFETY:` explanation comments
- [ ] No new `unsafe` blocks without justification
- [ ] `sigma_pledge` / `sigma_unveil` calls are correct for new syscalls
- [ ] New syscalls added to `syscall_dispatch.rs` and `sigma_pledge.rs` mapping
- [ ] PQC signatures verified on any new binary artifacts
- [ ] No secret material (keys, tokens) in the diff

## ABI / compatibility impact

<!-- Does this change any public ABI that drivers or apps depend on? -->

- [ ] No ABI change
- [ ] ABI-compatible extension (new fields at end of struct)
- [ ] ABI break — requires kabi version bump and MAINTAINERS notification

## Checklist

- [ ] Title follows Conventional Commits: `type(scope): description`
- [ ] Commits touching `kernel/` or `drivers/` have `Signed-off-by: Name <email>`
- [ ] SPDX-License-Identifier on all new source files
- [ ] MAINTAINERS updated if adding a new subsystem or driver
- [ ] Wiki / docs updated if behaviour changes are user-visible
- [ ] `CURRENT_PROBLEMS_MANIFEST.md` updated if this resolves an open issue

## Linked issues

<!-- Use "Closes #123" or "Related to #456" -->
Closes #

---
<!-- By submitting this PR you certify that your contribution is your
     original work and you have the right to submit it under the MIT/GPL-2.0
     license as indicated in the file. (Developer Certificate of Origin) -->

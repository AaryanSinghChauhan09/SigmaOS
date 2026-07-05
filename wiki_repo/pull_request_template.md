# PULL REQUEST TEMPLATE

<!-- SigmaOS Pull Request Template
     Fill in each section. Delete irrelevant items.
     Inspired by Linux kernel patch submission guidelines. -->

## Description

<!-- One paragraph: what does this PR do and why? -->

## Subsystem(s) affected

<!-- Check all that apply -->

- [ ] kernel/core (scheduler, MM, syscalls, IRQ, boot)

- [ ] kernel/fs (VFS, tmpfs, sigmafs, ext4)

- [ ] kernel/net (TCP, UDP, sockets, Wi-Fi, DHCP, DNS)

- [ ] kernel/security (pledge, unveil, seccomp, PQC, audit)

- [ ] drivers (NVMe, USB, GPU, Wi-Fi, audio, input, storage)

- [ ] arch (x86_64, ARM64, RISC-V)

- [ ] sigma-sh (shell REPL)

- [ ] sigma-pkg (package manager, registry)

- [ ] zenith_desktop (compositor, WM, GUI)

- [ ] userland/ai (sigma-ai, GGUF, NL-CLI)

- [ ] sdk (driver DDK, app SDK)

- [ ] ci / build system

- [ ] docs / wiki

- [ ] other: ___

## Type of change

- [ ] `feat` — new feature

- [ ] `fix` — bug fix

- [ ] `impl` — implements a roadmap item

- [ ] `perf` — performance improvement

- [ ] `security` — security fix/hardening

- [ ] `refactor` — code refactoring (no behaviour change)

- [ ] `driver` — new or updated hardware driver

- [ ] `docs` — documentation only

- [ ] `ci` — CI/workflow changes

- [ ] `test` — tests only

## Roadmap item (if applicable)

<!-- Reference the roadmap phase and item -->
Phase: ___
Item: ___

## Testing done

<!-- Required — describe what you tested and how -->

### Build targets tested:

- [ ] x86_64-unknown-none (kernel no_std)

- [ ] aarch64-unknown-none

- [ ] riscv64gc-unknown-none-elf

- [ ] x86_64-unknown-linux-gnu (host tools / sigma-sh)

### Tests run:

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

### Hardware tested on (if driver change):

- [ ] QEMU virtio

- [ ] Physical hardware: ___

## Safety / Security considerations

<!-- Required if touching kernel/, security/, drivers/, or kabi/ -->

- [ ] All `unsafe` blocks have `// SAFETY:` explanation comments

- [ ] No new `unsafe` blocks without justification

- [ ] `sigma_pledge` / `sigma_unveil` calls are correct for new syscalls

- [ ] New syscalls added to syscall dispatcher and pledge mapping

- [ ] PQC signatures verified on any new binary artifacts

- [ ] No secret material (keys, tokens) in the diff

- [ ] Security review completed for security-critical changes

## ABI / compatibility impact

<!-- Does this change any public ABI that drivers or apps depend on? -->

- [ ] No ABI change

- [ ] ABI-compatible extension (new fields at end of struct)

- [ ] ABI break — requires kabi version bump and MAINTAINERS notification

## Commit Message Format

<!-- Verify your commits follow the kernel-style format -->

- [ ] Title follows format: `subsystem: short description (=50 chars)`

- [ ] Commits include detailed explanation (motivation, technical details, testing)

- [ ] Commits include `Signed-off-by: Name <email>` (DCO requirement)

- [ ] References included (Issue #123, PR #456)

## Checklist

- [ ] Title follows Conventional Commits: `type(scope): description`

- [ ] Commits touching `kernel/` or `drivers/` have `Signed-off-by: Name <email>`

- [ ] SPDX-License-Identifier on all new source files

- [ ] MAINTAINERS updated if adding a new subsystem or driver

- [ ] Wiki / docs updated if behaviour changes are user-visible

- [ ] CODEOWNERS updated if adding new directories

- [ ] Tests added for new functionality

- [ ] No external dependencies added without review

## Linked issues

<!-- Use "Closes #123" or "Related to #456" -->
Closes #

---
<!-- By submitting this PR you certify that your contribution is your
     original work and you have the right to submit it under the MIT/GPL-2.0
     license as indicated in the file. (Developer Certificate of Origin) -->

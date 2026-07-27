# PULL REQUEST TEMPLATE

---

name: Pull Request
about: Submit a new Sovereign Shard or bug fix
---

# Pull Request

## Summary

<!-- One-line description of what this PR does -->

## Type

- [ ] New Shard

- [ ] Bug Fix

- [ ] Documentation Update

- [ ] GitHub Actions / CI improvement

## Closes

<!-- Reference the ROADMAP.md milestone, IDEAS_BACKLOG.md item, or issue number -->

<<<<<<< HEAD:docs/pull_request_template.md
- [ ] sigma-pkg (package manager, registry)

- [ ] zenith_desktop (compositor, WM, GUI)

- [ ] userland/ai (sigma-ai, GGUF, NL-CLI)

- [ ] sdk (driver DDK, app SDK)

- [ ] ci / build system

- [ ] docs / wiki

- [ ] other: ___

## Type of change

- [ ] `feat` \x97 new feature

- [ ] `fix` \x97 bug fix

- [ ] `impl` \x97 implements a roadmap item

- [ ] `perf` \x97 performance improvement

- [ ] `security` \x97 security fix/hardening

- [ ] `refactor` \x97 code refactoring (no behaviour change)

- [ ] `driver` \x97 new or updated hardware driver

- [ ] `docs` \x97 documentation only

- [ ] `ci` \x97 CI/workflow changes

- [ ] `test` \x97 tests only

## Roadmap item (if applicable)

<!-- Reference the roadmap phase and item -->
Phase: ___
Item: ___

## Testing done

<!-- Required \x97 describe what you tested and how -->

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

```text

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

- [ ] ABI break \x97 requires kabi version bump and MAINTAINERS notification

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
=======
>>>>>>> wiki/master:pull_request_template.md
Closes #

## Shard Checklist (New Shards Only)

- [ ] C++ OOP Singleton with `getInstance()`

- [ ] `extern "C"` wrappers for all public functions

- [ ] Registered in `SovereignUSR` via `usr_register_shard()`

- [ ] `cppcheck` passes with zero warnings

- [ ] Wiki page created or updated in `SigmaOS.wiki/`

- [ ] `IDEAS_BACKLOG.md` or `MISSING_COMPONENTS.md` updated

## Bug Fix Checklist

- [ ] Root cause identified and documented

- [ ] Regression test described

- [ ] No new `cppcheck` warnings introduced

## Testing

<!-- Describe how you tested this change -->

## Screenshots / Serial Output

<!-- Paste sigma_log output or screenshots if applicable -->

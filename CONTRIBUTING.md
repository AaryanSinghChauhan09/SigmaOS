# Contributing to SigmaOS

SigmaOS is an experimental operating-system project. Contributions should make one behavior clearer, safer, or more usable and include evidence for the claims they add. See the [GitHub Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki) for component notes and the [testing guide](wiki/Testing.md) for validation expectations.

## Before changing code

1. Read the owning module and its tests; search for callers and existing status claims.
2. Check the issue tracker and open pull requests for overlapping work.
3. State the intended behavior, failure cases, and acceptance check in the issue or pull request.
4. Keep changes small enough to review as one logical unit. Large design changes should begin with a short proposal.

## Implementation standards

- Prefer safe Rust and explicit ownership. Document every required `unsafe` block with its safety invariants.
- Preserve the current dependency policy: new third-party dependencies require maintainer review and a clear technical justification.
- Return meaningful errors at subsystem boundaries; do not report success for simulated or unavailable behavior.
- Add regression tests for fixes and test both normal and failure paths.
- Treat security, boot, storage, package, and update changes as high risk; include their threat or recovery assumptions.
- Keep performance claims out of descriptions unless a reproducible benchmark supports them.
- Follow existing formatting and naming in the touched component.

## Local verification

Run the checks relevant to the change from the repository root:

```sh
cargo fmt --all -- --check
cargo check --lib
cargo test --lib <module_or_test_filter>
```

For cross-component changes, run the broader suites:

```sh
cargo test --lib
./run_sigma_tests.sh
```

For boot, installer, driver, and desktop work, include a real QEMU or hardware check when the environment and implementation support it. Report unavailable checks as unavailable; a simulated harness is not boot evidence.

## Pull request checklist

- [ ] The change addresses one clear behavior or documentation problem.
- [ ] Tests cover the changed behavior and relevant failure path.
- [ ] Formatting and applicable build/test commands were run and their results are included.
- [ ] Public interfaces, security assumptions, and compatibility effects are documented.
- [ ] The owning component page and capability status reflect verified behavior.
- [ ] Reference-project ideas link to upstream sources and are adapted to SigmaOS constraints.

Use a descriptive pull-request title such as `fix(scheduler): preserve signed eligibility lag`. Include a concise summary, testing evidence, limitations, and screenshots or logs for user-visible work. Maintainers review and merge contributions; do not push directly to the protected default branch.

## Documentation conventions

Follow the [ArchWiki style and contribution guidance](https://wiki.archlinux.org/title/Help:Style): use descriptive page titles, concise sections, prerequisites, ordered steps, expected results, and troubleshooting. For desktop workflows, study [Omarchy's CLI and keyboard workflow](https://omarchy.org/manual/omarchy-cli/) and [Linux Mint's user and troubleshooting guides](https://linuxmint.com/documentation.php). Adapt those ideas; never copy their capability claims into SigmaOS documentation without SigmaOS evidence.

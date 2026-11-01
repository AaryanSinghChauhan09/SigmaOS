# Contributing to COMPONENT_NAME

Thank you for contributing to SigmaOS. This document covers everything you
need to contribute effectively and in alignment with SigmaOS values.

---

## Table of Contents

1. [Engineering Principles](#engineering-principles)
2. [Code Style](#code-style)
3. [Commit Format](#commit-format)
4. [Branch Naming](#branch-naming)
5. [Pull Request Process](#pull-request-process)
6. [Test Requirements](#test-requirements)
7. [Documentation](#documentation)
8. [Community Standards](#community-standards)

---

## Engineering Principles

Every contribution must respect the SigmaOS engineering principles.
These are not optional guidelines — they are the design constitution of
the project. Read each one; apply all relevant ones on every PR.

### Design & Architecture

| Principle | Summary |
|-----------|---------|
| **ABSTRACTION** | Expose the right level of detail. Hide complexity behind clean interfaces. |
| **ARCHITECTURE** | Decisions have lasting impact. Think before you design; document in ARCHITECTURE.md. |
| **AVOID FRAGMENTATION** | Don't split what belongs together; don't merge what belongs apart. |
| **BLACKBOX** | Components must be usable without knowledge of their internals. |
| **BOOCH METHOD** | Identify objects, their responsibilities, collaborations, and protocols before coding. |
| **CLEAR ABSTRACTION** | Abstraction boundaries must be obvious. If you have to explain them, redesign them. |
| **COMPOSITION OVER INHERITANCE** | Prefer composing small, focused types over deep inheritance hierarchies. |
| **DESIGN PATTERN** | Use established patterns where they fit; name them explicitly. |
| **DISCOVERABILITY** | APIs, modules, and commands should be findable without reading source. |
| **ENCAPSULATION** | Keep implementation details private. Only expose what callers truly need. |
| **GALL'S LAW** | Complex working systems grow from simple working systems. Start simple. |
| **GRASP** | Assign responsibilities to the objects best equipped to hold them. |
| **INFORMATION HIDING** | Internal representations are an implementation detail; never expose them. |
| **INHERITANCE** | Use with care; prefer traits over class-like inheritance chains. |
| **INTERFACE SEGREGATION** | Many focused interfaces beat one fat interface. |
| **INVERSION OF CONTROL** | High-level policy must not depend on low-level details. Inject dependencies. |
| **LAYERED DESIGN** | Higher layers depend on lower layers; never the reverse. |
| **LOOSE COUPLING** | Minimise the number and strength of dependencies between components. |
| **MAXIMISE COHESION** | Everything in a module should belong together conceptually. |
| **MINIMISE COUPLING** | The fewer the cross-module dependencies, the better. |
| **MODULARITY** | Break the system into independently buildable, testable, deployable units. |
| **NORMALISATION** | Data and logic should live in exactly one place. |
| **OOP** | Object-oriented principles (encapsulation, polymorphism, etc.) are tools, not religion. |
| **OPEN-CLOSED** | Open for extension; closed for modification. |
| **OPTIMISE FOR DELETION** | Write code that is easy to remove. Avoid load-bearing glue. |
| **PLUG-IN DRIVER MODEL** | Drivers and extensions are plug-ins; the core knows nothing specific about them. |
| **POLYMORPHISM** | Use dynamic dispatch through traits; avoid match-on-type anti-patterns. |
| **SEPARATION OF CONCERNS** | One module, one concern. Don't mix logging, business logic, and I/O. |
| **SEPARATION OF MECHANISM & POLICY** | Implement the how in the kernel/library; let callers decide the what. |
| **SINGLE RESPONSIBILITY** | A module/type/function does exactly one thing. |
| **UNIFORM ACCESS** | Whether a value is stored or computed, callers should not need to care. |
| **VIRTUAL MEMORY ABSTRACTION** | Physical addresses are an implementation detail of memory management. |
| **ZERO-ONE-INFINITY** | Allow zero, one, or unlimited instances. Never hard-code 2 or 7. |

### Code Quality

| Principle | Summary |
|-----------|---------|
| **BOY SCOUT RULE** | Leave the code cleaner than you found it. |
| **CODE REUSABILITY** | Before writing, look for existing solutions in this repo or the workspace. |
| **COMMAND-QUERY SEPARATION** | Functions either return a value or cause a side-effect; not both. |
| **DEFENSIVE PROGRAMMING** | Validate inputs, check preconditions, handle all error paths. |
| **DEMETER LAW** | Talk to friends, not strangers. Don't chain through unrelated objects. |
| **DON'T REPEAT YOURSELF** | Every piece of knowledge has a single authoritative representation. |
| **FAIL-FAST SYSTEM** | Detect errors as early as possible and surface them loudly. |
| **IF IT AIN'T BROKE DON'T FIX IT** | Churn carries risk. Change only what needs changing. |
| **INSTRUMENTATION** | Critical paths must be observable. Add metrics/tracing at design time. |
| **KISS** | Keep it simple. The simplest solution that works is usually the right one. |
| **LEAST ASTONISHMENT** | APIs and behaviour must match programmer expectations. |
| **NINETY-NINETY RULE** | The first 90% takes 10% of the time. Plan for the second 90%. |
| **OFFENSIVE PROGRAMMING** | In debug builds, crash loudly on invariant violations. |
| **REFACTORING** | Continuously improve internal structure without changing external behaviour. |
| **RULE OF THREE** | Abstract on the third repetition, not the second. |
| **SOLID** | Single Responsibility, Open-Closed, Liskov Substitution, Interface Segregation, Dependency Inversion. |
| **SPEC-DRIVEN DEVELOPMENT** | Write the specification (types, tests, docs) before the implementation. |
| **TALK IS CHEAP SHOW ME THE CODE** | Proposals are validated by working implementations, not discussion alone. |
| **YAGNI** | You Aren't Gonna Need It. Don't build for hypothetical futures. |
| **ZEN OF PYTHON** | Explicit > implicit. Simple > complex. Readability counts. Errors should never pass silently. |

### Systems & Kernel

| Principle | Summary |
|-----------|---------|
| **BOOTLOADER INTEGRATION** | Components must declare their boot-phase dependencies explicitly. |
| **CACHE CONSCIOUS DESIGN** | Structure data for cache locality. Prefer arrays of structs carefully. |
| **CROSS COMPILATION** | All code must build for the target architecture from any host. |
| **DEADLOCK AVOIDANCE** | Establish a strict lock ordering. Document it. Never acquire out-of-order. |
| **DEUTSCH LIMIT** | A complex system cannot be fully understood by any single person. Design accordingly. |
| **DIRECT DEVICE CONTROL** | Drivers speak directly to hardware; no opaque intermediary layers. |
| **EFFICIENT SCHEDULING** | Scheduling decisions must be O(1) or O(log n); never O(n) per-core. |
| **INTERRUPT HANDLING DISCIPLINE** | ISRs do the minimum possible. Defer work to task context. |
| **LOCALITY OF REFERENCE** | Hot data and code should be spatially and temporally close. |
| **LOW LEVEL CONTROL** | Critical paths must be controllable at the hardware level when needed. |
| **LOW LEVEL LANGUAGES** | Rust and C are the languages of the kernel and drivers. No GC in IRQ context. |
| **MANUAL MEMORY CONTROL** | Heap layout, alignment, and lifetimes are first-class design concerns. |
| **MEMORY DISCIPLINE** | No memory leaks. No use-after-free. No double-free. Verified by tooling. |
| **MEMORY SAFETY** | Rust's ownership model is the primary memory safety mechanism. `unsafe` requires ADR. |
| **MINIMAL KERNEL PHILOSOPHY** | The kernel does the minimum. Everything else is user space or a component. |
| **MODULAR KERNEL DESIGN** | Kernel features are loadable modules with well-defined interfaces. |
| **POWER OF TEN** | Follow JPL's ten rules for safety-critical code in kernel and driver components. |
| **PROCESS ISOLATION** | Components run in isolated address spaces. Crashes must not cascade. |
| **RACE-FREE DESIGN** | All shared mutable state is protected by synchronisation primitives. Prove it. |
| **SAFE CONCURRENCY** | Use Rust's `Send`/`Sync` discipline. No raw thread-unsafe sharing. |
| **ZERO-COPY** | Data in hot paths moves by reference or memory mapping, never unnecessary copy. |

### Security

| Principle | Summary |
|-----------|---------|
| **LEAST PRIVILEGE** | Components request only the permissions they need. No ambient authority. |
| **SECURE DEFAULTS** | The default configuration must be the most secure configuration. |

### Sustainability & Ecosystem

| Principle | Summary |
|-----------|---------|
| **ARRANGE-ACT-ASSERT** | Structure every test with explicit Arrange, Act, and Assert phases. |
| **COLLABORATION & SHARED RESPONSIBILITY** | No hero coding. Code is owned by the team, not the individual. |
| **COMMUNITY-ORIENTED DEVELOPMENT** | Decisions are made openly. Preferences are stated; data wins. |
| **CONTINUOUS IMPROVEMENT & LONG TERM SUSTAINABILITY** | Prefer boring, proven technology. Optimise for the 10-year horizon. |
| **DETERMINISTIC TESTING** | Tests must produce the same result on every run, on every machine. |
| **FAST-ISOLATED-REPEATABLE-SELF VALIDATING-TIMELY** | Every test satisfies FIRST. Flaky tests are bugs. |
| **FREE AND OPEN SOURCE SOFTWARE** | All contributions are MIT-licensed and freely redistributable. |
| **GOVERNANCE AND DO-CRACY** | Those who do the work make the decisions. Show up with code. |
| **MERITOCRACY AND INCLUSIVITY** | Contributions are evaluated on technical merit. All contributors are welcome. |
| **MULTIPLE REPOSITORY** | Each component is its own repo. No monorepo sprawl. |
| **OPEN STANDARD AND INTEROPERABILITY** | Use open standards. Avoid proprietary lock-in. |
| **PORTABILITY** | Target-specific code is isolated and gated by feature flags or cfg attributes. |
| **PRACTICAL LICENSING AWARENESS** | Understand the licence of every dependency before adding it. |
| **PRISTINE SOURCES** | The main branch contains only source code. No binaries, no build artifacts. |
| **RESPECT & ETHICAL PARTICIPATION** | Be respectful. Harassment, discrimination, and bad faith are grounds for removal. |
| **SINGLE REPOSITORY** | Each component's source lives in a single authoritative repository. |
| **SOFTWARE SUPPORT LIKE LINUX DISTROS** | Backport security fixes. Maintain stable release branches. |
| **SOFTWARE UPDATING LIKE LINUX DISTROS** | Components are independently updatable without rebooting the system. |
| **STABLE APIs** | Public APIs do not break without a deprecation period and major version bump. |
| **THEIRS** | When merging upstream, prefer their changes (git merge -X theirs) unless there is a reason. |
| **TRANSPARENCY & OPEN EXCHANGE** | Decisions, trade-offs, and failures are documented publicly. |
| **UNIFORM INFORMATION IN GITHUB REPO & WIKI** | Repo and wiki must be in sync. |
| **USER FREEDOM AND CONTROL** | Users can inspect, modify, and replace any component. |
| **VERSION CONTROL PRACTICES** | Atomic commits. Meaningful messages. No force-push to shared branches. |
| **WORSE IS BETTER** | A simpler, slightly inferior solution that ships beats a perfect one that doesn't. |
| **XTHEIRS** | Hard conflict resolution: accept upstream wholesale when local changes have no justification. |

---

## Code Style

### Rust

- Run `make fmt` (rustfmt) before every commit.
- Run `make lint` (clippy `--deny warnings`) and fix all warnings.
- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/).
- Use `thiserror` for library errors; `anyhow` is allowed only in binaries.
- Avoid `unwrap()` and `expect()` in non-test code; use `?` and explicit error handling.
- Every public item must have a `///` doc comment.
- Prefer `#[must_use]` on functions that return meaningful values.
- No `#[allow(dead_code)]` unless accompanied by a comment explaining why.

### C (drivers / kernel modules only)

- Follow the Linux kernel coding style (8-space tabs, 80-column soft limit).
- All allocations must have a paired free on every exit path.
- Use `__attribute__((warn_unused_result))` on functions returning error codes.
- No global mutable state outside of explicitly documented kernel structures.

### Python (tooling / scripts only)

- Formatted with `black`. Linted with `ruff`.
- Type-annotated (`mypy --strict`).
- Follow the Zen of Python.

### Shell

- Written for `bash`. Starts with `#!/usr/bin/env bash`.
- `set -euo pipefail` at the top of every script.
- Shellcheck-clean.
- Variables quoted; no word-splitting surprises.

---

## Commit Format

SigmaOS uses [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/).

```
<type>(<scope>): <short summary>

[optional body]

[optional footer(s)]
```

### Types

| Type       | When to use |
|------------|-------------|
| `feat`     | New feature or capability |
| `fix`      | Bug fix |
| `refactor` | Code restructuring with no behaviour change |
| `perf`     | Performance improvement |
| `test`     | Adding or fixing tests |
| `docs`     | Documentation only (docs branch) |
| `build`    | Build system, Makefile, Cargo.toml |
| `ci`       | CI/CD configuration |
| `chore`    | Routine maintenance, dependency bumps |
| `security` | Security fix or hardening |
| `revert`   | Reverts a previous commit |

### Rules

- Summary line: ≤ 72 characters, imperative mood, no trailing period.
- Body: wrap at 72 characters, explain *why* not *what*.
- Breaking changes: add `!` after type/scope and a `BREAKING CHANGE:` footer.
- Reference issues: `Closes #123`, `Fixes #456`.

### Examples

```
feat(config): add JSON schema validation for component config

Previously, invalid config fields were silently ignored. This change
adds serde-based schema validation at startup and returns a structured
Error::InvalidConfig with the offending field name.

Closes #42
```

```
fix(scheduler): correct priority inversion in lock acquisition

The lock was acquired in FIFO order, allowing a high-priority task to
be blocked indefinitely by a lower-priority one. Reorder to priority-
based acquisition.

BREAKING CHANGE: Scheduler::acquire() now takes a priority argument.
```

---

## Branch Naming

```
<type>/<issue-number>-<short-slug>
```

| Branch | Example |
|--------|---------|
| Feature | `feat/123-add-json-config` |
| Bug fix | `fix/456-null-deref-on-empty-buf` |
| Refactor | `refactor/789-extract-config-module` |
| Security | `security/101-harden-input-parsing` |
| Chore | `chore/321-bump-serde-1-0-197` |

Rules:
- Branch off `main` for all work.
- Delete branches after merging.
- Never commit directly to `main`.
- The `docs` branch is for documentation only.

---

## Pull Request Process

1. **Open an issue first** for any non-trivial change. Describe the problem, not the solution.
2. **Fork and branch** from the latest `main`.
3. **Write tests** before or alongside implementation (spec-driven development).
4. **Ensure CI passes**: `make sigma-check lint fmt-check test audit`.
5. **Fill in the PR template** completely. No partial descriptions.
6. **Request review** from at least one maintainer.
7. **Address all review comments** before merge or explicitly resolve with justification.
8. **Squash or rebase** to a clean commit history before merge; no merge commits.
9. **Delete the branch** after merge.

### PR Checklist

- [ ] `make sigma-check` passes (no `.md`/`.txt` on `main`)
- [ ] `make fmt-check` passes
- [ ] `make lint` passes (zero warnings)
- [ ] `make test` passes
- [ ] `make audit` passes
- [ ] All public items have `///` doc comments
- [ ] ARCHITECTURE.md updated if design changed (on `docs` branch)
- [ ] No secrets, credentials, or PII in diff
- [ ] Commit messages follow Conventional Commits format
- [ ] Issue linked in PR description

---

## Test Requirements

All code contributions must include tests. The following rules apply:

### Coverage

- New features: 100% of new public API paths tested.
- Bug fixes: a regression test that would have caught the bug.
- Refactors: existing tests must continue to pass without modification.

### Test Quality (FIRST)

| Property | Requirement |
|----------|-------------|
| **Fast** | Unit tests must complete in < 100 ms. Total suite < 30 s. |
| **Isolated** | Tests have no shared mutable state. Each test is independent. |
| **Repeatable** | No randomness, no clock reads, no external service dependencies. |
| **Self-validating** | Pass or fail with no human interpretation needed. |
| **Timely** | Written with or before the code they test. |

### Test Structure

Use the Arrange-Act-Assert (AAA) pattern:

```rust
#[test]
fn component_name_rejects_empty_config() {
    // Arrange
    let config = Config { name: String::new(), ..Config::default() };

    // Act
    let result = ComponentName::new(config);

    // Assert
    assert!(result.is_err());
}
```

### Test Locations

- **Unit tests**: `#[cfg(test)]` module at the bottom of each source file.
- **Integration tests**: `tests/` directory, one file per integration concern.
- **Property-based tests**: `tests/property_tests.rs` using `proptest`.
- **Benchmarks**: `benches/` directory using `criterion`.

---

## Documentation

- Documentation lives on the `docs` branch only.
- The `main` branch must contain zero `.md`, `.txt`, `.rst`, or `.adoc` files.
- Update `docs` branch documentation in the same PR (as a separate commit targeting the docs branch) as the code change it documents.
- API documentation (rustdoc `///` comments) lives in source on `main` — this is the one exception to the "no docs on main" rule, as it is embedded in code.

---

## Community Standards

- Be welcoming. Follow the [Contributor Covenant](https://www.contributor-covenant.org/).
- Critique code, not people.
- Assume good faith.
- Harassment, discrimination, or bad-faith participation results in removal.
- Governance is do-ocratic: the people doing the work make the calls.
- Meritocracy: contributions are evaluated on technical merit, not seniority.

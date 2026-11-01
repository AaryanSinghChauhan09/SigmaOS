# ADR-001 — Snapshot extraction strategy for SigmaOS-desktop

| Field      | Value                        |
|------------|------------------------------|
| Status     | Accepted                     |
| Date       | 2025-07-15                   |
| Author     | SigmaOS Phase-2 workflow     |
| Supersedes | —                            |

## Context

SigmaOS is a monorepo with hundreds of source files spread across
`src/compositor/`, `src/input/`, `src/theming/`, and `src/desktop/`.
Many of those files contain `no_std` code with cross-crate references to
`crate::ai::agent_runtime`, `crate::klib`, and other internal modules that
exist only inside the monorepo build.

The goal for Phase 2 is to extract a standalone `sigmaos-desktop` crate that:

1. Compiles as a regular `std` library with no monorepo dependencies.
2. Exposes the four canonical desktop abstractions (`WaylandCompositor`,
   `InputHandler`, `ThemeEngine`, `WindowManager`) with clean public APIs.
3. Ships integration tests and a CI pipeline.

## Decision

**Write a fresh `src/lib.rs` rather than copying and patching the monorepo files.**

The monorepo files (`src/compositor/sigma_compositor.rs` etc.) use `#![no_std]`,
Vulkan FFI, and internal crate paths that cannot be resolved outside the
monorepo.  Attempting to carry them verbatim would require stubbing out dozens
of types across multiple files, creating a maintenance burden with no benefit
because the monorepo remains the canonical source.

Instead, a single `src/lib.rs` is written that:

- Uses `std` (no `no_std` constraints in this standalone crate).
- Re-implements the four subsystem types as clean, self-contained stubs that
  match the public contracts observed in the monorepo.
- Depends only on `thiserror = "1.0"` for ergonomic error types.
- Has no `unsafe` code, no FFI, and no external system libraries.

## Principles applied

| Principle                     | How it is applied                                              |
|-------------------------------|----------------------------------------------------------------|
| Separation of mechanism/policy | `WaylandCompositor` owns surfaces; `WindowManager` owns layout |
| Composition over inheritance  | The four subsystems are composed at the call site, not inherited |
| Open/closed                   | `ThemeEngine::register` adds themes without modifying core code |
| YAGNI                         | Stubs only; Vulkan, Zig FFI, and AI hooks are not included here |
| Interface segregation         | Each error type is scoped to its subsystem                     |
| Single responsibility         | Each struct has one axis of change                             |
| KISS                          | One file, minimal dependencies                                 |

## Consequences

**Positive**

- `cargo check --lib` passes with zero errors and zero external system headers.
- Integration tests run with `cargo test` in an isolated environment.
- The crate can be published to crates.io independently.
- Future contributors can develop desktop logic without building the full kernel.

**Negative**

- The stub implementations are not production-ready; they are intentional
  placeholders.  Real Vulkan/Wayland integration is deferred to subsequent
  phases when the monorepo types become publishable as separate crates.

## Alternatives considered

| Alternative                        | Rejected because                                              |
|------------------------------------|---------------------------------------------------------------|
| Copy monorepo files verbatim        | Unresolvable `no_std` / cross-crate references                |
| Thin wrapper re-exporting monorepo | Requires the monorepo as a git dependency, defeating isolation |
| Single empty `lib.rs`               | Provides no value for downstream consumers or tests           |

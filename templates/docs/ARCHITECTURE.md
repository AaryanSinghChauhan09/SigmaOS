# Architecture — COMPONENT_NAME

> **Document status:** Draft / Review / Approved
> **Last updated:** YEAR-MM-DD
> **Authors:** <!-- names or GitHub handles -->

---

## 1. Component Purpose

<!-- One paragraph: What is this component? What single responsibility does it own?
     What does it explicitly NOT do? -->

`COMPONENT_NAME` is responsible for **<!-- primary responsibility -->**.

It does **not** handle:
- <!-- out-of-scope concern 1 -->
- <!-- out-of-scope concern 2 -->

### Goals

- <!-- goal 1 -->
- <!-- goal 2 -->

### Non-Goals

- <!-- non-goal 1 -->
- <!-- non-goal 2 -->

---

## 2. Context & Placement in SigmaOS

```
┌─────────────────────────────────────────────────────────┐
│  SigmaOS System                                         │
│                                                         │
│  ┌───────────┐    ┌─────────────────┐    ┌──────────┐  │
│  │ Component │───▶│  COMPONENT_NAME │───▶│ Component│  │
│  │     A     │    │                 │    │    B     │  │
│  └───────────┘    └─────────────────┘    └──────────┘  │
│                            │                            │
│                            ▼                            │
│                     ┌─────────────┐                     │
│                     │  Subsystem  │                     │
│                     └─────────────┘                     │
└─────────────────────────────────────────────────────────┘
```

<!-- Replace the ASCII diagram with an accurate depiction of where this
     component sits in the broader SigmaOS architecture. -->

---

## 3. Internal Structure

### Module Map

```
COMPONENT_NAME/
├── src/
│   ├── lib.rs          # Public API surface; re-exports
│   ├── main.rs         # Binary entry point (if applicable)
│   ├── config.rs       # Configuration types and validation
│   ├── error.rs        # Error and Result types
│   └── <!-- module -->/ # <!-- responsibility -->
└── tests/
    ├── integration.rs  # Integration tests
    └── property.rs     # Property-based tests
```

### Key Types

| Type | Module | Role |
|------|--------|------|
| `ComponentName` | `lib` | Primary handle; encapsulates all state |
| `Config` | `config` | Runtime configuration; validated on construction |
| `Error` | `error` | All error cases; non-exhaustive |
| <!-- Type --> | <!-- mod --> | <!-- role --> |

---

## 4. Public Interfaces

### Rust API

```rust
// Construction
let component = ComponentName::new(Config::default())?;

// Primary operation
component.run()?;

// Configuration access
let cfg: &Config = component.config();
```

### CLI Interface (if applicable)

```
COMPONENT_NAME [OPTIONS] <COMMAND>

Commands:
  run    Execute the primary operation
  check  Validate configuration

Options:
  -v, --verbose  Increase verbosity (-v, -vv, -vvv)
  -c, --config   Path to JSON config file
      --version  Print version
  -h, --help     Print help
```

### IPC / RPC Interface (if applicable)

<!-- Describe any inter-process communication channels: Unix sockets,
     message queues, shared memory regions, D-Bus, Cap'n Proto, etc. -->

---

## 5. Data Flow

```
Input
  │
  ▼
┌────────────────┐
│ Input Validation│  ← Fail fast: reject bad input at the boundary
└────────┬───────┘
         │
         ▼
┌────────────────┐
│  Core Logic    │  ← Pure, side-effect-free where possible
└────────┬───────┘
         │
         ▼
┌────────────────┐
│  Output / Side │  ← Isolated I/O layer
│    Effects     │
└────────────────┘
         │
         ▼
      Output
```

---

## 6. Dependencies

### Upstream (what this component consumes)

| Component / Service | Interface | Why |
|--------------------|-----------|-----|
| <!-- dep -->       | <!-- how -->| <!-- reason --> |

### Downstream (what consumes this component)

| Component / Service | Interface | Why |
|--------------------|-----------|-----|
| <!-- dep -->       | <!-- how -->| <!-- reason --> |

### External Crate Dependencies

See `Cargo.toml` for the authoritative list. Notable choices:

| Crate | Rationale |
|-------|-----------|
| `serde` | De-facto standard serialisation; zero-cost with derive macros |
| `thiserror` | Ergonomic error types with no runtime overhead |
| `clap` | Best-in-class CLI argument parsing with derive support |

---

## 7. Security Model

### Threat Model

| Threat | Likelihood | Impact | Mitigation |
|--------|-----------|--------|------------|
| Malformed input | High | Medium | Strict input validation at component boundary |
| Privilege escalation | Low | Critical | Least-privilege; no ambient authority |
| <!-- threat --> | <!-- L/M/H --> | <!-- L/M/H/C --> | <!-- mitigation --> |

### Trust Boundaries

```
Untrusted                │  Trusted
─────────────────────────┼─────────────────────
Network / user input     │  Validated Config
Environment variables    │  ComponentName state
Config files (pre-parse) │  (post-construction)
```

### Capabilities Required

| Capability | Reason | Minimised? |
|------------|--------|-----------|
| File read  | Config loading | Yes — only the specified path |
| <!-- cap -->| <!-- why -->| <!-- yes/no --> |

See [SECURITY.md](SECURITY.md) for full policy.

---

## 8. Error Handling Strategy

- All errors propagate as `Result<T, Error>` — no panics in production paths.
- `Error` is non-exhaustive; downstream callers must handle `_` variants.
- I/O errors are wrapped at the boundary with `#[from]` conversions.
- Panics are reserved for programmer errors (invariant violations) in debug builds.
- Exit codes follow POSIX conventions (see `main.rs`).

---

## 9. Performance Targets

| Metric | Target | Notes |
|--------|--------|-------|
| Startup latency | < <!-- Xms --> | Cold start, default config |
| Steady-state memory | < <!-- X MB --> | At nominal load |
| <!-- operation --> throughput | > <!-- X ops/s --> | <!-- conditions --> |
| P99 latency | < <!-- Xms --> | Under <!-- Y% --> load |

### Known Bottlenecks

- <!-- bottleneck 1 and rationale for accepting it -->

### Profiling Guide

```sh
# Build with debug symbols retained in release mode
RUSTFLAGS="-C debuginfo=2" cargo build --release

# Flame graph (requires cargo-flamegraph)
cargo flamegraph --bin COMPONENT_NAME -- run

# Heap profiling (requires heaptrack or valgrind)
valgrind --tool=massif ./target/release/COMPONENT_NAME run
```

---

## 10. Configuration Reference

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | `"COMPONENT_NAME"` | Instance identifier |
| `verbose` | `bool` | `false` | Enable verbose logging |
| <!-- field --> | <!-- type --> | <!-- default --> | <!-- description --> |

---

## 11. Architecture Decision Records (ADRs)

| # | Title | Status | Date |
|---|-------|--------|------|
| 001 | Use Rust as primary language | Accepted | <!-- date --> |
| 002 | Forbid unsafe code | Accepted | <!-- date --> |
| <!-- N --> | <!-- title --> | <!-- Proposed/Accepted/Deprecated --> | <!-- date --> |

---

## 12. Open Questions

- <!-- unresolved design question 1 -->
- <!-- unresolved design question 2 -->

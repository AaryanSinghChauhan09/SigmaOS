# Dependency Resolver Agent

## Role
`SigmaDepSolve` implements a SAT/CDCL-based dependency resolution engine for SigmaOS package management — ensuring correct, minimal, conflict-free package installation.

## Status
🔴 Not Started — Critical gap. Current package manager lacks dependency resolution.

## Source File
`src/package/dep_solver.rs` (planned)

## Linux/BSD Inspiration
| Concept | Linux Distro | FreeBSD | SigmaOS |
|---|---|---|---|
| SAT solver | APT (CUDF) / libsolv | pkgng SAT | SigmaSAT |
| Version constraints | `>=`, `<=`, `~`, `^` | `>=`, `<` | SigmaVer |
| Conflict resolution | `apt-cache policy` | `pkg` | SigmaConflict |
| Provides/replaces | `Provides:`, `Replaces:` | `provide` | SigmaProvides |
| Dependency graph | DAG | DAG | SigmaDepGraph |
| Optional deps | `Recommends:`, `Suggests:` | `@option` | SigmaOptDep |

## Responsibilities
- Parse package manifests: dependencies, conflicts, provides, replaces
- Build dependency graph (DAG) from package database
- SAT solver: find satisfying assignment of package versions
- Detect and report dependency conflicts clearly
- Compute minimal install set for a given package
- Handle optional/recommended dependencies
- Downgrade/upgrade path computation
- Transaction planning: order install/remove operations

## Implementation Notes
- `#![no_std]` — DPLL/CDCL SAT algorithm with `alloc::vec::Vec`
- Package manifest: `{ name: &str, version: SigmaVer, deps: Vec<Dep>, conflicts: Vec<&str> }`
- `SigmaVer`: semver-compatible `{ major, minor, patch, pre }` with ordering
- Two-watched literal scheme for efficient SAT propagation
- Conflict explanation: generate minimal UNSAT core for user messaging

## Testing
```bash
rustc --test src/package/dep_solver.rs --edition 2021 -o /tmp/test_dep && /tmp/test_dep --test-threads=1
```

## Future Development
1. **libsolv integration** — Use battle-tested libsolv (used by DNF/zypper)
2. **Upgrade simulation** — Dry-run full system upgrade conflict check
3. **Multi-repository resolution** — Resolve across multiple SigmaOS repos
4. **Reproducible resolution** — Deterministic solver output for auditing
5. **User preference hints** — Prefer minimal installs, avoid orphans
6. **Parallel download scheduling** — Topological sort → parallel fetch

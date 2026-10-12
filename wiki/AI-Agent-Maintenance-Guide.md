# AI Agent Master Maintenance Guide

> **SigmaOS Autonomous Agent Directive** — All AI agents, contributors, and automated systems operating on SigmaOS MUST strictly adhere to this specification.

---

## 1. Prime Invariants & Priority Order

In any situation involving design trade-offs or conflicting requirements, the priority order is absolute:

$$\text{Stability} > \text{Security} > \text{Performance} > \text{Feature Expansion}$$

1. **Stability First**: Never break existing builds, working tests, or public APIs. The repository must compile cleanly (`cargo check --lib`) at every step.
2. **Security Second**: Enforce constant-time cryptography, memory safety, least-privilege sandboxing (`pledge`, `unveil`, `capsicum`), and fail-closed error handling.
3. **Performance Third**: Optimize hot paths with zero-copy buffer passing, lock-free queues, and cache-friendly layout.
4. **Zero Dependency Bloat**: Eliminate third-party external library bloat by providing native, self-contained implementations in modern systems languages.

---

## 2. Permitted Languages Policy

| Language | Permitted Scope | Rules & Constraints |
|---|---|---|
| **Rust** | Primary OS components, kernel, drivers, userland, network, security | Default choice. Use `no_std` for kernel. Safe code preferred; all `unsafe` requires `// SAFETY:` rationale. |
| **Zig** | Early boot assembly, hardware init, C-ABI interoperability | Comptime safety, explicit allocators, zero hidden runtime. |
| **Nim** | Tooling, scripts, developer utilities | Must use `--mm:arc` or `--gc:none` for GC-free determinism. |
| **C / C++** | **STRICTLY FORBIDDEN** | Under no circumstances may C or C++ be introduced into OS components. |
| **Python** | **FORBIDDEN** for OS components | Only permitted for standalone offline repository maintenance harnesses. |
| **JavaScript / Go / Java** | **FORBIDDEN** for OS components | No runtime GC, no heavy web runtimes inside system daemons. |

---

## 3. GitHub Wiki & Documentation Rules

1. **Arch Linux Wiki Standard**: The wiki documentation must follow the structured, comprehensive standard of the Arch Linux Wiki.
2. **Strict Topic Uniqueness**: There MUST be strictly **ONE wiki page per topic**. Never create fragmented, duplicate, or version-suffixed pages (e.g., `Topic-V1.md`, `Topic-V2.md`).
3. **AI Maintenance Instructions Mandatory**: Every wiki page must conclude with a dedicated `## AI Agent Maintenance Instructions for Future Releases` section with precise technical invariants.
4. **Oldest-First Chronological Deprecation**: When a roadmap or specification `.md` file is fully implemented, its ideas and architecture must be merged into the authoritative topic wiki page, and the original `.md` specification file must be removed from the repository.

---

## 4. Git Repository & Branch Maintenance

1. **Single Source of Truth**: There must only be **ONE branch on GitHub: `main`**.
2. **No Stale Branches**: Remote branches must be merged into `main` and immediately deleted via `git push origin --delete <branch>`.
3. **Clean History**: Pull requests are reviewed, merged into `main`, and closed.
4. **CI Workflow Hygiene**: GitHub Actions workflows must specify least-privilege permissions (`permissions: contents: read`) and be kept minimal without redundant, non-functional stubs.

---

## 5. Verification Checklist for Every Agent Release

Before submitting any code or concluding a maintenance cycle, verify:
- [x] `cargo check --lib` exits with code 0.
- [x] No `unsafe` blocks without sound `// SAFETY:` explanations.
- [x] No hardcoded cryptographic values or secrets.
- [x] No raw pointer dereferences in safe functions.
- [x] Only `main` exists as a remote branch on `origin`.
- [x] Transferred `.md` specifications are reflected in the wiki and removed from the active tree.

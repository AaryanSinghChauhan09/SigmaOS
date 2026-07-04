# Contributing to SigmaOS

SigmaOS is a community-driven, sovereign OS. We welcome contributions of all kinds — kernel code, userspace tools, drivers, documentation, workflow templates, AI agent plugins, and package recipes.

See [Community Governance](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Community-Governance) for the full governance model, contributor roles, and decision-making process.

---

## Language Policy

SigmaOS uses a polyglot codebase. Each language has a specific domain:

| Language | Domain | Rules |
|---|---|---|
| **Rust** | Kernel core, drivers, critical subsystems | `#![no_std]` in kernel; no unsafe outside `kabi/` boundary |
| **Nim** | Userspace tools, CLI helpers, package manager | Compiled to C backend; no external packages |
| **Zig** | HAL, leaf performance-sensitive drivers | Static compilation only; no dynamic allocation |
| **Ada/SPARK** | Formal-verified security checkers | Must pass `gnatprove` before merge |
| **Assembly** | Boot, context switch, VMM fast paths | Only in `arch/` |
| **C** | Legacy driver compat shims in `drivers/linux/` | No new C in non-compat code |

Cross-language calls go through `kabi/` using `#[repr(C)]` structs.

---

## Quick Start

```bash
# 1. Fork and clone
git clone https://github.com/AaryanSinghChauhan09/SigmaOS
cd SigmaOS

# 2. Set up dev environment
./scripts/setup.sh          # installs Rust, Nim, Zig, QEMU toolchain

# 3. Build
cargo build --release       # Rust kernel + tools
nim c -d:release userland/agent/sigma_agent_main.nim  # Nim CLI tools

# 4. Run tests
cargo test                  # Rust unit tests
./sigma-agent benchmark quick  # Agent quality tests

# 5. Run in QEMU
make PROFILE=standalone qemu
```

---

## What to Work On

### Easy (good first issues)
- New sigma-agent workflow templates (`userland/agent/sigma_agent_workflow.nim`)
- New sigma-agent plugin (`sigma-agent plugin create my-skill`)
- Wiki page improvements (`wiki_repo/`)
- New package recipes (`sigma_pkg_registry/recipes/`)
- sigma-agent training samples (`userland/agent/sigma_agent_seed_v2.jsonl`)
- Translation/localisation (`locales/`)

### Medium
- New sigma-agent tools (implement `Tool` trait in `userland/agent/sigma_agent.rs`)
- New sigma-agent explain topics (`userland/agent/sigma_agent_explain.nim`)
- Package absorption improvements (`pkg/sigma_pkg_absorb.nim`)
- Linux compatibility shim (`userland/compat/sigma_linux_compat.nim`)
- New SDF drivers (`drivers/`)
- Benchmark test cases (`userland/agent/sigma_agent_benchmark.nim`)

### Hard (core team review required)
- Kernel subsystem changes (`kernel/`)
- Syscall interface changes (`kernel/syscalls/`)
- Security policy changes (`security/`)
- ABI changes (`kabi/`)

---

## Branch + Commit Conventions

```bash
# Branch naming
feature/sigma-agent-new-tool
fix/kernel-scheduler-edge-case
docs/wiki-migration-guide
refactor/pkg-absorb-rpm-support

# Commit format
type(scope): short description (≤50 chars)

Optional longer explanation.
Closes #123
```

Types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`, `security`, `perf`

---

## CI Requirements

All PRs must pass the 12-job CI pipeline:
- Rust build + clippy (kernel + tools)
- Nim build + type check (7 agent modules)
- 21-tool smoke tests
- GUI mirror validation (60+ mappings)
- Workflow automation tests
- Benchmark quick suite
- Shell integration test
- Training data seed generation

---

## Adding a sigma-agent Workflow Template

The fastest way to contribute automation value:

```bash
# 1. Write the YAML
# 2. Add to WORKFLOW_TEMPLATES in userland/agent/sigma_agent_workflow.nim
# 3. Test
sigma-agent workflow install my-template --dry-run
# 4. Submit PR
```

---

## Adding a sigma-agent Plugin

No core PR needed for plugins:

```bash
sigma-agent plugin create my-skill
# Edit ~/.config/sigma/agent/plugins/my-skill/plugin.toml
# Add commands, training.jsonl samples
# Publish to sigma_pkg_registry as sigma-agent-plugin-my-skill
```

---

## Documentation

- New wiki page → `wiki_repo/<Name>.md`
- New code doc → comment in the source file
- API reference → `docs/API_Reference.md`
- Do not duplicate existing canonical files (check `docs/README.md`)

---

## Licensing

All contributions use **MIT** for userspace and **GPL-2.0-or-later** for kernel code.

Add the appropriate SPDX header:
```rust
// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
```

---

## Getting Help

- GitHub Discussions: https://github.com/AaryanSinghChauhan09/SigmaOS/discussions
- Issues: https://github.com/AaryanSinghChauhan09/SigmaOS/issues
- Wiki: https://github.com/AaryanSinghChauhan09/SigmaOS/wiki
- Ask sigma-agent: `sigma-agent "how do I contribute to SigmaOS"`

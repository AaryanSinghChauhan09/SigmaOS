# OSS Absorption Strategy

> SigmaOS doesn't fork these projects. It studies them cleanroom, then builds sovereign replacements that are better.

---

## The 5 Absorption Categories

### 1. Kernel & System Foundation

| Project | What SigmaOS absorbs | SigmaOS equivalent |
|---|---|---|
| [torvalds/linux](https://github.com/torvalds/linux) | Driver model patterns, ABI stability techniques, multi-arch support (x86/ARM/RISC-V) | SDF (Sovereign Driver Framework), `kabi/`, multi-arch HAL |
| [redox-os/redox](https://github.com/redox-os/redox) | Rust microkernel design, memory-safe driver architecture, capability-based security | SigmaOS Rust kernel (`kernel/`), sigma_pledge/sigma_unveil |
| [serenityos/serenity](https://github.com/SerenityOS/serenity) | GUI ecosystem integration, browser engine, consistent UX patterns | Zenith Desktop (`zenith_desktop/`), sigma-browser |
| [QubesOS/qubes-core-admin](https://github.com/QubesOS/qubes-core-admin) | Compartmentalization, VM-based isolation, security domains | sigma_pledge + sigma_unveil + sigma-pod isolation |

### What to study:

- Linux: `Documentation/ABI/` for stable ABI patterns; `drivers/` for SDF driver model

- Redox: `kernel/src/` for memory-safe system calls; `drivers/` for Rust driver traits

- Serenity: `Userland/` for GUI app patterns; `Kernel/` for clean kernel architecture

- QubesOS: `qubes/` for domain isolation; `core-admin/` for policy enforcement

### 2. AI & CLI Integration

| Project | What SigmaOS absorbs | SigmaOS equivalent |
|---|---|---|
| [ggml-org/llama.cpp](https://github.com/ggml-org/llama.cpp) | GGUF model format, ChatML prompts, LoRA fine-tune, Whisper STT, llama-server HTTP API | sigma_llm.rs, sigma-ai daemon, llama.cpp backend |
| [BuilderIO/ai-shell](https://github.com/BuilderIO/ai-shell) | NL→shell translation, error auto-fix, `??` shorthand, self-update | sigma_agent_core.rs IntentParser, sigma_agent_explain.nim |
| [Aider-AI/aider](https://github.com/Aider-AI/aider) | Git-aware code editing, diff display, DPO training loop, file watching | sigma_agent_code.rs, sigma_agent_watch.nim, sigma_agent_learn.nim |
| [hermes-hq/hermes-ide](https://github.com/hermes-hq/hermes-ide) | Context-aware agent, IDE plugin HTTP API, notification integration | sigma_agent_daemon.nim /v1/ API, sigma_agent_notify.nim |
| [anthropics/claude-code](https://github.com/anthropics/claude-code) | ReAct loop, CLAUDE.md memory, `claude doctor`, streaming, supervised mode | sigma_agent_planner.rs, sigma_agent_memory.nim, sigma_agent_doctor.nim |
| [Aider-AI/aider](https://github.com/Aider-AI/aider) | `/remember`, `/architect`, workflow-from-description | sigma_agent_memory.nim, sigma_agent_script_gen.nim |

### What to study:

- llama.cpp: `src/llama.cpp` for GGUF parsing; `examples/server/` for HTTP inference API

- ai-shell: `src/commands/` for NL→command translation pipeline

- Aider: `aider/coders/` for diff-based code editing; `aider/history.py` for session memory

- Claude Code: `src/tools/` for tool schema design; system prompt patterns

### 3. Automation & Orchestration

| Project | What SigmaOS absorbs | SigmaOS equivalent |
|---|---|---|
| [n8n-io/n8n](https://github.com/n8n-io/n8n) | Node-based pipelines, event triggers, YAML format, step conditions | sigma_agent_workflow.nim — 8 templates, YAML + NL |
| [Azure/azure-cli](https://github.com/Azure/azure-cli) | Subcommand namespacing, extension system, `az upgrade`, automation runbooks | sigma_agent_main.nim 22-subcommand router, sigma_agent_plugin.nim |
| [github/copilot-cli](https://github.com/github/copilot-cli) | `??` explain, `?!` execute, shell integration, `git?` context | sigma_agent_explain.nim `??`, sigma_agent_shell_integration.nim |

### What to study:

- n8n: `packages/nodes-base/nodes/` for trigger/action node design

- azure-cli: `azure-cli/src/azure-cli/azure/cli/command_modules/` for namespace pattern

- copilot-cli: `src/commands/` for context-aware suggestion UX

### 4. Security & Sovereignty

| Project | What SigmaOS absorbs | SigmaOS equivalent |
|---|---|---|
| [openclaw/openclaw](https://github.com/openclaw/openclaw) | AI-driven policy advisor, event-driven agent actions, GUI parity principle | sigma_agent_security.nim policy advisor, sigma_agent_multi.nim |
| QubesOS | Hardware-enforced compartmentalization, Xen-based isolation | sigma_pledge/sigma_unveil, sigma-pod OCI isolation |
| OpenBSD pledge/unveil | Capability restriction, minimal attack surface | sigma_pledge + sigma_unveil syscalls (kernel/security/) |

### What to study:

- OpenBSD: `sys/kern/kern_pledge.c` for pledge() implementation

- QubesOS: `dom0/` for policy engine design

### 5. Package Ecosystem

| Project | What SigmaOS absorbs | SigmaOS equivalent |
|---|---|---|
| [NixOS/nixpkgs](https://github.com/NixOS/nixpkgs) | Declarative packaging, reproducible builds, derivation model | sigma_pkg_recipe.nim, reproducible_build.ps1 |
| [flatpak/flatpak](https://github.com/flatpak/flatpak) | Universal app packaging, sandboxed runtimes, OCI bundles | sigma_pkg_absorb.nim Flatpak support, sigma-pod |
| ArchLinux packages | AUR-style community packages, PKGBUILD format | sigma_pkg_registry/ recipes, sigpkg format |

### What to study:

- nixpkgs: `pkgs/` for derivation patterns; `lib/` for version resolution

- flatpak: `common/flatpak-oci-utils.c` for OCI bundle packaging

---

## Absorption Status

| Category | Study Done | Implemented | Production-Ready |
|---|---|---|---|
| Local LLM (llama.cpp) | ✅ | ✅ sigma_llm.rs 4 backends | 🔄 model files needed |
| NL→CLI (ai-shell) | ✅ | ✅ IntentParser + 22 subcommands | ✅ |
| Code editing (Aider) | ✅ | ✅ sigma_agent_code.rs | 🔄 LLM needed for quality |
| Workflow (n8n) | ✅ | ✅ sigma_agent_workflow.nim | ✅ 8 templates |
| CLI namespacing (azure-cli) | ✅ | ✅ 22 subcommand router | ✅ |
| Shell integration (copilot-cli) | ✅ | ✅ sigma_agent_shell_integration.nim | ✅ |
| Memory (Claude Code) | ✅ | ✅ sigma_agent_memory.nim | ✅ |
| Doctor (Claude Code) | ✅ | ✅ sigma_agent_doctor.nim | ✅ |
| Security advisor (OpenClaw) | ✅ | ✅ sigma_agent_security.nim | ✅ |
| Pkg absorption (Flatpak/Arch) | ✅ | ✅ sigma_pkg_absorb.nim | 🔄 tools needed |
| Linux compat (Linux kernel) | 🔄 | 🔄 sigma_linux_compat.nim | 🔄 compat-loader needed |
| GUI ecosystem (Serenity) | 🔄 | 🔄 Zenith DE Phase G | 🔄 in progress |
| Reproducible builds (Nix) | ✅ | ✅ scripts/reproducible_build.ps1 | 🔄 |
| Compartmentalization (QubesOS) | ✅ | ✅ sigma_pledge/sigma_unveil | ✅ |

---

## How to Contribute an Absorption

1. Pick a project from the list above

2. Study it cleanroom (read the code, don't copy)

3. Build a SigmaOS-sovereign equivalent

4. Add training samples to `sigma_agent_seed_v2.jsonl`

5. Add to OSS Reference Map (`docs/OSS_Reference_Map.md`)

6. Submit a PR

**Important:** All absorbed ideas must be implemented from scratch in SigmaOS's languages (Rust/Nim/Zig/Ada). We never copy GPL code. We study patterns and build sovereign replacements.

---

*See also: [Architecture Overview](Architecture-Overview) · [Linux Absorption Architecture](Linux-Absorption-Architecture) · [sigma-agent](sigma-agent) · [Security Model](Security-Model)*

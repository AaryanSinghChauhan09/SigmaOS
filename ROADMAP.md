# SigmaOS: The Sovereign Architecture Roadmap

As SigmaOS transcends its v1 Maturation Phase, our core engineering focus shifts toward absolute modularity, unbreakable automation, and the eradication of dependency bloat. This roadmap outlines the strategic transition of SigmaOS from a monolithic bare-metal entity into a highly scalable, Python-integrated ecosystem.

## 🧩 1. Modularisation & Subsystem Refactoring
**Flattening the Shard Explosion**
- We are moving away from scattered `SXX_` shard folders into a clean, unified canonical package hierarchy (e.g., `sigmaos/kernel/`, `sigmaos/gui/`, `sigmaos/cli/`, `sigmaos/agents/`).
- **Interface-Driven Contracts**: Every major subsystem will expose a formal Protocol/Abstract Base Class. This enforces the SOLID principles and makes swapping implementations trivial.
- **Plugin Architecture**: We will deprecate hardcoded agents in favor of auto-discovery via `~/.sigmaos/plugins/`. Community plugins will require only a `manifest.json` and a clean entry point.
- **Unified Test Tree**: Scattered testing fragments will be consolidated into a mirrored `tests/` tree (`tests/unit/kernel`, `tests/integration/boot`).

## ⚙️ 2. Automation & DevOps
**The `sigma-ci` Pipeline**
- We will deploy robust GitHub Actions (`sigma-ci.yml`) enforcing `mypy --strict`, linting, and automated boot smoke tests.
- **Auto-Shard Reconciliation**: Implement a tooling script (`tools/reconcile_shards.py`) to automate cleanup of redundant code shards and fossilized dependencies.
- **OmniAutomator Mission DSL**: Transition automation routines from hardcoded logic into declarative YAML/TOML missions.
- **Watch-Mode Daemon**: Add a `sigma watch` command to hot-reload missions dynamically without requiring a full reboot.

## 🎨 3. Customisation & Personalisation
**The `sigma_profile.toml` Identity Layer**
- Consolidate all scattered configurations into a single, per-user identity file managing themes, preferred agents, language, and startup routines.
- **Morphic UI Theme Engine**: Formalize the UI with drop-in theme folders (e.g., `~/.sigmaos/themes/`).
- **Module Feature Flags**: Dramatically improve boot times by allowing users to toggle heavy modules (e.g., `enable_gaming_hub = false`) at startup.
- **Agent Personas**: Allow users to dynamically hot-swap agent behavior (e.g., switching an AI assistant from a "verbose researcher" to a "terse developer").

## 🖥️ 4. Advanced CLI Mastery
**Unified `sigma` Entry Point**
- Eliminate scattered Python entry points in favor of a unified Click/Argparse group (`sigma boot`, `sigma agent`, `sigma audit`).
- **REPL Shell Mode**: Introduce `sigma shell` for persistent, chained, tab-completed command sessions.
- **`sigma doctor`**: Implement an environment health-check ensuring zero unexpected third-party packages have crept in.
- **`sigma diff`**: Enable binary and YAML state-comparisons to provide forensic audit trails of system changes.
- **Machine-Readable Outputs**: Guarantee that every CLI command supports `--output json` for composable scripting.

## 🚀 5. Ease of Use
**Portable Zero-Install Launchers**
- Bundle the entire SigmaOS entrypoint into a self-contained `sigma_nomad.py` payload. No setup required.
- **First-Run Interactive Wizard**: Guide users through jurisdiction, privacy level, and theme setup, generating the `sigma_profile.toml` instantly.
- **Contextual Help**: Implement `sigma help <topic>` providing plain-language explanations of core OS concepts rather than just dumping function signatures.

## ⚡ 6. Performance & Dependency Reduction
**Lazy Import Architecture**
- Audit the entire boot path. Force non-critical imports to load asynchronously, yielding 3-5x faster Time-To-First-Interaction (TTFI).
- **Compiled Hot-Paths**: Enforce `__slots__` on high-volume classes (agent tasks, kernel events) to slash memory overhead by 40-60%.
- **Async I/O for Logging**: Move all forensic logging, custody ledger writes, and inter-agent communication to non-blocking `asyncio` streams.
- **Strict Vendoring Policy**: When external cryptographic or utility primitives are absolutely necessary, we will copy the minimal source into `sigmaos/vendor/` and pin the hash. *We do not use `pip install` for core OS functions.*
- **Single HAL Module**: Centralize scattered `platform.system()` checks into a unified `sigmaos.hal` hardware abstraction layer.

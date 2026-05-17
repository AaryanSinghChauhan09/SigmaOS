# Σ SigmaOS Community Shard Reciper (sigma-build)

Inspired by **SlackBuilds** and **NixOS** custom overlays, `sigma-build` is the official repository of community-authored build scripts and declarative metadata recipes.

These recipes enable users to compile, attest, and package FOSS (Free and Open-Source Software) environments directly onto Silicon-Direct active computational shards of the SigmaOS lattice.

---

## 🛠️ Recipe Architecture

Each `.sb` (SigmaBuild) script runs natively inside the zero-dependency C++ package manager matrix (`SovereignPkgManager.cpp`), executing three isolated phases:

1. **`ATTEST`**: Validates the cryptographic PQC (Dilithium-5) signature of the upstream archive.
2. **`BUILD`**: Performs silicon-direct deterministic cross-compilation (x86_64, ARM64, or RISC-V).
3. **`INJECT`**: Installs files into the declarative read-only virtual overlay filesystem path (`S-OverlayFS`).

---

## 📂 Contained Recipes

- [deepseek.sb](file:///c:/Users/Aaryan/.gemini/antigravity/scratch/SigmaOS/sigma-build/deepseek.sb): Attested recipe to package DeepSeek LLM local Edge inference.
- [fritzing.sb](file:///c:/Users/Aaryan/.gemini/antigravity/scratch/SigmaOS/sigma-build/fritzing.sb): Attested recipe for Fritzing CAD electronic simulations.
- [ghidra.sb](file:///c:/Users/Aaryan/.gemini/antigravity/scratch/SigmaOS/sigma-build/ghidra.sb): Attested recipe to integrate Ghidra Forensic audits into CAINE workspaces.

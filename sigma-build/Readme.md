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


---

## 🚀 Usage Guide

To build a recipe from this repository, use the `sigma-build` CLI utility included in SigmaOS:

```bash

# 1. Sync the latest sigma-build repository

sigma-build sync

# 2. Build the package (downloads, attests, compiles)

sigma-build compile deepseek

# 3. Inject the compiled package into your active shard

sigma-build inject deepseek
```

### Profile Configuration

You can customize the compilation profile by editing `/etc/sigma/build.toml` or using environment variables:

```bash

# Build for a specific architecture (cross-compilation)

SIGMA_TARGET=riscv64gc sigma-build compile fritzing

# Enable LTO and PGO optimizations

SIGMA_PROFILE=production sigma-build compile ghidra
```

---

## 🛠️ Troubleshooting

- **Signature Verification Failed**: If the `ATTEST` phase fails, the upstream source tarball may have been tampered with or the sovereign root keys on your machine are outdated. Run `sigma-build keys update`.
- **Missing Build Dependencies**: `sigma-build` runs in a hermetic container. If a build fails due to missing headers (e.g., `linux/types.h`), the recipe (`.sb` file) must be updated to declare the dependency in its `DEPENDS=` array.
- **Out of Memory (OOM) during Linking**: Large C++ projects (like LLVM or Chromium-based apps) require significant RAM during the link phase (especially with LTO). Reduce parallel jobs: `SIGMA_JOBS=2 sigma-build compile ghidra`.

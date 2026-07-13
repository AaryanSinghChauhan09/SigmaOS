# Distro Absorption: Arch Linux & AUR Recipes

> **Status**: ✅ Absorbed | **Target Shard**: `SigmaRecipes.shard` | **Source Distro**: Arch Linux

---

## 1. Executive Summary

Arch Linux is favored by developers and power users for its lightweight core and the **Arch User Repository (AUR)**, which offers user-contributed build scripts (`PKGBUILD`). The strength of Arch is source-to-binary automation and up-to-date packaging.

The `SigmaRecipes.shard` absorbs this philosophy by implementing a sandboxed package-builder environment that parses PKGBUILD recipes, verifies dependencies, compiles packages from source, and generates signed SigmaOS Shard binaries (`.shard`).

---

## 2. Technical Features & Absorption Strategy

### 2.1 PKGBUILD Recipe Engine
- **Arch Concept**: A bash-based build description file (`PKGBUILD`) defining source URLs, integrity hashes, and compilation instructions (`build()`, `package()`).
- **Sovereign Implementation**: `SigmaRecipes` runs a secure, sandboxed shell engine that executes PKGBUILD procedures within a temporary, isolated namespace, capturing output file changes to build the package.

### 2.2 Sandboxed Source Builder
- **Arch Concept**: Compilation typically runs with the permissions of the building user, which risks system compromise if a PKGBUILD is malicious.
- **Sovereign Implementation**: Compilations are isolated in microVM templates with read-only network access (except for downloading source files). Once build completes, files are signed and saved as immutable objects.

---

## 3. Shard Architecture

```
┌─────────────────────────────────────────────────────────┐
│               ARCH ABSORPTION ENGINE                    │
├─────────────────────────────────────────────────────────┤
│  ┌───────────────────────┐   ┌───────────────────────┐  │
│  │   PKGBUILD Parser     │   │   Isolated MicroVM    │  │
│  │ (Recipe Shell Executor)│   │ (Secure Compile Room) │  │
│  └───────────┬───────────┘   └───────────┬───────────┘  │
│              └─────────────┬─────────────┘              │
│              ┌─────────────▼─────────────┐              │
│              │    Lattice Shard Packer   │              │
│              │  (Cryptographically Signed)│              │
│              └───────────────────────────┘              │
└─────────────────────────────────────────────────────────┘
```

---

## 4. Usage & Commands

To list and execute Arch absorption scripts:

```powershell
$ sigma distro list
Σ [INFO] Sovereign Linux Distro Absorption Registry:
  * Arch Linux   -> SigmaRecipes.shard          [Active]  (AUR-style recipe engine)
  ...

$ sigma distro absorb arch
Σ [INFO] Starting Deep-Lattice absorption of 'arch' paradigm...
Σ [INFO]   -> Loading SigmaRecipes.shard...
Σ [INFO]   -> Configuring AUR build environment...
Σ [SUCCESS] Arch Linux AUR recipe engine absorbed successfully!
```

---

## 5. References
- Arch Linux PKGBUILD Specifications
- Makepkg build tool guidelines
- Secure sandboxing rules for automated source builds

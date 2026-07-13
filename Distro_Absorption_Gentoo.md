# Distro Absorption: Gentoo Linux

> **Status**: 📋 Planned | **Source Paradigm**: Gentoo Linux + Portage | **Target Shard**: `SigmaOS Source-Build Layer / USE Flag System`

---

## 1. Executive Summary

Gentoo Linux is the ultimate source-code distribution: every package is compiled from source on the user's machine with fine-grained feature flags called **USE flags**. This enables extreme optimization — binaries tuned for the exact CPU, kernel, and feature set of each machine.

SigmaOS absorbs Gentoo's philosophy into the **Sovereign Build Layer** (`sigma-build`), allowing advanced users to:
- Compile system components with machine-specific CPU optimizations (`-march=native`)
- Toggle feature flags per-package to minimize binary size and attack surface
- Build reproducible, verifiable system images from source (mirroring NixOS reproducibility with Gentoo-style flexibility)

---

## 2. Key Features to Absorb

### 2.1 USE Flag System (`USE` → `sigma-features`)

Gentoo's USE flags allow per-package feature toggles at compile time. SigmaOS reimplements this as `sigma-features`:

```toml
# /etc/sigma/features.toml — machine-wide feature flags
[features]
# Hardware features
gpu_vulkan    = true    # Enable Vulkan renderer in all graphics packages
gpu_opencl    = false   # Disable OpenCL (no GPGPU usage on this machine)
cpu_avx512    = true    # Enable AVX-512 code paths (detected automatically)

# Security trade-offs
pie           = true    # Position-Independent Executables (all pkgs)
relro         = true    # Full RELRO hardening
stack_canary  = true    # Stack protection
lto           = true    # Link-time optimization (smaller, faster binaries)

# Desktop features
wayland       = true    # Wayland support
x11           = false   # Disable X11 shim (Wayland-only machine)
bluetooth     = true    # BT audio/input support
pulseaudio    = false   # Prefer PipeWire

# Language runtimes
python        = "3.12"  # Python version built into packages that need it
nodejs        = false   # No Node.js runtime built into system packages
java          = false   # No Java runtime
rust          = true    # Rust standard library present

[per_package]
"sigma-ssl" = { use = ["tls13_only", "+chacha20", "-rc4", "-md5"] }
"sigma-ssh" = { use = ["ed25519_only", "-dsa", "-rsa1"] }
"sigma-net" = { use = ["+quic", "+http3", "-ftp"] }
```

### 2.2 Native CPU Optimization (`-march=native` compilation)

```bash
# Detect and apply optimal compiler flags for this CPU
$ sigma build optimize-flags
Σ [INFO] Detecting CPU capabilities...
  Vendor: Intel Core i7-12700K
  Features: AVX2, AVX-512F, BMI2, AES-NI, POPCNT, F16C
  Recommended flags: -march=alderlake -O3 -pipe -fomit-frame-pointer

Σ [INFO] Generated: /etc/sigma/build/cflags.toml
  CFLAGS   = "-march=alderlake -O3 -pipe"
  CXXFLAGS = "-march=alderlake -O3 -pipe"
  RUSTFLAGS = "-C target-cpu=native -C opt-level=3"

Recompile affected packages? This will take ~2 hours. [y/N] y
Σ [BUILD] Rebuilding 47 packages with native optimizations...
```

### 2.3 Portage-Style Dependency Resolution

```rust
// userland/package_manager/portage.rs
// SPDX-License-Identifier: MIT

pub struct SigmaBuildGraph {
    packages: HashMap<PkgId, BuildSpec>,
    features: FeatureSet,
}

#[derive(Clone)]
pub struct BuildSpec {
    pub name:     String,
    pub version:  Version,
    pub sources:  Vec<SourceTarball>,
    pub patches:  Vec<Patch>,
    pub use_flags: Vec<UseFlag>,
    pub deps:     Vec<Dep>,         // Runtime deps
    pub bdeps:    Vec<Dep>,         // Build-time only deps
    pub configure: Vec<String>,     // ./configure flags or cargo features
}

impl SigmaBuildGraph {
    /// Resolve full build order for package + all dependencies
    pub fn resolve(&self, pkg: &str) -> Result<Vec<BuildSpec>> {
        let root = self.packages.get(pkg).ok_or(BuildError::NotFound)?;
        let resolved = topological_sort(root, &self.packages, &self.features)?;
        Ok(resolved)
    }

    /// Build a package from source with current feature flags
    pub fn build(&self, spec: &BuildSpec) -> Result<()> {
        let src = fetch_and_verify_source(&spec.sources, &spec.sha256)?;
        apply_patches(&src, &spec.patches)?;

        let flags = self.features.to_build_flags(&spec.use_flags);
        compile(src, flags)?;
        run_tests()?;
        install()?;
        Ok(())
    }
}
```

### 2.4 Binary Fallback (Emerge-like hybrid)

Not all users want to compile everything. SigmaOS uses a hybrid approach:

```bash
# Default: install pre-built binary
$ sigma pkg add firefox
Σ [PKG] Installing firefox 120.0 (pre-built, AVX2 optimized)

# Source mode: compile from scratch with local USE flags
$ sigma pkg add firefox --source
Σ [BUILD] Building firefox from source with features: [wayland, av1, no-x11]
  Fetching sources...    [██████████] 100%
  Applying patches...    [██████████] 100%
  Configuring...         Done (17s)
  Compiling (24 jobs)... [████░░░░░░] 40% — ETA 6m
```

---

## 3. Performance Gains from Native Compilation

| Package | Pre-built binary | Native `-march=native` | Speedup |
|:--------|:----------------|:-----------------------|:--------|
| `zstd` compression | 2,100 MB/s | 3,400 MB/s | +62% |
| `openssl` AES-GCM | 1.8 GB/s | 4.1 GB/s (AES-NI) | +128% |
| `ffmpeg` H.264 decode | 280 fps | 380 fps (AVX-512) | +36% |
| Python `numpy` matmul | 14 GFLOPS | 22 GFLOPS | +57% |

---

## 4. References & Standards

- Gentoo Linux — `gentoo.org` (GPL-2.0)
- Portage — `wiki.gentoo.org/wiki/Portage` (GPL-2.0)
- GCC `-march` optimization flags — GCC manual
- LLVM/Clang CPU target flags — LLVM documentation

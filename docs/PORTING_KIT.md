# Σ SigmaOS Porting Kit v0.1

Welcome to the **SigmaOS Sovereign Porting Kit**. This guide provides everything needed to port high-impact open-source tools (e.g., `ripgrep`, `fd`, `sqlite`, `tmux`, `git`, `python`) to run natively or in sandboxed WASM microVM containers under SigmaOS.

---

## 1. Quick Start: Porting a Rust CLI Tool

To port a standard Rust CLI application to SigmaOS:

### Step 1: Add `sigma.toml` Manifest
Create a `sigma.toml` file in the target repository root:

```toml
[package]
name = "ripgrep-sigma"
version = "14.1.0"
authors = ["Sovereign Porting Team"]
edition = "2021"

[target.x86_64-sigmaos]
linker = "sigma-ld"
opt-level = 3
lto = "fat"

[sandbox]
allow_read = ["/"]
allow_write = ["/tmp"]
network = "deny"
```

### Step 2: Compile with `sigma` CLI
Run the unified CLI build pipeline:

```bash
sigma build --release
```

### Step 3: Attest and Verify
Perform hardware-backed provenance verification:

```bash
sigma attest
```

---

## 2. GitHub Actions Porting Workflow Template

Save this template as `.github/workflows/sigma-port-verify.yml`:

```yaml
name: SigmaOS Porting & Attestation

on:
  push:
    branches: [ main, master ]
  pull_request:
    branches: [ main, master ]

jobs:
  verify-port:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install SigmaOS SDK
        run: |
          curl -sSL https://get.sigmaos.io/sdk.sh | sh
          echo "$HOME/.sigma/bin" >> $GITHUB_PATH

      - name: Compile Sovereign Recipe
        run: sigma build --release

      - name: Perform Hardware Attestation
        run: sigma attest

      - name: Run Test Suite
        run: sigma run --sandbox -- cargo test
```

---

## 3. "Port & Earn" Bounty Program

We offer bounties for porting high-impact open-source utilities to SigmaOS:

| Tier | Category | Examples | Reward |
| :--- | :--- | :--- | :--- |
| **Tier 1** | Search & File Utilities | `ripgrep`, `fd`, `fzf`, `bat` | $500 / port |
| **Tier 2** | Terminal & Shells | `tmux`, `zsh`, `fish`, `zellij` | $1,000 / port |
| **Tier 3** | Database & Runtime | `sqlite`, `redis`, `wasmtime` | $2,500 / port |
| **Tier 4** | Language Ecosystems | `python`, `node`, `go`, `zig` | $5,000 / port |

### How to Claim:
1. Fork the target repository and apply the `sigma.toml` manifest.
2. Submit a PR to `SigmaOS/recipes` containing the verified `.spkg` recipe.
3. Once the automated CI attestation pipeline passes, the bounty is awarded directly.

---

## 4. Verification & Testing Standards

All ports must satisfy three non-negotiable criteria:
1. **Zero External Dependencies**: Must compile against standard no_std or sovereign `klib` collections.
2. **Reproducible Provenance**: Must yield bit-for-bit identical binary hashes across independent builds.
3. **Capability Sandboxing**: Must enforce default-deny network and filesystem policies via `AppSandboxEngine`.

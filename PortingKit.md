# 🛠️ SigmaOS Porting Kit v0.1 & "Port & Earn" Bounty Program

> **"Lower friction, reproducible ports, measurable wins."**

***

## 🚀 Overview

The **SigmaOS Porting Kit** provides automated templates, WASM ABI fast-paths, and CI quality gates to port any open-source CLI or GUI tool to run natively under SigmaOS in < 15 minutes.

***

## 💰 "Port & Earn" Bounty Tiers

| Tier | Target OSS Tool | Reward | Requirements |
|---|---|---|---|
| **Tier 1 (CLI Essentials)** | `ripgrep`, `fd`, `fzf`, `bat`, `eza` | $500 / port | Bit-for-bit reproducible WASM/no\_std build + tests |
| **Tier 2 (Developer Tools)** | `git`, `tmux`, `neovim`, `sqlite` | $1,000 / port | Instant dev sandbox restore in < 1s + WASI hostcalls |
| **Tier 3 (Runtimes & DBs)** | Python 3, Node.js, Go, PostgreSQL | $2,500 / port | Multi-tenant isolation + hardware Dilithium attestation |

***

## ⚡ Quick Start: Porting a Rust CLI in 3 Commands

```bash
# 1. Initialize port template
sigma init my_ported_tool --template=wasm-port

# 2. Build reproducible WASM binary
sigma build

# 3. Attest and run in instant sandbox (<1ms)
sigma attest
sigma run
```

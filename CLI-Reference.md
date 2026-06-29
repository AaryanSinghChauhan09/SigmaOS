# SigmaOS CLI Reference

The `sigma` command-line tool is a unified orchestrator for every aspect of the SigmaOS development lifecycle.
It ships as a **single static binary** compiled from `tools/sigma-cli.rs` (Rust, no external dependencies).

## Installation

```bash
cargo build --release --manifest-path tools/Cargo.toml
# Binary output: tools/target/release/sigma
# Copy to PATH:
sudo cp tools/target/release/sigma /usr/local/bin/sigma
```

## Global Options

| Flag | Description |
|------|-------------|
| `--json` | Machine-readable JSON output on stdout |

## Commands

### `sigma init <name>`

Bootstrap a new SigmaOS kernel module, driver, or userland app.

```bash
sigma init my_driver
```

Generates:

```text
my_driver/
├── Config.sigma        # project metadata (name, arch, license)
└── src/
    └── main.rs         # no_std entry stub
```

---

### `sigma build [--target <arch>]`

Unified build orchestrator. Wraps CMake/Cargo/Go under one command.

```bash
sigma build --target riscv64gc
sigma build --target x86_64 --release
```

---

### `sigma run [--headless] [--serial]`

Boot the built kernel image inside QEMU.

```bash
sigma run
sigma run --headless --serial
sigma run --debug     # attaches gdb on :1234
sigma run --snapshot  # saves VM state on exit
```

---

### `sigma debug`

Launches QEMU with `-s -S`, waits for gdb on port `:1234`, and auto-loads kernel symbols.

---

### `sigma pkg <action> [name]`

Package manager for the Sigma Store registry.

| Action | Description |
|--------|-------------|
| `add <name>` | Download and install a package |
| `remove <name>` | Uninstall a package |
| `list` | Show installed packages |
| `search <query>` | Search the registry |
| `audit` | Check for vulnerabilities in installed packages |

---

### `sigma sdk <version>`

Toolchain manager — like `rustup` but for SigmaOS cross-compilers.

```bash
sigma sdk nightly
sigma sdk 0.3.0
```

---

### `sigma test [--bench]`

Run unit tests on the host and integration tests inside a booted QEMU instance.

---

### `sigma lint`

Static analysis: Clippy (Rust), clang-tidy (C/C++), and SigmaOS-specific kernel safety rules.

---

### `sigma fmt`

Multi-language formatter across the entire repository.

---

### `sigma trace [--pid <pid>]`

Live-attach to a running SigmaOS instance over serial/vsock and stream syscall + scheduler events.

---

### `sigma image [--minimal] [--with pkg1,pkg2]`

Build a reproducible bootable image (`.img`, `.iso`, UEFI ESP, Raspberry Pi SD).

---

### `sigma node <action>`

Fleet control: `enroll`, `status`, `update`, `ssh`, `logs`, `metrics`.

---

### `sigma key`

Generate device identity keys, sign packages/images, and verify the chain of trust (TPM/HSM integration).

---

### `sigma update [--channel stable|beta|nightly]`

Perform an A/B partition OTA swap with automatic rollback on boot failure.

---

### `sigma doctor`

Checks that all toolchain dependencies (Rust, Zig, QEMU, etc.) are healthy and correctly versioned.

## Shell Completions

```bash
sigma completions bash  >> ~/.bashrc
sigma completions zsh   >> ~/.zshrc
sigma completions fish  > ~/.config/fish/completions/sigma.fish
sigma completions pwsh  >> $PROFILE
```

## Plugin System

Any binary named `sigma-<name>` on `PATH` is auto-discovered as a subcommand (cargo-style):

```bash
# e.g., place sigma-profiler in /usr/local/bin:
sigma profiler start
```

# CLI Reference

SigmaOS ships three complementary CLI tools and one interactive shell.
All commands emit clean, coloured output by default and can switch to machine-readable JSON where noted.

---

## sigma — OS Development CLI

The unified developer CLI, compiled from `tools/sigma-cli.rs` (Rust, zero dependencies).

### Installation

```bash
cargo build --release --manifest-path tools/Cargo.toml
sudo cp tools/target/release/sigma /usr/local/bin/sigma
```

### Global flags

| Flag | Description |
|------|-------------|
| `--json` | Machine-readable JSON on stdout |
| `--verbose` / `-v` | Extra diagnostic output |
| `--version` / `-V` | Print version and exit |
| `--help` / `-h` | Show help and exit |

Any subcommand also accepts `--help` for detailed per-command usage:

```bash
sigma build --help
sigma update --help
```

---

### Development Commands

#### `sigma init <name>`

Scaffold a new SigmaOS kernel module, driver, or userland app.

```bash
sigma init my_driver
```

Generates:

```
my_driver/
├── Config.sigma        # project metadata (name, arch, license)
└── src/
    └── main.rs         # no_std entry stub
```

---

#### `sigma build [--target <arch>] [--release] [--profile <name>]`

Unified build orchestrator wrapping CMake / Cargo / Ninja.

```bash
sigma build --target riscv64gc --release
sigma build --profile minimal
```

Supported targets: `x86_64`, `aarch64`, `riscv64gc`

---

#### `sigma run [--headless] [--serial] [--debug] [--snapshot]`

Boot the kernel image inside QEMU.

```bash
sigma run
sigma run --headless --serial
sigma run --debug        # pause at entry, await gdb on :1234
sigma run --snapshot     # save VM state on exit
```

---

#### `sigma debug`

Launch QEMU with `-s -S`, wait for gdb on port `:1234`, and auto-load kernel symbols.

---

#### `sigma test [--bench]`

Run unit tests on the host and integration tests inside a booted QEMU instance.
`--bench` includes the benchmark suite.

---

#### `sigma lint`

Static analysis: Clippy (Rust), clang-tidy (C/C++), and SigmaOS kernel safety rules.

---

#### `sigma fmt [--check]`

Multi-language formatter. `--check` verifies formatting without writing any changes.

---

#### `sigma trace [--pid <pid>] [--filter <syscall>]`

Live-attach to a running SigmaOS instance over serial/vsock and stream syscall + scheduler events.

---

#### `sigma image [--minimal] [--with pkg1,pkg2] [--format iso|img|esp]`

Build a reproducible bootable image: `.iso`, raw `.img`, UEFI ESP, or Raspberry Pi SD layout.

---

### Packaging & SDK

#### `sigma pkg <action> [name]`

| Action | Description |
|--------|-------------|
| `add <name>` | Download and install a package |
| `remove <name>` | Uninstall a package |
| `list` | Show installed packages |
| `search <query>` | Search the Sigma Store registry |
| `audit` | Vulnerability scan of installed packages |

---

#### `sigma sdk <version>`

Toolchain manager. Switch the active cross-compiler version, similar to `rustup`.

```bash
sigma sdk nightly
sigma sdk 0.4.0
```

---

#### `sigma key [--algo dilithium5|ed25519] [--export]`

Generate device identity keys, sign packages/images, and verify the chain of trust.
Default algorithm: **Dilithium-5** (post-quantum).

---

#### `sigma update [--channel stable|beta|nightly] [--dry-run]`

Perform an A/B partition OTA swap with automatic rollback on boot failure.

```bash
sigma update --channel nightly
sigma update --dry-run          # preview without applying
```

---

### Infrastructure

#### `sigma node <action>`

Fleet control commands: `enroll`, `status`, `update`, `ssh`, `logs`, `metrics`.

---

#### `sigma config <validate|show|set>`

Manage the `sigma.toml` declarative configuration.

```bash
sigma config validate           # schema check
sigma config show               # print config
sigma config set kernel.debug=true
```

---

#### `sigma doctor [--fix]`

Check toolchain dependencies (Rust, Zig, Nim, QEMU, CMake, Ninja, GDB, Clang).
Detects real installed versions by running each tool.
`--fix` prints install guidance for any missing tool.

```
  ✓ Rust toolchain         rustc 1.81.0-nightly
  ✓ QEMU (x86_64)          QEMU emulator version 8.2.0
  ✗ Zig compiler           NOT FOUND
```

---

### Meta Commands

#### `sigma version`

Print version, build ID, and Rust toolchain info.

```
Σ SigmaOS Unified CLI
  Version  : 15.0 (Zenith)
  Build    : dev
  Rust     : nightly
  License  : GPL-2.0-or-later
```

#### `sigma completions <bash|zsh|fish|pwsh>`

Emit shell completion scripts and install them:

```bash
sigma completions bash  >> ~/.bashrc
sigma completions zsh   >> ~/.zshrc
sigma completions fish  > ~/.config/fish/completions/sigma.fish
sigma completions pwsh  >> $PROFILE
```

#### `sigma help [command]`

Show top-level help or detailed usage for a specific command:

```bash
sigma help
sigma help update
sigma help pkg
```

---

### Plugin System

Any binary named `sigma-<name>` on `PATH` is auto-discovered as a subcommand (cargo-style):

```bash
# Install a custom profiler plugin:
cp sigma-profiler /usr/local/bin/
sigma profiler start          # delegates to sigma-profiler start
```

Unknown commands attempt plugin lookup before printing an error.

---

## sigma — Developer App CLI

The app-lifecycle CLI for SigmaOS userland development, compiled from `tools/sigma-cli/main.go`.

### Commands

| Command | Description |
|---------|-------------|
| `sigma init <name>` | Scaffold a new app (`sigma.json` + `index.html`) |
| `sigma run <dir>` | Launch app against local `sigmad-process` |
| `sigma sign <dir>` | Ed25519-sign a bundle → `sigma.sig` |
| `sigma verify <dir>` | Verify bundle digest and signature |
| `sigma caps <dir>` | List capabilities declared in `sigma.json` |
| `sigma health` | Query `sigma-healthd` via Unix socket |
| `sigma sysctl <key>[=val]` | Read or write a kernel parameter |
| `sigma list` | List installed apps (local scan or daemon query) |
| `sigma pkg add\|remove\|search\|audit` | Package management |
| `sigma version` | Print version info |

### Examples

```bash
# Create and run a new app
sigma init my-app
sigma run my-app

# Sign and verify before distribution
sigma sign my-app
sigma verify my-app

# Inspect declared capabilities
sigma caps my-app

# System health check
sigma health

# Tweak a kernel parameter
sigma sysctl kernel.sched.latency_ns=50000

# Package management
sigma pkg add sigma-vr-compositor
sigma pkg search neuro
sigma pkg audit
```

### Error Messages

The Go CLI includes actionable error messages:

- `sigmad-process unreachable at 127.0.0.1:17382 — is sigmad running?`
- `healthd socket not found at /run/sigma/healthd.sock — is sigma-healthd running?`
- `cannot read my-app/sigma.json — Did you run 'sigma init my-app'?`

### Signing Key

The Ed25519 signing key is stored at `~/.sigmaos/signing.key` (auto-generated on first `sign`).
Override with `SIGMA_SIGNING_KEY=/path/to/key`.

---

## sigma-sh — Sovereign Interactive Shell

A full POSIX-compatible shell written in Rust, designed for SigmaOS userspace.

### Starting

```bash
sigma-sh                   # interactive REPL
sigma-sh script.sigma      # execute a script
sigma-sh --version         # print version
sigma-sh --help            # show usage
```

### Prompt

The interactive prompt shows `user@host:cwd (git-branch) ❯` in colour.
A red `✗` replaces `❯` when the last command exited non-zero.
The home directory is shortened to `~`.

### Built-in Commands

#### Navigation
| Command | Description |
|---------|-------------|
| `cd [dir\|-\|~]` | Change directory (`-` = OLDPWD, `~` = HOME) |
| `pwd` | Print working directory |

#### Output
| Command | Description |
|---------|-------------|
| `echo [-n] [...]` | Print text (supports `\n`, `\t` escapes) |

#### Variables & Environment
| Command | Description |
|---------|-------------|
| `export [K=V]` | Set/export environment variables |
| `unset [K]` | Remove environment variable |
| `env` | List all environment variables |
| `read <VAR>` | Read a line from stdin into a variable |

#### Aliases & Discovery
| Command | Description |
|---------|-------------|
| `alias [K='V']` | Define or list aliases |
| `unalias [-a] K` | Remove alias(es) |
| `type <name>` | Show how a name would be interpreted (builtin/alias/function/executable) |
| `which <name>` | Locate an executable on PATH |

#### Process Control
| Command | Description |
|---------|-------------|
| `kill [-SIGNAL] <pid>` | Send a signal to a process |

#### History & Session
| Command | Description |
|---------|-------------|
| `history` | Show numbered command history (consecutive duplicates suppressed) |
| `source <file>` / `.` | Execute script in current shell context |
| `exit [code]` | Exit sigma-sh |
| `help` | Show categorised built-in help |

#### Test/Conditions
| Command | Description |
|---------|-------------|
| `test <expr>` / `[ <expr> ]` | Evaluate conditions (built-in: `-e`, `-f`, `-d`, `-z`, `-n`, `=`, `!=`, `-eq`, `-ne`, `-lt`, `-gt`, `-le`, `-ge`) |

### Shell Syntax

```bash
# Pipes
ls -la | grep .sigma | wc -l

# Redirections
cmd > out.txt     # stdout overwrite
cmd >> out.txt    # stdout append
cmd < in.txt      # stdin
cmd 2> err.txt    # stderr

# Background
long_running_task &

# Sequences
compile; test; deploy

# Conditionals
make && echo "success" || echo "failed"

# Variables
name="sigma"
echo "Hello, ${name:-world}"
echo "Last exit: $?"

# Control flow (scripts)
if [ -f sigma.toml ]; then
    echo "config found"
else
    echo "no config"
fi

for arch in x86_64 aarch64 riscv64gc; do
    sigma build --target $arch
done

# Functions
greet() {
    echo "Hello from sigma-sh"
}
greet
```

### Scripting

Script files use the `.sigma` extension:

```bash
#!/usr/bin/env sigma-sh
# build-all.sigma

for target in x86_64 aarch64; do
    echo "Building $target..."
    sigma build --target $target --release
done
echo "All builds complete. Exit: $?"
```

Run with `sigma-sh build-all.sigma` or make executable with `chmod +x`.

---

## sigma_cli_host.sh — Host Dev Wrapper

A Bash wrapper that maps CLI verbs to automation scripts on the developer's host machine.

```bash
scripts/sigma_cli_host.sh <command> [args...]
```

| Command | Description |
|---------|-------------|
| `update` | Pull latest sources and rebuild |
| `backup` | Snapshot the current workspace |
| `sync [args]` | Git sync (delegates to `sigma_git_sync.sh`) |
| `branch-check [args]` | Verify branch naming/parity rules |
| `automation <sub>` | Run an automation sub-task (update/backup/clean/lint/test) |
| `profile` | List available build profiles |
| `status` | Show repo status and recent commits |
| `version` | Print wrapper version |
| `--help` | Show usage |

---

## Environment Variables

| Variable | Used by | Description |
|----------|---------|-------------|
| `SIGMA_SIGNING_KEY` | Go CLI | Path to Ed25519 private key |
| `SIGMA_BUILD_ID` | Rust CLI | Injected at build time for `version` output |
| `SIGMA_RUST_VERSION` | Rust CLI | Injected at build time |
| `HOME` | sigma-sh | Home directory for `~` expansion |
| `HOSTNAME` | sigma-sh | Shown in prompt |
| `USER` / `USERNAME` | sigma-sh | Shown in prompt |
| `PATH` | sigma-sh | Used by `which`, `type`, plugin discovery |

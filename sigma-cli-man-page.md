# sigma-cli Manual Page

## NAME

`sigma` — SigmaOS unified OS development CLI

## SYNOPSIS

```
sigma [--json] [--verbose] <command> [command-options]
sigma --version
sigma --help
sigma help <command>
```

## DESCRIPTION

`sigma` is the single-binary orchestrator for the full SigmaOS development lifecycle. It replaces `cmake`, `cargo`, `qemu`, `gdb`, and package manager invocations with a single consistent interface. Compiled from `tools/sigma-cli.rs` (Rust, zero third-party crates).

## GLOBAL OPTIONS

| Flag | Description |
|------|-------------|
| `--json` | Emit machine-readable JSON on stdout |
| `--verbose` / `-v` | Extra diagnostic output |
| `--version` / `-V` | Print version and exit |
| `--help` / `-h` | Show help and exit |

Any subcommand accepts `--help` for per-command detail:
```
sigma build --help
sigma pkg --help
```

## COMMANDS

### Development

| Command | Description |
|---------|-------------|
| `init <name>` | Scaffold a new kernel module / driver / app |
| `build [--target] [--release] [--profile]` | Unified CMake/Cargo/Ninja build |
| `run [--headless] [--serial] [--debug]` | Boot image in QEMU |
| `debug` | Attach gdb-server on `:1234` |
| `test [--bench]` | Unit + QEMU integration tests |
| `bench [suite] [--save]` | Performance benchmarks (8 suites) |
| `lint` | Clippy + clang-tidy + kernel safety rules |
| `fmt [--check]` | Multi-language formatter |
| `trace [--pid] [--filter]` | Live syscall event streaming |
| `image [--format] [--minimal]` | Reproducible bootable image |

### Packaging & SDK

| Command | Description |
|---------|-------------|
| `pkg add\|remove\|list\|search\|audit` | Sigma Store package manager |
| `sdk <version>` | Cross-compiler toolchain manager |
| `key [--algo] [--export]` | Identity & signing key management |
| `update [--channel] [--dry-run]` | A/B OTA partition swap |
| `profile list\|show\|set` | Build profiles (desktop/cloud/embedded/gaming) |

### Infrastructure

| Command | Description |
|---------|-------------|
| `node enroll\|status\|upgrade\|logs` | Fleet/cluster node control |
| `config validate\|show\|set` | sigma.toml management |
| `doctor [--fix]` | Toolchain health check (detects real versions) |

### Meta

| Command | Description |
|---------|-------------|
| `version` | Print version, build ID, Rust toolchain |
| `completions bash\|zsh\|fish\|pwsh` | Generate shell completions |
| `help [command]` | Detailed per-command help |

## PLUGIN SYSTEM

Any binary named `sigma-<name>` on `PATH` is auto-discovered:

```bash
cp sigma-profiler /usr/local/bin/
sigma profiler start   # delegates to sigma-profiler start
```

## EXAMPLES

```bash
# Scaffold and build a driver
sigma init my-nvme-driver
sigma build --target aarch64 --release

# Run in QEMU headless
sigma run --headless --serial

# Install a package
sigma pkg add sigma-vr-studio

# Check toolchain health
sigma doctor --fix

# Run benchmarks and save results
sigma bench all --save

# Set a build profile
sigma profile set cloud
sigma build --profile cloud

# Generate completions
sigma completions bash >> ~/.bashrc

# JSON output for CI
sigma doctor --json | jq '.[] | select(.status=="missing")'
```

## ENVIRONMENT VARIABLES

| Variable | Description |
|----------|-------------|
| `SIGMA_BUILD_ID` | Injected at build time, shown by `version` |
| `SIGMA_RUST_VERSION` | Rust toolchain version shown by `version` |

## SEE ALSO

`sigma-sh(1)`, `sigma-pkg(1)`, `sigma-debug(1)`, `sigma-monitor(1)`, `sigma-log(1)`

## VERSION

sigma 15.0 (Zenith)

## LICENSE

GPL-2.0-or-later

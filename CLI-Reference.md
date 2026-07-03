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

#### `sigma bench [suite] [--save]`

Run the built-in performance benchmark suite against the running kernel.

| Suite | Measures |
|-------|----------|
| `boot` | Cold-boot to interactive prompt |
| `syscall` | `getpid()` call throughput |
| `ipc` | Unix socket round-trip latency |
| `fs` | Random 4K NVMe read speed |
| `scheduler` | Context switch latency |
| `network` | TCP loopback throughput |
| `crypto` | AES-256-GCM throughput (AES-NI) |
| `pqc` | Dilithium-5 sign/verify operations/sec |
| `all` | All suites (default) |

```bash
sigma bench
sigma bench syscall
sigma bench all --save      # save results to bench-results.json
```

---

#### `sigma profile <list|show|set> [name]`

Manage build profiles — predefined shard sets for different deployment targets.

| Profile | Description |
|---------|-------------|
| `desktop` | Full GUI + driver set (default) |
| `minimal` | Kernel + essential userspace only |
| `cloud` | Headless, optimised for VM/server |
| `embedded` | RTOS-style, stripped memory footprint |
| `gaming` | GPU-optimised desktop + gaming stack |

```bash
sigma profile list
sigma profile show gaming
sigma profile set cloud
sigma build --profile embedded
```

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

---

## Specialist CLI Tools (`tools/cli/`)

These standalone binaries are auto-discoverable as `sigma-<name>` plugins.
Each one supports `--help`, `--version`, and `--json`.

---

### sigma-monitor — Real-time system monitor

```bash
sigma-monitor [mode] [--interval <sec>] [--count <n>] [--json]
```

| Mode | Description |
|------|-------------|
| `cpu` | CPU usage and frequency per core |
| `mem` | RAM and swap usage |
| `net` | Network I/O by interface |
| `disk` | Disk I/O and filesystem usage |
| `proc` | Top processes by CPU |
| `all` | All metrics (default) |
| `watch` | Continuous refresh (like top) |

```bash
sigma-monitor                     # single snapshot, all metrics
sigma-monitor cpu --json          # JSON CPU stats
sigma-monitor watch --interval 1 --count 10
```

---

### sigma-secure — Security hardening & audit

```bash
sigma-secure <audit|harden|pqc|attest|policy|report> [options]
```

| Command | Description |
|---------|-------------|
| `audit [--fix]` | Full system security audit; `--fix` auto-remediates |
| `harden [--profile <p>]` | Apply hardening profile (`cis`, `nist`, `stig`, `sovereign`) |
| `pqc <gen|list|verify>` | Manage Dilithium-5 post-quantum keys |
| `attest` | Verify TPM 2.0 attestation chain |
| `policy <list|set|export>` | Manage security policies |
| `report [--output <file>]` | Generate signed security report |

```bash
sigma-secure audit --fix
sigma-secure harden --profile cis
sigma-secure pqc gen
sigma-secure attest --json
sigma-secure report --output security.html
```

---

### sigma-forensics — Digital forensics

```bash
sigma-forensics <scan|carve|timeline|hash|report|chain> [options]
```

| Command | Description |
|---------|-------------|
| `scan [--path <dir>]` | Scan for IoCs and anomalies |
| `carve [--image <file>]` | File carving from raw disk image |
| `timeline [--start X] [--end Y]` | Build activity timeline |
| `hash <target>` | Compute SHA-256 integrity hashes |
| `report [--output <file>]` | Generate forensic report |
| `chain <file>` | Verify cryptographic chain of custody |

```bash
sigma-forensics scan --path /
sigma-forensics carve --image disk.img
sigma-forensics timeline --start 1751500000 --end 1751600000
sigma-forensics hash /etc
sigma-forensics report --output forensics.html
sigma-forensics chain evidence.bin
```

---

### sigma-snapshot — System snapshots

```bash
sigma-snapshot <create|list|restore|delete|diff|export> [options]
```

| Command | Description |
|---------|-------------|
| `create [--name <n>] [--type full\|incremental\|config]` | Take a snapshot |
| `list` | List all snapshots |
| `restore <id> [--dry-run]` | Restore a snapshot |
| `delete <id> [--force]` | Remove a snapshot |
| `diff <id1> <id2>` | Compare two snapshots |
| `export <id> --output <file>` | Export snapshot to archive |

```bash
sigma-snapshot create --name pre-update --type full
sigma-snapshot list
sigma-snapshot restore 2 --dry-run
sigma-snapshot diff 1 3
sigma-snapshot export 2 --output backup.tar.zst
```

---

### sigma-cluster — Cluster management

```bash
sigma-cluster <status|enroll|drain|evict|upgrade|logs|metrics> [options]
```

| Command | Description |
|---------|-------------|
| `status [--node <n>]` | Cluster and node health |
| `enroll --node <addr>` | Add a node to the cluster |
| `drain --node <name>` | Cordon and drain a node |
| `evict --node <name>` | Force-remove a node |
| `upgrade [--channel <ch>]` | Rolling upgrade of all/one nodes |
| `logs --node <n> [--tail <n>]` | Node log streaming |
| `metrics [--node <n>]` | Cluster performance metrics |

```bash
sigma-cluster status
sigma-cluster enroll --node 10.0.0.5
sigma-cluster upgrade --channel nightly
sigma-cluster logs --node sigma-node-01 --tail 50
sigma-cluster metrics --json
```

---

### sigma-hypervisor — VM management

```bash
sigma-hypervisor <list|create|start|stop|destroy|console|snapshot|info> [options]
```

| Command | Description |
|---------|-------------|
| `list` | List all VMs |
| `create --name <n> [opts]` | Create a new VM |
| `start --name <n>` | Start a VM |
| `stop --name <n> [--force]` | Stop a VM |
| `destroy --name <n> --force` | Permanently delete a VM |
| `console --name <n>` | Attach to serial console |
| `snapshot --name <n> [--label]` | Checkpoint a running VM |
| `info --name <n>` | Detailed VM information |

```bash
sigma-hypervisor create --name test-vm --mem 2048 --cpus 2 --arch aarch64
sigma-hypervisor start --name test-vm
sigma-hypervisor console --name test-vm
sigma-hypervisor snapshot --name test-vm --label pre-test
sigma-hypervisor stop --name test-vm
```

---

### sigma-recover — System recovery

```bash
sigma-recover <status|boot|filesystem|rollback|rescue|verify> [options]
```

| Command | Description |
|---------|-------------|
| `status` | Boot partition state and filesystem health |
| `boot [--partition A\|B] [--dry-run]` | Repair or switch boot partition |
| `filesystem [--dev <d>] [--dry-run]` | Run sigma_fsck on a device |
| `rollback [--to <id>] [--dry-run]` | Roll back to snapshot or OTA partition |
| `rescue` | Drop into minimal recovery shell |
| `verify` | Check kernel + initrd integrity |

```bash
sigma-recover status
sigma-recover verify
sigma-recover filesystem --dev /dev/sda1
sigma-recover rollback --to 2 --dry-run
sigma-recover boot --partition B
```

---

### sigma-hal-info — Hardware inspector

```bash
sigma-hal-info [subsystem] [--json]
```

| Subsystem | Description |
|-----------|-------------|
| `cpu` | CPU topology, features, microcode version |
| `mem` | Memory topology, DIMM slots, speed |
| `pci` | PCI/PCIe device tree |
| `usb` | USB device tree |
| `gpu` | GPU/display adapters and driver info |
| `storage` | Block devices (NVMe, SATA) |
| `net` | Network adapters and firmware |
| `sensors` | Thermal sensors, fan RPM, voltages |
| `all` | All subsystems (default) |

```bash
sigma-hal-info
sigma-hal-info cpu
sigma-hal-info sensors --json
sigma-hal-info pci
```

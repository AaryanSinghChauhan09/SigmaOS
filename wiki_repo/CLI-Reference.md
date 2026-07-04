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

---

## Additional Specialist CLI Tools

---

### sigma shard — Kernel Lattice Shard Manager

Managed through the main `sigma` CLI. Kernel lattice shards are hot-pluggable kernel modules with cryptographic attestation.

```bash
sigma shard <list|load|unload|info|reload|verify> [name|path]
```

| Action | Description |
|--------|-------------|
| `list` | Show all loaded shards with base address, version, status, size |
| `load <path>` | Load a `.shard` file — verifies Dilithium-5 sig before mapping |
| `unload <name>` | Unload a shard (`--force` to override protection on core shards) |
| `info <name>` | Show base address, sections, exported symbols, dependencies |
| `reload <name>` | Hot-reload a shard without rebooting |
| `verify` | Re-verify Dilithium-5 signatures of all loaded shards |

```bash
sigma shard list
sigma shard info sigma-net
sigma shard load ./my-driver.shard
sigma shard reload sigma-gpu-hal
sigma shard verify --json
sigma shard unload sigma-custom --force
```

---

### sigma-debug — Kernel Shard Debugger

GDB-style debugger for live kernel shards. Connects to `/run/sigma/debugd.sock` on bare metal; operates in simulation mode otherwise.

```bash
sigma-debug <command> [--pid <n>] [--addr <hex>] [--len <n>] [--json]
```

| Command | Description |
|---------|-------------|
| `shard list\|info\|load\|unload` | Shard lifecycle management |
| `mem read\|dump\|map` | Memory inspection and hex dump |
| `reg [--pid <n>]` | CPU register dump (rax–r15, rip, rflags, cr0–cr4) |
| `sym resolve\|search` | Address→symbol or name→address resolution |
| `bp set\|list\|del\|clear` | Breakpoint management |
| `bt [--pid <n>]` | Full stack backtrace with source line info |
| `attach --pid <n>` | Attach to running process, pause all threads |
| `script <file>` | Execute a debug script |
| `repl` | Interactive debug REPL session |

```bash
# Inspect a shard and dump memory
sigma-debug shard list
sigma-debug mem dump --addr 0xffff000000001000 --len 128

# Symbol resolution
sigma-debug sym resolve 0xffff000000001234
sigma-debug sym search sigma_syscall

# Set breakpoint and backtrace
sigma-debug bp set --addr 0xffff000000001234
sigma-debug bt --pid 1 --json

# Interactive session
sigma-debug repl
```

Full manual: [sigma-debug Manual](sigma-debug-Manual)

---

### sigma-log — Unified Log Viewer & Anomaly Detector

Reads from `/run/sigma/journal.sock`. Falls back to sample data when the socket is unavailable.

```bash
sigma-log <command> [options]
```

| Command | Description |
|---------|-------------|
| `tail [--lines <n>] [--source <s>] [--level <l>]` | Show recent log entries |
| `follow [--source <s>]` | Stream logs in real time (like `tail -f`) |
| `search --query <q> [--level <l>]` | Full-text search with term highlighting |
| `dump [--output <file>]` | Dump all entries to stdout or file |
| `stats` | Log level distribution with bar charts |
| `anomaly [--threshold <n>]` | Detect spikes (OOM, timeouts, auth failures) |
| `export --format json\|csv\|syslog` | Export in structured format |

Log levels (ascending): `trace` `debug` `info` `warn` `error` `critical`

```bash
sigma-log tail --lines 50 --level warn
sigma-log follow --source sigma-security
sigma-log search --query "OOM" --json
sigma-log stats
sigma-log anomaly --threshold 2
sigma-log export --format json --output /tmp/sigma.json
```

Full manual: [sigma-log Manual](sigma-log-Manual)

---

### sigma-fix — AI-Guided Patch Suggestion

Scans the system for fixable issues and generates targeted patches with root-cause explanations.

```bash
sigma-fix <scan|suggest|apply|rollback|explain|list> [options]
```

| Command | Description |
|---------|-------------|
| `scan [--path <dir>]` | Scan for fixable security/config/performance issues |
| `suggest --id <fix-id>` | Show AI-generated diff patch for an issue |
| `apply --id <fix-id> [--auto] [--dry-run]` | Apply a patch (with or without confirmation) |
| `rollback --id <fix-id>` | Undo a previously applied fix |
| `explain --id <fix-id>` | Root cause analysis and fix rationale |
| `list` | Show all available/applied fixes with severity |

Severity levels: `CRITICAL` `HIGH` `MEDIUM` `LOW`

```bash
# Scan and see what's fixable
sigma-fix scan
sigma-fix list

# Inspect a fix before applying
sigma-fix suggest --id FIX-0001
sigma-fix explain --id FIX-0001

# Apply fixes
sigma-fix apply --id FIX-0001 --dry-run
sigma-fix apply --id FIX-0001 --auto
sigma-fix apply --id FIX-0007

# Undo if needed
sigma-fix rollback --id FIX-0001

# JSON output for CI pipelines
sigma-fix scan --json | jq '.scan.fixes[] | select(.severity=="CRITICAL")'
```

Built-in fix database covers:

| ID | Severity | Category | Issue |
|----|----------|----------|-------|
| FIX-0001 | CRITICAL | security | SSH root login enabled |
| FIX-0002 | HIGH | security | Unexpected SUID binaries |
| FIX-0003 | HIGH | pqc | Missing Dilithium-5 keys |
| FIX-0004 | MEDIUM | config | sigma.toml missing [network] section |
| FIX-0005 | MEDIUM | kernel | GPU shard driver version mismatch |
| FIX-0006 | LOW | perf | Transparent huge pages disabled |
| FIX-0007 | LOW | security | kernel.kptr_restrict not set |

Full manual: [sigma-fix Manual](sigma-fix-Manual)

---

### sigma-top — Process Monitor

htop-style real-time process monitor. Reads `/proc/meminfo` and `/proc/stat` on Linux; simulated on other platforms.

```bash
sigma-top [--sort cpu|mem|pid|name] [--filter <str>] [--count <n>]
          [--interval <sec>] [--once] [--json]
```

| Option | Description |
|--------|-------------|
| `--sort <field>` | Sort column: `cpu` (default), `mem`, `pid`, `name` |
| `--filter <str>` | Show only processes matching name/cmd/user |
| `--count <n>` | Show top N processes (default: 20) |
| `--interval <sec>` | Refresh every N seconds (default: 2) |
| `--once` | Single snapshot then exit |
| `--json` | Machine-readable output |

```bash
sigma-top                              # live, sorted by CPU
sigma-top --sort mem --count 10        # top 10 by memory
sigma-top --filter sigma               # only sigma* processes
sigma-top --once --json                # single JSON snapshot
sigma-top --interval 1                 # refresh every second
```

Display shows: CPU usage bar, MEM usage bar, per-process PID/PPID/user/state/CPU%/MEM/threads/command.
Running processes are highlighted in cyan; high CPU (>50%) in yellow/red.

---

### sigma_fsck — Filesystem Consistency Checker

Checks and optionally repairs sigma-fs filesystems.

```bash
sigma_fsck [--dev <path>] [--repair] [--verbose] [--dry-run] [--json]
```

| Option | Description |
|--------|-------------|
| `--dev <path>` | Device or image to check (default: `/dev/sda1`) |
| `--repair` | Attempt to fix found errors |
| `--verbose` | Show per-phase progress |
| `--dry-run` | Report errors without writing anything |
| `--journal` | Check journal log only |
| `--json` | Machine-readable output |

Error types detected: `ORPHAN_INODE`, `BAD_CHECKSUM`, `JOURNAL_DIRTY`

```bash
sigma_fsck                                  # check /dev/sda1
sigma_fsck /dev/nvme0n1p2                   # check by positional arg
sigma_fsck --dev /dev/sda1 --repair         # check and fix
sigma_fsck --dev /dev/sda1 --dry-run        # preview fixes
sigma_fsck --dev /dev/sda1 --json           # CI-friendly output
```

Exit code 0 = clean filesystem. Exit code 1 = unresolved errors remain.

---

### sigma_diagnostics — Comprehensive System Diagnostics

Multi-module diagnostic tool for field support and CI health gates.

```bash
sigma_diagnostics [mode] [--output <file>] [--json]
```

| Mode | Description |
|------|-------------|
| `full` | All diagnostic modules (default) |
| `quick` | Critical failures and warnings only |
| `kernel` | Kernel state: version, lockdown, SMEP/SMAP, watchdog |
| `network` | Network connectivity, firewall, DNS, IPv6 forwarding |
| `storage` | Filesystem health, SMART, encryption, mount options |
| `security` | PQC keys, SSH config, CVE scan, LSM, kptr_restrict |
| `report` | Same as `full` + write to `--output` file |

```bash
sigma_diagnostics                            # full diagnostic sweep
sigma_diagnostics quick                      # fast pass, critical only
sigma_diagnostics security --json           # CI security gate
sigma_diagnostics full --output report.txt  # generate report file
sigma_diagnostics kernel --json | jq '.[] | select(.status=="fail")'
```

Each check outputs: ✓ (pass), ⚠ (warn), ✗ (fail), with an actionable fix command shown for failures.

---

## sigma-pkg — Package Manager (Nim CLI)

Full-featured sovereign package manager. Source: `pkg/sigma_pkg_cli.nim`

```bash
sigma-pkg <command> [options] [packages...]
```

| Command | Description |
|---------|-------------|
| `install <pkg...>` | Install packages from Sigma Store |
| `remove <pkg...>` | Remove installed packages |
| `search <query>` | Search registry by name or description |
| `list [--filter <s>]` | List installed packages |
| `update [pkg...]` | Check for updates (all if none specified) |
| `audit` | Scan installed packages for CVEs |
| `info <pkg>` | Detailed package metadata |
| `clean` | Remove orphaned packages and cache |
| `pin <pkg>` | Prevent a package from auto-updating |
| `unpin <pkg>` | Re-enable auto-updates |
| `export [--output <file>]` | Export installed package list |

```bash
sigma-pkg install zenith-desktop sigma-browser
sigma-pkg search vr
sigma-pkg list
sigma-pkg update
sigma-pkg audit
sigma-pkg info sigma-agent
sigma-pkg pin sigma-core
sigma-pkg export --output packages.txt
sigma-pkg install sigma-ml --json
```

All commands support `--json`, `--dry-run`, and `--force`.

---

## sigma-net — Network Management CLI

Full-featured network management. Source: `tools/sigma-net.rs`

```bash
sigma-net <command> [options]
```

### Interface Commands

| Command | Description |
|---------|-------------|
| `status [iface]` | Show all interfaces or one specific (addr, MAC, speed, RX/TX) |
| `up <iface>` | Bring interface up |
| `down <iface>` | Take interface down |
| `ip <iface> <addr/prefix>` | Set static IP address (e.g. `10.0.0.1/24`) |
| `dhcp <iface>` | Request DHCP lease |
| `mac <iface> [new-mac]` | Show or set MAC address |
| `stats [iface]` | RX/TX bytes, packets, errors, drops |

### Routing

| Command | Description |
|---------|-------------|
| `route list` | Show routing table |
| `route add <prefix> via <gw>` | Add static route |
| `route del <prefix>` | Remove route |

### DNS

| Command | Description |
|---------|-------------|
| `dns show` | Show configured DNS servers |
| `dns set <server>` | Set DNS resolver (supports DoT) |
| `dns resolve <hostname>` | Perform a DNS lookup |

### Diagnostics

| Command | Description |
|---------|-------------|
| `ping <host> [-c n]` | ICMP ping (default 4 packets) |
| `trace <host>` | Traceroute hop-by-hop |
| `scan <subnet>` | ARP network discovery |
| `capture <iface> [-n n]` | Packet capture (writes `/tmp/sigma-cap.pcap`) |

### WiFi

| Command | Description |
|---------|-------------|
| `wifi scan` | Scan for available networks with RSSI bars |
| `wifi connect <ssid> <psk>` | Connect to a WPA3 network |
| `wifi disconnect` | Disconnect from current network |
| `wifi status` | Show connection status |

### Firewall

| Command | Description |
|---------|-------------|
| `fw list` | Show all sigma-fw rules |
| `fw allow <rule>` | Add allow rule |
| `fw deny <rule>` | Add deny rule |
| `fw flush --force` | Remove all rules |

```bash
# Interface management
sigma-net status
sigma-net up eth0
sigma-net ip eth0 10.0.0.10/24
sigma-net dhcp wlan0

# Routing
sigma-net route list
sigma-net route add 192.168.1.0/24 via 10.0.0.1

# Diagnostics
sigma-net ping 8.8.8.8 -c 10
sigma-net trace sigmaos.app
sigma-net scan 10.0.0.0/24

# WiFi
sigma-net wifi scan
sigma-net wifi connect "MyNetwork" "passphrase"
sigma-net wifi status

# Firewall
sigma-net fw list
sigma-net fw allow "tcp dport 8080"
sigma-net fw deny "tcp dport 23"

# JSON output for scripts
sigma-net status --json | jq '.[].addr'
sigma-net route list --json | jq '.[0].gateway'
```

---

## Professional Calculator Tools (`tools/cli/`)

---

### dose-calc — Clinical Dosage Calculator

Evidence-based drug dose calculator. Reference tool — clinical decisions require a licensed practitioner.

```bash
dose-calc <command> [options]
```

| Command | Description |
|---------|-------------|
| `drug <name> --weight <kg>` | Weight-based dosing for named drug |
| `list [--category <cat>]` | Browse drug database (15 drugs: antibiotics, analgesics, etc.) |
| `bsa --height <cm> --weight <kg>` | Body surface area (Mosteller formula) |
| `creatinine --age --weight --scr [--female]` | CrCl via Cockcroft-Gault |
| `ideal-bw --height <cm> [--female]` | Ideal body weight (Devine formula) |
| `renal --egfr <n> --drug <name>` | Renal dose adjustment by eGFR |
| `hepatic --class A\|B\|C --drug <name>` | Hepatic dose adjustment (Child-Pugh) |
| `aki --baseline <n> --current <n>` | AKI staging (KDIGO 2012) |

```bash
dose-calc drug paracetamol --weight 70
dose-calc drug vancomycin --weight 85
dose-calc list --category antibiotic
dose-calc creatinine --age 65 --weight 70 --scr 1.4 --female
dose-calc renal --egfr 28 --drug ciprofloxacin
dose-calc hepatic --class B --drug paracetamol
dose-calc aki --baseline 0.9 --current 2.1
dose-calc bsa --height 170 --weight 75
```

---

### gst-calc — India GST Calculator

GST, TDS, TCS, HSN/SAC lookups, and e-Invoice IRN helper for Indian businesses.

```bash
gst-calc <command> [options]
```

| Command | Description |
|---------|-------------|
| `tax --amount <n> --rate <n> [--inter]` | Calculate CGST+SGST or IGST |
| `invoice --amount <n> --rate <n> --desc <s>` | Generate invoice line item |
| `hsn <code\|keyword>` | HSN/SAC code lookup (27 codes) |
| `reverse --amount <n> --rate <n>` | Extract GST from inclusive amount |
| `tds --amount <n> --section <s>` | TDS calculation (12 sections) |
| `tcs --amount <n> --rate <n>` | TCS calculation |
| `cess --amount <n> --rate <n>` | GST Compensation Cess |
| `irn --gstin <n> --amount <n>` | e-Invoice IRN generation helper |
| `gstr1 [--period MMYYYY]` | GSTR-1 filing summary |

```bash
# Standard GST on professional services
gst-calc tax --amount 50000 --rate 18

# Inter-state (IGST)
gst-calc tax --amount 100000 --rate 18 --inter

# Extract GST from GST-inclusive price
gst-calc reverse --amount 59000 --rate 18

# HSN lookup
gst-calc hsn computer
gst-calc hsn 8471

# TDS on contractor payment
gst-calc tds --amount 500000 --section 194C

# Invoice line for JSON export
gst-calc invoice --amount 25000 --rate 18 --desc "Software Development" --json
```

---

### struct-load — Structural Load Analysis

Civil/structural engineering load calculations per IS 456:2000 and IS 800:2007.

```bash
struct-load <command> [options]
```

| Command | Description |
|---------|-------------|
| `beam --span --udl --pl [--cantilever]` | Reactions, max moment and shear |
| `column --width --depth --height --axial` | IS 456 Cl. 39.3 capacity + utilisation |
| `slab --span --ly --dl --ll` | Two-way slab moments (IS 456 Table 26) |
| `foundation --width --depth --axial --sbc` | Isolated footing pressure + adequacy |
| `wind --vb --cf --area` | Wind load (IS 875 Part 3) |
| `seismic --seismic-weight --zone --sa --r --i` | Base shear (IS 1893) |
| `combo --dl --ll --wind` | Load combinations (IS 456 Table 18) |
| `section --profile <name>` | Steel section properties (IS 808) |

```bash
# Simply-supported beam: 6m span, 12 kN/m UDL, 20 kN point load
struct-load beam --span 6 --udl 12 --pl 20

# Cantilever beam
struct-load beam --span 3 --udl 8 --cantilever

# Column capacity check
struct-load column --width 0.4 --depth 0.4 --height 3.5 --axial 800

# Two-way slab design moments
struct-load slab --span 4 --ly 6 --dl 3.5 --ll 2.0

# Isolated footing adequacy
struct-load foundation --width 1.5 --depth 0.45 --axial 600 --sbc 150

# IS 1893 seismic base shear, Zone III
struct-load seismic --seismic-weight 5000 --zone III --sa 2.5 --r 5 --i 1

# Load combinations governing
struct-load combo --dl 5 --ll 3 --wind 1.5

# Steel section lookup
struct-load section --profile "ISMB 300"
struct-load section --profile "ISMB"    # list all ISMB sections
```

---

## sigma-drv — Driver Lifecycle Manager

Source: `tools/sigma-drv.rs` · Pillar 1: Driver & Hardware Support

```bash
sigma-drv <list|load|unload|probe|log|bench|reload|info|abi|port> [options]
```

| Command | Description |
|---------|-------------|
| `list [--category net\|storage\|gpu\|input\|audio\|usb]` | List loaded SDF drivers with state and device count |
| `load <name>` | Load a driver (verifies Dilithium-5 ABI sig first) |
| `unload <name> [--force]` | Unload (sigma-heal auto-restarts if crashes) |
| `probe --pci <id>` | Run probe() on a PCI device (e.g. `8086:15f3`) |
| `log <name> [--tail <n>]` | Show driver log ring buffer |
| `bench <name> [--duration <sec>]` | Driver throughput benchmark |
| `reload <name>` | Hot-swap driver without reboot |
| `info <name>` | Version, ABI, vendor, devices |
| `abi check` | Verify all loaded drivers against SDF ABI v3 |
| `port --linux <module>` | AI-assisted Linux → SDF driver porting guide |

```bash
sigma-drv list --category gpu
sigma-drv probe --pci 8086:15f3
sigma-drv bench sigma-e1000 --duration 10
sigma-drv reload sigma-nvidia-hal
sigma-drv abi check
sigma-drv port --linux iwlwifi
sigma-drv list --json | jq '.[] | select(.state=="loaded") | .name'
```

---

## sigma-ai — Sovereign AI Agent

Source: `tools/sigma-ai.rs` · Pillar 3: AI & Automation

```bash
sigma-ai <ask|explain|heal|workflow|model|status|script|translate|security|predict> [options]
```

| Command | Description |
|---------|-------------|
| `ask "<prompt>" [--lang hi]` | Query local LLM (offline, no telemetry) |
| `explain <command>` | Explain a command before running (educational mode) |
| `heal [--crash <dump>]` | Analyse crash dumps and system anomalies |
| `workflow <list\|run\|install> [name]` | Manage automation workflows |
| `model <list\|load\|download> [name]` | Manage GGUF models |
| `status` | Agent daemon health (uptime, model, request count) |
| `script "<intent>"` | Generate a .sigma script from natural language |
| `translate "<cmd>" --to hi` | Translate CLI command to a language |
| `security <scan\|advise\|explain>` | AI security advisor |
| `predict <cpu\|mem\|disk\|network>` | ML-based resource usage prediction |

```bash
sigma-ai ask "why is my system slow?"
sigma-ai ask "डिस्क क्यों भर रही है?" --lang hi
sigma-ai explain "sigma-secure audit --fix"
sigma-ai heal
sigma-ai script "check security and auto-fix every Sunday"
sigma-ai workflow run security-hardening
sigma-ai model list
sigma-ai security scan
sigma-ai predict mem
sigma-ai translate "sigma update --channel nightly" --to hi
```

All AI commands are logged to `/var/log/sigma/ai-audit.jsonl` for transparency.

---

## sigma-fleet — Enterprise Device Management

Source: `tools/sigma-fleet.rs` · Pillar 5: Community & Enterprise

```bash
sigma-fleet <status|register|deregister|policy|update|inventory|audit|lock|unlock|list|logs> [options]
```

| Command | Description |
|---------|-------------|
| `status` | Agent heartbeat, policy, health |
| `register --server <url> --token <t>` | Register device with fleet server |
| `policy <get\|show\|set>` | Fetch and apply `.sigma-policy` |
| `update <status\|pull\|apply>` | OTA update lifecycle |
| `inventory` | Push hardware inventory to fleet server |
| `audit [--push]` | Show or push tamper-evident audit log |
| `list` | All managed devices (from fleet server) |
| `lock [--wipe]` | Lock device remotely |
| `unlock --token <t>` | Unlock device |
| `logs <push\|show>` | Fleet log management |

```bash
sigma-fleet register --server fleet.sigmaos.app --token mytoken
sigma-fleet status
sigma-fleet policy set
sigma-fleet update pull
sigma-fleet update apply
sigma-fleet inventory
sigma-fleet audit --push
sigma-fleet list --json | jq '.[] | select(.status=="online")'
```

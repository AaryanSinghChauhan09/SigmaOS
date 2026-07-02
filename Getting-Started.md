# Getting Started with SigmaOS

> Everything you need to go from zero to running SigmaOS in under 10 minutes.

---

## Option 1: Try it in QEMU (fastest)

```bash
# Clone the repo
git clone https://github.com/AaryanSinghChauhan09/SigmaOS
cd SigmaOS

# Install dependencies (Ubuntu/Debian)
sudo apt install qemu-system-x86 nasm gcc make nim rustup

# Build and run
make PROFILE=standalone qemu

# Or use the quick script
./qemu-boot.sh
```

---

## Option 2: Install on real hardware

```bash
# Download the latest ISO
wget https://github.com/AaryanSinghChauhan09/SigmaOS/releases/latest/download/sigmaos-zenith.iso

# Write to USB (replace /dev/sdX with your USB device)
sudo dd if=sigmaos-zenith.iso of=/dev/sdX bs=4M status=progress

# Boot from USB — follow the installer
```

---

## Option 3: Dual boot alongside Linux

See [Migration Guide](Migration-Guide) for step-by-step dual-boot setup alongside Ubuntu, Fedora, or Arch.

---

## First Steps After Installing

```bash
# 1. Run the AI agent health check
sigma-agent doctor

# 2. Install shell integration (adds `ai` alias, Ctrl+K, autocomplete)
sigma-agent install --shell-integration
source ~/.sigma_agent_rc

# 3. Start the AI daemon (GitHub knowledge sync + LLM completions)
sigma-agent daemon start

# 4. Install automation workflow templates
sigma-agent workflow install --all

# 5. Try your first AI commands
sigma-agent "system info"
sigma-agent "set dark mode"
sigma-agent "install sigma-edit"
sigma-agent security scan
```

---

## The AI Agent (sigma-agent)

sigma-agent is the heart of SigmaOS — every GUI action has a CLI equivalent:

```bash
# Natural language → OS commands
sigma-agent "open app sigma-terminal"
sigma-agent "connect wifi HomeNetwork secretpass"
sigma-agent "accessibility high-contrast on"
sigma-agent "workspace 2"

# Explain anything
sigma-agent explain "what is sigma_pledge"
sigma-agent explain --concept "how paging works"
sigma-agent explain --error "cargo build" "linker error"

# Automate workflows
sigma-agent workflow create "backup my code every Friday"
sigma-agent workflow run weekly-backup --dry-run

# Security
sigma-agent security scan
sigma-agent security policies

# Voice control
sigma-agent voice                    # speak your command
sigma-agent voice --session          # continuous hands-free
```

Full docs: [sigma-agent](sigma-agent)

---

## Running Linux Apps

SigmaOS can run Linux packages without modification:

```bash
# Install from sigma-pkg (native)
sigma-pkg install firefox

# Absorb a .deb package
sigma-pkg absorb firefox.deb && sigma-pkg install firefox

# Absorb an AppImage
sigma-pkg absorb Blender.AppImage && sigma-pkg install blender

# Run a Docker container
sigma-compat container ubuntu:22.04 bash

# Check compatibility status
sigma-compat status
```

See [Linux Absorption Architecture](Linux-Absorption-Architecture) for details.

---

## Building from Source

```bash
# Install toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh  # Rust
curl -sSL https://nim-lang.org/choosenim/init.sh | bash -s -- -y  # Nim

# Build kernel (Rust)
cargo build --release

# Build AI agent (Nim)
cd userland/agent
nim c -d:release --opt:speed -o:sigma-agent sigma_agent_main.nim
cp sigma-agent /usr/local/bin/

# Build Rust engine (optional, improves accuracy)
cargo build --release -p sigma-agent-core
cp target/release/sigma-agent-core /usr/local/bin/

# Full OS build
make PROFILE=standalone all -j$(nproc)
```

---

## Your Development Workflow

```bash
# 1. Set up memory for your project
sigma-agent memory project init           # creates .sigma_memory
sigma-agent memory add "my project is a web server in Rust"

# 2. Use the AI for code help
sigma-agent "fix src/main.rs add error handling"
sigma-agent "explain what this function does" --code src/handler.rs
sigma-agent multi --agent developer "review my Rust code for safety issues"

# 3. Automate your build pipeline
sigma-agent workflow create "build, test, and notify on completion" -o dev.yaml
sigma-agent workflow run dev-workflow

# 4. Watch files + auto-suggest
sigma-agent watch . --ext .rs,.nim --suggest
```

---

## Key Concepts

| Concept | What it is |
|---|---|
| **shard** | Atomic capability module (600+ in SigmaOS) |
| **sigma_pledge** | Declare what your app can do — kernel enforces it |
| **sigma_unveil** | Declare which files your app can access |
| **sigma-bus** | Typed IPC between shards |
| **sigma-pkg** | PQC-signed package manager |
| **sigma-agent** | AI CLI agent (36 modules, 22 subcommands) |
| **SDF** | Sovereign Driver Framework |
| **sigpkg** | SigmaOS native package format |

Learn more: `sigma-agent explain "<concept>"`

---

## Getting Help

```bash
sigma-agent explain "<anything>"     # AI explanation
sigma-agent doctor                   # diagnose issues
sigma-agent "how do I <task>"        # NL help
```

- Wiki: https://github.com/AaryanSinghChauhan09/SigmaOS/wiki
- Issues: https://github.com/AaryanSinghChauhan09/SigmaOS/issues
- Discussions: https://github.com/AaryanSinghChauhan09/SigmaOS/discussions

---

*See also: [sigma-agent](sigma-agent) · [Migration Guide](Migration-Guide) · [SDK Guide](SDK-Guide) · [Architecture Overview](Architecture-Overview)*

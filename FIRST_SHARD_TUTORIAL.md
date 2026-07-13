# First Shard Tutorial

> **Audience**: New contributors | **Time**: ~30 minutes | **Prerequisites**: Rust basics, basic Linux knowledge

Welcome to SigmaOS! This guide will teach you how to write, build, and load your first **Shard** — the fundamental unit of modularity in the Sovereign Lattice.

---

## What is a Shard?

A **Shard** is a self-contained, independently loadable kernel or userland module in SigmaOS. Shards communicate over the **sigma-bus** zero-copy IPC ring-buffer, are signed with Dilithium5 post-quantum signatures, and are managed via the `SHARDS.manifest`.

```
┌─────────────────────────────────────────┐
│          SOVEREIGN LATTICE              │
│                                         │
│  ┌──────────┐   sigma-bus   ┌────────┐ │
│  │  YourShard│ ────────────▶ │CoreOS  │ │
│  │  (new!)   │ ◀──────────── │Shard   │ │
│  └──────────┘               └────────┘ │
│       ↑                         ↑       │
│  SHARDS.manifest registers both shards  │
└─────────────────────────────────────────┘
```

---

## Step 1: Prerequisites

```bash
# Ensure the SigmaOS build environment is ready
just check-env

# Expected output:
# ✅ Rust nightly-2025-04-01 (or later)
# ✅ llvm-17 / clang-17
# ✅ just (justfile runner)
# ✅ sigma-bus kernel headers
```

---

## Step 2: Create the Shard Directory

```bash
# Navigate to the shards directory
cd kernel/shards/

# Create your shard directory
mkdir hello_world
cd hello_world
```

Your shard directory structure:
```
kernel/shards/hello_world/
├── Cargo.toml          # Rust manifest
├── src/
│   └── lib.rs          # Shard implementation
└── shard.toml          # SigmaOS shard metadata
```

---

## Step 3: Write the Shard Manifest

Create `shard.toml`:

```toml
[shard]
name        = "HelloWorld"
version     = "0.1.0"
author      = "Your Name <you@example.com>"
category    = "example"        # core | security | driver | ui | example
priority    = 50               # Boot order (lower = earlier)
bus_channel = "hello.world"    # sigma-bus channel name

[capabilities]
# Declare what capabilities this shard needs
memory_mb   = 4
ipc_send    = ["kernel.log"]
ipc_recv    = ["hello.world"]

[dependencies]
sigma_core  = { version = ">=0.1" }
```

---

## Step 4: Write the Shard Code

Create `src/lib.rs`:

```rust
#![no_std]
#![no_main]

use sigma_core::{
    shard_init, shard_main,
    bus::{SigmaBus, Message},
    log::sigma_log,
    SigmaResult,
};

/// Called once during system boot
#[shard_init]
pub fn init() -> SigmaResult<()> {
    sigma_log!("HelloWorld shard: initializing on sigma-bus channel 'hello.world'");
    Ok(())
}

/// Main event loop — called for each incoming IPC message
#[shard_main]
pub fn run(bus: &mut SigmaBus, msg: Message) -> SigmaResult<()> {
    match msg.tag {
        // Respond to ping with pong
        b"PING" => {
            sigma_log!("HelloWorld: received PING, sending PONG");
            bus.send("hello.world", b"PONG", msg.payload)?;
        }
        // Echo any other message back to sender
        _ => {
            sigma_log!("HelloWorld: echoing message: {:?}", msg.tag);
            bus.reply(&msg, msg.payload)?;
        }
    }
    Ok(())
}
```

---

## Step 5: Create the Cargo.toml

```toml
[package]
name    = "sigma-shard-hello-world"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["staticlib"]

[dependencies]
sigma-core = { path = "../../../libs/sigma-core" }

[profile.release]
opt-level = "z"       # Size optimization for small shards
lto       = true
codegen-units = 1
panic     = "abort"   # No unwinding in kernel context
```

---

## Step 6: Register in SHARDS.manifest

Open the root `SHARDS.manifest` and add your shard:

```toml
# Add to the [optional_shards] section for development
[optional_shards.hello_world]
path     = "kernel/shards/hello_world"
enabled  = true
priority = 50
profile  = ["dev", "full"]   # Only loaded in dev/full profiles
```

---

## Step 7: Build

```bash
# Build just your shard
just build-shard hello_world

# Expected output:
# Compiling sigma-shard-hello-world v0.1.0
# Linking hello_world.shard
# Signing with Dilithium5...
# ✅ hello_world.shard (12.4 KiB)

# Or build the entire kernel (shards included)
just build
```

---

## Step 8: Test Your Shard

```bash
# Run in QEMU (no real hardware needed)
just run-qemu -- --shard-test hello_world

# Or run the shard unit test suite
just test-shard hello_world

# Expected test output:
# test init ... ok
# test ping_pong ... ok
# test echo ... ok
# test result: 3 passed; 0 failed
```

Writing a test in `src/lib.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use sigma_core::testing::MockBus;

    #[test]
    fn test_ping_pong() {
        let mut bus = MockBus::new();
        let msg = Message::new(b"PING", b"");
        run(&mut bus, msg).unwrap();
        assert_eq!(bus.last_sent().tag, b"PONG");
    }

    #[test]
    fn test_echo() {
        let mut bus = MockBus::new();
        let msg = Message::new(b"HI", b"hello");
        run(&mut bus, msg).unwrap();
        assert_eq!(bus.last_reply().payload, b"hello");
    }
}
```

---

## Step 9: Verify and Submit

```bash
# Run the quality gate check (no stubs allowed)
./scripts/sigma_quality_check.sh

# Check your shard is properly registered
sigma-lattice list-shards | grep hello_world
# hello_world  v0.1.0  [example]  LOADED  ✅

# Submit a PR!
git checkout -b feat/hello-world-shard
git add kernel/shards/hello_world/ SHARDS.manifest
git commit -m "feat(shards): add HelloWorld example shard

Demonstrates the basic shard lifecycle: init → event loop → IPC.
Useful as a template for new contributors.

Signed-off-by: Your Name <you@example.com>"
git push origin feat/hello-world-shard
```

---

## Next Steps

Now that you've built your first shard, explore:

| Resource | Description |
|---|---|
| [CORE_SHARDS.md](CORE_SHARDS.md) | How the essential kernel shards work |
| [ESSENTIAL_SHARDS.md](ESSENTIAL_SHARDS.md) | Critical system shards to study |
| [sigma-bus spec](IPC.md) | Deep dive into zero-copy IPC |
| [KERNEL_DEVELOPER_HANDBOOK.md](KERNEL_DEVELOPER_HANDBOOK.md) | Full kernel dev guide |
| [CONTRIBUTING.md](CONTRIBUTING.md) | Contribution workflow |
| [Good First Issues](GOOD_FIRST_ISSUES.md) | Real issues to tackle next |

**Happy hacking! The Sovereign Lattice grows with every shard.** 🛡️

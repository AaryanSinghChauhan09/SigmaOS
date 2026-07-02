# Shard Development Guide

## What Is a Shard?

A **shard** is SigmaOS's atomic capability unit. Every running piece of software — a device driver, a system daemon, a GUI app — is a shard. Shards are isolated processes with a formal capability contract enforced by the kernel.

Key properties:
- Isolated address space with ASLR
- A capability token issued at registration (256-bit, unforgeable)
- A `sigma_pledge` syscall allowlist (shrinkable only)
- A `sigma_unveil` filesystem path allowlist (additive, irreversible)
- All I/O via **sigma-bus** typed message passing

---

## Shard Lifecycle

```
sigma-init
  │  spawn + register
  ▼
REGISTER  →  kernel issues CapToken, assigns ShardId
  │
  ▼
INIT      →  shard calls sigma_pledge + sigma_unveil
             loads config, allocates resources
  │
  ▼
TICK      →  event loop: sigma-bus recv → process → reply
  │         (repeating)
  ▼
SHUTDOWN  →  shard drains message queue, releases resources
             calls sigma_exit(0)
  │
  ▼
DEAD      →  kernel reclaims address space, revokes CapToken
```

---

## Full Rust Example: hello-shard

### Directory Structure

```
shards/hello-shard/
├── Cargo.toml
└── src/
    └── main.rs
```

### Cargo.toml

```toml
[package]
name    = "hello-shard"
version = "0.1.0"
edition = "2021"

[dependencies]
sigma-shard-sdk = { path = "../../sdk/sigma-shard-sdk" }
sigma-bus-types = { path = "../../kernel/sigma_bus/types" }
log             = "0.4"
env_logger      = "0.11"
```

### src/main.rs

```rust
//! hello-shard: minimal SigmaOS shard example.
//! Responds to Ping messages with a Pong.

use sigma_shard_sdk::{Shard, ShardContext, BusMessage, MessageResult};
use sigma_bus_types::{Msg, ShardCapability};

struct HelloShard {
    ping_count: u64,
}

impl Shard for HelloShard {
    fn name(&self) -> &str { "hello-shard" }

    fn capabilities(&self) -> &[ShardCapability] {
        &[ShardCapability::RespondTo(Msg::Ping)]
    }

    fn init(&mut self, ctx: &mut ShardContext) -> Result<(), Box<dyn std::error::Error>> {
        // Restrict syscalls to the minimum needed
        ctx.pledge("stdio")?;
        // No filesystem access needed
        log::info!("hello-shard: initialized");
        Ok(())
    }

    fn on_message(
        &mut self,
        ctx: &mut ShardContext,
        msg: BusMessage,
    ) -> MessageResult {
        match msg.payload {
            Msg::Ping(seq) => {
                self.ping_count += 1;
                log::debug!("hello-shard: ping #{} (seq={})", self.ping_count, seq);
                MessageResult::Reply(Msg::Pong(seq))
            }
            Msg::Shutdown => {
                log::info!("hello-shard: shutdown requested");
                MessageResult::Shutdown
            }
            other => {
                log::warn!("hello-shard: unexpected message {:?}", other);
                MessageResult::Ignore
            }
        }
    }

    fn shutdown(&mut self) {
        log::info!("hello-shard: shutting down after {} pings", self.ping_count);
    }
}

fn main() {
    env_logger::init();
    let shard = HelloShard { ping_count: 0 };
    sigma_shard_sdk::run(shard);
}
```

---

## sigma-bus IPC: Sending and Receiving Typed Messages

### Sending a Message

```rust
use sigma_shard_sdk::BusClient;
use sigma_bus_types::{ShardId, Msg};

let mut bus = BusClient::connect()?;

// Send a Ping and wait for Pong (blocking, with 100ms timeout)
let response = bus.call(
    ShardId::named("hello-shard"),
    Msg::Ping(42),
    std::time::Duration::from_millis(100),
)?;

match response {
    Msg::Pong(seq) => println!("Got pong with seq={}", seq),
    other => eprintln!("Unexpected: {:?}", other),
}
```

### Receiving (in a tick loop)

```rust
// Inside a shard's event loop (managed by sigma-shard-sdk):
while let Some(msg) = ctx.recv_nonblocking() {
    self.on_message(ctx, msg);
}
// Then yield or wait on epoll
ctx.wait_for_message(std::time::Duration::from_millis(10));
```

---

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use sigma_shard_sdk::testing::MockShardContext;

    #[test]
    fn test_ping_pong() {
        let mut shard  = HelloShard { ping_count: 0 };
        let mut ctx    = MockShardContext::new();
        let msg = BusMessage::new(Msg::Ping(99));
        let result = shard.on_message(&mut ctx, msg);
        assert_eq!(result, MessageResult::Reply(Msg::Pong(99)));
        assert_eq!(shard.ping_count, 1);
    }
}
```

### Integration Test with sigma-ktest

```bash
sigma-ktest run shards/hello-shard/tests/integration_test.toml
```

---

## Publishing

```bash
# Build the PKGBUILD
sigma-pkg build shards/hello-shard/SIGPKG

# Sign with your key
sigma-pkg sign dist/hello-shard-0.1.0.spkg

# Publish to the registry
sigma-pkg publish dist/hello-shard-0.1.0.spkg \
  --registry https://registry.sigmaos.dev
```

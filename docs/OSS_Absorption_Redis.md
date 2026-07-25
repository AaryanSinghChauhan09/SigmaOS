# OSS Absorption: Redis — In-Memory Data Structure Store

> **Status**: 📋 Planned | **Source Project**: Redis | **Target Shard**: `SigmaOS Sovereign Cache & IPC Bus`

---

## 1. Executive Summary

Redis is an in-memory data structure store used as a database, cache, message broker, and streaming engine. Its power comes from offering rich data types (strings, hashes, lists, sets, sorted sets) that operate entirely in RAM with optional persistence.

SigmaOS absorbs Redis's **in-memory data structures**, **Pub/Sub messaging**, and **Append-Only File (AOF) persistence** into `sigma-cache`, providing a blazing-fast memory store for all SigmaOS services.

---

## 2. Key Features to Absorb

### 2.1 Native In-Memory Data Structures

SigmaOS services can use `sigma-cache` as a shared, lock-free memory store without running a separate database daemon.

```bash
$ sigma cache set user:1000:session "active" --ttl 3600
$ sigma cache hset user:1000:profile name "Alice" theme "dark"
$ sigma cache get user:1000:session
"active"
```

### 2.2 Pub/Sub IPC Bus

`sigma-cache` provides a lightweight Publish/Subscribe messaging bus that services can use for real-time event broadcasting, bypassing heavy message queues.

```rust
// sigma-cache pub/sub example
let mut sub = sigma_cache::subscribe("events::system::shutdown").await?;
while let Some(msg) = sub.next().await {
    println!("Received shutdown signal, saving state...");
    break;
}
```

### 2.3 AOF-Inspired Persistence

For durability, `sigma-cache` optionally logs every write operation to a sequential Append-Only File (AOF). On reboot, the memory state is reconstructed from this log. The log is periodically compacted in the background.

---

## 3. References & Standards

- Redis — `redis.io`

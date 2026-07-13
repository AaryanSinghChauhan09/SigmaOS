# OSS Absorption: Redis — In-Memory Key-Value Store & IPC

> **Status**: 📋 Planned | **Source Project**: Redis | **Target Shard**: `SigmaOS In-Memory State & Cache Shard`

---

## 1. Executive Summary

Redis is an open-source, in-memory data structure store used as a database, cache, message broker, and queue. It offers sub-millisecond latency by operating entirely in RAM with optional persistence.

SigmaOS absorbs the **in-memory data structure architecture** and **Pub/Sub messaging model** of Redis, incorporating them directly into the system's low-latency Inter-Process Communication (IPC) bus (`sigma-bus`) and caching subsystem.

---

## 2. Key Features Absorbed

### 2.1 Low-Latency In-Memory IPC Cache

Instead of serialized Unix sockets, `sigma-bus` utilizes an in-memory key-value state store structured similarly to Redis. Microservices and kernel subsystems share high-performance state via standard structures (Strings, Lists, Sets, Hashes) without serialization overhead.

```rust
// kernel/ipc/bus_store.rs
// SPDX-License-Identifier: MIT

pub struct SharedStore {
    db: HashMap<String, DataStructure>,
}

impl SharedStore {
    pub fn push_list(&mut self, key: &str, value: Vec<u8>) -> Result<()> {
        let entry = self.db.entry(key.to_string()).or_insert(DataStructure::List(Vec::new()));
        if let DataStructure::List(ref mut list) = entry {
            list.push(value);
            Ok(())
        } else {
            Err(Error::TypeMismatch)
        }
    }
}
```

### 2.2 Native Pub/Sub System

Subsystems subscribe to channels natively. When an event fires, the publisher broadcasts to the channel, waking up listening tasks with zero latency.

```bash
$ sigma bus subscribe system:events
Σ [BUS] Subscribed to system:events. Waiting for events...
  [system:events] (12:35:01) hw_event: thermal_alert (Core 2: 85C)
```

---

## 3. References & Standards

- Redis — `redis.io` (BSD-3-Clause)
- sigma-bus specification docs

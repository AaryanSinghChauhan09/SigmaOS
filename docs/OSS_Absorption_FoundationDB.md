# OSS Absorption: FoundationDB — Distributed ACID Database

> **Status**: 📋 Planned | **Source Project**: Apple FoundationDB | **Target Shard**: `SigmaOS Sovereign State Store`

---

## 1. Executive Summary

FoundationDB is an open-source distributed database developed by Apple that provides ACID transactions across multiple key-value pairs in a distributed setting. Its key innovation is a layered architecture: a core KV store with strict ACID guarantees, on top of which any data model (SQL, document, graph) can be built.

SigmaOS absorbs FoundationDB's **multi-key ACID transaction model** and **layered API design** into `sigma-statedb`, the central state store for cluster-wide configuration, service registrations, and distributed locks.

---

## 2. Key Features to Absorb

### 2.1 Multi-Key ACID Transactions

Unlike Redis or etcd's single-key atomicity, `sigma-statedb` supports true multi-key atomic transactions spanning arbitrary key ranges.

```rust
// sigma-statedb transaction API
let tx = statedb.begin_transaction().await?;
tx.set("/services/my-api", "10.0.1.5:8080");
tx.set("/health/my-api", "healthy");
tx.clear("/services/old-api");
tx.commit().await?; // All-or-nothing
```

### 2.2 Watches with Transactional Consistency

Unlike etcd's eventual watch notifications, `sigma-statedb` guarantees that a watch event is only delivered after the transaction that caused the change is fully committed and replicated.

```bash
$ sigma statedb watch /services/
Σ [STATEDB] Watching /services/ (transactionally consistent)
  [12:01:00.001] COMMIT tx#7a3b: /services/my-api = 10.0.1.5:8080
```

---

## 3. References & Standards

- FoundationDB — `foundationdb.org` (Apache-2.0)

# OSS Absorption: etcd — Distributed Consensus KV Store

> **Status**: 📋 Planned | **Source Project**: etcd (CNCF) | **Target Shard**: `SigmaOS Sovereign Consensus`

---

## 1. Executive Summary

etcd is a strongly consistent, distributed key-value store used as the backbone of Kubernetes for storing all cluster state. It implements the Raft consensus algorithm to ensure linearizable reads and writes across a cluster of nodes.

SigmaOS absorbs etcd's **Raft consensus engine**, **watch/notify API**, and **lease-based ephemeral keys** into `sigma-consensus`, providing a native distributed state store for multi-node SigmaOS deployments.

---

## 2. Key Features Absorbed

### 2.1 Raft Consensus Engine

`sigma-consensus` embeds a Raft implementation ensuring that cluster-critical state (service registrations, configuration values, leader elections) is replicated across an odd number of nodes with automatic leader failover.

```bash
$ sigma consensus status
Σ [CONSENSUS] Raft cluster status:
  Node ID   Role      Term  Log Index  State
  node-1    LEADER    42    18934      Healthy
  node-2    FOLLOWER  42    18934      Healthy
  node-3    FOLLOWER  42    18933      Syncing (1 behind)
```

### 2.2 Watch API for Reactive Configuration

Applications subscribe to key prefixes and receive push notifications when values change, eliminating the need for polling.

```rust
// sigma-consensus watch API
let mut watcher = consensus.watch("/config/gateway/").await?;
while let Some(event) = watcher.next().await {
    match event.kind {
        EventKind::Put => println!("Key {} set to {}", event.key, event.value),
        EventKind::Delete => println!("Key {} deleted", event.key),
    }
}
```

### 2.3 Lease-Based Ephemeral Keys

Services register themselves with a time-limited lease. If the service crashes without renewing, the key is automatically deleted, providing built-in failure detection.

```bash
$ sigma consensus lease grant 30s
Σ [CONSENSUS] Lease granted: id=7f3a2b1c ttl=30s

$ sigma consensus put /services/my-api --lease=7f3a2b1c "10.0.1.5:8080"
# Key auto-deletes if not renewed within 30s
```

---

## 3. References & Standards

- etcd — `etcd.io` (Apache-2.0)
- Raft consensus — `raft.github.io`

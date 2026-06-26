# SigmaOS Phase F: Offline-First Architecture

## Overview

One of SigmaOS's decisive competitive advantages over browser-based operating systems (ChromeOS, CloudReady, WebOS) is its **offline-first** data model. Browser OS platforms are fundamentally cloud-dependent — when the network drops, productivity stops.

SigmaOS's Offline-First CRDT Sync Engine (`net/sigma_offline_sync.cpp`) ensures users **never lose work**, even without a network connection.

---

## Technical Architecture

### Conflict-Free Replicated Data Types (CRDTs)

SigmaOS uses a **Last-Write-Wins Register (LWW-Register)** with **Vector Clocks** for conflict resolution:

```
Key/Value Store
┌──────────────────────────────────────────┐
│  key: "docs/report.md"                   │
│  value: <encrypted blob>                 │
│  vector_clock: [5, 3, 0, 2]             │  ← one counter per peer
│  wall_timestamp: 1719000000000           │
│  dirty: true (needs sync)               │
└──────────────────────────────────────────┘
```

**Conflict resolution rule:**
- If remote VC **dominates** local VC (all counters ≥) → remote wins
- Otherwise → local wins (last writer on this device)

### Network Topology

```
   SigmaOS Node A                  SigmaOS Node B
   ┌──────────────┐                ┌──────────────┐
   │ CRDT Store   │◄── Sync ──────►│ CRDT Store   │
   │ (encrypted)  │   (reconnect)  │ (encrypted)  │
   └──────────────┘                └──────────────┘
          │                                │
          ▼                                ▼
   /sigma/var/store/            /sigma/var/store/
   (NVMe, AES-256-GCM)          (NVMe, AES-256-GCM)
```

### Exponential Backoff Retry

When sync fails:

| Attempt | Delay |
|---------|-------|
| 1st     | 500 ms |
| 2nd     | 1 s   |
| 3rd     | 2 s   |
| 4th     | 4 s   |
| max     | 30 s  |

---

## Key API

```c
// Initialise with unique peer ID (assigned at first boot)
sigma_offline_sync_init(peer_id);

// Write data — immediately durable locally
sigma_offline_sync_put("docs/report.md", data, len);

// Read data — always available, even offline
sigma_offline_sync_get("docs/report.md", buf, buf_len, &out_len);

// Called by network subsystem when connection changes
sigma_offline_sync_set_online(true);  // triggers flush

// Merge remote updates (called by sync daemon on reconnect)
sigma_offline_sync_merge(key, remote_val, remote_len, remote_vc, ...);
```

---

## Competitive Advantage vs Browser OS

| Feature | SigmaOS | ChromeOS | CloudReady |
|---------|---------|----------|------------|
| Works offline | ✅ Full CRDT store | ⚠️ Limited (Drive only) | ❌ Cloud-dependent |
| Conflict resolution | ✅ Vector clock merge | ❌ Last-sync-wins | ❌ Last-sync-wins |
| Local encryption | ✅ AES-256-GCM | ⚠️ Partial | ❌ None |
| Offline apps | ✅ Native ELF + WASM | ⚠️ Limited PWAs | ❌ None |
| Bandwidth-aware sync | ✅ Metered connection aware | ❌ Always syncs | ❌ N/A |

---

## Related Modules

- `net/sigma_offline_sync.cpp` — CRDT engine
- `crypto/SovereignKyber.cpp` — PQC key wrapping for encryption
- `kernel/core/orchestrator/sigma_orchestrator.cpp` — container-aware sync isolation

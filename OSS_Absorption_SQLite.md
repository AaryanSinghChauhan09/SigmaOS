# OSS Absorption: SQLite — Universal Metadata Storage

> **Status**: 🔄 Active | **Source Project**: SQLite (Richard Hipp) | **Target Shard**: `SigmaOS Metadata & Configuration Store`

---

## 1. Executive Summary

SQLite is the most widely deployed database engine in the world. It is a C-language library that implements a small, fast, self-contained, high-reliability, full-featured, SQL database engine.

Rather than relying on scattered text files, INI formats, or complex registries, SigmaOS absorbs SQLite as the **universal data format** for all structured OS metadata, telemetry, and non-declarative state.

---

## 2. Key Features Absorbed

### 2.1 The OS Configuration Database

While core system specs are declarative (`sigma.toml`), operational state and user metadata are stored in atomic SQLite databases. This guarantees ACID transactions — the OS state can never be corrupted by a power loss.

```
/var/lib/sigma/
├── telemetry.db     # System performance metrics
├── pkg_cache.db     # Package manager local index
└── portal_perms.db  # Flatpak/sandbox permission grants
```

```rust
// kernel/db/sysdb.rs
// SPDX-License-Identifier: MIT

pub fn query_portal_permissions(app_id: &str) -> Result<Vec<Permission>> {
    let conn = sqlite::open("/var/lib/sigma/portal_perms.db")?;
    
    let mut stmt = conn.prepare("SELECT perm, state FROM portals WHERE app_id = ?")?;
    stmt.bind((1, app_id))?;
    
    // ...
}
```

### 2.2 Built-in OS SQL CLI

SigmaOS exposes a unified SQL interface to query running system state (inspired by osquery), mapped dynamically via SQLite virtual tables.

```bash
$ sigma sql "SELECT pid, name, memory_mb FROM processes ORDER BY memory_mb DESC LIMIT 5"
| pid  | name     | memory_mb |
|------|----------|-----------|
| 1450 | firefox  | 1450.2    |
| 890  | zenith   | 412.5     |
| 112  | sigma-db | 89.1      |
```

---

## 3. References & Standards

- SQLite — `sqlite.org` (Public Domain)
- osquery — `osquery.io`

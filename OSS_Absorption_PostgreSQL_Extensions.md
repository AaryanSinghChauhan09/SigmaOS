# OSS Absorption: PostgreSQL Extensions

> **Status**: 📋 Planned | **Source Projects**: pgvector, TimescaleDB, PostGIS | **Target Shard**: `SigmaOS Database Extensibility`

---

## 1. Executive Summary

PostgreSQL's popularity is greatly amplified by its extension ecosystem, which allows it to handle specialized workloads like timeseries data (TimescaleDB), geospatial data (PostGIS), and AI vector search (pgvector) inside the same database engine.

SigmaOS absorbs the **extension loading architecture** of PostgreSQL, enabling `sigma-db` to dynamically link specialized plugins to support native system-level vector search, indexing, and metrics logging.

---

## 2. Key Features Absorbed

### 2.1 Native System Vector Search (`sigma-vector`)

To support the AI Agent overlay, `sigma-db` is compiled with native vector indexing support (utilizing HNSW graphs). This allows the local LLM agent to perform rapid semantic vector lookups over the system logs and user data.

```sql
-- Querying semantic event patterns in system db
SELECT log_id, content 
FROM system_logs 
ORDER BY embedding <=> '[0.12, 0.45, -0.09, ...]' 
LIMIT 5;
```

---

## 3. References & Standards

- pgvector — `github.com/pgvector/pgvector` (PostgreSQL License)
- TimescaleDB — `timescale.com`

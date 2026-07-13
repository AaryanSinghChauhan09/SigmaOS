# OSS Absorption: PostgreSQL — Relational Database

> **Status**: 📋 Planned | **Source Project**: PostgreSQL | **Target Shard**: `SigmaOS Persistent Data Store`

---

## 1. Executive Summary

PostgreSQL is a powerful, open-source object-relational database system known for reliability, feature robustness, and performance.

SigmaOS absorbs PostgreSQL's **Write-Ahead Logging (WAL) engine** and **MVCC (Multi-Version Concurrency Control)** mechanics, incorporating them into system database layers (`sigma-db`) to manage concurrent, crash-safe modifications of complex state.

---

## 2. Key Features Absorbed

### 2.1 Write-Ahead Logging (WAL) for Config

Every system configuration change in SigmaOS is committed to an append-only WAL before being applied. This ensures that even if power is abruptly lost mid-write, the system database can safely replay the logs to arrive at a consistent state.

### 2.2 Multi-Version Concurrency Control (MVCC)

To allow telemetry probes and management agents to read system state without blocking live operations, `sigma-db` utilizes MVCC. Readers do not block writers, and writers do not block readers, ensuring zero performance drops during heavy auditing tasks.

---

## 3. References & Standards

- PostgreSQL — `postgresql.org` (PostgreSQL License)

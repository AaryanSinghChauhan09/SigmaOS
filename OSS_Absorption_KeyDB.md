# OSS Absorption: KeyDB — Multithreaded In-Memory Database

> **Status**: 📋 Planned | **Source Project**: KeyDB | **Target Shard**: `SigmaOS High-Throughput Memory Cache`

---

## 1. Executive Summary

KeyDB is a high-performance, multithreaded fork of Redis. It scales out by executing commands on multiple threads, eliminating the single-threaded bottleneck of standard Redis databases.

SigmaOS absorbs KeyDB's **multithreaded architecture** and **ACTIVE-ACTIVE replication**, applying them to the core `sigma-bus` IPC layer to allow massive concurrent message processing across multi-core systems.

---

## 2. Key Features Absorbed

### 2.1 Multithreaded Core IPC

Unlike standard Redis-inspired single-threaded queues, `sigma-bus` utilizes KeyDB-style concurrent read/write locks, permitting different threads to process independent messaging channels in parallel without locking the entire store.

---

## 3. References & Standards

- KeyDB — `keydb.dev` (BSD-3-Clause)

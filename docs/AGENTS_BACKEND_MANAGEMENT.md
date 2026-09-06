# SigmaOS Backend Management Guide for AI Agents

This guide provides technical specifications, operational procedures, and architectural principles for AI agents managing backend services, server engines, database persistence, message queuing, and distributed orchestration in SigmaOS.

---

## 1. Zero-Dependency `#![no_std]` Backend Architecture

SigmaOS implements high-performance backend infrastructure directly in Rust without external crate dependencies (`#![no_std]` compliant core):

* **Sovereign Web Server (`SovereignWebServer` in `src/open_source_obsoletion.rs`):** Zero-copy HTTP/1.1 and HTTP/2 reverse proxy routing, TLS termination, and static asset serving (obsoletes Nginx / Caddy).
* **Sovereign Embedded Database (`SovereignEmbeddedDb` in `src/open_source_obsoletion.rs`):** ACID transactional B-Tree storage engine and SQL query translator (obsoletes SQLite).
* **Sovereign Cache Engine (`SovereignCacheEngine` in `src/open_source_obsoletion.rs`):** In-memory key-value cache with TTL expiration, LRU eviction, and key purging parity (obsoletes Redis / Memcached).
* **Sovereign Message Broker (`SovereignMessageBroker` in `src/open_source_obsoletion.rs`):** Pub-sub queueing, topic partitioning, and streaming log storage (obsoletes RabbitMQ / Kafka / NATS).
* **Sovereign Distributed Storage (`SovereignDistributedStorage` in `src/open_source_obsoletion.rs`):** Content-addressed blob object store with erasure coding and S3-compatible API (obsoletes Ceph / MinIO).
* **Sovereign Secret Vault (`SovereignSecretVault` in `src/open_source_obsoletion.rs`):** PQC-encrypted secret storage, automatic key rotation, and dynamic credential leasing (obsoletes HashiCorp Vault).
* **Sovereign Orchestrator Engine (`SovereignK8sOrchestratorEngine` in `src/open_source_obsoletion.rs`):** Container lifecycle reconciliation, rolling deployments, and service mesh discovery (obsoletes Kubernetes / K3s / Nomad).

---

## 2. Backend Supervision & Lifecycle Management

### 2.1 Service Supervisor Integration (`src/init/systemd_init.rs` & `src/unimplemented_features.rs`)
Backend services are supervised by native init engines:
1. **Systemd-Compatible Supervisor (`SystemdInit`):** Resolves dependency DAGs (`Wants`, `Requires`, `Before`, `After`) and parallel execution stages (`BsdRcParallelStageSolver`).
2. **Systemd-Free Runit Supervisor (`SovereignRunitSupervisor`):** 3-stage service lifecycle supervision (`stage1` boot initialization, `stage2` process monitoring & auto-restart, `stage3` graceful shutdown).

### 2.2 Process Resource Control & Isolation
* **Resource Throttling (`AutomatedRacctPolicy` in `src/automation/system_level.rs`):** FreeBSD RACCT/RCTL inspired CPU, memory RSS, swap, and I/O rate limiting.
* **Process Sandboxing (`AutomatedSandboxPolicy` in `src/automation/system_level.rs`):** OpenBSD `pledge` (restricting system calls) and `unveil` (restricting filesystem paths) auto-enforcement.

---

## 3. Storage & Cache Eviction Rules for AI Agents

When working with backend cache engines or key-value stores in `src/open_source_obsoletion.rs`:

1. **Key Uniqueness & Invalidation:**
   `SovereignCacheEngine::set` MUST purge pre-existing key entries via `self.entries.retain(|e| e.key != key)` before inserting updated entries to prevent duplicate key pollution.
2. **CAS Blob Storage:**
   All stored data objects MUST generate a deterministic SHA-256 / Merkle hash ID (`CasBlobDescriptor`) for content-addressed lookup.

---

## 4. Backend Observability & Metrics Collection

* **Metrics Engine (`SovereignOpenTelemetryMetricsCollector` in `src/open_source_os_gap_closure.rs`):** Provides native counter, gauge, and histogram metric collection with OpenTelemetry wire protocol export (obsoletes Prometheus / Datadog agents).
* **Metric Invariants (`src/monitoring/metrics.rs`):** `SimpleMetric` stores `name_len: u8` initialized during `new()`, enabling $O(1)$ slice lookup to replace $O(N)$ null-byte scans.

---

## 5. Agent Verification Workflow for Backend Changes

After modifying or adding backend services, AI agents MUST perform:

1. **Standalone Test Compilation:**
   ```bash
   rustc --test --edition=2021 --cfg 'feature="standalone_test"' src/open_source_os_gap_closure.rs
   ```
2. **Full Pipeline Verification:**
   ```bash
   ./run_sigma_tests.sh
   ```
3. **Audit Results:** Confirm all backend unit, integration, and performance inspection tests pass without warning regressions.

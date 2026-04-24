# 📟 SigmaOS: Sovereign CLI Usage Guide (v2.6)

The **S-CLI** is the high-performance, native orchestrator for the Sovereign Lattice. It provides unified control over build, verification, and shard management.

## 🏗️ Core Management

### Build the Lattice
```bash
./s-cli build [arch]
```
Compiles the core lattice and required shards for the specified architecture.

### Industrial Verification
```bash
./s-cli verify
```
Performs a cryptographic audit of the shard lattice to ensure state consistency.

## 🧩 Shard Orchestration

### Manage Individual Shards
```bash
./s-cli shard [list|start|stop|info] [shard_id]
```
Allows for fine-grained control over the lifecycle of individual silicon shards.

### Security Auditing
```bash
./s-cli audit
```
Triggers a deep system-wide security scan, utilizing **S23 (Hardening)** and **S36 (BPF Probes)** to detect vulnerabilities.

## 📊 Observability & Store

### System Telemetry
```bash
./s-cli telemetry
```
Displays real-time performance metrics, including CPU/Memory usage and IPC throughput across the lattice.

### Lattice Store (S-MARKET)
```bash
./s-cli market
```
Connects to the decentralized shard registry to browse and download community-contributed shards.

---
*Unified Control for a Sovereign Future.*

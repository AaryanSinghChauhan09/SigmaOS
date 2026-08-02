# Σ SigmaOS Sovereign Wiki

Welcome to the official offline documentation archive for **SigmaOS** — a sovereign, AI-native, post-quantum secure microkernel operating system.

---

## 🗺️ Architectural Philosophy

Unlike legacy Unix-derived or Windows systems, SigmaOS avoids monolithic system bottlenecks and overlapping, bloated software layers:
- **Zero-Trust Hardened Boundaries**: All processes run as isolated user-space shards, mapped and validated via hardware capability tokens.
- **Transactional System State**: Replaces chaotic `/etc` layout configurations with pure declarative configuration states.
- **Zero-Allocation Core**: Uses custom, memory-safe, allocation-free Rust data structures throughout kernel hot-paths.
- **Post-Quantum Trust Hierarchies**: All driver signatures, package files, and inter-shard communication channels require cryptographic authorization validated via `Kyber-1024` KEM and `Dilithium-5` signatures.

---

## ⚙️ Core System Daemons

SigmaOS relies on dedicated, lightweight background daemons inspired by Linux distribution best practices:

1. **Sigma-Claw (Web-Crawling Service)**:
   - Dynamic mirror ranking via automated network latency measurements.
   - Paced downloading via simulated bandwidth rate-limiting (in KB/s).
   - Exponential backoff retry loops on connection timeouts.

2. **Sigma-Update (Transactional Update Daemon)**:
   - A/B partition atomic update state machine.
   - Strict lock file concurrency checking to prevent parallel updates.
   - Post-staging health checks with automatic rollback to active safe partitions.

3. **Sigma-Voice (Sovereign Screen Reader Daemon)**:
   - Speech rate and volume configuration metrics.
   - Dynamic prioritizations via High, Normal, and Low sound queues.
   - Advanced technical abbreviation and pronunciation dictionaries.

4. **Sigma-IME (Universal Input Method Editor)**:
   - Keyboard hotkey interception and toggle mappings (e.g. Ctrl+Space).
   - Candidate suggestion list matching and filtering for CJK input.
   - Custom user dictionaries for local phrases.

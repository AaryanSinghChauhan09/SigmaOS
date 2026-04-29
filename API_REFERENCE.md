# 📘 SigmaOS Sovereign API Reference (v4.0 Singularity)

This document details the primary C11 exported functions across the Sovereign Kernel 33-Suite Lattice.

## 🏗️ S01: Genesis (Foundation)

### Master Types (`SovereignCommon.h`)
- `sigma_sz_t`: Unified 64-bit size type.
- `sigma_err_t`: Standard error return type (`SIGMA_OK`, etc.).

### Registry API (`SovereignRegistry.h`)
- `SovereignRegistry_Init()`: Bootstraps the master silicon registry.
- `SovereignRegistry_Register(name, category, init_fn)`: Binds a Sovereign Shard to the lattice.
- `SovereignRegistry_Audit()`: Verifies the integrity of all active shards via hashing.

## 🧠 S10: Orchestration (AI, ML, DS, UDF)

### Neural Inference (`SovereignNeuralShard.c`)
- `sigma_neural_load(name, in_dim, hidden, out_dim)`: Loads a neural model into the kernel.
- `sigma_neural_infer(input, output, in_dim, hidden, out_dim)`: Performs a 2-layer forward pass.

### Data Science (`SovereignDataframeMatrix.c`)
- `sigma_df_create(name)`: Initializes a new columnar dataframe.
- `sigma_df_add_column(df, name, values, count)`: Binds data to a dataframe.
- `sigma_df_mean(df, col_idx)`: Returns the arithmetic mean of a column.
- `sigma_df_describe(df)`: Prints a statistical summary (Pandas-style).

### UDF Engine (`SovereignUDFEngine.c`)
- `sigma_udf_register(name, func, perms, budget)`: Registers a sandboxed user function.
- `sigma_udf_execute(name, data)`: Dispatches a UDF within the sandbox enclosure.

## 💾 S06: Storage (Database & ACID)

### ACID Engine (`SovereignACIDEngine.c`)
- `sigma_txn_begin()`: Starts a new WAL-backed transaction.
- `sigma_txn_commit(txn_id)`: Finalizes and flushes transaction changes.
- `sigma_txn_rollback(txn_id)`: Discards changes and aborts transaction.

## ⚡ S05: Memory (Concurrency)

### Sync Primitives (`SovereignConcurrencyEngine.c`)
- `sigma_spinlock_acquire(lock)`: Busy-wait for a spinlock.
- `sigma_semaphore_wait(sem)`: Atomic wait on a counting semaphore.
- `sigma_ring_push(rb, value)`: Lock-free push into a ring buffer.

## 🚀 S07: Network (Consensus)

### Raft Consensus (`SovereignConsensusShard.c`)
- `sigma_raft_elect()`: Initiates a leader election cycle.
- `sigma_raft_heartbeat(tick)`: Leader suppression of follower elections.

## 🌿 S04: HAL (Sustainability)

### Green Power (`SovereignGreenShard.c`)
- `sigma_green_transition(state)`: Changes DVFS p-state (Turbo/Eco/Idle).
- `sigma_green_thermal_check(temp)`: Automatic thermal throttling trigger.

## 🌐 Web Dashboard Shard Loader (`kernel_loader.js`)

### Modular Loading API
- `loadSystem()`: Asynchronously initiates the hierarchical loading of all 600 shards (Core -> Essential -> Optional -> Third-Party -> Infinite).
- `loadScript(src)`: Low-level promise-based script injector for dynamic shard activation.
- `SYSTEM_MODULES`: Constant array containing the prioritized path list of all active shards.

---
*For full implementation details, refer to the corresponding `.c` files in `kernel/suites/`.*

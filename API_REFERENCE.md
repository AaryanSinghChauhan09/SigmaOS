# SigmaOS API Reference

Complete API mapping for all Sovereign Singleton shards.

## Core Security

<<<<<<< HEAD
### Master Types (`SovereignCommon.h`)


- `sigma_sz_t`: Unified 64-bit size type.
- `sigma_err_t`: Standard error return type (`SIGMA_OK`, etc.).

### Registry API (`SovereignRegistry.h`)


- `SovereignRegistry_Init()`: Bootstraps the master silicon registry.
- `SovereignRegistry_Register(name, category, init_fn)`: Binds a Sovereign Shard to the lattice.

- `SovereignRegistry_Audit()`: Verifies the integrity of all active shards via hashing.
=======
### `SovereignCapabilityEngine`

Per-process syscall gating with bitmask precision.
>>>>>>> 7759f274e222d74141c499a7b379a060016fe9a1

- `void capability_grant(sigma_u32 pid, sigma_u64 caps);`
- `bool capability_check(sigma_u32 pid, sigma_u64 required);`

<<<<<<< HEAD
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
=======
### `SovereignEnclaveEngine`

Hardware-level PQC key isolation.

- `void enclave_provision(sigma_u32 id);`
- `void enclave_store_key(sigma_u32 id, const char* key);`
>>>>>>> 7759f274e222d74141c499a7b379a060016fe9a1

### `SovereignMultiUserEngine`

<<<<<<< HEAD
### ACID Engine (`SovereignACIDEngine.c`)


- `sigma_txn_begin()`: Starts a new WAL-backed transaction.
- `sigma_txn_commit(txn_id)`: Finalizes and flushes transaction changes.

- `sigma_txn_rollback(txn_id)`: Discards changes and aborts transaction.
=======
UID/GID process identity with PQC attestation.
>>>>>>> 7759f274e222d74141c499a7b379a060016fe9a1

- `sigma_u32 multiuser_register(sigma_u32 uid, sigma_u32 gid, const char* user, sigma_u64 caps);`
- `bool multiuser_authenticate(sigma_u32 uid, const char* username);`

<<<<<<< HEAD
### Sync Primitives (`SovereignConcurrencyEngine.c`)


- `sigma_spinlock_acquire(lock)`: Busy-wait for a spinlock.
- `sigma_semaphore_wait(sem)`: Atomic wait on a counting semaphore.

- `sigma_ring_push(rb, value)`: Lock-free push into a ring buffer.
=======
## Package & Automation
>>>>>>> 7759f274e222d74141c499a7b379a060016fe9a1

### `SovereignPackageEngine`

<<<<<<< HEAD
### Raft Consensus (`SovereignConsensusShard.c`)


- `sigma_raft_elect()`: Initiates a leader election cycle.
- `sigma_raft_heartbeat(tick)`: Leader suppression of follower elections.
=======
S-PKG: Sovereign App Bundle installer.
>>>>>>> 7759f274e222d74141c499a7b379a060016fe9a1

- `bool spkg_install(const char* name, sigma_u32 version);

<<<<<<< HEAD
### Green Power (`SovereignGreenShard.c`)


- `sigma_green_transition(state)`: Changes DVFS p-state (Turbo/Eco/Idle).
- `sigma_green_thermal_check(temp)`: Automatic thermal throttling trigger.
=======
 `bool spkg_remove(const char* name);`
>>>>>>> 7759f274e222d74141c499a7b379a060016fe9a1

### `SovereignTaskScheduler`

Ring-0 cron replacement with macro recording.

- `void scheduler_add_task(const char* name, sigma_u32 interval_ms);`
- `void scheduler_tick(sigma_u32 elapsed_ms);`

### `SovereignAutomatorEngine`

AI-driven predictive workflow macro engine.

- `void automator_register_macro(const char* trigger, const char* action);`
- `void automator_context_tick(const char* context);`

## Compute & Hardware

### `SovereignGPUEngine`

Vendor-agnostic GPU compute dispatcher.

- `void gpu_register(const char* vendor_id, sigma_u32 vram_mb);`
- `bool gpu_dispatch(const char* workload_type);`

### `SovereignHWTranspilerEngine`

Self-learning PCIe driver shim generator.

- `void hw_transpiler_profile(sigma_u32 vendor_id, sigma_u32 device_id);`

### `SovereignWatchdogEngine`

Hardware hang recovery timer.

- `void watchdog_init(sigma_u32 timeout_ms);`
- `void watchdog_service(sigma_u32 tick_ms);`

## Networking & Storage

### `SovereignNetStackEngine`

Zero-trust Ring-0 TCP/IP stack.

- `void netstack_register_interface(const char* mac_addr);`
- `bool netstack_dispatch_packet(const char* payload, sigma_u32 length);`

### `SovereignVFSEngine`

Distributed, multi-node replicated filesystem.

- `void vfs_mount_node(const char* node_address);`
- `void vfs_write_file(const char* filepath, const char* data);`

### `SovereignContainerStorageEngine`

SovereignVFS bridging for Micro-VM containers.

- `void container_storage_mount(const char* container, const char* path);`

## UI & Personalization

### `SovereignThemeEngine`

Adaptive ambient-aware UI theme switcher.

- `void theme_update_ambient(sigma_u32 lux);`

### `SovereignGestureEngine`

Ring-0 hardware-accelerated gesture recognition.

- `void gesture_process_touch(sigma_u32 fingers, sigma_u32 dx, sigma_u32 dy);`

### `SovereignOnboardingEngine`

Persona-driven sovereign setup wizard.

- `void onboarding_apply_persona(const char* type);`

### `SovereignShortcutsEngine`

Contextual predictive quick-action engine.

- `void shortcuts_suggest(const char* context, const char* suggestion);`

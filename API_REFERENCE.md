# SigmaOS API Reference

Complete API mapping for all Sovereign Singleton shards.

## Core Security

### `SovereignCapabilityEngine`

Per-process syscall gating with bitmask precision.

- `void capability_grant(sigma_u32 pid, sigma_u64 caps);`

- `bool capability_check(sigma_u32 pid, sigma_u64 required);`

### `SovereignEnclaveEngine`

Hardware-level PQC key isolation.

- `void enclave_provision(sigma_u32 id);`

- `void enclave_store_key(sigma_u32 id, const char* key);`

### `SovereignMultiUserEngine`

UID/GID process identity with PQC attestation.

- `sigma_u32 multiuser_register(sigma_u32 uid, sigma_u32 gid, const char* user, sigma_u64 caps);`

- `bool multiuser_authenticate(sigma_u32 uid, const char* username);`

## Package & Automation

### `SovereignPackageEngine`

S-PKG: Sovereign App Bundle installer.

- `bool spkg_install(const char* name, sigma_u32 version);

 `bool spkg_remove(const char* name);`

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


#ifndef SIGMA_CONFIG_REGISTRY_H
#define SIGMA_CONFIG_REGISTRY_H

#include "suites/S01_Genesis/shards/sigma_types.h"

// SigmaOS Configuration Registry (S10)
// Absorbs the centralized control of the Windows Registry but powered by robust JSON nodes.

// Query the system registry for a configuration value
void* reg_query_value(const char* json_path);

// Atomically write a system update to the registry
void reg_write_value(const char* json_path, void* data, uint32_t data_len);

// Subscribe to a registry change (e.g., UI Theme changed) via callback
void reg_subscribe_to_change(const char* json_path, void (*callback)(void*));

// Safely execute a system-wide update transaction from the Package Manager
void reg_apply_system_update_transaction(const char* signed_update_package_path);

#endif // SIGMA_CONFIG_REGISTRY_H


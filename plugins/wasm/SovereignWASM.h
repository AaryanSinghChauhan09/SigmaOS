#ifndef SOVEREIGN_WASM_H
#define SOVEREIGN_WASM_H

#include <stdint.h>
#include <stdbool.h>
#include "../../include/core/SovereignDriver.h"

// Define WASM Sandbox Resource Limits (Hard Bounds)
typedef struct {
    uint32_t max_memory_mb;       // Max memory the WASM instance can allocate
    uint32_t max_execution_ms;    // Time limit for JIT execution (0 = infinite)
    bool network_access;          // Whether the shard can hit the host network stack
    bool fs_access;               // Whether the shard can read/write disk
} WasmSandboxConfig_t;

// The struct defining an integrated Shard inside the Lattice
typedef struct {
    char shard_id[64];
    uint8_t* bytecode;            // Raw WASM payload
    size_t bytecode_size;
    WasmSandboxConfig_t config;   // Active security bounds
    
    // Internal JIT State Pointer
    void* jit_context;
} SovereignWasmInstance_t;

// --- API ---

/**
 * Validates, compiles, and loads a WASM Shard into the Secure Sandbox.
 */
SovereignStatus_t wasm_load_shard(const char* filepath, WasmSandboxConfig_t config, SovereignWasmInstance_t* out_instance);

/**
 * Safely executes the "main" entrypoint of the WASM module within memory bounds.
 */
SovereignStatus_t wasm_execute_shard(SovereignWasmInstance_t* instance);

/**
 * Destroys the sandbox context and frees hardware resources safely.
 */
void wasm_destroy_shard(SovereignWasmInstance_t* instance);

#endif // SOVEREIGN_WASM_H

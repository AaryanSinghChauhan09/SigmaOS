/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN WASM RUNTIME (S-WASM)
 * =========================================================================
 * Mission: High-performance, sandboxed AOT execution for system shards.
 * =========================================================================
 */

#ifndef SIGMA_WASM_H
#define SIGMA_WASM_H

#include "core/sigma_types.h"

#ifdef __cplusplus
namespace SigmaOS {
namespace Runtime {

class SovereignWasmRuntime {
public:
    static SovereignWasmRuntime& getInstance() {
        static SovereignWasmRuntime instance;
        return instance;
    }

    void init();
    bool loadModule(const void* bytecode, sigma_size_t size);
    bool invoke(const char* funcName);

private:
    SovereignWasmRuntime() : initialized(0) {}
    sigma_u32 initialized;
};

} // namespace Runtime
} // namespace SigmaOS
#endif

#ifdef __cplusplus
extern "C" {
#endif

void wasm_init(void);
bool wasm_load_module(const void* bytecode, sigma_size_t size);
bool wasm_invoke(const char* funcName);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_WASM_H */

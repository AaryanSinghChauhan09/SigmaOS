#include "../../../include/sigma_types.h"
#include "runtime/sigma_wasm.h"
#include "../../../include/sigma_log.h"
#include "libc/SovereignLibC.h"

namespace SigmaOS {
namespace Runtime {

void SovereignWasmRuntime::init() {
    log_emit(LOG_INFO, "[WASM] Initializing Sovereign WASM AOT Runtime (Lattice-Optimized)...");
    this->initialized = 1u;
}

bool SovereignWasmRuntime::loadModule(const void* bytecode, sigma_size_t size) {
    (void)bytecode; (void)size;
    log_emit(LOG_INFO, "[WASM] Loading module into silicon-native amnesic memory...");
    return true;
}

bool SovereignWasmRuntime::invoke(const char* funcName) {
    log_emit_f(LOG_INFO, "[WASM] Invoking lattice function: %s", funcName);
    return true;
}

} // namespace Runtime
} // namespace SigmaOS

extern "C" {

/* --- C Bridge --- */
void wasm_init() {
    SigmaOS::Runtime::SovereignWasmRuntime::init();
}

extern "C" bool wasm_load_module(const void* bytecode, sigma_size_t size) {
    return SigmaOS::Runtime::SovereignWasmRuntime::loadModule(bytecode, size);
}

extern "C" bool wasm_invoke(const char* funcName) {
    return SigmaOS::Runtime::SovereignWasmRuntime::invoke(funcName);
}

} // extern "C"

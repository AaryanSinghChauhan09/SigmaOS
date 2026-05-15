#pragma once

#include "../../include/sigma_log.h"

namespace SigmaOS {
namespace Runtime {

class WasmRuntime {
public:
    WasmRuntime();
    ~WasmRuntime();

    // Load a WASM binary from a file path. Returns true on success.
    bool loadFromFile(const char* path);

    // Invoke an exported function by name. Returns true on success.
    bool invoke(const char* funcName);
};

} // namespace Runtime
} // namespace SigmaOS

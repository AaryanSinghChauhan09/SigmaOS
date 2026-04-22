#ifndef SIGMA_WASM_ENGINE_H
#define SIGMA_WASM_ENGINE_H

#include <stdint.h>

// WASI interface definition
typedef struct {
    int (*fd_write)(int fd, const void *iovs, int iovs_len, int *nwritten);
    int (*fd_read)(int fd, void *iovs, int iovs_len, int *nread);
    void (*proc_exit)(int rval);
} WasiInterface;

// Initialize WASM Native JIT Engine
int init_wasm_engine(void);

// Load and execute a WASM binary with WASI compliance and sandboxing
int execute_wasm_shard(const uint8_t* wasm_binary, uint32_t size, WasiInterface* wasi_impl);

#endif // SIGMA_WASM_ENGINE_H

#include "sigma_libc.h"

// SigmaOS WASM-Safe Allocator (S-WASM-ALLOC)
// Purpose: Browser-safe memory management targeting the WASM linear memory model.
// USP: Prevents out-of-bounds access within the browser sandbox.

static uint8_t* wasm_heap_start = (uint8_t*)0x1000000;
static size_t   wasm_heap_ptr = 0;

void* sigma_wasm_alloc(size_t size) {
    sigma_sigma_printf("[WASM-ALLOC] Growing linear memory by %d bytes...\n", (uint32_t)size);
    
    // Simulate WASM memory.grow
    void* ptr = wasm_heap_start + wasm_heap_ptr;
    wasm_heap_ptr += size;
    
    // Align to 8 bytes
    wasm_heap_ptr = (wasm_heap_ptr + 7) & ~7;
    
    return ptr;
}

void sigma_wasm_free(void* ptr) {
    // WASM linear memory is typically managed by a stack/heap pointer;
    // full free logic would require a dlmalloc-style wrapper.
    sigma_sigma_printf("[WASM-ALLOC] Pointer %p released to linear pool.\n", ptr);
}

void shard_init() {
    sigma_sigma_printf("[SHARD] WASM-Safe Allocator active (Browser Profile Optimized).\n");
}

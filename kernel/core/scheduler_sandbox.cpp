#include "sigma_sandbox.h"
#include <stddef.h>

// ---------------------------------------------------------
// SigmaOS Sandbox Scheduler Stubs
// Enforces capability bounds during context switches and
// memory allocations.
// ---------------------------------------------------------

namespace sigma {
namespace core {

using namespace sigma::security;

// Mock active context
static SovereignSandboxContext current_process_context;

extern "C" {

// Initialize a strict WASM sandbox context for a new process
void init_wasm_sandbox(SovereignSandboxContext* ctx, uint32_t pid, uint64_t mem_limit) {
    ctx->process_id = pid;
    ctx->ring_level = SandboxRing::RING_4_WASM;
    
    // Strict default-deny policy
    ctx->caps.can_network = false;
    ctx->caps.can_fs_read = false;  // Needs explicit mounting
    ctx->caps.can_fs_write = false;
    ctx->caps.can_spawn_process = false;
    ctx->caps.can_allocate_rwx = false; // JIT disabled by default
    
    ctx->max_memory_bytes = mem_limit;
    ctx->current_memory_bytes = 0;
}

// Check if a process is allowed to allocate executable memory
bool verify_allocation(const SovereignSandboxContext* ctx, uint64_t requested_bytes, bool is_executable) {
    if (ctx->current_memory_bytes + requested_bytes > ctx->max_memory_bytes) {
        return false; // OOM in sandbox
    }
    
    if (is_executable && !ctx->check_capability(&CapabilityMask::can_allocate_rwx)) {
        return false; // W^X violation
    }
    
    return true;
}

} // extern "C"

} // namespace core
} // namespace sigma

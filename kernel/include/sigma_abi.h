#ifndef SIGMA_ABI_H
#define SIGMA_ABI_H

#include "core/sigma_types.h"

/**
 * @file sigma_abi.h
 * @brief The Sovereign Application Binary Interface (Sigma-ABI)
 * 
 * Defines the standard syscall numbers and entry points for WASM Shards.
 * This is the "Bridge" that allows sandboxed code to talk to the metal.
 */

/* --- Syscall Categories --- */
#define SIGMA_SYS_FILE_OPEN    0x101
#define SIGMA_SYS_FILE_READ    0x102
#define SIGMA_SYS_FILE_WRITE   0x103
#define SIGMA_SYS_FILE_CLOSE   0x104

#define SIGMA_SYS_NET_SEND     0x201
#define SIGMA_SYS_NET_RECV     0x202

#define SIGMA_SYS_UI_FLUSH     0x301 // Flush shard framebuffer to Zenith Compositor
#define SIGMA_SYS_UI_EVENT     0x302 // Poll for input events

#define SIGMA_SYS_AI_INFER     0x401 // Offload task to NPU/LLM Shard

/**
 * @brief Standard Syscall Wrapper for WASM.
 * In AOT-compiled code, this translates to a specialized 'syscall' or 'vmcall' instruction.
 */
typedef struct {
    sigma_u32 call_id;
    sigma_u64 arg0;
    sigma_u64 arg1;
    sigma_u64 arg2;
    sigma_u64 arg3;
} sigma_syscall_req_t;

typedef struct {
    sigma_s64 result;
    sigma_u32 error_code;
} sigma_syscall_res_t;

#endif // SIGMA_ABI_H

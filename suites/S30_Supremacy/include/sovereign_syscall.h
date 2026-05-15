#ifndef SOVEREIGN_SYSCALL_H
#define SOVEREIGN_SYSCALL_H

#include "../../../include/core/sigma_types.h"

/**
 * SigmaOS Sovereign Syscall Interface (v1.0)
 * Defines the atomistic interaction protocol for all 33 Sovereign Suites.
 */

typedef enum {
    SYS_SHARD_LOAD   = 0x01,
    SYS_SHARD_UNLOAD = 0x02,
    SYS_SHARD_SIGNAL = 0x03,
    
    SYS_MEM_ALLOC    = 0x10,
    SYS_MEM_FREE     = 0x11,
    
    SYS_IPC_SEND     = 0x20,
    SYS_IPC_RECV     = 0x21,
    
    SYS_SEC_VERIFY   = 0x30,
    SYS_SEC_REVOKE   = 0x31,
    
    SYS_HAL_NOTIFY   = 0x40,
    
    SYS_SOVEREIGN_EXIT = 0xFF
} sovereign_syscall_t;

typedef struct {
    uint64_t arg1;
    uint64_t arg2;
    uint64_t arg3;
    uint64_t arg4;
    uint64_t arg5;
} syscall_args_t;

/**
 * The master entry point for all shard-level requests.
 * Handled by S00 SovereignCore.
 */
uint64_t sovereign_invoke(sovereign_syscall_t call, syscall_args_t* args);

#endif

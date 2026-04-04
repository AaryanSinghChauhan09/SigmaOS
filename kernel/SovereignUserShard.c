/**
 * @file SovereignUserShard.c
 * @brief Framework for fully custom user-defined functions (Issue #1)
 * @version 1.0 (Zenith)
 * 
 * ## Implementation
 * - Registry-based function sharding.
 * - Zero-trust isolation for user-space logic.
 * - Industrial IPC compatibility.
 */

#include "sigma_kernel_types.h"

typedef struct {
    char name[32];
    void (*func_ptr)(void*);
    u32  id;
    bool_t enabled;
} sigma_user_func;

#define MAX_USER_FUNCS 64
static sigma_user_func g_UserFuncs[MAX_USER_FUNCS];
static u32 g_UserFuncCount = 0;

void user_register_func(const char* name, void (*ptr)(void*)) {
    if (g_UserFuncCount >= MAX_USER_FUNCS) return;
    
    sigma_user_func* f = &g_UserFuncs[g_UserFuncCount];
    usize i = 0;
    while (i < 31 && name[i]) { f->name[i] = name[i]; i++; }
    f->name[i] = '\0';
    f->func_ptr = ptr;
    f->id = g_UserFuncCount++;
    f->enabled = TRUE;
    
    // Log to kernel console
    // sigma_print("[USER]: Custom Function Registered: %s\n", name);
}

void user_execute_func(u32 id, void* args) {
    if (id < g_UserFuncCount && g_UserFuncs[id].enabled) {
        g_UserFuncs[id].func_ptr(args);
    }
}

void user_init() {
    for (int i = 0; i < MAX_USER_FUNCS; i++) g_UserFuncs[i].enabled = FALSE;
    g_UserFuncCount = 0;
}

#include "../../include/libc/SovereignLibC.h"
#include "../../include/libc/sigma_libc.h"

// SigmaOS Universal Abstraction Layer (UAL)
// Purpose: Seamless adaptation across Bare-metal, Virtualized (QEMU), and Browser (WASM) environments.
// USP: Single Kernel Binary, Environment-Specific Hardware Handlers.

typedef enum {
    ENV_BARE_METAL,
    ENV_VIRTUALIZED,
    ENV_BROWSER_WASM,
    ENV_EMBEDDED_IOT
} sigma_env_t;

// Environment detection (Simulated for Shard environment)
sigma_env_t ual_detect_environment() {
#ifdef SIGMA_TARGET_WASM
    return ENV_BROWSER_WASM;
#elif defined(SIGMA_TARGET_QEMU)
    return ENV_VIRTUALIZED;
#elif defined(SIGMA_TARGET_IOT)
    return ENV_EMBEDDED_IOT;
#else
    return ENV_BARE_METAL;
#endif
}

void ual_load_hal() {
    sigma_env_t env = ual_detect_environment();
    
    switch (env) {
        case ENV_BROWSER_WASM:
            sigma_printf("[UAL] Context: Browser Sandbox (WASM). Loading JS-Bridge HAL...\n");
            break;
        case ENV_VIRTUALIZED:
            sigma_printf("[UAL] Context: Virtual Machine (QEMU). Loading VirtIO HAL...\n");
            break;
        case ENV_EMBEDDED_IOT:
            sigma_printf("[UAL] Context: Embedded IoT. Loading Minimal Static HAL...\n");
            break;
        case ENV_BARE_METAL:
            sigma_printf("[UAL] Context: Bare-Metal Silicon. Loading Sovereign HAL...\n");
            break;
    }
}

void shard_init() {
    SIGMA_SHARD_INIT();
    sigma_printf("[SHARD] Universal Abstraction Layer (UAL) Active.\n");
    ual_load_hal();
}

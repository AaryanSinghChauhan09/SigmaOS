/**
 * =========================================================================
 * Σ SIGMAOS: NATIVE KMS/GPU ABSTRACTION (Phase E)
 * =========================================================================
 * Kernel Mode Setting (KMS) abstractions for AMD and Intel GPUs.
 * Provides a unified API for the Zenith Compositor.
 * Inspired by Clear Linux performance profiles and SteamOS self-healing
 * GPU recovery models.
 * =========================================================================
 */

#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_error_codes.h"
#include <sigma_libc.h>

namespace SigmaOS {
namespace Graphics {

// -------------------------------------------------------------------------
// Types
// -------------------------------------------------------------------------
typedef enum {
    GPU_VENDOR_AMD = 0,
    GPU_VENDOR_INTEL = 1,
    GPU_VENDOR_GENERIC_VGA = 2
} GPUVendor;

typedef enum {
    GPU_PERF_POWERSAVE = 0,
    GPU_PERF_BALANCED = 1,
    GPU_PERF_HIGH_PERFORMANCE = 2
} GPUPerfProfile;

struct KMSContext {
    GPUVendor vendor;
    sigma_u32 width;
    sigma_u32 height;
    sigma_u32 pitch;
    sigma_u32 bpp;
    void*     framebuffer_ptr;
    bool      is_hardware_accelerated;

    // Linux-inspired enhancements
    GPUPerfProfile perf_profile;
    bool           gpu_hung;
    sigma_u32      frame_count;
    sigma_u32      skipped_frames;
    sigma_u32      current_fps;
    sigma_u32      latency_ms;
    sigma_u32      recovery_count;
};

static KMSContext g_active_kms_ctx = {
    GPU_VENDOR_GENERIC_VGA, 1024, 768, 1024 * 4, 32, SIGMA_NULL, false,
    GPU_PERF_BALANCED, false, 0, 0, 60, 8, 0
};

// -------------------------------------------------------------------------
// AMD GPU Stubs
// -------------------------------------------------------------------------
static sigma_status init_amdgpu(KMSContext* ctx) {
    sys_print("[KMS] Initializing AMD Radeon GPU Driver...\n");
    // Hardware-specific setup
    ctx->vendor = GPU_VENDOR_AMD;
    ctx->is_hardware_accelerated = true;
    ctx->latency_ms = 4;
    return SIGMA_SUCCESS;
}

// -------------------------------------------------------------------------
// Intel GPU Stubs
// -------------------------------------------------------------------------
static sigma_status init_i915(KMSContext* ctx) {
    sys_print("[KMS] Initializing Intel i915 GPU Driver...\n");
    // Hardware-specific setup
    ctx->vendor = GPU_VENDOR_INTEL;
    ctx->is_hardware_accelerated = true;
    ctx->latency_ms = 6;
    return SIGMA_SUCCESS;
}

// -------------------------------------------------------------------------
// Public API
// -------------------------------------------------------------------------
sigma_status sigma_kms_init(sigma_u16 pci_vendor_id) {
    sys_print("[KMS] Starting Kernel Mode Setting...\n");

    g_active_kms_ctx.gpu_hung = false;
    g_active_kms_ctx.frame_count = 0;
    g_active_kms_ctx.skipped_frames = 0;

    if (pci_vendor_id == 0x1002) { // AMD
        return init_amdgpu(&g_active_kms_ctx);
    } else if (pci_vendor_id == 0x8086) { // Intel
        return init_i915(&g_active_kms_ctx);
    } else {
        sys_print("[KMS] Unsupported GPU. Falling back to Generic VGA.\n");
        g_active_kms_ctx.vendor = GPU_VENDOR_GENERIC_VGA;
        g_active_kms_ctx.is_hardware_accelerated = false;
        g_active_kms_ctx.latency_ms = 16;
        return ZEN_ERROR; // Represents fallback
    }
}

void* sigma_kms_get_framebuffer() {
    if (!g_active_kms_ctx.framebuffer_ptr) {
        // Allocate a dummy framebuffer for now
        g_active_kms_ctx.framebuffer_ptr = sigma_malloc(
            g_active_kms_ctx.width * g_active_kms_ctx.height * (g_active_kms_ctx.bpp / 8)
        );
    }
    return g_active_kms_ctx.framebuffer_ptr;
}

// SteamOS-inspired self-healing GPU recovery logic
sigma_status sigma_kms_recover_gpu() {
    if (!g_active_kms_ctx.gpu_hung) {
        return SIGMA_SUCCESS;
    }

    sys_print("[KMS] 🔄 [SteamOS recovery] GPU Hang detected! Initiating dynamic reset...\n");

    // Reset performance mode back to safe default (Balanced)
    g_active_kms_ctx.perf_profile = GPU_PERF_BALANCED;

    // Clear/Reset ring buffers or framebuffer allocations safely
    if (g_active_kms_ctx.framebuffer_ptr) {
        sigma_memset(g_active_kms_ctx.framebuffer_ptr, 0,
                     g_active_kms_ctx.width * g_active_kms_ctx.height * (g_active_kms_ctx.bpp / 8));
    }

    g_active_kms_ctx.gpu_hung = false;
    g_active_kms_ctx.skipped_frames += 5; // Track skipped frames during recovery
    g_active_kms_ctx.recovery_count++;

    sys_print("[KMS] ✅ GPU pipeline successfully recovered and re-initialized.\n");
    return SIGMA_SUCCESS;
}

void sigma_kms_simulate_hang() {
    sys_print("[KMS] Simulating GPU hardware freeze...\n");
    g_active_kms_ctx.gpu_hung = true;
}

// Clear Linux-inspired Performance scaling profile selection
sigma_status sigma_kms_set_perf_profile(GPUPerfProfile profile) {
    if (g_active_kms_ctx.gpu_hung) {
        sys_print("[KMS] Cannot adjust performance: GPU is currently frozen!\n");
        return K_ERR_BUSY;
    }

    g_active_kms_ctx.perf_profile = profile;

    switch (profile) {
        case GPU_PERF_POWERSAVE:
            sys_print("[KMS] [Clear Linux profile] Performance Profile: POWERSAVE (Clock-gated, Latency=16ms)\n");
            g_active_kms_ctx.latency_ms = 16;
            g_active_kms_ctx.current_fps = 30;
            break;
        case GPU_PERF_BALANCED:
            sys_print("[KMS] [Clear Linux profile] Performance Profile: BALANCED (Adaptive, Latency=8ms)\n");
            g_active_kms_ctx.latency_ms = 8;
            g_active_kms_ctx.current_fps = 60;
            break;
        case GPU_PERF_HIGH_PERFORMANCE:
            sys_print("[KMS] [Clear Linux profile] Performance Profile: HIGH PERFORMANCE (Max Clocks, Latency=1ms)\n");
            g_active_kms_ctx.latency_ms = 1;
            g_active_kms_ctx.current_fps = 144;
            break;
    }
    return SIGMA_SUCCESS;
}

void sigma_kms_flip_buffers() {
    // Check GPU health first
    if (g_active_kms_ctx.gpu_hung) {
        sys_print("[KMS] Page flip failed: GPU is hung.\n");
        sigma_kms_recover_gpu();
        return;
    }

    g_active_kms_ctx.frame_count++;

    // Simulate minor frame skipping depending on performance profile
    if (g_active_kms_ctx.perf_profile == GPU_PERF_POWERSAVE && (g_active_kms_ctx.frame_count % 10 == 0)) {
        g_active_kms_ctx.skipped_frames++;
    }

    // In hardware-accelerated modes, this would swap the display pointers.
    // sys_print("[KMS] Page flip triggered.\n");
}

} // namespace Graphics
} // namespace SigmaOS

extern "C" {
    sigma_status sigma_kms_init_c(sigma_u16 pci_vendor_id) {
        return SigmaOS::Graphics::sigma_kms_init(pci_vendor_id);
    }
    
    void* sigma_kms_get_fb() {
        return SigmaOS::Graphics::sigma_kms_get_framebuffer();
    }

    sigma_status sigma_kms_set_perf_profile_c(int profile) {
        return SigmaOS::Graphics::sigma_kms_set_perf_profile((SigmaOS::Graphics::GPUPerfProfile)profile);
    }

    int sigma_kms_get_perf_profile_c() {
        return (int)SigmaOS::Graphics::g_active_kms_ctx.perf_profile;
    }

    void sigma_kms_simulate_hang_c() {
        SigmaOS::Graphics::sigma_kms_simulate_hang();
    }

    sigma_status sigma_kms_recover_gpu_c() {
        return SigmaOS::Graphics::sigma_kms_recover_gpu();
    }

    bool sigma_kms_is_gpu_hung_c() {
        return SigmaOS::Graphics::g_active_kms_ctx.gpu_hung;
    }

    sigma_u32 sigma_kms_get_fps_c() {
        return SigmaOS::Graphics::g_active_kms_ctx.current_fps;
    }

    sigma_u32 sigma_kms_get_latency_c() {
        return SigmaOS::Graphics::g_active_kms_ctx.latency_ms;
    }
}

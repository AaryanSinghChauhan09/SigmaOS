/**
 * =========================================================================
 * Σ SIGMAOS: NATIVE KMS/GPU ABSTRACTION (Phase E)
 * =========================================================================
 * Kernel Mode Setting (KMS) abstractions for AMD and Intel GPUs.
 * Provides a unified API for the Zenith Compositor.
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

struct KMSContext {
    GPUVendor vendor;
    sigma_u32 width;
    sigma_u32 height;
    sigma_u32 pitch;
    sigma_u32 bpp;
    void*     framebuffer_ptr;
    bool      is_hardware_accelerated;
};

static KMSContext g_active_kms_ctx = {
    GPU_VENDOR_GENERIC_VGA, 1024, 768, 1024 * 4, 32, SIGMA_NULL, false
};

// -------------------------------------------------------------------------
// AMD GPU Stubs
// -------------------------------------------------------------------------
static sigma_status init_amdgpu(KMSContext* ctx) {
    sys_print("[KMS] Initializing AMD Radeon GPU Driver...\n");
    // Hardware-specific setup
    ctx->vendor = GPU_VENDOR_AMD;
    ctx->is_hardware_accelerated = true;
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
    return SIGMA_SUCCESS;
}

// -------------------------------------------------------------------------
// Public API
// -------------------------------------------------------------------------
sigma_status sigma_kms_init(sigma_u16 pci_vendor_id) {
    sys_print("[KMS] Starting Kernel Mode Setting...\n");

    if (pci_vendor_id == 0x1002) { // AMD
        return init_amdgpu(&g_active_kms_ctx);
    } else if (pci_vendor_id == 0x8086) { // Intel
        return init_i915(&g_active_kms_ctx);
    } else {
        sys_print("[KMS] Unsupported GPU. Falling back to Generic VGA.\n");
        g_active_kms_ctx.vendor = GPU_VENDOR_GENERIC_VGA;
        g_active_kms_ctx.is_hardware_accelerated = false;
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

void sigma_kms_flip_buffers() {
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
}

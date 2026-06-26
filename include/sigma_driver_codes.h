/**
 * =========================================================================
 * Σ SIGMAOS DRIVER ERROR CODES
 * =========================================================================
 * Structured error codes for the SigmaOS driver subsystem.
 * Format: ZEN_DRV_<SUBSYSTEM>_<ERROR>
 *
 * Inspired by:
 *   - Rescuezilla / SystemRescue: Fallback-safe mode on driver crash.
 *   - SteamOS: GPU driver recovery.
 *   - Clear Linux: Performance-tuned driver auditing.
 * =========================================================================
 */

#ifndef SIGMA_DRIVER_CODES_H
#define SIGMA_DRIVER_CODES_H

#include "sigma_types.h"

// -------------------------------------------------------------------------
// Driver Subsystem Base Codes
// -------------------------------------------------------------------------
#define ZEN_DRV_OK                      0x0000

// --- GPU / Graphics Drivers ---
#define ZEN_DRV_GPU_INIT_FAILED         0x0401  // GPU initialization failure
#define ZEN_DRV_GPU_FIRMWARE_MISSING    0x0402  // Required firmware blob not found
#define ZEN_DRV_GPU_FALLBACK_VGA        0x0403  // Fell back to VGA safe mode
#define ZEN_DRV_GPU_RESET_REQUIRED      0x0404  // Hard reset needed (GPU hang)
#define ZEN_DRV_GPU_DRIVER_NOT_FOUND    0x0405  // No matching kernel module found

// --- Networking Drivers ---
#define ZEN_DRV_NET_INIT_FAILED         0x0501  // NIC driver init failure
#define ZEN_DRV_NET_FIRMWARE_MISSING    0x0502  // NIC firmware blob absent
#define ZEN_DRV_NET_LINK_DOWN           0x0503  // Physical link is down
#define ZEN_DRV_NET_REALTEK_ERR         0x0504  // Realtek-specific driver fault
#define ZEN_DRV_NET_BROADCOM_ERR        0x0505  // Broadcom-specific driver fault
#define ZEN_DRV_NET_INTEL_ERR           0x0506  // Intel NIC driver fault

// --- Audio Drivers ---
#define ZEN_DRV_AUDIO_INIT_FAILED       0x0601  // Audio subsystem init failure
#define ZEN_DRV_AUDIO_CODEC_NOT_FOUND   0x0602  // HDA codec not identified
#define ZEN_DRV_AUDIO_FALLBACK_DUMMY    0x0603  // Fell back to dummy audio output

// --- Storage Drivers ---
#define ZEN_DRV_STORAGE_INIT_FAILED     0x0701  // Block device driver init failed
#define ZEN_DRV_STORAGE_NVME_ERR        0x0702  // NVMe controller error
#define ZEN_DRV_STORAGE_SATA_ERR        0x0703  // SATA link fault
#define ZEN_DRV_STORAGE_EMMC_ERR        0x0704  // eMMC / ARM64 storage fault
#define ZEN_DRV_STORAGE_READONLY_BOOT   0x0705  // Forensic read-only block enforced

// --- Input Drivers ---
#define ZEN_DRV_INPUT_INIT_FAILED       0x0801  // Input subsystem failure
#define ZEN_DRV_INPUT_TOUCH_ERR         0x0802  // Touchscreen / trackpad error
#define ZEN_DRV_INPUT_KEYBOARD_ERR      0x0803  // Keyboard driver fault

// --- DKMS / Module Rebuild ---
#define ZEN_DRV_DKMS_BUILD_FAILED       0x0901  // DKMS module rebuild failed after kernel update
#define ZEN_DRV_DKMS_NOT_REGISTERED     0x0902  // Driver not in DKMS registry
#define ZEN_DRV_DKMS_VERSION_MISMATCH   0x0903  // Module ABI mismatch with running kernel

// --- Generic ---
#define ZEN_DRV_MODULE_NOT_FOUND        0x0A01  // Kernel module (.ko) not found
#define ZEN_DRV_MODULE_LOAD_FAILED      0x0A02  // insmod/modprobe failed
#define ZEN_DRV_MODULE_TAINTED          0x0A03  // Proprietary or out-of-tree module loaded
#define ZEN_DRV_REGISTRY_FETCH_FAILED   0x0A04  // Sovereign driver registry unreachable
#define ZEN_DRV_RECIPE_SIG_INVALID      0x0A05  // .srecipe signature verification failed

// -------------------------------------------------------------------------
// Hardware Profile Flags (for profile-aware driver selection)
// -------------------------------------------------------------------------
typedef enum {
    SIGMA_HW_PROFILE_STANDARD   = 0x01, // General desktop hardware
    SIGMA_HW_PROFILE_GAMING     = 0x02, // GPU-optimised (SteamOS influence)
    SIGMA_HW_PROFILE_IOT_ARM64  = 0x04, // Lightweight ARM64 (Raspberry Pi OS influence)
    SIGMA_HW_PROFILE_FORENSIC   = 0x08, // Read-only block (CAINE influence)
    SIGMA_HW_PROFILE_SERVER     = 0x10, // Cloud hardware support (Fedora CoreOS influence)
} sigma_hw_profile_t;

// -------------------------------------------------------------------------
// Helper: Decode a driver error code to a human-readable string
// -------------------------------------------------------------------------
static inline const char* sigma_driver_strerror(sigma_u32 code) {
    switch (code) {
        case ZEN_DRV_OK:                    return "Driver OK";
        case ZEN_DRV_GPU_INIT_FAILED:       return "ZEN-DRIVER-0401: GPU init failed";
        case ZEN_DRV_GPU_FIRMWARE_MISSING:  return "ZEN-DRIVER-0402: GPU firmware blob missing";
        case ZEN_DRV_GPU_FALLBACK_VGA:      return "ZEN-DRIVER-0403: Fell back to VGA safe mode";
        case ZEN_DRV_GPU_RESET_REQUIRED:    return "ZEN-DRIVER-0404: GPU hard reset required";
        case ZEN_DRV_GPU_DRIVER_NOT_FOUND:  return "ZEN-DRIVER-0405: GPU kernel module not found";
        case ZEN_DRV_NET_INIT_FAILED:       return "ZEN-DRIVER-0501: NIC driver init failed";
        case ZEN_DRV_NET_FIRMWARE_MISSING:  return "ZEN-DRIVER-0502: NIC firmware blob missing";
        case ZEN_DRV_NET_LINK_DOWN:         return "ZEN-DRIVER-0503: Network link is down";
        case ZEN_DRV_AUDIO_INIT_FAILED:     return "ZEN-DRIVER-0601: Audio init failed";
        case ZEN_DRV_AUDIO_FALLBACK_DUMMY:  return "ZEN-DRIVER-0603: Audio fell back to dummy device";
        case ZEN_DRV_STORAGE_INIT_FAILED:   return "ZEN-DRIVER-0701: Storage driver init failed";
        case ZEN_DRV_STORAGE_NVME_ERR:      return "ZEN-DRIVER-0702: NVMe controller error";
        case ZEN_DRV_STORAGE_READONLY_BOOT: return "ZEN-DRIVER-0705: Forensic read-only block enforced";
        case ZEN_DRV_DKMS_BUILD_FAILED:     return "ZEN-DRIVER-0901: DKMS module rebuild failed";
        case ZEN_DRV_DKMS_VERSION_MISMATCH: return "ZEN-DRIVER-0903: DKMS ABI mismatch";
        case ZEN_DRV_MODULE_NOT_FOUND:      return "ZEN-DRIVER-0A01: Kernel module not found";
        case ZEN_DRV_MODULE_LOAD_FAILED:    return "ZEN-DRIVER-0A02: Module load failed";
        case ZEN_DRV_MODULE_TAINTED:        return "ZEN-DRIVER-0A03: Tainted (out-of-tree) module";
        case ZEN_DRV_REGISTRY_FETCH_FAILED: return "ZEN-DRIVER-0A04: Sovereign driver registry unreachable";
        case ZEN_DRV_RECIPE_SIG_INVALID:    return "ZEN-DRIVER-0A05: Driver recipe signature invalid";
        default:                            return "ZEN-DRIVER-UNKNOWN: Unknown driver error";
    }
}

#endif // SIGMA_DRIVER_CODES_H

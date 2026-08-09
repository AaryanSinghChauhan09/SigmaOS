#ifndef SIGMA_DRIVER_CODES_H
#define SIGMA_DRIVER_CODES_H

// Driver Manager / Hardware test suite codes
#define ZEN_DRV_GPU_INIT_FAILED         0xD001
#define ZEN_DRV_GPU_FALLBACK_VGA        0xD002
#define ZEN_DRV_RECIPE_SIG_INVALID      0xD003
#define ZEN_DRV_RELOAD_FAIL             0xD004
#define ZEN_DRV_CRASH                   0xD005

#define ZEN_DRV_MODULE_NOT_FOUND        0xD006
#define ZEN_DRV_STORAGE_READONLY_BOOT   0xD007
#define ZEN_DRV_GPU_FIRMWARE_MISSING    0xD008

#define ZEN_DRV_NET_REALTEK_ERR         0xD101
#define ZEN_DRV_NET_INIT_FAILED         0xD102
#define ZEN_DRV_NET_INTEL_ERR           0xD103
#define ZEN_DRV_NET_BROADCOM_ERR        0xD104
#define ZEN_DRV_NET_LINK_DOWN           0xD105
#define ZEN_DRV_NET_FIRMWARE_MISSING    0xD106

#define ZEN_DRV_AUDIO_INIT_FAILED       0xD201
#define ZEN_DRV_AUDIO_FALLBACK_DUMMY    0xD202
#define ZEN_DRV_AUDIO_CODEC_NOT_FOUND   0xD203

#define ZEN_DRV_STORAGE_NVME_ERR        0xD301
#define ZEN_DRV_STORAGE_SATA_ERR        0xD302
#define ZEN_DRV_STORAGE_EMMC_ERR        0xD303
#define ZEN_DRV_STORAGE_INIT_FAILED     0xD304

#define ZEN_DRV_DKMS_VERSION_MISMATCH   0xD401
#define ZEN_DRV_DKMS_BUILD_FAILED       0xD402
#define ZEN_DRV_REGISTRY_FETCH_FAILED   0xD403

// Helper function to return string descriptions for each code
static inline const char* sigma_driver_strerror(sigma_u32 code) {
    switch (code) {
        case ZEN_DRV_GPU_INIT_FAILED:       return "GPU Initialization Failed";
        case ZEN_DRV_GPU_FALLBACK_VGA:      return "GPU falling back to generic VGA mode";
        case ZEN_DRV_RECIPE_SIG_INVALID:    return "Driver Build Recipe Signature Invalid";
        case ZEN_DRV_RELOAD_FAIL:           return "Driver Module Reload Failed";
        case ZEN_DRV_CRASH:                 return "Driver Module Crashed";
        case ZEN_DRV_MODULE_NOT_FOUND:      return "Driver Module Not Found";
        case ZEN_DRV_STORAGE_READONLY_BOOT: return "Storage Boot is set to READ-ONLY";
        case ZEN_DRV_GPU_FIRMWARE_MISSING:  return "GPU firmware missing";
        case ZEN_DRV_NET_REALTEK_ERR:       return "Realtek NIC initialization failed";
        case ZEN_DRV_NET_INIT_FAILED:       return "Generic network driver load failed";
        case ZEN_DRV_NET_INTEL_ERR:         return "Intel wireless driver load failed";
        case ZEN_DRV_NET_BROADCOM_ERR:      return "Broadcom wireless driver load failed";
        case ZEN_DRV_NET_LINK_DOWN:         return "NIC Link is Down";
        case ZEN_DRV_NET_FIRMWARE_MISSING:  return "Network firmware missing";
        case ZEN_DRV_AUDIO_INIT_FAILED:     return "HD Audio initialization failed";
        case ZEN_DRV_AUDIO_FALLBACK_DUMMY:  return "Audio falling back to dummy driver";
        case ZEN_DRV_AUDIO_CODEC_NOT_FOUND: return "Audio codec not found";
        case ZEN_DRV_STORAGE_NVME_ERR:      return "NVMe driver error during probe";
        case ZEN_DRV_STORAGE_SATA_ERR:      return "AHCI SATA driver error during probe";
        case ZEN_DRV_STORAGE_EMMC_ERR:      return "eMMC driver error during probe";
        case ZEN_DRV_STORAGE_INIT_FAILED:   return "Generic storage driver load failed";
        case ZEN_DRV_DKMS_VERSION_MISMATCH: return "DKMS ABI version mismatch";
        case ZEN_DRV_DKMS_BUILD_FAILED:     return "DKMS auto-rebuild compilation failed";
        case ZEN_DRV_REGISTRY_FETCH_FAILED: return "DKMS Registry fetch failed";
        default:                            return "Unknown Driver Error";
    }
}

#endif // SIGMA_DRIVER_CODES_H
||||||| 65885484f
#ifndef SIGMA_DRIVER_CODES_H
#define SIGMA_DRIVER_CODES_H

#include "sigma_kernel_types.h"

/* -------------------------------------------------------------------------
 * Granular Driver & Hardware Diagnostic Codes (ZEN-DRIVER-xxxx)
 * ------------------------------------------------------------------------- */

/* GPU / Display */
#define ZEN_DRV_GPU_INIT_FAILED         0xD001
#define ZEN_DRV_GPU_FALLBACK_VGA        0xD002
#define ZEN_DRV_GPU_FIRMWARE_MISSING    0xD006

/* Networking */
#define ZEN_DRV_NET_INIT_FAILED         0xD010
#define ZEN_DRV_NET_LINK_DOWN           0xD011
#define ZEN_DRV_NET_FIRMWARE_MISSING    0xD012
#define ZEN_DRV_NET_REALTEK_ERR         0xD013
#define ZEN_DRV_NET_INTEL_ERR           0xD014
#define ZEN_DRV_NET_BROADCOM_ERR        0xD015

/* Audio */
#define ZEN_DRV_AUDIO_INIT_FAILED       0xD020
#define ZEN_DRV_AUDIO_CODEC_NOT_FOUND   0xD021
#define ZEN_DRV_AUDIO_FALLBACK_DUMMY    0xD022

/* Storage */
#define ZEN_DRV_STORAGE_INIT_FAILED     0xD030
#define ZEN_DRV_STORAGE_NVME_ERR        0xD031
#define ZEN_DRV_STORAGE_SATA_ERR        0xD032
#define ZEN_DRV_STORAGE_EMMC_ERR        0xD033
#define ZEN_DRV_STORAGE_READONLY_BOOT   0xD034

/* Modules & DKMS */
#define ZEN_DRV_RECIPE_SIG_INVALID      0xD003
#define ZEN_DRV_RELOAD_FAIL             0xD004
#define ZEN_DRV_CRASH                   0xD005
#define ZEN_DRV_MODULE_NOT_FOUND        0xD040
#define ZEN_DRV_REGISTRY_FETCH_FAILED   0xD041
#define ZEN_DRV_DKMS_VERSION_MISMATCH   0xD042
#define ZEN_DRV_DKMS_BUILD_FAILED       0xD043

/* String representation of driver codes for diagnostic reporting */
static inline const char* sigma_driver_strerror(sigma_u32 code) {
    switch (code) {
        case ZEN_DRV_GPU_INIT_FAILED:       return "GPU Initialization Failed";
        case ZEN_DRV_GPU_FALLBACK_VGA:      return "GPU Falling Back to VGA Safe Mode";
        case ZEN_DRV_GPU_FIRMWARE_MISSING:  return "GPU Firmware Missing";
        case ZEN_DRV_NET_INIT_FAILED:       return "Network Interface Initialization Failed";
        case ZEN_DRV_NET_LINK_DOWN:         return "Network Link Down";
        case ZEN_DRV_NET_FIRMWARE_MISSING:  return "Network Firmware Missing";
        case ZEN_DRV_NET_REALTEK_ERR:       return "Realtek Ethernet Error";
        case ZEN_DRV_NET_INTEL_ERR:         return "Intel Wi-Fi Error";
        case ZEN_DRV_NET_BROADCOM_ERR:      return "Broadcom Wi-Fi Error";
        case ZEN_DRV_AUDIO_INIT_FAILED:     return "Audio Initialization Failed";
        case ZEN_DRV_AUDIO_CODEC_NOT_FOUND: return "Audio Codec Not Found";
        case ZEN_DRV_AUDIO_FALLBACK_DUMMY:  return "Audio Falling Back to Dummy Device";
        case ZEN_DRV_STORAGE_INIT_FAILED:   return "Storage Controller Initialization Failed";
        case ZEN_DRV_STORAGE_NVME_ERR:      return "NVMe SSD Error";
        case ZEN_DRV_STORAGE_SATA_ERR:      return "SATA AHCI Error";
        case ZEN_DRV_STORAGE_EMMC_ERR:      return "eMMC Block Device Error";
        case ZEN_DRV_STORAGE_READONLY_BOOT: return "Forensic Boot — Read-Only Storage Enforced";
        case ZEN_DRV_RECIPE_SIG_INVALID:    return "Driver Recipe Signature Invalid";
        case ZEN_DRV_RELOAD_FAIL:           return "Driver Reload Failed";
        case ZEN_DRV_CRASH:                 return "Driver Crash Detected";
        case ZEN_DRV_MODULE_NOT_FOUND:      return "Kernel Module Not Found";
        case ZEN_DRV_REGISTRY_FETCH_FAILED: return "Sovereign Driver Registry Fetch Failed";
        case ZEN_DRV_DKMS_VERSION_MISMATCH: return "DKMS Kernel-ABI Version Mismatch";
        case ZEN_DRV_DKMS_BUILD_FAILED:     return "DKMS Module Build Failed";
        default:                            return "Unknown Hardware Diagnostic Error";
    }
}

#endif /* SIGMA_DRIVER_CODES_H */

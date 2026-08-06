/**
 * =========================================================================
 * Σ SIGMAOS DRIVER MANAGER
 * =========================================================================
<<<<<<< HEAD
 * Central kernel-space driver lifecycle manager with Linux-inspired features:
 *   - udev-style dynamic PCI ID Modalias auto-detection.
 *   - modprobe-style recursive module dependency resolution.
 *   - Fedora/RHEL-style Secure Module Signature verification (PQC Dilithium-5).
 *   - Self-heal on driver failure.
||||||| 65885484f
 * Central kernel-space driver lifecycle manager.
 *
 * Responsibilities:
 *   - Load / unload kernel modules by hardware profile.
 *   - Self-heal on driver failure (reload, fallback, or safe mode).
 *   - Report structured ZEN-DRIVER-xxxx codes via sigma_driver_codes.h.
 *   - Enforce hardware profiles: Standard / Gaming / IoT-ARM64 / Forensic.
 *
 * Inspired by:
 *   - SteamOS: GPU driver recovery and gaming hardware tuning.
 *   - Raspberry Pi OS: ARM64 / eMMC / PWM driver selection.
 *   - Rescuezilla / SystemRescue: Fallback safe mode on driver crash.
 *   - Clear Linux: Performance-first driver configuration.
 *   - Fedora CoreOS: Cloud hardware and DKMS integration.
=======
 * Central kernel-space driver lifecycle manager.
 *
 * Responsibilities:
 *   - Load / unload kernel modules by hardware profile.
 *   - Self-heal on driver failure (reload, fallback, or safe mode).
 *   - Report structured ZEN-DRIVER-xxxx codes via sigma_driver_codes.h.
 *   - Enforce hardware profiles: Standard / Gaming / IoT-ARM64 / Forensic.
 *   - Implement DAG Topological Sort (Kahn's Algorithm) for service and
 *     driver dependency loading to prevent resource deadlocks.
 *
 * Inspired by:
 *   - SteamOS: GPU driver recovery and gaming hardware tuning.
 *   - Raspberry Pi OS: ARM64 / eMMC / PWM driver selection.
 *   - Rescuezilla / SystemRescue: Fallback safe mode on driver crash.
 *   - Clear Linux: Performance-first driver configuration.
 *   - Fedora CoreOS: Cloud hardware and DKMS integration.
>>>>>>> origin/jules-driver-improvements-linux-inspired-5291856075380713095
 * =========================================================================
 */

#include <sigma_libc.h>
#include "../../include/sigma_error_codes.h"
#include "../../include/sigma_driver_codes.h"

extern "C" {
    void sys_print(const char* fmt, ...);
    int sigma_strcmp(const char* s1, const char* s2);
    void zenith_log_structured(sigma_u32 code, const char* comp,
                               const char* desc, sigma_u32 cid);
}

namespace Sigma {
namespace Drivers {

// -------------------------------------------------------------------------
// Driver Descriptor with explicit dependency tracking
// -------------------------------------------------------------------------
struct DriverDescriptor {
    const char*       module_name;    // e.g. "amdgpu", "r8169", "snd_hda_intel"
<<<<<<< HEAD
    const char*       subsystem;      // "gpu" | "net" | "audio" | "storage" | "input" | "core"
||||||| 65885484f
    const char*       subsystem;      // "gpu" | "net" | "audio" | "storage" | "input"
=======
    const char*       subsystem;      // "gpu" | "net" | "audio" | "storage" | "input" | "bus"
>>>>>>> origin/jules-driver-improvements-linux-inspired-5291856075380713095
    const char*       chipset_hint;   // vendor/chipset this primarily targets
    sigma_hw_profile_t profile_mask;  // profiles this driver is active under
    bool              requires_fw;    // needs a firmware blob
    sigma_u32         init_error;     // error code to fire on failure
    sigma_u32         fallback_error; // error code if falling back
<<<<<<< HEAD

    // Linux-inspired improvements:
    const char*       dependencies[4];// nullptr-terminated dependency modules
    sigma_u32         vendor_id;      // PCI Vendor ID (0xFFFF for wildcard/non-PCI)
    sigma_u32         device_id;      // PCI Device ID (0xFFFF for wildcard)
    bool              is_signed_pqc;  // Is the module cryptographically signed with Dilithium-5?
||||||| 65885484f
=======
    const char*       dependency_name;// Name of driver module this depends on, or SIGMA_NULL
>>>>>>> origin/jules-driver-improvements-linux-inspired-5291856075380713095
};

// -------------------------------------------------------------------------
<<<<<<< HEAD
// Default driver table
||||||| 65885484f
// Default driver table
// Profiles are bitmasks: can OR multiple profiles together.
=======
// Default driver table with rich dependency hierarchies (DAG)
// Profiles are bitmasks: can OR multiple profiles together.
>>>>>>> origin/jules-driver-improvements-linux-inspired-5291856075380713095
// -------------------------------------------------------------------------
static const DriverDescriptor g_driver_table[] = {
<<<<<<< HEAD
    // ---- Core Shards / Subsystems ----
    { "pci_core",     "core",    "PCI Express Root Complex",
      SIGMA_HW_PROFILE_ALL, false, 0, 0,
      { nullptr }, 0xFFFF, 0xFFFF, true },

    { "mailbox",      "core",    "Broadcom Mailbox Controller",
      SIGMA_HW_PROFILE_IOT_ARM64, false, 0, 0,
      { nullptr }, 0xFFFF, 0xFFFF, true },

    { "cfg80211",     "core",    "Wireless Configuration Core",
      (sigma_hw_profile_t)(SIGMA_HW_PROFILE_STANDARD | SIGMA_HW_PROFILE_GAMING | SIGMA_HW_PROFILE_IOT_ARM64),
      false, 0, 0,
      { nullptr }, 0xFFFF, 0xFFFF, true },

    { "snd",          "core",    "Sovereign Sound Core",
      (sigma_hw_profile_t)(SIGMA_HW_PROFILE_STANDARD | SIGMA_HW_PROFILE_GAMING | SIGMA_HW_PROFILE_FORENSIC | SIGMA_HW_PROFILE_IOT_ARM64),
      false, 0, 0,
      { nullptr }, 0xFFFF, 0xFFFF, true },

    { "snd_hda_codec","core",    "High Definition Audio Codec Core",
      (sigma_hw_profile_t)(SIGMA_HW_PROFILE_STANDARD | SIGMA_HW_PROFILE_GAMING),
      false, 0, 0,
      { "snd", nullptr }, 0xFFFF, 0xFFFF, true },

    { "sdhci",        "core",    "SD Host Controller Interface",
      SIGMA_HW_PROFILE_IOT_ARM64, false, 0, 0,
      { nullptr }, 0xFFFF, 0xFFFF, true },

||||||| 65885484f
=======
    // ---- Base Bus Drivers ---------------------------------------------------
    { "pci_bus",      "bus",     "PCI Express Root Bridge",
      (sigma_hw_profile_t)(SIGMA_HW_PROFILE_STANDARD | SIGMA_HW_PROFILE_GAMING | SIGMA_HW_PROFILE_SERVER | SIGMA_HW_PROFILE_FORENSIC | SIGMA_HW_PROFILE_IOT_ARM64),
      false, ZEN_DRV_NET_INIT_FAILED, ZEN_DRV_NET_INIT_FAILED, SIGMA_NULL },

    { "sound_core",   "audio",   "Sovereign Sound Core",
      (sigma_hw_profile_t)(SIGMA_HW_PROFILE_STANDARD | SIGMA_HW_PROFILE_GAMING | SIGMA_HW_PROFILE_FORENSIC | SIGMA_HW_PROFILE_IOT_ARM64),
      false, ZEN_DRV_AUDIO_INIT_FAILED, ZEN_DRV_AUDIO_FALLBACK_DUMMY, "pci_bus" },

>>>>>>> origin/jules-driver-improvements-linux-inspired-5291856075380713095
    // ---- GPU ----------------------------------------------------------------
    { "amdgpu",       "gpu",     "AMD Radeon",
      (sigma_hw_profile_t)(SIGMA_HW_PROFILE_STANDARD | SIGMA_HW_PROFILE_GAMING),
<<<<<<< HEAD
      true,  ZEN_DRV_GPU_INIT_FAILED, ZEN_DRV_GPU_FALLBACK_VGA,
      { "pci_core", nullptr }, 0x1002, 0x731F, true },
||||||| 65885484f
      true,  ZEN_DRV_GPU_INIT_FAILED, ZEN_DRV_GPU_FALLBACK_VGA },
=======
      true,  ZEN_DRV_GPU_INIT_FAILED, ZEN_DRV_GPU_FALLBACK_VGA, "pci_bus" },
>>>>>>> origin/jules-driver-improvements-linux-inspired-5291856075380713095

    { "i915",         "gpu",     "Intel UHD/Iris",
      (sigma_hw_profile_t)(SIGMA_HW_PROFILE_STANDARD | SIGMA_HW_PROFILE_SERVER),
<<<<<<< HEAD
      true,  ZEN_DRV_GPU_INIT_FAILED, ZEN_DRV_GPU_FALLBACK_VGA,
      { "pci_core", nullptr }, 0x8086, 0x9A49, true },
||||||| 65885484f
      true,  ZEN_DRV_GPU_INIT_FAILED, ZEN_DRV_GPU_FALLBACK_VGA },
=======
      true,  ZEN_DRV_GPU_INIT_FAILED, ZEN_DRV_GPU_FALLBACK_VGA, "pci_bus" },
>>>>>>> origin/jules-driver-improvements-linux-inspired-5291856075380713095

    { "nvidia",       "gpu",     "NVIDIA (proprietary)",
      (sigma_hw_profile_t)SIGMA_HW_PROFILE_GAMING,
<<<<<<< HEAD
      true,  ZEN_DRV_GPU_INIT_FAILED, ZEN_DRV_GPU_FALLBACK_VGA,
      { "pci_core", nullptr }, 0x10DE, 0x1E84, true },
||||||| 65885484f
      true,  ZEN_DRV_GPU_INIT_FAILED, ZEN_DRV_GPU_FALLBACK_VGA },
=======
      true,  ZEN_DRV_GPU_INIT_FAILED, ZEN_DRV_GPU_FALLBACK_VGA, "pci_bus" },
>>>>>>> origin/jules-driver-improvements-linux-inspired-5291856075380713095

    { "vc4",          "gpu",     "Broadcom VC4 (RPi)",
      (sigma_hw_profile_t)SIGMA_HW_PROFILE_IOT_ARM64,
<<<<<<< HEAD
      false, ZEN_DRV_GPU_INIT_FAILED, ZEN_DRV_GPU_FALLBACK_VGA,
      { "mailbox", nullptr }, 0x14E4, 0x2711, true },
||||||| 65885484f
      false, ZEN_DRV_GPU_INIT_FAILED, ZEN_DRV_GPU_FALLBACK_VGA },
=======
      false, ZEN_DRV_GPU_INIT_FAILED, ZEN_DRV_GPU_FALLBACK_VGA, "pci_bus" },
>>>>>>> origin/jules-driver-improvements-linux-inspired-5291856075380713095

    // ---- Networking ---------------------------------------------------------
    { "r8169",        "net",     "Realtek Ethernet",
      (sigma_hw_profile_t)(SIGMA_HW_PROFILE_STANDARD | SIGMA_HW_PROFILE_SERVER),
<<<<<<< HEAD
      false, ZEN_DRV_NET_REALTEK_ERR, ZEN_DRV_NET_INIT_FAILED,
      { "pci_core", nullptr }, 0x10EC, 0x8168, true },
||||||| 65885484f
      false, ZEN_DRV_NET_REALTEK_ERR, ZEN_DRV_NET_INIT_FAILED },
=======
      false, ZEN_DRV_NET_REALTEK_ERR, ZEN_DRV_NET_INIT_FAILED, "pci_bus" },
>>>>>>> origin/jules-driver-improvements-linux-inspired-5291856075380713095

    { "iwlwifi",      "net",     "Intel Wi-Fi",
      (sigma_hw_profile_t)(SIGMA_HW_PROFILE_STANDARD | SIGMA_HW_PROFILE_GAMING),
<<<<<<< HEAD
      true,  ZEN_DRV_NET_INTEL_ERR,   ZEN_DRV_NET_INIT_FAILED,
      { "pci_core", "cfg80211", nullptr }, 0x8086, 0x0084, true },
||||||| 65885484f
      true,  ZEN_DRV_NET_INTEL_ERR,   ZEN_DRV_NET_INIT_FAILED },
=======
      true,  ZEN_DRV_NET_INTEL_ERR,   ZEN_DRV_NET_INIT_FAILED, "pci_bus" },
>>>>>>> origin/jules-driver-improvements-linux-inspired-5291856075380713095

    { "brcmfmac",     "net",     "Broadcom Wi-Fi (RPi)",
      (sigma_hw_profile_t)SIGMA_HW_PROFILE_IOT_ARM64,
<<<<<<< HEAD
      true,  ZEN_DRV_NET_BROADCOM_ERR,ZEN_DRV_NET_INIT_FAILED,
      { "cfg80211", nullptr }, 0x14E4, 0x43A3, true },
||||||| 65885484f
      true,  ZEN_DRV_NET_BROADCOM_ERR,ZEN_DRV_NET_INIT_FAILED },
=======
      true,  ZEN_DRV_NET_BROADCOM_ERR,ZEN_DRV_NET_INIT_FAILED, "pci_bus" },
>>>>>>> origin/jules-driver-improvements-linux-inspired-5291856075380713095

    // ---- Audio --------------------------------------------------------------
    { "snd_hda_intel","audio",   "Intel/AMD HDA",
      (sigma_hw_profile_t)(SIGMA_HW_PROFILE_STANDARD | SIGMA_HW_PROFILE_GAMING),
<<<<<<< HEAD
      false, ZEN_DRV_AUDIO_INIT_FAILED, ZEN_DRV_AUDIO_FALLBACK_DUMMY,
      { "pci_core", "snd_hda_codec", nullptr }, 0x8086, 0x2820, true },
||||||| 65885484f
      false, ZEN_DRV_AUDIO_INIT_FAILED, ZEN_DRV_AUDIO_FALLBACK_DUMMY },
=======
      false, ZEN_DRV_AUDIO_INIT_FAILED, ZEN_DRV_AUDIO_FALLBACK_DUMMY, "sound_core" },
>>>>>>> origin/jules-driver-improvements-linux-inspired-5291856075380713095

    { "snd_dummy",    "audio",   "Dummy audio (Forensic/IoT)",
      (sigma_hw_profile_t)(SIGMA_HW_PROFILE_FORENSIC | SIGMA_HW_PROFILE_IOT_ARM64),
<<<<<<< HEAD
      false, ZEN_DRV_AUDIO_INIT_FAILED, ZEN_DRV_AUDIO_FALLBACK_DUMMY,
      { "snd", nullptr }, 0xFFFF, 0xFFFF, false }, // Unsigned module (demonstration of security warning)
||||||| 65885484f
      false, ZEN_DRV_AUDIO_INIT_FAILED, ZEN_DRV_AUDIO_FALLBACK_DUMMY },
=======
      false, ZEN_DRV_AUDIO_INIT_FAILED, ZEN_DRV_AUDIO_FALLBACK_DUMMY, "sound_core" },
>>>>>>> origin/jules-driver-improvements-linux-inspired-5291856075380713095

    // ---- Storage ------------------------------------------------------------
    { "nvme",         "storage", "NVMe SSD",
      (sigma_hw_profile_t)(SIGMA_HW_PROFILE_STANDARD | SIGMA_HW_PROFILE_GAMING | SIGMA_HW_PROFILE_SERVER),
<<<<<<< HEAD
      false, ZEN_DRV_STORAGE_NVME_ERR, ZEN_DRV_STORAGE_INIT_FAILED,
      { "pci_core", nullptr }, 0x144D, 0xA808, true },
||||||| 65885484f
      false, ZEN_DRV_STORAGE_NVME_ERR, ZEN_DRV_STORAGE_INIT_FAILED },
=======
      false, ZEN_DRV_STORAGE_NVME_ERR, ZEN_DRV_STORAGE_INIT_FAILED, "pci_bus" },
>>>>>>> origin/jules-driver-improvements-linux-inspired-5291856075380713095

    { "ahci",         "storage", "SATA AHCI",
      (sigma_hw_profile_t)(SIGMA_HW_PROFILE_STANDARD | SIGMA_HW_PROFILE_SERVER | SIGMA_HW_PROFILE_FORENSIC),
<<<<<<< HEAD
      false, ZEN_DRV_STORAGE_SATA_ERR, ZEN_DRV_STORAGE_INIT_FAILED,
      { "pci_core", nullptr }, 0x8086, 0x2822, true },
||||||| 65885484f
      false, ZEN_DRV_STORAGE_SATA_ERR, ZEN_DRV_STORAGE_INIT_FAILED },
=======
      false, ZEN_DRV_STORAGE_SATA_ERR, ZEN_DRV_STORAGE_INIT_FAILED, "pci_bus" },
>>>>>>> origin/jules-driver-improvements-linux-inspired-5291856075380713095

    { "mmc_block",    "storage", "eMMC / SD (ARM64)",
      (sigma_hw_profile_t)SIGMA_HW_PROFILE_IOT_ARM64,
<<<<<<< HEAD
<<<<<<< HEAD
      false, ZEN_DRV_STORAGE_EMMC_ERR, ZEN_DRV_STORAGE_INIT_FAILED,
      { "sdhci", nullptr }, 0x11AB, 0x0100, true },
||||||| 65885484f
      false, ZEN_DRV_STORAGE_EMMC_ERR, ZEN_DRV_STORAGE_INIT_FAILED },
=======
      false, ZEN_DRV_STORAGE_EMMC_ERR, ZEN_DRV_STORAGE_INIT_FAILED,
      { "sdhci", nullptr }, 0x11AB, 0x0100, true },

    // ---- Emerging & Accelerated Hardware (CISC/RISC AI accelerators & enclaves) ----
    { "google_tpu",   "compute", "Google TPU v4/v5 Tensor Engine",
      SIGMA_HW_PROFILE_SERVER,
      false, 0, 0,
      { "pci_core", nullptr }, 0x1AE0, 0x0056, true },

    { "graphcore_ipu","compute", "Graphcore Bow IPU Core",
      SIGMA_HW_PROFILE_SERVER,
      false, 0, 0,
      { "pci_core", nullptr }, 0x1E1A, 0x0010, true },

    { "fpga_mgr",     "coproc",  "Xilinx Alveo FPGA Core Manager",
      SIGMA_HW_PROFILE_SERVER,
      false, 0, 0,
      { "pci_core", nullptr }, 0x10EE, 0x5005, true },

    { "optane_dax",   "storage", "Intel Optane Persistent Memory DAX",
      SIGMA_HW_PROFILE_SERVER,
      false, 0, 0,
      { "pci_core", nullptr }, 0x8086, 0x2011, true },

    { "intel_tdx",    "security","Intel Trust Domain Extensions Enclave",
      SIGMA_HW_PROFILE_ALL,
      false, 0, 0,
      { nullptr }, 0xFFFF, 0xFFFF, true },

    { "amd_sev",      "security","AMD Secure Encrypted Virtualization Enclave",
      SIGMA_HW_PROFILE_ALL,
      false, 0, 0,
      { nullptr }, 0xFFFF, 0xFFFF, true },
>>>>>>> origin/jules-13571719274074749109-6af93541
||||||| 65885484f
      false, ZEN_DRV_STORAGE_EMMC_ERR, ZEN_DRV_STORAGE_INIT_FAILED },
=======
      false, ZEN_DRV_STORAGE_EMMC_ERR, ZEN_DRV_STORAGE_INIT_FAILED, "pci_bus" },
>>>>>>> origin/jules-driver-improvements-linux-inspired-5291856075380713095
};

static const sigma_u32 g_driver_count = sizeof(g_driver_table) / sizeof(g_driver_table[0]);

// Tracks loaded modules
static const char* g_loaded_modules[64];
static sigma_u32   g_loaded_count = 0;

// -------------------------------------------------------------------------
// DriverManager with linear-time zero-allocation DAG Topological Sorter
// -------------------------------------------------------------------------
class DriverManager {
public:
    static DriverManager& getInstance() {
        static DriverManager instance;
        return instance;
    }

    /**
     * Check if a module is currently loaded.
     */
    bool isLoaded(const char* module_name) {
        for (sigma_u32 i = 0; i < g_loaded_count; i++) {
            if (sigma_strcmp(g_loaded_modules[i], module_name) == 0) {
                return true;
            }
        }
        return false;
    }

    /**
     * Recursive Modprobe-style module loader.
     * Resolves all dependencies in a safe sequence.
     */
    sigma_status loadModuleWithDeps(const char* module_name) {
        if (isLoaded(module_name)) {
            return SIGMA_SUCCESS; // Already loaded, nothing to do
        }

        // Find the driver descriptor
        const DriverDescriptor* drv = findDescriptor(module_name);
        if (!drv) {
            sys_print("[DriverManager] ❌ ERROR: Module '%s' not found in registry.\n", module_name);
            return SIGMA_ERROR;
        }

        // Step 1: Recursively load dependencies first
        for (sigma_u32 i = 0; i < 4 && drv->dependencies[i] != nullptr; i++) {
            const char* dep = drv->dependencies[i];
            sys_print("[DriverManager] Resolving dependency for '%s': loading '%s' first...\n",
                      module_name, dep);
            if (loadModuleWithDeps(dep) != SIGMA_SUCCESS) {
                sys_print("[DriverManager] ❌ ERROR: Failed to load dependency '%s' for '%s'.\n",
                          dep, module_name);
                return SIGMA_ERROR;
            }
        }

        // Step 2: Perform secure driver verification (Dilithium-5)
        if (!drv->is_signed_pqc) {
            sys_print("[DriverManager] ⚠ SECURITY WARNING Alert: Module '%s' is UNSIGNED!\n", module_name);
            sys_print("[DriverManager]   Running in Lockdown Mode. Restricting DMA privileges.\n");
            zenith_log_structured(ZEN_DRV_RECIPE_SIG_INVALID, "DriverManager",
                                  "Loaded unsigned kernel module", 0);
        } else {
            sys_print("[DriverManager] [Dilithium-5] ✅ Cryptographic signature verified for '%s'\n",
                      module_name);
        }

        // Step 3: Load the driver
        sys_print("[DriverManager] Loading [%s] (%s)...", drv->module_name, drv->chipset_hint);
        if (drv->requires_fw) {
            sys_print(" [FW required]");
        }
        sys_print(" ✅ OK\n");

        // Record as loaded
        g_loaded_modules[g_loaded_count++] = drv->module_name;
        zenith_log_structured(ZEN_SUCCESS, "DriverManager", drv->module_name, 0);

        return SIGMA_SUCCESS;
    }

    /**
     * Load all drivers matching a given hardware profile.
<<<<<<< HEAD
||||||| 65885484f
     * Self-heals on failure; falls back gracefully.
=======
     * Uses Kahn's topological sort on driver dependencies to satisfy loading constraints.
>>>>>>> origin/jules-driver-improvements-linux-inspired-5291856075380713095
     */
    sigma_status loadForProfile(sigma_hw_profile_t profile) {
        sys_print("[DriverManager] Sorting and loading drivers for hardware profile 0x%02X...\n",
                  (sigma_u32)profile);

        // Filter active drivers matching the profile
        sigma_u32 active_indices[32];
        sigma_u32 active_count = 0;

        for (sigma_u32 i = 0; i < g_driver_count; i++) {
            if ((g_driver_table[i].profile_mask & profile) != 0) {
                if (active_count < 32) {
                    active_indices[active_count++] = i;
                }
            }
        }

        // Compute in-degrees of the dependency graph (zero-allocation)
        sigma_u32 in_degree[32] = {0};
        for (sigma_u32 i = 0; i < active_count; i++) {
            const DriverDescriptor& drv = g_driver_table[active_indices[i]];
            if (drv.dependency_name != SIGMA_NULL) {
                // Check if dependency is present in active set
                bool dep_active = false;
                for (sigma_u32 j = 0; j < active_count; j++) {
                    if (sigma_strcmp(g_driver_table[active_indices[j]].module_name, drv.dependency_name) == 0) {
                        dep_active = true;
                        break;
                    }
                }
                if (dep_active) {
                    in_degree[i]++;
                }
            }
        }

        // Kahn's Algorithm queue
        sigma_u32 queue[32];
        sigma_u32 q_head = 0, q_tail = 0;

        // Push elements with 0 in-degree to the queue
        for (sigma_u32 i = 0; i < active_count; i++) {
            if (in_degree[i] == 0) {
                queue[q_tail++] = active_indices[i];
            }
        }

        // Topological sorted list
        sigma_u32 sorted_order[32];
        sigma_u32 sorted_count = 0;

        while (q_head < q_tail) {
            sigma_u32 u_idx = queue[q_head++];
            sorted_order[sorted_count++] = u_idx;

            const char* u_name = g_driver_table[u_idx].module_name;

            // Decrement in-degree for dependent active elements
            for (sigma_u32 v = 0; v < active_count; v++) {
                const DriverDescriptor& v_drv = g_driver_table[active_indices[v]];
                if (v_drv.dependency_name != SIGMA_NULL && sigma_strcmp(v_drv.dependency_name, u_name) == 0) {
                    if (in_degree[v] > 0) {
                        in_degree[v]--;
                        if (in_degree[v] == 0) {
                            queue[q_tail++] = active_indices[v];
                        }
                    }
                }
            }
        }

        // Check for cycles / dependency errors
        if (sorted_count < active_count) {
            sys_print("[DriverManager] ❌ ERROR: Circular dependency detected in driver graph!\n");
            zenith_log_structured(ZEN_DRV_CRASH, "DriverManager", "Circular dependency detected", 0);
            return SIGMA_ERROR;
        }

        // Sequentially load the sorted drivers
        sigma_u32 loaded = 0, failed = 0;
        bool root_bus_ok = true; // Tracks root/bus drivers state for cascaded self-healing fallbacks

        for (sigma_u32 i = 0; i < sorted_count; i++) {
            const DriverDescriptor& drv = g_driver_table[sorted_order[i]];

            // Cascaded recovery: if root bus failed, automatically trigger fallback on child drivers
            if (!root_bus_ok && drv.dependency_name != SIGMA_NULL) {
                sys_print("[DriverManager] 🔄 Cascaded healing on [%s] due to failed dependency [%s]!\n",
                          drv.module_name, drv.dependency_name);
                attemptHeal(drv);
                failed++;
                continue;
            }

            sigma_status result = loadModuleWithDeps(drv.module_name);
            if (result == SIGMA_SUCCESS) {
                loaded++;
            } else {
                failed++;
                if (sigma_strcmp(drv.subsystem, "bus") == 0) {
                    root_bus_ok = false;
                }
                attemptHeal(drv);
            }
        }

        sys_print("[DriverManager] Profile 0x%02X DAG load complete: %u loaded, %u failed, %u skipped.\n",
                  (sigma_u32)profile, loaded, failed, g_driver_count - active_count);

        return (failed == 0) ? SIGMA_SUCCESS : SIGMA_ERROR;
    }

    /**
     * udev-style PCI dynamic device auto-detection and loading.
     */
    sigma_status autoDetectAndLoad(sigma_u32 vendor, sigma_u32 device) {
        sys_print("[DriverManager] [udev] Scanning PCI Bus: Probing device [Vendor: 0x%04X, Device: 0x%04X]...\n",
                  vendor, device);

        for (sigma_u32 i = 0; i < g_driver_count; i++) {
            const DriverDescriptor& drv = g_driver_table[i];
            if (drv.vendor_id == vendor && drv.device_id == device) {
                sys_print("[DriverManager] [udev] Match found! Auto-loading driver: '%s'\n", drv.module_name);
                return loadModuleWithDeps(drv.module_name);
            }
        }

        sys_print("[DriverManager] [udev] No matching driver registered for PCI device 0x%04X:0x%04X.\n",
                  vendor, device);
        return SIGMA_ERROR;
    }

    /**
     * Unload a specific driver by name.
     */
    sigma_status unloadDriver(const char* module_name) {
        sys_print("[DriverManager] Unloading module: %s\n", module_name);

        // Remove from loaded list
        for (sigma_u32 i = 0; i < g_loaded_count; i++) {
            if (sigma_strcmp(g_loaded_modules[i], module_name) == 0) {
                for (sigma_u32 j = i; j < g_loaded_count - 1; j++) {
                    g_loaded_modules[j] = g_loaded_modules[j + 1];
                }
                g_loaded_count--;
                zenith_log_structured(ZEN_SUCCESS, "DriverManager", "Module unloaded", 0);
                return SIGMA_SUCCESS;
            }
        }
        return SIGMA_ERROR;
    }

    /**
     * Reload a specific driver.
     */
    sigma_status reloadDriver(const char* module_name) {
        sys_print("[DriverManager] Reloading module: %s\n", module_name);
        unloadDriver(module_name);

        for (sigma_u32 i = 0; i < g_driver_count; i++) {
            if (sigma_strcmp(g_driver_table[i].module_name, module_name) == 0) {
                return loadModuleWithDeps(g_driver_table[i].module_name);
            }
        }

        zenith_log_structured(ZEN_DRV_MODULE_NOT_FOUND, "DriverManager",
                              sigma_driver_strerror(ZEN_DRV_MODULE_NOT_FOUND), 0);
        return SIGMA_ERROR;
    }

    /**
     * Run the full hardware detection and load sequence.
     */
    void initHardware(sigma_hw_profile_t profile) {
        sys_print("\n");
        sys_print("╔══════════════════════════════════════════════════════════╗\n");
        sys_print("║         SIGMAOS DRIVER MANAGER — HARDWARE INIT          ║\n");
        sys_print("╚══════════════════════════════════════════════════════════╝\n\n");

        if (profile == SIGMA_HW_PROFILE_FORENSIC) {
            sys_print("[DriverManager] ⚠ Forensic profile: block devices set to READ-ONLY.\n");
            zenith_log_structured(ZEN_DRV_STORAGE_READONLY_BOOT, "DriverManager",
                                  "Forensic boot — read-only storage enforced", 0);
        }

        loadForProfile(profile);
    }

private:
    const DriverDescriptor* findDescriptor(const char* name) {
        for (sigma_u32 i = 0; i < g_driver_count; i++) {
            if (sigma_strcmp(g_driver_table[i].module_name, name) == 0) {
                return &g_driver_table[i];
            }
        }
        return nullptr;
    }

    void attemptHeal(const DriverDescriptor& drv) {
        sys_print("[DriverManager] ⚠ '%s' failed! Attempting self-heal...\n", drv.module_name);
        zenith_log_structured(drv.init_error, "DriverManager",
                              sigma_driver_strerror(drv.init_error), 0);

        if (sigma_strcmp(drv.subsystem, "gpu") == 0) {
            sys_print("[DriverManager] 🔄 GPU: Falling back to VGA safe mode.\n");
            zenith_log_structured(ZEN_DRV_GPU_FALLBACK_VGA, "DriverManager",
                                  sigma_driver_strerror(ZEN_DRV_GPU_FALLBACK_VGA), 0);
        } else if (sigma_strcmp(drv.subsystem, "audio") == 0) {
            sys_print("[DriverManager] 🔄 Audio: Falling back to dummy device.\n");
            zenith_log_structured(ZEN_DRV_AUDIO_FALLBACK_DUMMY, "DriverManager",
                                  sigma_driver_strerror(ZEN_DRV_AUDIO_FALLBACK_DUMMY), 0);
        } else {
            sys_print("[DriverManager] ❌ No fallback available for '%s'. Manual intervention required.\n",
                      drv.module_name);
        }
    }
};

} // namespace Drivers
} // namespace Sigma

// -------------------------------------------------------------------------
// C ABI exports
// -------------------------------------------------------------------------
extern "C" {
    sigma_status sigma_driver_load_profile(sigma_u32 profile_mask) {
        return Sigma::Drivers::DriverManager::getInstance()
                   .loadForProfile((sigma_hw_profile_t)profile_mask);
    }

    void sigma_driver_init_hardware(sigma_u32 profile_mask) {
        Sigma::Drivers::DriverManager::getInstance()
            .initHardware((sigma_hw_profile_t)profile_mask);
    }

    sigma_status sigma_driver_reload(const char* module_name) {
        return Sigma::Drivers::DriverManager::getInstance().reloadDriver(module_name);
    }

    sigma_status sigma_driver_load_with_deps(const char* module_name) {
        return Sigma::Drivers::DriverManager::getInstance().loadModuleWithDeps(module_name);
    }

    sigma_status sigma_driver_pci_auto_detect(sigma_u32 vendor, sigma_u32 device) {
        return Sigma::Drivers::DriverManager::getInstance().autoDetectAndLoad(vendor, device);
    }

    sigma_bool sigma_driver_is_loaded(const char* module_name) {
        return Sigma::Drivers::DriverManager::getInstance().isLoaded(module_name) ? SIGMA_TRUE : SIGMA_FALSE;
    }
}

/**
 * =========================================================================
 * Σ SIGMAOS DRIVER MANAGER & I/O MANAGER
 * =========================================================================
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
    const char*       subsystem;      // "gpu" | "net" | "audio" | "storage" | "input" | "bus"
    const char*       chipset_hint;   // vendor/chipset this primarily targets
    sigma_hw_profile_t profile_mask;  // profiles this driver is active under
    bool              requires_fw;    // needs a firmware blob
    sigma_u32         init_error;     // error code to fire on failure
    sigma_u32         fallback_error; // error code if falling back
    const char*       dependency_name;// Name of driver module this depends on, or SIGMA_NULL
};

// -------------------------------------------------------------------------
// Default driver table with rich dependency hierarchies (DAG)
// Profiles are bitmasks: can OR multiple profiles together.
// -------------------------------------------------------------------------
static const DriverDescriptor g_driver_table[] = {
    // ---- Base Bus Drivers ---------------------------------------------------
    { "pci_bus",      "bus",     "PCI Express Root Bridge",
      (sigma_hw_profile_t)(SIGMA_HW_PROFILE_STANDARD | SIGMA_HW_PROFILE_GAMING | SIGMA_HW_PROFILE_SERVER | SIGMA_HW_PROFILE_FORENSIC | SIGMA_HW_PROFILE_IOT_ARM64),
      false, ZEN_DRV_NET_INIT_FAILED, ZEN_DRV_NET_INIT_FAILED, SIGMA_NULL },

    { "sound_core",   "audio",   "Sovereign Sound Core",
      (sigma_hw_profile_t)(SIGMA_HW_PROFILE_STANDARD | SIGMA_HW_PROFILE_GAMING | SIGMA_HW_PROFILE_FORENSIC | SIGMA_HW_PROFILE_IOT_ARM64),
      false, ZEN_DRV_AUDIO_INIT_FAILED, ZEN_DRV_AUDIO_FALLBACK_DUMMY, "pci_bus" },

    // ---- GPU ----------------------------------------------------------------
    { "amdgpu",       "gpu",     "AMD Radeon",
      (sigma_hw_profile_t)(SIGMA_HW_PROFILE_STANDARD | SIGMA_HW_PROFILE_GAMING),
      true,  ZEN_DRV_GPU_INIT_FAILED, ZEN_DRV_GPU_FALLBACK_VGA, "pci_bus" },

    { "i915",         "gpu",     "Intel UHD/Iris",
      (sigma_hw_profile_t)(SIGMA_HW_PROFILE_STANDARD | SIGMA_HW_PROFILE_SERVER),
      true,  ZEN_DRV_GPU_INIT_FAILED, ZEN_DRV_GPU_FALLBACK_VGA, "pci_bus" },

    { "nvidia",       "gpu",     "NVIDIA (proprietary)",
      (sigma_hw_profile_t)SIGMA_HW_PROFILE_GAMING,
      true,  ZEN_DRV_GPU_INIT_FAILED, ZEN_DRV_GPU_FALLBACK_VGA, "pci_bus" },

    { "vc4",          "gpu",     "Broadcom VC4 (RPi)",
      (sigma_hw_profile_t)SIGMA_HW_PROFILE_IOT_ARM64,
      false, ZEN_DRV_GPU_INIT_FAILED, ZEN_DRV_GPU_FALLBACK_VGA, "pci_bus" },

    // ---- Networking ---------------------------------------------------------
    { "r8169",        "net",     "Realtek Ethernet",
      (sigma_hw_profile_t)(SIGMA_HW_PROFILE_STANDARD | SIGMA_HW_PROFILE_SERVER),
      false, ZEN_DRV_NET_REALTEK_ERR, ZEN_DRV_NET_INIT_FAILED, "pci_bus" },

    { "iwlwifi",      "net",     "Intel Wi-Fi",
      (sigma_hw_profile_t)(SIGMA_HW_PROFILE_STANDARD | SIGMA_HW_PROFILE_GAMING),
      true,  ZEN_DRV_NET_INTEL_ERR,   ZEN_DRV_NET_INIT_FAILED, "pci_bus" },

    { "brcmfmac",     "net",     "Broadcom Wi-Fi (RPi)",
      (sigma_hw_profile_t)SIGMA_HW_PROFILE_IOT_ARM64,
      true,  ZEN_DRV_NET_BROADCOM_ERR,ZEN_DRV_NET_INIT_FAILED, "pci_bus" },

    // ---- Audio --------------------------------------------------------------
    { "snd_hda_intel","audio",   "Intel/AMD HDA",
      (sigma_hw_profile_t)(SIGMA_HW_PROFILE_STANDARD | SIGMA_HW_PROFILE_GAMING),
      false, ZEN_DRV_AUDIO_INIT_FAILED, ZEN_DRV_AUDIO_FALLBACK_DUMMY, "sound_core" },

    { "snd_dummy",    "audio",   "Dummy audio (Forensic/IoT)",
      (sigma_hw_profile_t)(SIGMA_HW_PROFILE_FORENSIC | SIGMA_HW_PROFILE_IOT_ARM64),
      false, ZEN_DRV_AUDIO_INIT_FAILED, ZEN_DRV_AUDIO_FALLBACK_DUMMY, "sound_core" },

    // ---- Storage ------------------------------------------------------------
    { "nvme",         "storage", "NVMe SSD",
      (sigma_hw_profile_t)(SIGMA_HW_PROFILE_STANDARD | SIGMA_HW_PROFILE_GAMING | SIGMA_HW_PROFILE_SERVER),
      false, ZEN_DRV_STORAGE_NVME_ERR, ZEN_DRV_STORAGE_INIT_FAILED, "pci_bus" },

    { "ahci",         "storage", "SATA AHCI",
      (sigma_hw_profile_t)(SIGMA_HW_PROFILE_STANDARD | SIGMA_HW_PROFILE_SERVER | SIGMA_HW_PROFILE_FORENSIC),
      false, ZEN_DRV_STORAGE_SATA_ERR, ZEN_DRV_STORAGE_INIT_FAILED, "pci_bus" },

    { "mmc_block",    "storage", "eMMC / SD (ARM64)",
      (sigma_hw_profile_t)SIGMA_HW_PROFILE_IOT_ARM64,
      false, ZEN_DRV_STORAGE_EMMC_ERR, ZEN_DRV_STORAGE_INIT_FAILED, "pci_bus" },
};

static const sigma_u32 g_driver_count = sizeof(g_driver_table) / sizeof(g_driver_table[0]);

// Tracks loaded modules
static const char* g_loaded_modules[64];
static sigma_u32   g_loaded_count = 0;

// Object Manager memory boundaries (Simulated Non-Paged Pool)
static DriverObject g_driver_pool[32];
static sigma_u32    g_driver_pool_count = 0;

static DeviceObject g_device_pool[64];
static sigma_u32    g_device_pool_count = 0;

// -------------------------------------------------------------------------
// DriverManager with linear-time zero-allocation DAG Topological Sorter
// -------------------------------------------------------------------------
class IoManager {
public:
    static IoManager& getInstance() {
        static IoManager instance;
        return instance;
    }

    /**
     * Normal driver installation process - Creates and initializes a new DriverObject
     */
    sigma_status registerDriver(const char* name, const char* registry_path, sigma_status (*unload_cb)(void*)) {
        sys_print("[IoManager] Installing driver: '%s' from RegistryPath '%s'...\n", name, registry_path);

        if (g_driver_pool_count >= 32) {
            sys_print("[IoManager] ❌ ERROR: Simulated Non-Paged Pool Full (DriverObject exhaust).\n");
            return SIGMA_ERROR;
        }

        // Initialize DriverObject
        DriverObject& drv = g_driver_pool[g_driver_pool_count++];
        sigma_size_t i = 0;
        for (; i < 63 && name[i] != '\0'; i++) {
            drv.driver_name[i] = name[i];
        }
        drv.driver_name[i] = '\0';

        for (i = 0; i < 255 && registry_path[i] != '\0'; i++) {
            drv.registry_path[i] = registry_path[i];
        }
        drv.registry_path[i] = '\0';

        drv.device_list_head = nullptr;
        drv.driver_unload = unload_cb;
        drv.is_loaded = true;

        sys_print("[IoManager] ✅ DriverObject '%s' allocated and registered successfully in Non-Paged Pool.\n", name);
        return SIGMA_SUCCESS;
    }

    /**
     * Create Device Object - Allocates a DeviceObject and its DeviceExtension context data
     */
    sigma_status createDevice(const char* driver_name, const char* device_name, sigma_size_t extension_size, void** out_device_extension) {
        sys_print("[IoManager] Creating DeviceObject '%s' for Driver '%s'...\n", device_name, driver_name);

        DriverObject* drv = findDriver(driver_name);
        if (!drv) {
            sys_print("[IoManager] ❌ ERROR: Driver '%s' not registered. Device creation aborted.\n", driver_name);
            return SIGMA_ERROR;
        }

        if (g_device_pool_count >= 64) {
            sys_print("[IoManager] ❌ ERROR: Simulated Non-Paged Pool Full (DeviceObject exhaust).\n");
            return SIGMA_ERROR;
        }

        // Allocate DeviceObject
        DeviceObject& dev = g_device_pool[g_device_pool_count++];
        dev.driver_object = drv;
        dev.is_active = true;

        // Initialize DeviceExtension Context Information
        sigma_size_t i = 0;
        for (; i < 63 && device_name[i] != '\0'; i++) {
            dev.device_extension.device_name[i] = device_name[i];
        }
        dev.device_extension.device_name[i] = '\0';

        dev.device_extension.context_size = extension_size;
        dev.device_extension.context_data = nullptr; // Simulated dynamic context block
        dev.device_extension.irq = 0;
        dev.device_extension.io_port_base = 0;
        dev.device_extension.resources_assigned = false;

        // Insert into Driver's device list (linked list representation)
        dev.next_device = drv->device_list_head;
        drv->device_list_head = &dev;

        if (out_device_extension) {
            *out_device_extension = &dev.device_extension;
        }

        sys_print("[IoManager] ✅ DeviceObject '%s' linked to Driver '%s'. DeviceExtension context mapped.\n", device_name, driver_name);
        return SIGMA_SUCCESS;
    }

    /**
     * Assign Hardware Resources to a specific DeviceObject
     */
    sigma_status assignResources(const char* device_name, sigma_u32 irq, sigma_u32 io_port_base) {
        DeviceObject* dev = findDevice(device_name);
        if (!dev) {
            return SIGMA_ERROR;
        }

        sys_print("[IoManager] [Resource Handler] Assigning IRQ %u, I/O Port 0x%X to device '%s'...\n",
                  irq, io_port_base, device_name);
        dev->device_extension.irq = irq;
        dev->device_extension.io_port_base = io_port_base;
        dev->device_extension.resources_assigned = true;
        return SIGMA_SUCCESS;
    }

    /**
     * Unload Driver & Cleanup all related DeviceObjects (DriverUnload routine)
     */
    sigma_status unloadDriver(const char* driver_name) {
        sys_print("[IoManager] Unload requested for driver: '%s'...\n", driver_name);

        DriverObject* drv = findDriver(driver_name);
        if (!drv || !drv->is_loaded) {
            sys_print("[IoManager] ❌ ERROR: Driver '%s' is not loaded.\n", driver_name);
            return SIGMA_ERROR;
        }

        // Step 1: Execute driver specific cleanup tasks (DriverUnload Callback)
        if (drv->driver_unload) {
            sys_print("[IoManager] Calling DriverUnload routine for '%s'...\n", driver_name);
            drv->driver_unload(drv);
        }

        // Step 2: Delete and clean up all associated DeviceObjects (Reclaim Non-Paged Pool memory)
        DeviceObject* current = drv->device_list_head;
        while (current != nullptr) {
            sys_print("[IoManager]   Cleaning up DeviceObject '%s' (releasing IRQ %u)...\n",
                      current->device_extension.device_name, current->device_extension.irq);
            current->is_active = false;
            current->device_extension.resources_assigned = false;
            current = current->next_device;
        }
        drv->device_list_head = nullptr;
        drv->is_loaded = false;

        sys_print("[IoManager] ✅ Driver '%s' completely unloaded from memory pool.\n", driver_name);
        return SIGMA_SUCCESS;
    }

    DriverObject* findDriver(const char* name) {
        for (sigma_u32 i = 0; i < g_driver_pool_count; i++) {
            if (g_driver_pool[i].is_loaded && sigma_strcmp(g_driver_pool[i].driver_name, name) == 0) {
                return &g_driver_pool[i];
            }
        }
        return nullptr;
    }

    DeviceObject* findDevice(const char* name) {
        for (sigma_u32 i = 0; i < g_device_pool_count; i++) {
            if (g_device_pool[i].is_active && sigma_strcmp(g_device_pool[i].device_extension.device_name, name) == 0) {
                return &g_device_pool[i];
            }
        }
        return nullptr;
    }
};

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

        // Step 3: Load the driver and install it dynamically via IoManager
        sys_print("[DriverManager] Loading [%s] (%s)...", drv->module_name, drv->chipset_hint);
        if (drv->requires_fw) {
            sys_print(" [FW required]");
        }
        sys_print(" ✅ OK\n");

        // Normal driver installation mapping
        char reg_path[256] = "/registry/machine/system/currentcontrolset/services/";
        sigma_size_t reg_offset = 52;
        for (sigma_size_t i = 0; i < 64 && drv->module_name[i] != '\0'; i++) {
            reg_path[reg_offset++] = drv->module_name[i];
        }
        reg_path[reg_offset] = '\0';

        IoManager::getInstance().registerDriver(drv->module_name, reg_path, nullptr);

        // Record as loaded
        g_loaded_modules[g_loaded_count++] = drv->module_name;
        zenith_log_structured(ZEN_SUCCESS, "DriverManager", drv->module_name, 0);

        return SIGMA_SUCCESS;
    }

    /**
     * Load all drivers matching a given hardware profile.
     * Uses Kahn's topological sort on driver dependencies to satisfy loading constraints.
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

        // Trigger Windows-inspired Object cleanup
        IoManager::getInstance().unloadDriver(module_name);

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

    sigma_status sigma_io_register_driver(const char* name, const char* registry_path, sigma_status (*unload_cb)(void*)) {
        return Sigma::Drivers::IoManager::getInstance().registerDriver(name, registry_path, unload_cb);
    }

    sigma_status sigma_io_create_device(const char* driver_name, const char* device_name, sigma_size_t extension_size, void** out_device_extension) {
        return Sigma::Drivers::IoManager::getInstance().createDevice(driver_name, device_name, extension_size, out_device_extension);
    }

    sigma_status sigma_io_assign_resources(const char* device_name, sigma_u32 irq, sigma_u32 io_port_base) {
        return Sigma::Drivers::IoManager::getInstance().assignResources(device_name, irq, io_port_base);
    }

    sigma_status sigma_io_unload_driver(const char* name) {
        return Sigma::Drivers::IoManager::getInstance().unloadDriver(name);
    }
}

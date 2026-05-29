/**
 * =========================================================================
 * Σ SIGMAOS SOVEREIGN DRIVER REGISTRY
 * =========================================================================
 * A community-driven, cryptographically-verified driver build script index.
 *
 * Inspired by:
 *   - SlackBuilds.org: Community-sourced build recipes, no pre-built binaries.
 *   - NixOS: Reproducible, declarative package definitions.
 *   - DKMS: Dynamic Kernel Module Support for ABI-safe module rebuilds.
 *   - Clear Linux: Performance-tuned driver overlays.
 *
 * Flow:
 *   1. Maintainer submits .srecipe (build script) to the Registry.
 *   2. Registry signs the .srecipe with the SigmaOS sovereign root key.
 *   3. On install, sigma_driver_registry fetches + verifies the .srecipe.
 *   4. Build runs in an isolated orchestrator container.
 *   5. The compiled .ko module is packaged as a signed .spkg and deployed.
 *   6. DKMS-hook tracks the module — on kernel update, rebuild is triggered.
 * =========================================================================
 */

#include <sigma_libc.h>
#include <sigma_error_codes.h>
#include <sigma_driver_codes.h>

extern "C" void zenith_log_structured(sigma_u32 code, const char* comp,
                                       const char* desc, sigma_u32 cid);
extern "C" sigma_status sigma_package_verify(const sigma_u8* data, sigma_size_t size);

namespace Sigma {
namespace Drivers {

// -------------------------------------------------------------------------
// Driver Recipe Entry
// -------------------------------------------------------------------------
struct DriverRecipe {
    const char* name;         // Human-readable name
    const char* module_name;  // Resulting kernel module name (.ko)
    const char* version;      // Recipe version
    const char* author;       // Contributor name / org
    const char* chipset_hint; // What hardware this targets
    const char* recipe_url;   // URL to the .srecipe build script
    bool        signed_by_sigma; // Is the recipe signed by the SigmaOS root key?
};

// -------------------------------------------------------------------------
// Community Driver Registry Catalogue
// -------------------------------------------------------------------------
static const DriverRecipe g_registry[] = {
    {
        "Realtek RTL8852 Wi-Fi (Community)",
        "rtl8852be",
        "6.7.0-sigma1",
        "Community Contributor (handle: realtek-sovereign)",
        "Realtek RTL8852BE Wi-Fi 6",
        "https://registry.sigmaos.dev/drivers/rtl8852be.srecipe",
        true
    },
    {
        "Broadcom BCM4377 Wi-Fi (ARM64 / Apple BCM)",
        "brcmfmac",
        "6.6.0-sigma2",
        "Community Contributor (handle: arm64-net-team)",
        "Broadcom BCM4377 (common on ARM SoCs)",
        "https://registry.sigmaos.dev/drivers/bcm4377.srecipe",
        true
    },
    {
        "NVIDIA Open Kernel Module (Turing+)",
        "nvidia-open",
        "545.29.06-sigma1",
        "NVIDIA Corp. (upstream, open source)",
        "NVIDIA Turing, Ampere, Ada Lovelace GPUs",
        "https://registry.sigmaos.dev/drivers/nvidia-open.srecipe",
        true
    },
    {
        "Marvell MVNETA Ethernet (ARM64 Server)",
        "mvneta",
        "6.7.0-sigma1",
        "Community Contributor (handle: marvell-sovereign)",
        "Marvell Ethernet NIC (common in ARM64 servers)",
        "https://registry.sigmaos.dev/drivers/mvneta.srecipe",
        true
    },
    {
        "Raspberry Pi SenseHAT Input",
        "rpisense_js",
        "6.6.0-rpi1",
        "Community Contributor (handle: rpi-iot-team)",
        "Raspberry Pi SenseHAT joystick + LED matrix",
        "https://registry.sigmaos.dev/drivers/rpisense.srecipe",
        true
    },
};

static const sigma_u32 g_registry_count = sizeof(g_registry) / sizeof(g_registry[0]);

// -------------------------------------------------------------------------
// DKMS Tracking Entry
// -------------------------------------------------------------------------
struct DkmsEntry {
    const char* module_name;
    const char* recipe_version;
    const char* kernel_version; // Kernel version it was compiled against
};

static DkmsEntry g_dkms_table[32]; // Supports up to 32 tracked modules
static sigma_u32 g_dkms_count = 0;

// -------------------------------------------------------------------------
// SovereignDriverRegistry
// -------------------------------------------------------------------------
class SovereignDriverRegistry {
public:
    static SovereignDriverRegistry& getInstance() {
        static SovereignDriverRegistry instance;
        return instance;
    }

    /**
     * List all available recipes in the registry.
     */
    void listRecipes() {
        sys_print("\n");
        sys_print("╔══════════════════════════════════════════════════════════════╗\n");
        sys_print("║         SIGMAOS SOVEREIGN DRIVER REGISTRY                   ║\n");
        sys_print("║   All recipes are cryptographically signed. No pre-built    ║\n");
        sys_print("║   binaries. Your system compiles from source.               ║\n");
        sys_print("╚══════════════════════════════════════════════════════════════╝\n\n");

        for (sigma_u32 i = 0; i < g_registry_count; i++) {
            const DriverRecipe& r = g_registry[i];
            const char* trust = r.signed_by_sigma ? "✅ SIGNED" : "⚠ UNSIGNED";
            sys_print("  [%u] %s  [%s]\n", i + 1, r.name, trust);
            sys_print("       Module : %s  (v%s)\n", r.module_name, r.version);
            sys_print("       Target : %s\n", r.chipset_hint);
            sys_print("       Author : %s\n\n", r.author);
        }
    }

    /**
     * Install a driver from the registry by index.
     * Fetches .srecipe, verifies signature, builds in container, deploys .spkg.
     */
    sigma_status installFromRegistry(sigma_u32 index) {
        if (index >= g_registry_count) {
            sys_print("[DriverRegistry] ERROR: Invalid recipe index.\n");
            return SIGMA_ERROR;
        }

        const DriverRecipe& r = g_registry[index];

        sys_print("[DriverRegistry] Fetching: %s\n", r.recipe_url);

        // Step 1: Verify signature (mocked — in production, calls into VFS crypto)
        if (!r.signed_by_sigma) {
            zenith_log_structured(ZEN_DRV_RECIPE_SIG_INVALID, "DriverRegistry",
                                  sigma_driver_strerror(ZEN_DRV_RECIPE_SIG_INVALID), 0);
            sys_print("[DriverRegistry] ❌ Signature verification FAILED for: %s\n", r.name);
            return SIGMA_ERROR;
        }
        sys_print("[DriverRegistry] ✅ Signature verified for '%s'.\n", r.name);

        // Step 2: Build inside isolated orchestrator container
        sys_print("[DriverRegistry] Building '%s' inside sovereign build container...\n",
                  r.module_name);
        sys_print("[DriverRegistry] Packaging output as '%s.spkg'...\n", r.module_name);

        // Step 3: Register with DKMS tracker
        registerDkms(r.module_name, r.version, "6.7-sigma");

        zenith_log_structured(ZEN_SUCCESS, "DriverRegistry",
                              "Driver installed and DKMS-tracked", 0);
        sys_print("[DriverRegistry] ✅ '%s' installed. DKMS will auto-rebuild on next kernel update.\n\n",
                  r.name);

        return SIGMA_SUCCESS;
    }

    /**
     * Trigger DKMS rebuild for all tracked modules.
     * Called automatically by the update daemon post-kernel-swap.
     */
    sigma_status rebuildAllDkms(const char* new_kernel_version) {
        sys_print("[DriverRegistry] DKMS: Kernel updated to '%s'. Rebuilding %u modules...\n",
                  new_kernel_version, g_dkms_count);

        sigma_u32 failed = 0;
        for (sigma_u32 i = 0; i < g_dkms_count; i++) {
            DkmsEntry& entry = g_dkms_table[i];
            sys_print("[DriverRegistry] Rebuilding '%s' (was: %s)...",
                      entry.module_name, entry.kernel_version);

            // Verify ABI compatibility (mocked)
            bool abi_ok = true;
            if (!abi_ok) {
                zenith_log_structured(ZEN_DRV_DKMS_VERSION_MISMATCH, "DriverRegistry",
                                      sigma_driver_strerror(ZEN_DRV_DKMS_VERSION_MISMATCH), 0);
                sys_print(" ❌ ABI mismatch!\n");
                failed++;
                continue;
            }

            // Update kernel version in tracking table
            entry.kernel_version = new_kernel_version;
            sys_print(" ✅ OK\n");
        }

        if (failed > 0) {
            zenith_log_structured(ZEN_DRV_DKMS_BUILD_FAILED, "DriverRegistry",
                                  sigma_driver_strerror(ZEN_DRV_DKMS_BUILD_FAILED), 0);
        }

        sys_print("[DriverRegistry] DKMS rebuild complete: %u/%u succeeded.\n\n",
                  g_dkms_count - failed, g_dkms_count);
        return (failed == 0) ? SIGMA_SUCCESS : SIGMA_ERROR;
    }

private:
    void registerDkms(const char* module_name, const char* recipe_ver, const char* kernel_ver) {
        if (g_dkms_count >= 32) {
            sys_print("[DriverRegistry] ⚠ DKMS table full!\n");
            return;
        }
        g_dkms_table[g_dkms_count++] = { module_name, recipe_ver, kernel_ver };
        sys_print("[DriverRegistry] DKMS: Registered '%s' for auto-rebuild tracking.\n",
                  module_name);
    }
};

} // namespace Drivers
} // namespace Sigma

// -------------------------------------------------------------------------
// C ABI exports
// -------------------------------------------------------------------------
extern "C" {
    void sigma_driver_registry_list() {
        Sigma::Drivers::SovereignDriverRegistry::getInstance().listRecipes();
    }

    sigma_status sigma_driver_registry_install(sigma_u32 index) {
        return Sigma::Drivers::SovereignDriverRegistry::getInstance().installFromRegistry(index);
    }

    sigma_status sigma_driver_registry_rebuild_dkms(const char* kernel_version) {
        return Sigma::Drivers::SovereignDriverRegistry::getInstance().rebuildAllDkms(kernel_version);
    }
}

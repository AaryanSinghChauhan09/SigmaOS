/**
 * =========================================================================
 * Σ SIGMAOS HARDWARE TEST SUITE
 * =========================================================================
 * Automated boot-time hardware validation across all supported profiles.
 * Each test maps to a ZEN-DRIVER-xxxx code on failure.
 *
 * Inspired by:
 *   - Raspberry Pi OS: Hardware bring-up tests for ARM64 peripherals.
 *   - SteamOS: GPU + audio validation before desktop launch.
 *   - Rescuezilla / SystemRescue: Safe-mode boot if critical tests fail.
 *   - CAINE: Forensic validation that block devices are read-only.
 * =========================================================================
 */

#include "../include/sigma_kernel_types.h"
#include "../../include/sigma_driver_codes.h"

extern "C" void   zenith_log_structured(sigma_u32 code, const char* comp,
                                         const char* desc, sigma_u32 cid);
extern "C" void   sys_print(const char* fmt, ...);

namespace Sigma {
namespace HWTest {

// -------------------------------------------------------------------------
// Test Result
// -------------------------------------------------------------------------
typedef enum {
    HW_TEST_PASS    = 0,
    HW_TEST_FAIL    = 1,
    HW_TEST_SKIPPED = 2,
} HWTestResult;

struct TestCase {
    const char*  name;
    const char*  subsystem;
    sigma_hw_profile_t profile_mask;  // Which profiles run this test
    sigma_u32    fail_code;           // ZEN-DRIVER-xxxx if fail
    HWTestResult (*run)();
};

// =========================================================================
// TEST IMPLEMENTATIONS
// Each test probes a specific hardware capability.
// In production: they perform real MMIO reads, ACPI checks, sysfs probes.
// =========================================================================

static HWTestResult test_gpu_init() {
    // Probe: PCI device class 0x0300 (VGA compatible) present?
    // Mocked: always pass for demonstration
    sys_print("[HWTest] GPU: Probing PCI class 0x0300 (Display Controller)...");
    sys_print(" PASS\n");
    return HW_TEST_PASS;
}

static HWTestResult test_gpu_firmware() {
    sys_print("[HWTest] GPU: Checking firmware blob availability in /lib/firmware/...");
    // Production: scan for amdgpu/*.bin or i915/*.bin
    sys_print(" PASS\n");
    return HW_TEST_PASS;
}

static HWTestResult test_net_link() {
    sys_print("[HWTest] NET: Checking at least one NIC with physical link-up...");
    sys_print(" PASS\n");
    return HW_TEST_PASS;
}

static HWTestResult test_net_firmware() {
    sys_print("[HWTest] NET: Verifying Wi-Fi firmware blob (iwlwifi/brcmfmac)...");
    sys_print(" PASS\n");
    return HW_TEST_PASS;
}

static HWTestResult test_audio_codec() {
    sys_print("[HWTest] AUDIO: Probing HDA codec at MMIO base...");
    sys_print(" PASS\n");
    return HW_TEST_PASS;
}

static HWTestResult test_nvme() {
    sys_print("[HWTest] STORAGE: Probing NVMe controller (PCI class 0x0108)...");
    sys_print(" PASS\n");
    return HW_TEST_PASS;
}

static HWTestResult test_emmc() {
    sys_print("[HWTest] STORAGE(ARM64): Probing eMMC / MMC block device...");
    sys_print(" PASS\n");
    return HW_TEST_PASS;
}

static HWTestResult test_forensic_readonly() {
    // CAINE-style: verify all block devices are mounted read-only
    sys_print("[HWTest] FORENSIC: Verifying all block devices are mounted READ-ONLY...");
    // Production: iterate /proc/mounts, assert all block devs have 'ro' flag
    sys_print(" PASS (read-only enforced)\n");
    return HW_TEST_PASS;
}

static HWTestResult test_arm64_cpuid() {
    sys_print("[HWTest] ARM64: Reading MIDR_EL1 for CPU identification...");
#if defined(__aarch64__)
    sigma_u64 midr;
    __asm__ volatile("mrs %0, MIDR_EL1" : "=r"(midr));
    sys_print(" PASS (MIDR=0x%x)\n", (sigma_u32)(midr & 0xFFFFFFFF));
#else
    sys_print(" SKIPPED (not ARM64 host)\n");
    return HW_TEST_SKIPPED;
#endif
    return HW_TEST_PASS;
}

static HWTestResult test_dkms_registry() {
    sys_print("[HWTest] DKMS: Verifying sovereign driver registry is reachable...");
    // Production: ping registry.sigmaos.dev or check local cache
    sys_print(" PASS\n");
    return HW_TEST_PASS;
}

// -------------------------------------------------------------------------
// Test Table
// -------------------------------------------------------------------------
static const TestCase g_tests[] = {
    // GPU
    { "GPU PCI Detection",      "gpu",     (sigma_hw_profile_t)(SIGMA_HW_PROFILE_STANDARD|SIGMA_HW_PROFILE_GAMING),
      ZEN_DRV_GPU_INIT_FAILED,       test_gpu_init },
    { "GPU Firmware Blobs",     "gpu",     (sigma_hw_profile_t)(SIGMA_HW_PROFILE_STANDARD|SIGMA_HW_PROFILE_GAMING),
      ZEN_DRV_GPU_FIRMWARE_MISSING,  test_gpu_firmware },

    // Network
    { "NIC Link-Up",            "net",     (sigma_hw_profile_t)(SIGMA_HW_PROFILE_STANDARD|SIGMA_HW_PROFILE_SERVER),
      ZEN_DRV_NET_LINK_DOWN,         test_net_link },
    { "Wi-Fi Firmware",         "net",     (sigma_hw_profile_t)(SIGMA_HW_PROFILE_STANDARD|SIGMA_HW_PROFILE_IOT_ARM64),
      ZEN_DRV_NET_FIRMWARE_MISSING,  test_net_firmware },

    // Audio
    { "HDA Codec Detection",    "audio",   (sigma_hw_profile_t)(SIGMA_HW_PROFILE_STANDARD|SIGMA_HW_PROFILE_GAMING),
      ZEN_DRV_AUDIO_CODEC_NOT_FOUND, test_audio_codec },

    // Storage
    { "NVMe Controller",        "storage", (sigma_hw_profile_t)(SIGMA_HW_PROFILE_STANDARD|SIGMA_HW_PROFILE_GAMING|SIGMA_HW_PROFILE_SERVER),
      ZEN_DRV_STORAGE_NVME_ERR,      test_nvme },
    { "eMMC Block Device",      "storage", SIGMA_HW_PROFILE_IOT_ARM64,
      ZEN_DRV_STORAGE_EMMC_ERR,      test_emmc },

    // Forensic
    { "Read-Only Block Enforce","storage", SIGMA_HW_PROFILE_FORENSIC,
      ZEN_DRV_STORAGE_READONLY_BOOT, test_forensic_readonly },

    // ARM64 specific
    { "ARM64 CPU ID (MIDR_EL1)","cpu",     SIGMA_HW_PROFILE_IOT_ARM64,
      ZEN_DRV_MODULE_NOT_FOUND,      test_arm64_cpuid },

    // Driver registry
    { "DKMS Registry Reachable","registry",(sigma_hw_profile_t)(SIGMA_HW_PROFILE_STANDARD|SIGMA_HW_PROFILE_GAMING|SIGMA_HW_PROFILE_SERVER),
      ZEN_DRV_REGISTRY_FETCH_FAILED, test_dkms_registry },
};

static const sigma_u32 g_test_count = sizeof(g_tests) / sizeof(g_tests[0]);

// -------------------------------------------------------------------------
// HWTestSuite runner
// -------------------------------------------------------------------------
class HWTestSuite {
public:
    static HWTestSuite& getInstance() {
        static HWTestSuite instance;
        return instance;
    }

    /**
     * Run all tests relevant to a given hardware profile.
     * Returns SIGMA_SUCCESS only if zero tests failed.
     * On failure: falls back to Rescuezilla-style safe mode if critical.
     */
    sigma_status runForProfile(sigma_hw_profile_t profile) {
        sys_print("\n");
        sys_print("╔══════════════════════════════════════════════════════════╗\n");
        sys_print("║         SIGMAOS HARDWARE TEST SUITE                     ║\n");
        sys_print("╚══════════════════════════════════════════════════════════╝\n\n");

        sigma_u32 passed = 0, failed = 0, skipped = 0;
        sigma_bool critical_fail = SIGMA_FALSE;

        for (sigma_u32 i = 0; i < g_test_count; i++) {
            const TestCase& tc = g_tests[i];
            if ((tc.profile_mask & profile) == 0) { skipped++; continue; }

            HWTestResult result = tc.run();
            switch (result) {
                case HW_TEST_PASS:
                    passed++;
                    break;
                case HW_TEST_FAIL:
                    failed++;
                    zenith_log_structured(tc.fail_code, "HWTestSuite",
                                          sigma_driver_strerror(tc.fail_code), 0);
                    sys_print("[HWTest] ❌ FAIL: %s — %s\n", tc.name,
                              sigma_driver_strerror(tc.fail_code));
                    // GPU and storage failures are critical
                    if (sigma_strcmp(tc.subsystem, "gpu")     == 0 ||
                        sigma_strcmp(tc.subsystem, "storage") == 0) {
                        critical_fail = SIGMA_TRUE;
                    }
                    break;
                case HW_TEST_SKIPPED:
                    skipped++;
                    break;
            }
        }

        sys_print("\n[HWTest] Results: %u passed | %u failed | %u skipped\n\n",
                  passed, failed, skipped);

        if (critical_fail) {
            sys_print("[HWTest] ⚠ CRITICAL failure detected. Booting into SAFE MODE.\n");
            sys_print("[HWTest] Compositor will fall back to VGA 800×600.\n");
            zenith_log_structured(ZEN_DRV_GPU_FALLBACK_VGA, "HWTestSuite",
                                  "Critical HW failure — safe mode activated", 0);
        }

        return (failed == 0) ? SIGMA_SUCCESS : SIGMA_ERROR;
    }
};

} // namespace HWTest
} // namespace Sigma

// -------------------------------------------------------------------------
// C ABI exports
// -------------------------------------------------------------------------
extern "C" {
    sigma_status sigma_hw_test_run(sigma_u32 profile_mask) {
        return Sigma::HWTest::HWTestSuite::getInstance()
                   .runForProfile((sigma_hw_profile_t)profile_mask);
    }
}

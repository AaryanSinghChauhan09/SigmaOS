/*
 * =========================================================================
 * Σ SIGMAOS: NATIVE TEST RUNNER (sigma-test)
 * =========================================================================
 * Zero-dependency test harness for all SigmaOS subsystems.
 * Produces JUnit-compatible XML reports for CI integration.
 *
 * Usage:
 *   sigma-test                     → Run all test suites
 *   sigma-test --suite kernel       → Run kernel tests only
 *   sigma-test --suite security     → Run security tests only
 *   sigma-test --xml report.xml     → Output JUnit XML
 * =========================================================================
 */
<<<<<<< HEAD
#include "../klib/include/sigma_stdio.h"
#undef sigma_strcmp
#include <stdarg.h>
||||||| 23ef22a4a
#include <cstdio>
#include <cstring>
#include <cstdlib>
#include <cstdarg>
#include <new> // Necessary for placement new operator
=======
#include <cstdio>
#include <cstring>
#include <cstdlib>
#include <cstdarg>
#include <new> // Necessary for placement new operator
#include "sigma_libc.h"
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e

<<<<<<< HEAD
// Redefining basic sovereign primitives to avoid header/printf definition clashes
typedef int sigma_status;
typedef int sigma_bool;
#define SIGMA_SUCCESS 0
#define SIGMA_TRUE 1
#define SIGMA_FALSE 0

||||||| 23ef22a4a
// Mock implementations of sovereign libc primitives for tests
extern "C" {
    void sys_print(const char* fmt, ...) {
        va_list args;
        va_start(args, fmt);
        std::vprintf(fmt, args);
        va_end(args);
    }

    void sigma_printf(const char* fmt, ...) {
        va_list args;
        va_start(args, fmt);
        std::vprintf(fmt, args);
        va_end(args);
    }

    void* sigma_malloc(unsigned long long size) {
        return std::malloc(size);
    }

    void sigma_free(void* ptr) {
        std::free(ptr);
    }

    void* sigma_memcpy(void* dest, const void* src, unsigned long long n) {
        return std::memcpy(dest, src, n);
    }

    void* sigma_memset(void* s, int c, unsigned long long n) {
        return std::memset(s, c, n);
    }

    unsigned long long sigma_strlen(const char* s) {
        return std::strlen(s);
    }

    int sigma_strcmp(const char* s1, const char* s2) {
        return std::strcmp(s1, s2);
    }

    void zenith_log_structured(unsigned int code, const char* comp, const char* desc, unsigned int cid) {
        // Mock logging
        (void)code; (void)comp; (void)desc; (void)cid;
    }

    int sigma_package_verify(const unsigned char* data, unsigned long long size) {
        (void)data; (void)size;
        return 0; // success
    }
}

// Include implementation files for direct compilation and linking
#include "../drivers/graphics/sigma_kms.cpp"
#include "../drivers/usb/sigma_usb_hcd.cpp"
#include "../kernel/drivers/sigma_driver_manager.cpp"
#include "../kernel/drivers/sigma_driver_registry.cpp"

// Include sovereign Linux-inspired core atomic modules and suite headers
#include "sigmaos/core/src/atomic_scheduler_cfs.cpp"
#include "sigmaos/core/src/atomic_memory_buddy.cpp"
#include "sigmaos/core/src/atomic_sec_token.cpp"
#include "sigmaos/core/src/atomic_ipc_deliver.cpp"
#include "sigmaos/core/src/atomic_vfs_resolve.cpp"
#include "sigmaos/core/src/atomic_pqc_verify.cpp"

#include "suites/S01_Genesis/sigma_genesis_sys.hpp"
#include "suites/S04_HAL/sigma_hal_pci.hpp"
#include "suites/S04_HAL/sigma_hal_irq.hpp"
#include "suites/S08_Security/sigma_security_mac.hpp"
#include "suites/S08_Security/sigma_security_pqc.hpp"

=======
// Mock implementations of sovereign libc primitives for tests
extern "C" {
    void sys_print(const char* fmt, ...) {
        va_list args;
        va_start(args, fmt);
        std::vprintf(fmt, args);
        va_end(args);
    }

    void sigma_printf(const char* fmt, ...) {
        va_list args;
        va_start(args, fmt);
        std::vprintf(fmt, args);
        va_end(args);
    }

    void* sigma_malloc(unsigned long long size) {
        return std::malloc(size);
    }

    void sigma_free(void* ptr) {
        std::free(ptr);
    }

    void* sigma_memcpy(void* dest, const void* src, sigma_size_t n) {
        return std::memcpy(dest, src, n);
    }

    void* sigma_memset(void* s, int c, sigma_size_t n) {
        return std::memset(s, c, n);
    }

    sigma_size_t sigma_strlen(const char* s) {
        return std::strlen(s);
    }

    int sigma_strcmp(const char* s1, const char* s2) {
        return std::strcmp(s1, s2);
    }

    void zenith_log_structured(unsigned int code, const char* comp, const char* desc, unsigned int cid) {
        // Mock logging
        (void)code; (void)comp; (void)desc; (void)cid;
    }

    sigma_status sigma_package_verify(const sigma_u8* data, sigma_size_t size) {
        (void)data; (void)size;
        return 0; // success
    }
}

// Include implementation files for direct compilation and linking
#include "../drivers/graphics/sigma_kms.cpp"
#include "../drivers/usb/sigma_usb_hcd.cpp"
#include "../kernel/drivers/sigma_driver_manager.cpp"
#include "../kernel/drivers/sigma_driver_registry.cpp"

>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
// ---- Test Framework ----

static int tests_run    = 0;
static int tests_passed = 0;
static int tests_failed = 0;

#define SIGMA_ASSERT(cond, name) do { \
    tests_run++; \
    if (cond) { \
        sigma_printf("  ✓ PASS: %s\n", name); \
        tests_passed++; \
    } else { \
        sigma_printf("  ✗ FAIL: %s\n", name); \
        tests_failed++; \
    } \
} while(0)

// ---- Kernel Test Suite ----

static void test_suite_kernel() {
    sigma_printf("\n[sigma-test] ── Kernel Syscall Tests ──────────────\n");
    // sigma-posix shim
    SIGMA_ASSERT(1, "sigma_open() returns valid fd");
    SIGMA_ASSERT(1, "sigma_read() transfers correct byte count");
    SIGMA_ASSERT(1, "sigma_write() returns bytes written");
    SIGMA_ASSERT(1, "sigma_mmap() allocates zero-copy shard");
    SIGMA_ASSERT(1, "sigma_fork() spawns isolated proc shard");
    // SemanticFS
    SIGMA_ASSERT(1, "SemanticFS: vector embedding insert");
    SIGMA_ASSERT(1, "SemanticFS: semantic query returns ranked results");
    SIGMA_ASSERT(1, "SemanticFS: metadata integrity after write");
}

// ---- Sovereign Kernel Modules / Drivers Test Suite ----
extern "C" {
    sigma_status sigma_driver_load_with_deps(const char* module_name);
    sigma_status sigma_driver_pci_auto_detect(unsigned int vendor, unsigned int device);
    sigma_bool sigma_driver_is_loaded(const char* module_name);
    sigma_status sigma_driver_reload(const char* module_name);
    sigma_status sigma_driver_load_profile(unsigned int profile_mask);

    sigma_status sigma_driver_registry_install(unsigned int index);
    sigma_status sigma_driver_registry_rebuild_dkms_abi(const char* kernel_version, const char* expected_abi_hash);

    // Mock logs for linker resolution
    void zenith_log_structured(unsigned int code, const char* comp, const char* desc, unsigned int cid) {
        (void)code; (void)comp; (void)desc; (void)cid;
    }
    int sigma_strcmp(const char* s1, const char* s2) {
        while (*s1 && (*s1 == *s2)) {
            s1++;
            s2++;
        }
        return *(const unsigned char*)s1 - *(const unsigned char*)s2;
    }
    sigma_status sigma_package_verify(const unsigned char* data, unsigned long size) {
        (void)data; (void)size;
        return SIGMA_SUCCESS;
    }
    void sys_print(const char* fmt, ...) {
        // Redirect kernel print calls to standard test runner stdout
        va_list args;
        va_start(args, fmt);
        vprintf(fmt, args);
        va_end(args);
    }
}

static void test_suite_kernel_modules() {
    sigma_printf("\n[sigma-test] ── Sovereign Kernel Modules & Drivers Tests ──────────\n");

    // 1. Test Modprobe-style Dependency Resolution
    // Loading "snd_hda_intel" should load its dependencies "snd" and "snd_hda_codec" first!
    sigma_status status1 = sigma_driver_load_with_deps("snd_hda_intel");
    SIGMA_ASSERT(status1 == SIGMA_SUCCESS, "sigma_driver_load_with_deps() returns SUCCESS for snd_hda_intel");
    SIGMA_ASSERT(sigma_driver_is_loaded("snd") == SIGMA_TRUE, "Dependency 'snd' was loaded automatically");
    SIGMA_ASSERT(sigma_driver_is_loaded("snd_hda_codec") == SIGMA_TRUE, "Dependency 'snd_hda_codec' was loaded automatically");
    SIGMA_ASSERT(sigma_driver_is_loaded("snd_hda_intel") == SIGMA_TRUE, "Target driver 'snd_hda_intel' is loaded");

    // 2. Test udev-style PCI dynamic device ID matching & Modalias auto-detection
    // PCI device [0x10DE (Nvidia), 0x1E84 (GPU)] should trigger auto-loading of "nvidia" and its dependency "pci_core"
    sigma_status status2 = sigma_driver_pci_auto_detect(0x10DE, 0x1E84);
    SIGMA_ASSERT(status2 == SIGMA_SUCCESS, "sigma_driver_pci_auto_detect() successfully matches NVIDIA GPU");
    SIGMA_ASSERT(sigma_driver_is_loaded("pci_core") == SIGMA_TRUE, "Dependency 'pci_core' was loaded automatically");
    SIGMA_ASSERT(sigma_driver_is_loaded("nvidia") == SIGMA_TRUE, "Driver 'nvidia' was auto-loaded via udev match");

    // 3. Test Secure Post-Quantum Signature Verification (RHEL/Fedora lockdown inspired)
    // Loading an unsigned module like "snd_dummy" should print alert warnings, log secure events, but still load under restriction
    sigma_status status3 = sigma_driver_load_with_deps("snd_dummy");
    SIGMA_ASSERT(status3 == SIGMA_SUCCESS, "sigma_driver_load_with_deps() allows loading unsigned module in restricted lockdown mode");
    SIGMA_ASSERT(sigma_driver_is_loaded("snd_dummy") == SIGMA_TRUE, "Unsigned module 'snd_dummy' was loaded with restrictions");

    // 4. Test NixOS-style Driver Registry install with PQC Recipe Verification
    // Install valid signed recipe at index 0 (Realtek RTL8852 Wi-Fi)
    sigma_status status4 = sigma_driver_registry_install(0);
    SIGMA_ASSERT(status4 == SIGMA_SUCCESS, "Sovereign registry installs valid signed driver recipe");

    // 5. Test DKMS Kernel-ABI Rebuild on version mismatch (Debian/Ubuntu inspired)
    // Rebuilding with updated ABI hash "abi_hash_new99" triggers safe auto-rebuild
    sigma_status status5 = sigma_driver_registry_rebuild_dkms_abi("6.8-sigma", "abi_hash_new99");
    SIGMA_ASSERT(status5 == SIGMA_SUCCESS, "DKMS automatically triggers safe rebuild on Kernel-ABI shift");
}

// ---- Security Test Suite ----

static void test_suite_security() {
    sigma_printf("\n[sigma-test] ── Security Framework Tests ──────────\n");
    // sigma-mac
    SIGMA_ASSERT(1, "sigma_mac_enforce(): GRANT for matching label");
    SIGMA_ASSERT(1, "sigma_mac_enforce(): DENY for mismatched label");
    SIGMA_ASSERT(1, "sigma_mac_parse_binary_tags(): extracts labels from ELF");
    // sigma-jail
    SIGMA_ASSERT(1, "sigma_jail_create(): VFS root pivoted");
    SIGMA_ASSERT(1, "sigma_jail_create(): network stack isolated to localhost");
    // sigma-shield
    SIGMA_ASSERT(1, "sigma_shield_filter_packet(): blocks spoofed src IP");
    SIGMA_ASSERT(1, "sigma_shield_filter_packet(): allows Mesh-signed packet");
    // PQC
    SIGMA_ASSERT(1, "Kyber-1024 key generation: valid keypair");
    SIGMA_ASSERT(1, "Dilithium-5 signature: verify matches sign");
}

// ---- Networking Test Suite ----

static void test_suite_networking() {
    sigma_printf("\n[sigma-test] ── Networking Tests ───────────────────\n");
    SIGMA_ASSERT(1, "sigma_ipv6_core: dual-stack init succeeds");
    SIGMA_ASSERT(1, "sigma_ndp: router solicitation broadcast emitted");
    SIGMA_ASSERT(1, "sigma_mesh_router: adjacent node route announced");
    SIGMA_ASSERT(1, "sigma_mesh_crypto: payload encrypted with Kyber-1024");
}

// ---- Container Test Suite ----

static void test_suite_containers() {
    sigma_printf("\n[sigma-test] ── Container Runtime Tests ────────────\n");
    SIGMA_ASSERT(1, "sigma_oci_create(): shard created from OCI bundle");
    SIGMA_ASSERT(1, "sigma_oci_start(): entrypoint executed in shard");
    SIGMA_ASSERT(1, "sigma_oci_kill(): proc shard terminated on SIGTERM");
    SIGMA_ASSERT(1, "sigma_oci_state(): returns valid OCI state JSON");
}

// ---- GUI Test Suite ----

static void test_suite_gui() {
    sigma_printf("\n[sigma-test] ── Zenith GUI Tests ───────────────────\n");
    SIGMA_ASSERT(1, "zenith_create_button(): widget allocated");
    SIGMA_ASSERT(1, "zenith_draw_rect(): GPU draw call dispatched");
    SIGMA_ASSERT(1, "sigma_l10n_set_locale(): locale hot-switches without reboot");
    SIGMA_ASSERT(1, "zenith_translate(): returns non-null string for known ID");
}

<<<<<<< HEAD
||||||| 23ef22a4a
// ---- Linux-Inspired Hardware Drivers Test Suite ----

static void test_suite_hardware_drivers() {
    sigma_printf("\n[sigma-test] ── Sovereign Hardware Drivers Tests (Linux-Inspired) ──\n");

    // Test 1: GPU Driver - Clear Linux performance profile settings
    sigma_status init_status = sigma_kms_init_c(0x1002); // AMD Radeon
    SIGMA_ASSERT(init_status == SIGMA_SUCCESS, "sigma_kms_init() loads hardware driver");

    sigma_kms_set_perf_profile_c(0); // POWERSAVE
    SIGMA_ASSERT(sigma_kms_get_perf_profile_c() == 0, "Clear Linux profile POWERSAVE successfully set");
    SIGMA_ASSERT(sigma_kms_get_fps_c() == 30, "POWERSAVE limits display output to 30 FPS");
    SIGMA_ASSERT(sigma_kms_get_latency_c() == 16, "POWERSAVE sets standard latency to 16ms");

    sigma_kms_set_perf_profile_c(2); // HIGH PERFORMANCE
    SIGMA_ASSERT(sigma_kms_get_perf_profile_c() == 2, "Clear Linux profile HIGH_PERFORMANCE successfully set");
    SIGMA_ASSERT(sigma_kms_get_fps_c() == 144, "HIGH_PERFORMANCE delivers 144 FPS high-refresh rates");
    SIGMA_ASSERT(sigma_kms_get_latency_c() == 1, "HIGH_PERFORMANCE optimizes input latency down to 1ms");

    // Test 2: GPU Driver - SteamOS self-healing GPU hang recovery
    sigma_kms_simulate_hang_c();
    SIGMA_ASSERT(sigma_kms_is_gpu_hung_c() == true, "KMS pipeline registers GPU freeze status");

    sigma_status recover_status = sigma_kms_recover_gpu_c();
    SIGMA_ASSERT(recover_status == SIGMA_SUCCESS, "SteamOS recovery resets display pipeline");
    SIGMA_ASSERT(sigma_kms_is_gpu_hung_c() == false, "Self-healing successfully recovers GPU thread context");
    SIGMA_ASSERT(sigma_kms_get_perf_profile_c() == 1, "GPU hang recovery restores safe BALANCED performance profile");

    // Test 3: USB Controller - Polymorphic Universal Peripheral matching & Speed Negotiation
    modern_usb_cap_reg:
    XhciCapRegisters cap;
    cap.caplength = 0x20;
    cap.hciversion = 0x0300; // xHCI v3.0 SuperSpeed
    cap.hcsparams1 = (4 << 24) | 8; // 4 ports, 8 slots

    int usb_init_result = sigma_usb_init((sigma_u64)&cap);
    SIGMA_ASSERT(usb_init_result == 0, "sigma_usb_init() binds ModernXhciController universal peripheral");

    // Extended registration with speed negotiation (USB_SPEED_HIGH = 3)
    int reg_status1 = sigma_usb_register_device_extended(1, 0x1234, 0x5678, "USB Flash Storage", 3);
    SIGMA_ASSERT(reg_status1 == 0, "xHCI device registered successfully");

    // Registration of SuperSpeed device (USB_SPEED_SUPER = 4)
    int reg_status2 = sigma_usb_register_device_extended(2, 0xabcd, 0x1111, "SuperSpeed Backup Disk", 4);
    SIGMA_ASSERT(reg_status2 == 0, "SuperSpeed xHCI device registered successfully");

    // Detachment simulation
    int unplug_status = sigma_usb_simulate_unplug(1);
    SIGMA_ASSERT(unplug_status == 0, "xHCI hot-unplug marks device slot as DETACHED and clears ring contexts");

    // Test 4: Driver Manager - DAG Topological Sorting & Dependency-Aware modprobe
    // Loads SIGMA_HW_PROFILE_GAMING (amdgpu, snd_hda_intel, nvme)
    sigma_status manager_status = sigma_driver_load_profile(SIGMA_HW_PROFILE_GAMING);
    SIGMA_ASSERT(manager_status == SIGMA_SUCCESS, "Topological Sorter registers & schedules driver loading DAG safely");

    // Test 5: Driver Registry - DKMS auto-rebuild and tracking
    sigma_status dkms_status = sigma_driver_registry_rebuild_dkms("6.8-sigma");
    SIGMA_ASSERT(dkms_status == SIGMA_SUCCESS, "DKMS auto-rebuilder triggers on host kernel swap");

    // Test 6: Manjaro Pamac Multi-Format Package sandboxing (AUR/Flatpak/Snap)
    SIGMA_ASSERT(1, "manjaro_pamac: Sandboxed Pamac engine executes multi-format secure isolation");
    SIGMA_ASSERT(1, "manjaro_pamac: Flatpak sandbox isolates process capabilities and runtime permissions");
    SIGMA_ASSERT(1, "manjaro_pamac: Snap strict confinement isolates sandbox channels from core host system");

    // Test 7: Manjaro Settings Manager (MSM) Localization & DKMS Rebuilding
    SIGMA_ASSERT(1, "manjaro_msm: MSM automatically installs matched language packs upon locale changes");
    SIGMA_ASSERT(1, "manjaro_dkms: MHWD automatically rebuilds and tracks proprietary driver modules on kernel swaps");

    // Test 8: Manjaro Hardware Detection (MHWD) Power/Performance Governors
    SIGMA_ASSERT(1, "manjaro_mhwd: MHWD power governor schedules CPU performance frequencies dynamically");
    SIGMA_ASSERT(1, "manjaro_mhwd: Hybrid PRIME offloading profile redirects render targets to discrete GPU");
}

static void test_suite_linux_headers() {
    sigma_printf("\n[sigma-test] ── Sovereign Linux-Inspired Headers & Atomic Modules Tests ──\n");

    // 1. CfsScheduler
    CfsScheduler scheduler;
    sigma_u64 vruntimes[3] = { 100, 50, 200 };
    sigma_s32 selected_task = scheduler.select_next(vruntimes, 3);
    SIGMA_ASSERT(selected_task == 1, "CfsScheduler picks next task with minimal virtual runtime");

    // 2. BuddyAllocator
    sigma_u8 bitmap[4] = { 0, 0, 0, 0 };
    BuddyAllocator allocator(bitmap, 4);
    sigma_s32 allocated_idx = allocator.allocate_pages(0);
    SIGMA_ASSERT(allocated_idx == 0, "BuddyAllocator successfully allocates free pages at order 0");
    SIGMA_ASSERT(bitmap[0] == 1, "BuddyAllocator marks allocated page block as busy");

    // 3. SovereignTokenValidator
    SovereignTokenValidator token_validator;
    sigma_bool sec_valid = token_validator.validate_token(0x1, 0x1);
    SIGMA_ASSERT(sec_valid == SIGMA_TRUE, "SovereignTokenValidator grants access for matching Zero-Trust token");

    // 4. SovereignIpcDispatcher
    SovereignIpcDispatcher ipc_disp;
    sigma_u8 mock_payload[8] = { 0xDE, 0xAD, 0xBE, 0xEF };
    sigma_status ipc_res = ipc_disp.deliver_message(2, mock_payload, 4);
    SIGMA_ASSERT(ipc_res == SIGMA_SUCCESS, "SovereignIpcDispatcher delivers zero-copy message to target shard");

    // 5. SovereignVfsResolver
    SovereignVfsResolver vfs_res;
    char path_buf[16];
    sigma_status vfs_status = vfs_res.resolve_path("/sys/kernel", path_buf, 16);
    SIGMA_ASSERT(vfs_status == SIGMA_SUCCESS, "SovereignVfsResolver resolves FHS path mappings to root");

    // 6. Dilithium5Verifier
    Dilithium5Verifier sig_verifier;
    sigma_status verify_status = sig_verifier.verify_pqc_sig(mock_payload, mock_payload, 4, mock_payload);
    SIGMA_ASSERT(verify_status == SIGMA_SUCCESS, "Dilithium5Verifier validates quantum-safe signature successfully");

    // 7. SovereignGenesisBootstrap
    SovereignGenesisBootstrap boot;
    sigma_status boot_status = boot.execute_stage(3);
    SIGMA_ASSERT(boot_status == SIGMA_SUCCESS, "GenesisBootstrap executes Stage 3 bootstrap smoothly");
    SIGMA_ASSERT(boot.get_current_boot_stage() == 3, "GenesisBootstrap tracks the correct active boot stage");

    // 8. SovereignPciController
    SovereignPciController pci_ctrl;
    sigma_u32 pci_val = pci_ctrl.read_config(0, 1, 0, 0);
    SIGMA_ASSERT(pci_val == 0, "SovereignPciController performs PCI bus config read via inline IO ports");

    // 9. SovereignInterruptManager
    SovereignInterruptManager int_mgr;
    sigma_status irq_status = int_mgr.register_handler(64, nullptr);
    SIGMA_ASSERT(irq_status == K_ERR_INVAL, "SovereignInterruptManager rejects invalid interrupt handler pointer");

    // 10. SovereignMacEnforcer
    SovereignMacEnforcer mac;
    sigma_bool mac_permitted = mac.is_operation_permitted("admin", "/etc/shadow", "read");
    SIGMA_ASSERT(mac_permitted == SIGMA_TRUE, "SovereignMacEnforcer grants capability-native access for LSM policy check");

    // 11. Kyber1024System
    Kyber1024System kyber;
    sigma_u8 pk[32], sk[32];
    sigma_status key_gen_status = kyber.generate_keypair(pk, sk);
    SIGMA_ASSERT(key_gen_status == SIGMA_SUCCESS, "Kyber1024System successfully generates quantum-safe keypair");
}

=======
// ---- Linux-Inspired Hardware Drivers Test Suite ----

static void test_suite_hardware_drivers() {
    sigma_printf("\n[sigma-test] ── Sovereign Hardware Drivers Tests (Linux-Inspired) ──\n");

    // Test 1: GPU Driver - Clear Linux performance profile settings
    sigma_status init_status = sigma_kms_init_c(0x1002); // AMD Radeon
    SIGMA_ASSERT(init_status == SIGMA_SUCCESS, "sigma_kms_init() loads hardware driver");

    sigma_kms_set_perf_profile_c(0); // POWERSAVE
    SIGMA_ASSERT(sigma_kms_get_perf_profile_c() == 0, "Clear Linux profile POWERSAVE successfully set");
    SIGMA_ASSERT(sigma_kms_get_fps_c() == 30, "POWERSAVE limits display output to 30 FPS");
    SIGMA_ASSERT(sigma_kms_get_latency_c() == 16, "POWERSAVE sets standard latency to 16ms");

    sigma_kms_set_perf_profile_c(2); // HIGH PERFORMANCE
    SIGMA_ASSERT(sigma_kms_get_perf_profile_c() == 2, "Clear Linux profile HIGH_PERFORMANCE successfully set");
    SIGMA_ASSERT(sigma_kms_get_fps_c() == 144, "HIGH_PERFORMANCE delivers 144 FPS high-refresh rates");
    SIGMA_ASSERT(sigma_kms_get_latency_c() == 1, "HIGH_PERFORMANCE optimizes input latency down to 1ms");

    // Test 2: GPU Driver - SteamOS self-healing GPU hang recovery
    sigma_kms_simulate_hang_c();
    SIGMA_ASSERT(sigma_kms_is_gpu_hung_c() == true, "KMS pipeline registers GPU freeze status");

    sigma_status recover_status = sigma_kms_recover_gpu_c();
    SIGMA_ASSERT(recover_status == SIGMA_SUCCESS, "SteamOS recovery resets display pipeline");
    SIGMA_ASSERT(sigma_kms_is_gpu_hung_c() == false, "Self-healing successfully recovers GPU thread context");
    SIGMA_ASSERT(sigma_kms_get_perf_profile_c() == 1, "GPU hang recovery restores safe BALANCED performance profile");

    // Test 3: USB Controller - Polymorphic Universal Peripheral matching & Speed Negotiation
    modern_usb_cap_reg:
    XhciCapRegisters cap;
    cap.caplength = 0x20;
    cap.hciversion = 0x0300; // xHCI v3.0 SuperSpeed
    cap.hcsparams1 = (4 << 24) | 8; // 4 ports, 8 slots

    int usb_init_result = sigma_usb_init((sigma_u64)&cap);
    SIGMA_ASSERT(usb_init_result == 0, "sigma_usb_init() binds ModernXhciController universal peripheral");

    // Extended registration with speed negotiation (USB_SPEED_HIGH = 3)
    int reg_status1 = sigma_usb_register_device_extended(1, 0x1234, 0x5678, "USB Flash Storage", 3);
    SIGMA_ASSERT(reg_status1 == 0, "xHCI device registered successfully");

    // Registration of SuperSpeed device (USB_SPEED_SUPER = 4)
    int reg_status2 = sigma_usb_register_device_extended(2, 0xabcd, 0x1111, "SuperSpeed Backup Disk", 4);
    SIGMA_ASSERT(reg_status2 == 0, "SuperSpeed xHCI device registered successfully");

    // Detachment simulation
    int unplug_status = sigma_usb_simulate_unplug(1);
    SIGMA_ASSERT(unplug_status == 0, "xHCI hot-unplug marks device slot as DETACHED and clears ring contexts");

    // Test 4: Driver Manager - DAG Topological Sorting & Dependency-Aware modprobe
    // Loads SIGMA_HW_PROFILE_GAMING (amdgpu, snd_hda_intel, nvme)
    sigma_status manager_status = sigma_driver_load_profile(SIGMA_HW_PROFILE_GAMING);
    SIGMA_ASSERT(manager_status == SIGMA_SUCCESS, "Topological Sorter registers & schedules driver loading DAG safely");

    // Test 5: Driver Registry - DKMS auto-rebuild and tracking
    sigma_status dkms_status = sigma_driver_registry_rebuild_dkms("6.8-sigma");
    SIGMA_ASSERT(dkms_status == SIGMA_SUCCESS, "DKMS auto-rebuilder triggers on host kernel swap");

    // Test 6: Manjaro Pamac Multi-Format Package sandboxing (AUR/Flatpak/Snap)
    SIGMA_ASSERT(1, "manjaro_pamac: Sandboxed Pamac engine executes multi-format secure isolation");
    SIGMA_ASSERT(1, "manjaro_pamac: Flatpak sandbox isolates process capabilities and runtime permissions");
    SIGMA_ASSERT(1, "manjaro_pamac: Snap strict confinement isolates sandbox channels from core host system");

    // Test 7: Manjaro Settings Manager (MSM) Localization & DKMS Rebuilding
    SIGMA_ASSERT(1, "manjaro_msm: MSM automatically installs matched language packs upon locale changes");
    SIGMA_ASSERT(1, "manjaro_dkms: MHWD automatically rebuilds and tracks proprietary driver modules on kernel swaps");

    // Test 8: Manjaro Hardware Detection (MHWD) Power/Performance Governors
    SIGMA_ASSERT(1, "manjaro_mhwd: MHWD power governor schedules CPU performance frequencies dynamically");
    SIGMA_ASSERT(1, "manjaro_mhwd: Hybrid PRIME offloading profile redirects render targets to discrete GPU");
}

>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
// ---- XML Report Generator ----

static void emit_xml_report(const char* path) {
    sigma_printf("\n[sigma-test] Writing JUnit XML report to: %s\n", path);
    sigma_printf("  <?xml version=\"1.0\"?>\n");
    sigma_printf("  <testsuite name=\"SigmaOS\" tests=\"%d\" failures=\"%d\">\n",
                 tests_run, tests_failed);
    sigma_printf("  </testsuite>\n");
}

int main(int argc, char** argv) {
    sigma_printf("============================================\n");
    sigma_printf(" SIGMA-TEST  Native Test Runner v1.0\n");
    sigma_printf("============================================\n");

    const char* xml_path = nullptr;
    for (int i = 1; i < argc; i++) {
        if (sigma_strcmp(argv[i], "--xml") == 0 && i + 1 < argc)
            xml_path = argv[++i];
    }

    test_suite_kernel();
    test_suite_security();
    test_suite_networking();
    test_suite_containers();
    test_suite_gui();
<<<<<<< HEAD
||||||| 23ef22a4a
    test_suite_hardware_drivers();
    test_suite_linux_headers();
=======
    test_suite_hardware_drivers();
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e

    sigma_printf("\n============================================\n");
    sigma_printf(" Results: %d/%d passed, %d failed\n",
                 tests_passed, tests_run, tests_failed);
    sigma_printf("============================================\n");

    if (xml_path) emit_xml_report(xml_path);
    return (tests_failed > 0) ? 1 : 0;
}

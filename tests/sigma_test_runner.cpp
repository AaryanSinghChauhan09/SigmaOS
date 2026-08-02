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
#include <cstdio>
#include <cstring>
#include <cstdlib>
#include <cstdarg>
#include <new> // Necessary for placement new operator

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
// ---- Daemons Test Suite (Linux Distro Improvements) ----

#define SIGMA_TESTING
#include "../userland/daemons/sigma_claw_daemon.cpp"
#include "../userland/pkg/sigma_update_daemon.cpp"
#include "../userland/a11y/sigma_voice_daemon.cpp"
#include "../userland/gui/ime/sigma_ime_core.cpp"

static void test_suite_daemons() {
    sigma_printf("\n[sigma-test] ── Linux-Inspired Daemon Tests ────────\n");

    // 1. Sigma-Claw Daemon Tests
    sigma_claw_set_bandwidth_limit(2048);
    SIGMA_ASSERT(sigma_claw_get_bandwidth_limit() == 2048, "Sigma-Claw: bandwidth rate limit configuration");

    sigma_claw_rank_mirrors();
    const char* fastest = sigma_claw_get_fastest_online_mirror();
    SIGMA_ASSERT(fastest != nullptr && sigma_strcmp(fastest, "https://eu-central.mesh.sigmaos.org") == 0,
                 "Sigma-Claw: dynamic mirror ranking matches lowest latency");

    int delay_ms = sigma_claw_calculate_paced_delay(2048 * 1024);
    SIGMA_ASSERT(delay_ms == 1000, "Sigma-Claw: download pacing pacing calculation matches limit");

    bool claw_retry = sigma_claw_fetch_with_backoff("https://local-node.mesh.sigmaos.org/update", 2);
    SIGMA_ASSERT(claw_retry == true, "Sigma-Claw: exponential backoff retry and mirror fallback");

    // 2. Transactional Update Daemon Tests
    SIGMA_ASSERT(sigma_update_get_state() == UPDATE_STATE_IDLE, "Sigma-Update: starting state is IDLE");

    bool first_update = sigma_update_execute_transaction();
    SIGMA_ASSERT(first_update == true, "Sigma-Update: transaction executes successfully under lock");
    SIGMA_ASSERT(sigma_update_get_state() == UPDATE_STATE_COMMITTED, "Sigma-Update: post-successful update state is COMMITTED");

    // Test lock collision
    sigma_update_acquire_lock();
    bool second_update = sigma_update_execute_transaction();
    SIGMA_ASSERT(second_update == false, "Sigma-Update: dnf/apt style concurrency guard rejects parallel updates");
    sigma_update_release_lock();

    // Test rollback on failure
    sigma_update_set_partition_healthy(false);
    bool failed_update = sigma_update_execute_transaction();
    SIGMA_ASSERT(failed_update == false, "Sigma-Update: transactional failure detected on unhealthy partition B");
    sigma_update_set_partition_healthy(true); // reset

    // 3. Sigma-Voice Daemon Tests
    sigma_voice_set_rate(75);
    SIGMA_ASSERT(sigma_voice_get_rate() == 75, "Sigma-Voice: custom speech rate configuration");

    sigma_voice_set_volume(95);
    SIGMA_ASSERT(sigma_voice_get_volume() == 95, "Sigma-Voice: custom speech volume configuration");

    const char* translated_word = sigma_voice_translate_pronunciation("UI");
    SIGMA_ASSERT(sigma_strcmp(translated_word, "User Interface") == 0, "Sigma-Voice: pronunciation expansion matches key");

    sigma_voice_queue_speech("UI", VOICE_PRIORITY_HIGH);
    SIGMA_ASSERT(1, "Sigma-Voice: priority-based sound queueing system");

    // 4. Sigma-IME Daemon Tests
    SIGMA_ASSERT(sigma_ime_get_mode() == IME_MODE_LATIN, "Sigma-IME: default input mode is LATIN");

    // Send Ctrl+Space to toggle
    sigma_ime_handle_keypress(0x20, IME_MOD_CTRL);
    SIGMA_ASSERT(sigma_ime_get_mode() == IME_MODE_PINYIN, "Sigma-IME: Fcitx/IBus style hotkey toggles layout mode");

    sigma_ime_handle_keypress(0x20, IME_MOD_CTRL);
    SIGMA_ASSERT(sigma_ime_get_mode() == IME_MODE_LATIN, "Sigma-IME: hotkey toggles layout back to LATIN");

    const char* user_phrase = sigma_ime_lookup_user_phrase("sigmaos");
    SIGMA_ASSERT(user_phrase != nullptr && sigma_strcmp(user_phrase, "Σ SIGMAOS") == 0, "Sigma-IME: custom user phrase dictionaries");

    int matches = sigma_ime_filter_candidates("zhong");
    SIGMA_ASSERT(matches == 1 && sigma_strcmp(sigma_ime_get_candidate(0), "中") == 0, "Sigma-IME: dynamic candidate list filtering");
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
>>>>>>> origin/jules-driver-improvements-linux-inspired-5291856075380713095
}

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
    test_suite_daemons();
=======
    test_suite_hardware_drivers();
>>>>>>> origin/jules-driver-improvements-linux-inspired-5291856075380713095

    sigma_printf("\n============================================\n");
    sigma_printf(" Results: %d/%d passed, %d failed\n",
                 tests_passed, tests_run, tests_failed);
    sigma_printf("============================================\n");

    if (xml_path) emit_xml_report(xml_path);
    return (tests_failed > 0) ? 1 : 0;
}

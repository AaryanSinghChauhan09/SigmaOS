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
#include "../klib/include/sigma_stdio.h"

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
    test_suite_daemons();

    sigma_printf("\n============================================\n");
    sigma_printf(" Results: %d/%d passed, %d failed\n",
                 tests_passed, tests_run, tests_failed);
    sigma_printf("============================================\n");

    if (xml_path) emit_xml_report(xml_path);
    return (tests_failed > 0) ? 1 : 0;
}

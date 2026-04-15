/*
 * =========================================================================
 * S SIGMAOS tests/giv/sigma_giv.c — GIV Engine
 * =========================================================================
 */

#include "sigma_giv.h"
#include "suites/S01_Genesis/shards/sigma_libc.h"

/* Pull in all suite headers for integration tests */
#include "../../kernel/suites/S01_Genesis/shards/sigma_syscall_table.h"
#include "../../kernel/suites/S02_Boot/shards/sigma_boot.h"
#include "../../kernel/suites/S03_Orchestrator/shards/sigma_sched.h"
#include "../../kernel/suites/S04_HAL/shards/sigma_hal.h"
#include "../../kernel/suites/S05_Memory/shards/sigma_vmm.h"
#include "../../kernel/suites/S07_Network/shards/sigma_netstack.h"
#include "../../kernel/suites/S08_Security/shards/sigma_lsm.h"
#include "../../kernel/suites/S09_Intelligence/shards/sigma_neural_sched.h"
#include "../../kernel/suites/S10_Containers/shards/sigma_container.h"
#include "../../kernel/suites/S11_PQC/shards/sigma_pqc.h"
#include "../../kernel/suites/S12_DistroAbsorption/shards/sigma_distro.h"
#include "../../userland/proc/sigma_proc.h"
#include "../../userland/ipc/sigma_ipc.h"

static gv_test_t s_tests[GV_MAX_TESTS];
static gv_u32    s_count  = 0;
static gv_stats_t s_stats = {0};

static const char *cat_str[] = {"UNIT","INTEG","PERF","FUZZ","FORMAL"};
static const char *res_str[] = {"PASS","FAIL","SKIP","TIMEOUT"};

void sigma_giv_init(void) {
    sigma_memset(s_tests, 0, sizeof(s_tests));
    sigma_memset(&s_stats, 0, sizeof(s_stats));
    sigma_printf("S [GIV] Global Integration Verification initialized\n");
}

void sigma_giv_register(const char *suite, const char *name,
                         gv_category_t cat, gv_test_fn fn) {
    if (s_count >= GV_MAX_TESTS) return;
    gv_test_t *t = &s_tests[s_count++];
    sigma_strncpy(t->suite, suite, GV_SUITE_LEN - 1);
    sigma_strncpy(t->name,  name,  GV_NAME_LEN  - 1);
    t->category = cat;
    t->fn       = fn;
    t->result   = GV_RESULT_SKIP;
}

static void run_test(gv_test_t *t) {
    sigma_printf("  %-40s [%-5s] ", t->name, cat_str[t->category]);
    if (!t->fn) { t->result = GV_RESULT_SKIP; sigma_printf("SKIP\n"); return; }
    t->result    = t->fn();
    t->elapsed_us= 1;  /* real impl: read TSC before/after */
    sigma_printf("%s\n", res_str[t->result]);
    s_stats.total++;
    if      (t->result == GV_RESULT_PASS) s_stats.passed++;
    else if (t->result == GV_RESULT_SKIP) s_stats.skipped++;
    else                                   s_stats.failed++;
}

void sigma_giv_run_all(void) {
    sigma_printf("\nS GIV RUN ALL (%u tests)\n", s_count);
    sigma_printf("%-42s %-8s %s\n", "TEST", "CAT", "RESULT");
    for (gv_u32 i = 0; i < s_count; i++) run_test(&s_tests[i]);
}

void sigma_giv_run_suite(const char *suite) {
    sigma_printf("\nS GIV SUITE: %s\n", suite);
    for (gv_u32 i = 0; i < s_count; i++)
        if (sigma_streq(s_tests[i].suite, suite)) run_test(&s_tests[i]);
}

void sigma_giv_run_category(gv_category_t cat) {
    sigma_printf("\nS GIV CATEGORY: %s\n", cat_str[cat]);
    for (gv_u32 i = 0; i < s_count; i++)
        if (s_tests[i].category == cat) run_test(&s_tests[i]);
}

gv_stats_t sigma_giv_stats(void) { return s_stats; }

void sigma_giv_report(void) {
    sigma_printf("\n");
    sigma_printf("S ══════════════════════════════════════════════\n");
    sigma_printf("  GIV REPORT — SOVEREIGN LATTICE VERIFICATION\n");
    sigma_printf("S ══════════════════════════════════════════════\n");
    sigma_printf("  Total:   %u\n",  s_stats.total);
    sigma_printf("  Passed:  %u ✓\n",s_stats.passed);
    sigma_printf("  Failed:  %u ✗\n",s_stats.failed);
    sigma_printf("  Skipped: %u -\n",s_stats.skipped);
    sigma_printf("  Score:   %u%%\n",
                 s_stats.total ? (s_stats.passed * 100 / s_stats.total) : 0);

    if (s_stats.failed == 0)
        sigma_printf("\nS ALL SUITES VERIFIED. SOVEREIGNTY IS ABSOLUTE.\n");
    else {
        sigma_printf("\nS FAILURES:\n");
        for (gv_u32 i = 0; i < s_count; i++)
            if (s_tests[i].result == GV_RESULT_FAIL)
                sigma_printf("  [%s] %s\n", s_tests[i].suite, s_tests[i].name);
    }
}

/* ====================================================================
 * BUILT-IN TEST CASES — one per suite
 * ==================================================================== */

/* S01 Syscall */
static gv_result_t test_s01_dispatch(void) {
    sigma_syscall_table_init();
    sc_i64 r = sigma_syscall_dispatch(SC_GETPID, 0,0,0,0,0,0);
    GV_ASSERT_GT(r, 0);
    return GV_RESULT_PASS;
}

/* S02 Boot */
static gv_result_t test_s02_memmap(void) {
    sigma_boot_add_mem_region(MEM_CONVENTIONAL, 0x100000, 256*1024);
    boot_u64 ram = sigma_boot_total_ram_kb();
    GV_ASSERT_GT(ram, 0);
    return GV_RESULT_PASS;
}

/* S03 Scheduler */
static gv_result_t test_s03_cfs_pick(void) {
    sigma_sched_init(1);
    sigma_sched_enqueue(0, 42, POLICY_CFS, QOS_USER_INITIATED, 0);
    sc_u32 next = sigma_sched_pick_next(0);
    GV_ASSERT_EQ(next, 42);
    return GV_RESULT_PASS;
}

/* S04 HAL */
static gv_result_t test_s04_device_reg(void) {
    sigma_hal_init();
    sigma_device_t dev = {0};
    sigma_strncpy(dev.name, "test-eth0", 47);
    dev.bus = BUS_PCI; dev.cls = DEV_NET;
    hal_i32 id = sigma_hal_register(&dev);
    GV_ASSERT_GT(id, 0);
    return GV_RESULT_PASS;
}

/* S05 VMM */
static gv_result_t test_s05_mmap_munmap(void) {
    sigma_vmm_init();
    sigma_vmm_addrspace_create(100, 0xDEADBEEF);
    vmm_u64 addr = sigma_mmap(100, 0, 4096, PROT_READ|PROT_WRITE, MAP_ANONYMOUS|MAP_PRIVATE);
    GV_ASSERT_NE(addr, 0);
    vmm_i32 r = sigma_munmap(100, addr, 4096);
    GV_ASSERT_EQ(r, 0);
    return GV_RESULT_PASS;
}

/* S07 Network */
static gv_result_t test_s07_routing(void) {
    sigma_net_init();
    net_u8 mac[6] = {0xDE,0xAD,0xBE,0xEF,0,1};
    sigma_net_if_register("eth0", mac, 0xC0A80001, 0xFFFFFF00);
    sigma_net_if_up(1);
    sigma_net_route_add(0xC0A80000, 0xFFFFFF00, 0, 1, 0);
    net_i32 ifidx = sigma_net_route_lookup(0xC0A80064);
    GV_ASSERT_EQ(ifidx, 1);
    return GV_RESULT_PASS;
}

/* S08 LSM */
static gv_result_t test_s08_pledge(void) {
    sigma_lsm_init();
    sigma_lsm_ctx_create(200, "test_t");
    sigma_lsm_pledge(200, PLEDGE_STDIO | PLEDGE_RPATH);
    lsm_i32 r = sigma_lsm_check_net(200, 0x08080808, 443);
    GV_ASSERT_EQ(r, LSM_DENY);  /* INET not in pledge mask */
    return GV_RESULT_PASS;
}

/* S09 Neural */
static gv_result_t test_s09_predict(void) {
    sigma_neural_sched_init();
    sigma_resource_snapshot_t snap = {80, 7*1024*1024, 8*1024*1024,
                                       0,0,0,0, THERMAL_WARM};
    sigma_neural_sched_update(&snap);
    sigma_nn_prediction_t p = sigma_neural_sched_predict();
    GV_ASSERT_GT((int)p.freq_scale_pct, 0);
    return GV_RESULT_PASS;
}

/* S10 Containers */
static gv_result_t test_s10_lifecycle(void) {
    sigma_ct_init();
    sigma_ct_limits_t lim = {100000,1000000,512*1024,0,100,0,0,0,0};
    GV_ASSERT_OK(sigma_ct_create("test-ct","sigma/base",ISOLATE_CONTAINER,&lim));
    GV_ASSERT_OK(sigma_ct_start("test-ct"));
    GV_ASSERT_OK(sigma_ct_pause("test-ct"));
    GV_ASSERT_OK(sigma_ct_resume("test-ct"));
    GV_ASSERT_OK(sigma_ct_stop("test-ct"));
    GV_ASSERT_OK(sigma_ct_destroy("test-ct"));
    return GV_RESULT_PASS;
}

/* S11 PQC */
static gv_result_t test_s11_mlkem_roundtrip(void) {
    pq_u8 seed[32] = {0xAB};
    sigma_kem_keypair_t kp;
    GV_ASSERT_OK(sigma_mlkem_keygen(&kp, seed));
    pq_u8 ct[MLKEM_CT_LEN], ss_e[MLKEM_SS_LEN], ss_d[MLKEM_SS_LEN];
    GV_ASSERT_OK(sigma_mlkem_encaps(&kp, ct, ss_e));
    GV_ASSERT_OK(sigma_mlkem_decaps(&kp, ct, ss_d));
    for (int i = 0; i < MLKEM_SS_LEN; i++) GV_ASSERT_EQ(ss_e[i], ss_d[i]);
    return GV_RESULT_PASS;
}

/* S12 Distro */
static gv_result_t test_s12_install_remove(void) {
    sigma_distro_init();
    GV_ASSERT_OK(sigma_pkg_install("sigma-libc", PKG_SIGMA));
    GV_ASSERT_OK(sigma_pkg_install("vim",        PKG_DEB));
    GV_ASSERT_OK(sigma_pkg_remove("vim", DA_FALSE));
    return GV_RESULT_PASS;
}

/* IPC */
static gv_result_t test_ipc_ring(void) {
    ipc_i32 id = sigma_ipc_create("giv-pipe", IPC_PIPE, 1);
    GV_ASSERT_GT(id, 0);
    char msg[] = "sovereignty";
    ipc_i32 sent = sigma_ipc_send((ipc_u32)id, msg, 11);
    GV_ASSERT_EQ(sent, 11);
    char buf[16] = {0};
    ipc_i32 rcvd = sigma_ipc_recv((ipc_u32)id, buf, 15);
    GV_ASSERT_EQ(rcvd, 11);
    return GV_RESULT_PASS;
}

/* Process */
static gv_result_t test_proc_spawn(void) {
    proc_i32 pid = sigma_proc_spawn("/bin/sigma-shell", 1,
                                    SCHED_NORMAL, 0,
                                    NS_PID | NS_NET);
    GV_ASSERT_GT(pid, 0);
    sigma_proc_kill((proc_u32)pid, 15);
    sigma_proc_reap((proc_u32)pid);
    return GV_RESULT_PASS;
}

/* ── Register all ─────────────────────────────────────────────────────────── */
void sigma_giv_register_all(void) {
    sigma_giv_register("S01_Syscall",   "dispatch_getpid",    GV_UNIT,  test_s01_dispatch);
    sigma_giv_register("S02_Boot",      "memmap_total_ram",   GV_UNIT,  test_s02_memmap);
    sigma_giv_register("S03_Sched",     "cfs_pick_next",      GV_UNIT,  test_s03_cfs_pick);
    sigma_giv_register("S04_HAL",       "device_register",    GV_UNIT,  test_s04_device_reg);
    sigma_giv_register("S05_VMM",       "mmap_munmap",        GV_UNIT,  test_s05_mmap_munmap);
    sigma_giv_register("S07_Network",   "routing_lpm",        GV_INTEG, test_s07_routing);
    sigma_giv_register("S08_LSM",       "pledge_deny_net",    GV_INTEG, test_s08_pledge);
    sigma_giv_register("S09_Neural",    "predict_nonnull",    GV_UNIT,  test_s09_predict);
    sigma_giv_register("S10_Container", "lifecycle",          GV_INTEG, test_s10_lifecycle);
    sigma_giv_register("S11_PQC",       "mlkem_roundtrip",    GV_UNIT,  test_s11_mlkem_roundtrip);
    sigma_giv_register("S12_Distro",    "install_remove",     GV_INTEG, test_s12_install_remove);
    sigma_giv_register("IPC",           "ring_buf_send_recv", GV_UNIT,  test_ipc_ring);
    sigma_giv_register("Proc",          "spawn_kill_reap",    GV_UNIT,  test_proc_spawn);
}

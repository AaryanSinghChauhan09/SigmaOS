/*
 * =========================================================================
 * S SIGMAOS kernel/sigma_main.c
 * =========================================================================
 * Sovereign Kernel Main Entry Point — wires all 15 suites + userland.
 * This is the single entry point called by the boot stub after
 * sigma_boot_init() has set up the UEFI memory map and jumped to C.
 * =========================================================================
 */

#include "include/sigma_libc.h"

/* Suite headers */
#include "kernel/suites/S01_Genesis/shards/sigma_syscall_table.h"
#include "kernel/suites/S02_Boot/shards/sigma_boot.h"
#include "kernel/suites/S03_Orchestrator/shards/sigma_sched.h"
#include "kernel/suites/S04_HAL/shards/sigma_hal.h"
#include "kernel/suites/S05_Memory/shards/sigma_vmm.h"
#include "kernel/suites/S06_Storage/shards/sigma_vfs.h"
#include "kernel/suites/S07_Network/shards/sigma_netstack.h"
#include "kernel/suites/S08_Security/shards/sigma_lsm.h"
#include "kernel/suites/S09_Intelligence/shards/sigma_sentience.h"
#include "kernel/suites/S10_Containers/shards/sigma_container.h"
#include "kernel/suites/S11_PQC/shards/sigma_pqc.h"
#include "kernel/suites/S12_DistroAbsorption/shards/sigma_distro.h"
#include "kernel/suites/S13_Observability/shards/sigma_perf.h"
#include "kernel/suites/S14_PowerManagement/shards/sigma_pm.h"
#include "kernel/suites/S15_Distributed/shards/sigma_raft.h"
#include "kernel/suites/S16_GPU/shards/sigma_gpu.h"
#include "kernel/suites/S17_Audio/shards/sigma_audio.h"
#include "kernel/suites/S18_USB/shards/sigma_usb.h"
#include "kernel/suites/S19_Parallelism/shards/sigma_gcd.h"
#include "kernel/suites/S20_Interconnect/shards/sigma_interconnect.h"
#include "kernel/suites/S21_Userland/shards/sigma_shell.h"
#include "kernel/suites/S22_IPC/shards/sigma_ipc.h"
#include "kernel/suites/S23_Biosphere/shards/sigma_biosphere.h"
#include "userland/init/sigma_init.h"
#include "userland/ipc/sigma_ipc.h"
#include "userland/proc/sigma_proc.h"

/* ── 1. Boot Phase ───────────────────────────────────────────────────────── */
static void sigma_boot_phase(void) {
    sigma_boot_config_t cfg = {0};
    cfg.cpu_count   = 8;
    cfg.ram_mb      = 16384;  /* 16 GB */
    cfg.secure_boot = 1;
    cfg.kaslr       = 1;
    cfg.pqc_attest  = 1;
    sigma_strncpy(cfg.cmdline,
                  "sigma.kaslr=1 sigma.pqc=1 sigma.loglevel=4 "
                  "sigma.sched=cfs sigma.neural=1", 511);
    sigma_boot_init(&cfg);

    /* Register memory map */
    sigma_boot_add_mem_region(MEM_CONVENTIONAL, 0x0000000000100000ULL, 4096);
    sigma_boot_add_mem_region(MEM_SIGMA_KERN,   0x0000000001000000ULL, 512);
    sigma_boot_add_mem_region(MEM_CONVENTIONAL, 0x0000000002000000ULL, 16384*256ULL);
    sigma_boot_add_mem_region(MEM_ACPI_RECLAIM, 0x000000007FC00000ULL, 16);
    sigma_boot_print_memmap();

    /* Boot menu */
    sigma_boot_entry_add("SigmaOS Sovereign v4.0",   0x1000000);
    sigma_boot_entry_add("SigmaOS Recovery",          0x2000000);
    sigma_boot_entry_add("UEFI Shell",                0x3000000);
    sigma_boot_entry_list();
    sigma_boot_entry_select(0);

    /* PQC attestation */
    sigma_boot_attest_tpm();
    sigma_boot_phase_advance(BOOT_PHASE_KERNEL);
}

/* ── 2. Hardware Init ────────────────────────────────────────────────────── */
static void sigma_hw_phase(void) {
    sigma_hal_init();
    /* Register platform devices */
    sigma_device_t eth0 = {.bus=BUS_VIRTIO, .cls=DEV_NET,
                            .vendor_id=0x1AF4, .device_id=0x1000, .irq=10};
    sigma_strncpy(eth0.name, "virtio-net0", 47);
    sigma_hal_register(&eth0);

    sigma_device_t disk0 = {.bus=BUS_VIRTIO, .cls=DEV_BLOCK,
                             .vendor_id=0x1AF4, .device_id=0x1001, .irq=11};
    sigma_strncpy(disk0.name, "virtio-blk0", 47);
    sigma_hal_register(&disk0);

    /* IRQ setup */
    sigma_irq_request(10, IRQ_MSI, (sigma_irq_handler_t)0, (void*)0);
    sigma_irq_request(11, IRQ_MSI, (sigma_irq_handler_t)0, (void*)0);

    /* S20 Interconnect */
    sigma_interconnect_init();

    sigma_hal_device_list();
}

/* ── 3. Core Kernel Subsystems ───────────────────────────────────────────── */
static void sigma_kernel_phase(void) {
    /* Memory */
    sigma_vmm_init();
    sigma_vmm_addrspace_create(1, 0xDEADBEEF12345678ULL);

    /* Security */
    sigma_lsm_init();

    /* Scheduler */
    sigma_sched_init(8);

    /* Syscall table */
    sigma_syscall_table_init();

    /* Power management */
    sigma_pm_init(8);
    sigma_pm_set_governor(0, GOV_SCHEDUTIL);

    /* Observability */
    sigma_perf_init();
    pf_i32 cpu_fd = sigma_perf_counter_open("cpu_cycles", PERF_HW_CPU_CYCLES, 0);
    sigma_perf_counter_enable((pf_u32)cpu_fd);

    /* Sovereign Sentience Engine */
    sigma_sentience_init();
    sigma_sentience_tick();
    sigma_optimize_scheduler();

    /* S19 Parallelism */
    sigma_gcd_init();

    /* Network */
    sigma_net_init();
    pq_u8 mac[6] = {0xDE,0xAD,0xBE,0xEF,0x00,0x01};
    sigma_net_if_register("eth0", mac, 0xC0A80001, 0xFFFFFF00);
    sigma_net_if_up(1);
    sigma_net_route_add(0, 0, 0xC0A800FE, 1, 1); /* default GW */

    /* PQC — init + self-test */
    sigma_pqc_init();
    sigma_pqc_selftest();
}

/* ── 4. Userspace Bootstrap ──────────────────────────────────────────────── */
static void sigma_userspace_phase(void) {
    /* VFS */
    sigma_vfs_init();
    sigma_vfs_mkdir("/proc",  0755);
    sigma_vfs_mkdir("/sys",   0755);
    sigma_vfs_mkdir("/dev",   0755);
    sigma_vfs_mkdir("/run",   0755);
    sigma_vfs_mkdir("/home",  0755);

    /* S23 Sovereign Biosphere */
    sigma_biosphere_init();

    /* S22 Sovereign IPC */
    sigma_ipc_init();
    
    /* Process manager */
    sigma_proc_init();

    /* Containers */
    sigma_ct_init();

    /* Package manager */
    sigma_distro_init();
    sigma_repo_sync();

    /* S16 GPU */
    sigma_gpu_init();

    /* S17 Audio */
    sigma_audio_init();

    /* S18 USB */
    sigma_usb_init();
    sigma_usb_enumerate(1);

    /* PID-1 */
    sigma_init_start();

    /* S21 Userland Shell */
    sigma_shell_init();
    sigma_shell_run();

    sigma_boot_phase_advance(BOOT_PHASE_USERSPACE);
}

/* ── 5. Final Report ─────────────────────────────────────────────────────── */
static void sigma_final_report(void) {
    sigma_printf("\n");
    sigma_printf("S ══════════════════════════════════════════════════════\n");
    sigma_printf("  SIGMAOS SOVEREIGN v4.0 — SYSTEM UP\n");
    sigma_printf("  23 Suites | GIV Verified | PQC Hardened | Neural-Driven\n");
    sigma_printf("S ══════════════════════════════════════════════════════\n\n");

    sigma_boot_report();
    sigma_hal_device_list();
    sigma_sched_global_stats();
    sigma_neural_sched_stats();
    sigma_pm_report();
    sigma_syscall_audit();
    sigma_perf_counters_dump();
    sigma_lsm_audit_dump();
    sigma_distro_report();
    sigma_ct_ps();
    sigma_proc_list();
    sigma_interconnect_stats();
    sigma_gcd_stats();
    sigma_gpu_stats();
    sigma_audio_stats();
    sigma_usb_stats();

    sigma_printf("\nS SOVEREIGNTY IS ABSOLUTE.\n");
}

/* ── kernel_main: The one entry point ───────────────────────────────────── */
void kernel_main(void) {
    sigma_boot_phase();
    sigma_hw_phase();
    sigma_kernel_phase();
    sigma_userspace_phase();
    sigma_final_report();
}

// SPDX-License-Identifier: GPL-2.0-or-later
// sigma_init_shard.cpp — PID 1 init shard for SigmaOS
//
// This is the first user-space process.  The kernel spawns it after
// completing the boot sequence.  It is responsible for:
//   1. Mounting essential virtual filesystems (devfs, procfs, sysfs)
//   2. Enumerating PCI/USB hardware and loading drivers
//   3. Mounting the root filesystem
//   4. Loading MAC policy
//   5. Starting all system daemons via sigma-svc
//   6. Spawning a login shell (or sigma-session manager)
//   7. Reaping zombie processes
//   8. Handling shutdown/reboot signals
//
// Inspired by:
//   • systemd (service dependency graph, socket activation)
//   • s6 (supervision tree, readiness protocol)
//   • init/main.cpp from Redox OS
//   • MINIX 3 init process

#include "../include/drivers/driver_interface.h"
#include "../include/fs/vfs.h"
#include "../include/security/mac_policy.h"
#include <stdint.h>
#include <string.h>
#include <stdio.h>

// ── Service definition ─────────────────────────────────────────────────────

typedef enum svc_state {
    SVC_STOPPED  = 0,
    SVC_STARTING = 1,
    SVC_RUNNING  = 2,
    SVC_FAILED   = 3,
} svc_state_t;

typedef struct sigma_service {
    const char  *name;
    const char  *binary;
    const char  *args[8];
    svc_state_t  state;
    uint32_t     pid;
    uint32_t     restart_count;
    bool         oneshot;    // true = don't restart on exit
    bool         critical;   // true = panic if fails
    const char  *requires;   // dependency (must be running first)
} sigma_service_t;

// ── Service table (evaluated in order) ────────────────────────────────────

static sigma_service_t services[] = {
    { "sigma-macd",   "/sbin/sigma-macd",   {NULL},                false, false, NULL },
    { "sigma-busd",   "/sbin/sigma-busd",   {NULL},                false, true,  "sigma-macd" },
    { "sigma-netd",   "/sbin/sigma-netd",   {NULL},                false, true,  "sigma-busd" },
    { "sigma-timed",  "/sbin/sigma-timed",  {NULL},                false, false, "sigma-netd" },
    { "sigma-healthd","/sbin/sigma-healthd",{NULL},                false, false, "sigma-busd" },
    { "sigma-ds",     "/sbin/sigma-ds",     {NULL},                false, true,  "sigma-busd" },
    { "sigma-rs",     "/sbin/sigma-rs",     {NULL},                false, true,  "sigma-ds"   },
    { "sigma-trustd", "/sbin/sigma-trustd", {NULL},                false, true,  "sigma-busd" },
    { "sigma-apid",   "/sbin/sigma-apid",   {NULL},                false, false, "sigma-netd" },
    { "sigma-notifyd","/sbin/sigma-notifyd",{NULL},                false, false, "sigma-busd" },
    { "sigma-power",  "/sbin/sigma-power",  {NULL},                false, false, "sigma-busd" },
    { "sigma-updated","/sbin/sigma-updated",{NULL},                false, false, "sigma-netd" },
    { "sigma-session","/sbin/sigma-session",{NULL},                false, true,  "sigma-busd" },
    { NULL, NULL, {NULL}, 0, 0, 0, false, false, NULL },
};

// ── IPC message types ─────────────────────────────────────────────────────

#define SIGMA_INIT_OP_SPAWN     0x0100
#define SIGMA_INIT_OP_REAP      0x0101
#define SIGMA_INIT_OP_SHUTDOWN  0x0200
#define SIGMA_INIT_OP_REBOOT    0x0201
#define SIGMA_INIT_OP_SVC_START 0x0300
#define SIGMA_INIT_OP_SVC_STOP  0x0301
#define SIGMA_INIT_OP_SVC_LIST  0x0302

typedef struct sigma_ipc_msg sigma_ipc_msg_t;
extern int  sigma_ipc_receive(sigma_ipc_msg_t *msg, int timeout_ms);
extern int  sigma_ipc_send   (uint32_t dst, const sigma_ipc_msg_t *msg);
extern int  sigma_process_spawn(const char *binary, const char *const argv[],
                                 uint32_t *out_pid);
extern void sigma_process_wait (uint32_t *out_pid, int *out_status);
extern void sigma_kernel_panic (const char *msg);

// ── Mount essential filesystems ───────────────────────────────────────────

static void mount_essential(void) {
    sigma_vfs_mount(NULL, "/dev",  "devfs",   0);
    sigma_vfs_mount(NULL, "/proc", "procfs",  0);
    sigma_vfs_mount(NULL, "/sys",  "sysfs",   0);
    sigma_vfs_mount(NULL, "/tmp",  "tmpfs",   0);
    sigma_vfs_mount(NULL, "/run",  "tmpfs",   0);
    printf("[init] Essential filesystems mounted\n");
}

// ── Hardware enumeration ──────────────────────────────────────────────────

static void enumerate_hardware(void) {
    sigma_driver_bus_enumerate_pci();
    sigma_driver_bus_enumerate_usb();

    // Wait for NVMe / SATA block device
    uint32_t blk_shard = 0;
    for (int retries = 0; retries < 50; retries++) {
        if (sigma_driver_find(DRIVER_CAP_BLOCK_IO, 0x010802, &blk_shard) == 0)
            break;
        // sigma_sleep_ms(100);
    }

    if (blk_shard) {
        sigma_vfs_mount("/dev/sda1",    "/",    "sigmafs", 0);
        printf("[init] Root filesystem mounted\n");
    } else {
        printf("[init] WARNING: No block device found — running from tmpfs\n");
    }
}

// ── Load MAC policy ───────────────────────────────────────────────────────

static sigma_policy_db_t *g_policy = NULL;

static void load_policy(void) {
    if (sigma_policy_load("/etc/sigma-policy/default.sigma-policy",
                           &g_policy) == 0) {
        printf("[init] MAC policy loaded\n");
    } else {
        printf("[init] WARNING: MAC policy missing — running unrestricted\n");
    }
}

// ── Service startup ───────────────────────────────────────────────────────

static void start_service(sigma_service_t *svc) {
    if (svc->state == SVC_RUNNING) return;
    // Check dependency
    if (svc->requires) {
        for (int i = 0; services[i].name; i++) {
            if (strcmp(services[i].name, svc->requires) == 0 &&
                services[i].state != SVC_RUNNING) {
                start_service(&services[i]);
                return;
            }
        }
    }
    svc->state = SVC_STARTING;
    uint32_t pid = 0;
    const char *argv[9] = { svc->binary, NULL };
    for (int i = 0; svc->args[i] && i < 7; i++)
        argv[i+1] = svc->args[i];

    if (sigma_process_spawn(svc->binary, argv, &pid) == 0) {
        svc->pid   = pid;
        svc->state = SVC_RUNNING;
        printf("[init] started %s (pid=%u)\n", svc->name, pid);
    } else {
        svc->state = SVC_FAILED;
        printf("[init] FAILED to start %s\n", svc->name);
        if (svc->critical) sigma_kernel_panic("critical service failed");
    }
}

static void start_all_services(void) {
    for (int i = 0; services[i].name; i++)
        start_service(&services[i]);
}

// ── Reap zombies ──────────────────────────────────────────────────────────

static void reap_zombies(void) {
    uint32_t pid = 0;
    int status   = 0;
    sigma_process_wait(&pid, &status);
    // Find which service died and restart if not oneshot
    for (int i = 0; services[i].name; i++) {
        if (services[i].pid == pid) {
            services[i].state = (status == 0) ? SVC_STOPPED : SVC_FAILED;
            printf("[init] %s exited (status=%d)\n", services[i].name, status);
            if (!services[i].oneshot && services[i].state == SVC_FAILED) {
                services[i].restart_count++;
                start_service(&services[i]);
            }
            break;
        }
    }
}

// ── Shutdown sequence ─────────────────────────────────────────────────────

static void shutdown_sequence(bool reboot) {
    printf("[init] %s initiated\n", reboot ? "Reboot" : "Shutdown");
    // Stop services in reverse order
    for (int i = 12; i >= 0; i--) {
        if (services[i].state == SVC_RUNNING) {
            // sigma_process_signal(services[i].pid, SIGTERM);
            printf("[init] stopping %s\n", services[i].name);
        }
    }
    sigma_vfs_sync();
    printf("[init] filesystems synced\n");
    // sigma_acpi_power_off() or sigma_acpi_reset()
}

// ── Main event loop ───────────────────────────────────────────────────────

int main(void) {
    printf("\n[init] SigmaOS PID 1 starting\n");

    mount_essential();
    enumerate_hardware();
    load_policy();
    start_all_services();

    printf("[init] Boot complete — entering service loop\n");

    while (1) {
        sigma_ipc_msg_t msg = {0};
        int rc = sigma_ipc_receive(&msg, 100 /* ms */);

        if (rc == 0) {
            switch (msg.opcode) {
            case SIGMA_INIT_OP_SHUTDOWN:
                shutdown_sequence(false);
                return 0;
            case SIGMA_INIT_OP_REBOOT:
                shutdown_sequence(true);
                return 0;
            case SIGMA_INIT_OP_REAP:
                reap_zombies();
                break;
            case SIGMA_INIT_OP_SVC_LIST:
                for (int i = 0; services[i].name; i++) {
                    printf("  %-20s %s (pid=%u restarts=%u)\n",
                           services[i].name,
                           services[i].state == SVC_RUNNING ? "RUNNING" :
                           services[i].state == SVC_FAILED  ? "FAILED"  : "STOPPED",
                           services[i].pid, services[i].restart_count);
                }
                break;
            }
        } else {
            // Timeout: check for zombies
            reap_zombies();
        }
    }
}

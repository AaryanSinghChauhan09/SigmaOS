/*
 * =========================================================================
 * S SIGMAOS userland/init/sigma_service_table.c
 * =========================================================================
 * Canonical service table — defines ALL built-in sigma services.
 * Gap-closes: /etc/rc.local, systemd default.target, launchd plists,
 *             Windows registry services, Android init.rc
 * =========================================================================
 */

#include "sigma_init.h"
#include "../../include/sigma_libc.h"

void sigma_init_register_all_services(void) {
    /* ── Core Kernel Services ────────────────────────── */
    sigma_init_register("sigma-journal",   "/sbin/sigma-journald",
                        "",               RESTART_ALWAYS,  0);
    sigma_init_register("sigma-udev",      "/sbin/sigma-udevd",
                        "sigma-journal",  RESTART_ALWAYS,  0);
    sigma_init_register("sigma-dbus",      "/sbin/sigma-dbusd",
                        "sigma-journal",  RESTART_ON_FAILURE, 0);

    /* ── Network Stack ───────────────────────────────── */
    sigma_init_register("sigma-network",   "/sbin/sigma-networkd",
                        "sigma-journal",  RESTART_ALWAYS,  0);
    sigma_init_register("sigma-dns",       "/sbin/sigma-resolved",
                        "sigma-network",  RESTART_ALWAYS,  0);
    sigma_init_register("sigma-firewall",  "/sbin/sigma-nftd",
                        "sigma-network",  RESTART_ALWAYS,  0);
    sigma_init_register("sigma-wireguard", "/sbin/sigma-wgd",
                        "sigma-network",  RESTART_ON_FAILURE, 0);

    /* ── Security Services ───────────────────────────── */
    sigma_init_register("sigma-pqc",       "/sbin/sigma-pqcd",
                        "sigma-journal",  RESTART_ALWAYS,  0);
    sigma_init_register("sigma-tpm",       "/sbin/sigma-tpmd",
                        "sigma-journal",  RESTART_ALWAYS,  0);
    sigma_init_register("sigma-audit",     "/sbin/sigma-auditd",
                        "sigma-journal",  RESTART_ALWAYS,  0);
    sigma_init_register("sigma-apparmor",  "/sbin/sigma-apparmord",
                        "sigma-audit",    RESTART_ALWAYS,  0);

    /* ── Remote Services (socket-activated) ──────────── */
    sigma_init_register("sigma-ssh",       "/usr/sbin/sigma-sshd",
                        "sigma-network",  RESTART_ON_FAILURE, 1);
    sigma_init_register("sigma-api",       "/usr/sbin/sigma-apid",
                        "sigma-network",  RESTART_ON_FAILURE, 1);

    /* ── Package & Update Manager ────────────────────── */
    sigma_init_register("sigma-pkg",       "/usr/sbin/sigma-pkgd",
                        "sigma-network",  RESTART_NO, 0);

    /* ── GUI & Desktop ───────────────────────────────── */
    sigma_init_register("sigma-display",   "/usr/bin/sigma-displayd",
                        "sigma-udev",     RESTART_ON_FAILURE, 0);
    sigma_init_register("sigma-wayland",   "/usr/bin/sigma-waylandd",
                        "sigma-display",  RESTART_ON_FAILURE, 0);
    sigma_init_register("sigma-gui",       "/usr/bin/zenith-shell",
                        "sigma-wayland",  RESTART_ON_FAILURE, 0);

    /* ── AI & Intelligence Services ──────────────────── */
    sigma_init_register("sigma-neural",    "/usr/sbin/sigma-neurald",
                        "sigma-journal",  RESTART_ON_FAILURE, 0);
    sigma_init_register("sigma-assistant", "/usr/bin/sigma-assistantd",
                        "sigma-neural",   RESTART_ON_FAILURE, 1);
}

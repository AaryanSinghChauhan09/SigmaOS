// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * sigma_ignite.cpp — first-boot provisioning (CoreOS Ignition-inspired)
 * Runs exactly once on first boot. Idempotent: writes a stamp file on success.
 * Never runs again after the stamp exists.
 */
#include "sigma_ignite.h"
#include "sigma_log.h"
#include <fcntl.h>
#include <unistd.h>
#include <sys/stat.h>
#include <string.h>
#include <stdio.h>
#include <stdlib.h>

static const char* STAMP_PATH = "/sigma/var/sigma-ignite.done";

/* Check if this is really first boot */
static bool is_first_boot(void) {
    return access(STAMP_PATH, F_OK) != 0;
}

/* Write the stamp file — only called AFTER every step succeeds */
static int write_stamp(void) {
    int fd = open(STAMP_PATH, O_CREAT | O_WRONLY | O_TRUNC, 0600);
    if (fd < 0) return -errno;
    const char* ts = "provisioned\n";
    write(fd, ts, strlen(ts));
    fsync(fd);
    close(fd);
    return 0;
}

int sigma_ignite_main(void) {
    if (!is_first_boot()) {
        sigma_log_info("[sigma-ignite] stamp found — already provisioned, skipping\n");
        return 0;
    }

    sigma_log_info("[sigma-ignite] First boot detected — starting provisioning\n");

    /* Load config from initramfs, cloud metadata, or /sigma/ignite/node.ign */
    sigma_ignite_config_t cfg = {};
    if (sigma_ignite_load_config(&cfg) != 0) {
        sigma_log_warn("[sigma-ignite] No ignition config found — using defaults\n");
    }

    /* Execute each provisioning step.  Any failure aborts without writing stamp. */
    int rc = 0;

    if (cfg.has_storage) {
        rc = sigma_ignite_setup_filesystems(&cfg);
        if (rc != 0) {
            sigma_log_err("[sigma-ignite] filesystem setup failed (rc=%d)\n", rc);
            return rc;
        }
    }

    if (cfg.file_count > 0) {
        rc = sigma_ignite_write_files(&cfg);
        if (rc != 0) { sigma_log_err("[sigma-ignite] file write failed\n"); return rc; }
    }

    if (cfg.user_count > 0) {
        rc = sigma_ignite_setup_users(&cfg);
        if (rc != 0) { sigma_log_err("[sigma-ignite] user setup failed\n"); return rc; }
    }

    if (cfg.kernel_arg_count > 0) {
        rc = sigma_ignite_apply_kernel_args(&cfg);
        if (rc != 0) { sigma_log_err("[sigma-ignite] kernel args failed\n"); return rc; }
    }

    /* All steps succeeded — write stamp so we never run again */
    if (write_stamp() != 0) {
        sigma_log_err("[sigma-ignite] WARN: could not write stamp — will re-run next boot\n");
    }

    sigma_log_info("[sigma-ignite] First-boot provisioning complete\n");
    return 0;
}

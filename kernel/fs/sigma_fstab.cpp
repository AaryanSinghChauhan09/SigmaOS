// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * sigma_fstab.cpp — hardened mount table (OCI runc rootfs_linux.go-inspired)
 * Every mount gets MS_NOEXEC|MS_NOSUID|MS_NODEV by default.
 * hidepid=2 on /proc prevents process enumeration from sandboxed workloads.
 */
#include "sigma_fstab.h"
#include "sigma_log.h"
#include <sys/mount.h>
#include <string.h>
#include <errno.h>

#define SIGMA_DEFAULT_FLAGS (MS_NOEXEC | MS_NOSUID | MS_NODEV | MS_RELATIME)

static const sigma_mount_entry_t sigma_fstab[] = {
    { "proc",        "/proc",       "proc",        SIGMA_DEFAULT_FLAGS,               "hidepid=2" },
    { "sysfs",       "/sys",        "sysfs",        SIGMA_DEFAULT_FLAGS | MS_RDONLY,   NULL },
    { "devtmpfs",    "/dev",        "devtmpfs",     MS_NOSUID | MS_RELATIME,           "mode=755,size=5%" },
    { "devpts",      "/dev/pts",    "devpts",       MS_NOEXEC | MS_NOSUID,             "gid=5,mode=620" },
    { "tmpfs",       "/tmp",        "tmpfs",        SIGMA_DEFAULT_FLAGS,               "size=512M,mode=1777" },
    { "tmpfs",       "/run",        "tmpfs",        SIGMA_DEFAULT_FLAGS,               "mode=755,size=64M" },
    { "sigma_semfs", "/sigma/sys",  "sigma_semfs",  SIGMA_DEFAULT_FLAGS | MS_RDONLY,   NULL },
    /* /sigma/data is mounted by sigma-cryptfs before this table runs */
    { "/dev/mapper/sigma-data", "/sigma/data", "ext4", SIGMA_DEFAULT_FLAGS, "errors=remount-ro" },
};
static const int FSTAB_COUNT = (int)(sizeof(sigma_fstab) / sizeof(sigma_fstab[0]));

int sigma_mount_all(void) {
    for (int i = 0; i < FSTAB_COUNT; i++) {
        const sigma_mount_entry_t* e = &sigma_fstab[i];
        if (mount(e->device, e->mountpoint, e->fstype, e->flags, e->options) != 0) {
            /* Non-fatal for optional mounts (e.g. sigma-data not present in QEMU) */
            sigma_log_warn("[sigma-fstab] mount(%s → %s [%s]): %s\n",
                           e->device, e->mountpoint, e->fstype, strerror(errno));
            continue;
        }
        sigma_log_info("[sigma-fstab] mounted %s → %s (flags=0x%lx)\n",
                       e->device, e->mountpoint, (unsigned long)e->flags);
    }
    return 0;
}

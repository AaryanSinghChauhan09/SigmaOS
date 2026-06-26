// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * sigma_cgroup.cpp — cgroup v2 resource management (OCI runc libcontainer/cgroups/fs2-inspired)
 *
 * Writes resource limits directly to cgroup v2 controller files under
 * /sys/fs/cgroup/sigma/<name>/. If ANY limit write fails, the cgroup is
 * destroyed and the workload is not started — matches runc's behavior.
 */
#include "sigma_cgroup.h"
#include "sigma_log.h"
#include <stdio.h>
#include <string.h>
#include <fcntl.h>
#include <unistd.h>
#include <errno.h>
#include <sys/stat.h>

#define SIGMA_CGROOT "/sys/fs/cgroup/sigma"

static int cg_write(const char* cg_name, const char* file, const char* value) {
    char path[256];
    snprintf(path, sizeof(path), "%s/%s/%s", SIGMA_CGROOT, cg_name, file);
    int fd = open(path, O_WRONLY | O_TRUNC);
    if (fd < 0) {
        sigma_log_warn("[sigma-cgroup] cannot open %s: %s\n", path, strerror(errno));
        return -errno;
    }
    ssize_t n = write(fd, value, strlen(value));
    close(fd);
    if (n < 0) {
        sigma_log_err("[sigma-cgroup] write to %s failed: %s\n", path, strerror(errno));
        return -errno;
    }
    return 0;
}

static int cg_read_u64(const char* cg_name, const char* file, sigma_u64* out) {
    char path[256], buf[64];
    snprintf(path, sizeof(path), "%s/%s/%s", SIGMA_CGROOT, cg_name, file);
    int fd = open(path, O_RDONLY);
    if (fd < 0) return -errno;
    ssize_t n = read(fd, buf, sizeof(buf) - 1);
    close(fd);
    if (n <= 0) return -1;
    buf[n] = '\0';
    *out = (sigma_u64)strtoull(buf, nullptr, 10);
    return 0;
}

int sigma_cgroup_create(const char* name, const sigma_cgroup_resources_t* r) {
    char cg_dir[256];
    snprintf(cg_dir, sizeof(cg_dir), "%s/%s", SIGMA_CGROOT, name);

    if (mkdir(cg_dir, 0755) != 0 && errno != EEXIST) {
        sigma_log_err("[sigma-cgroup] mkdir %s failed: %s\n", cg_dir, strerror(errno));
        return -errno;
    }

    char buf[64];
    int  rc = 0;

    /* ── CPU limits ─────────────────────────────────────────────────────── */
    if (r->cpu_quota_us >= 0) {
        snprintf(buf, sizeof(buf), "%lld %lld",
                 (long long)r->cpu_quota_us, (long long)r->cpu_period_us);
        rc |= cg_write(name, "cpu.max", buf);
    } else {
        rc |= cg_write(name, "cpu.max", "max 100000");
    }

    snprintf(buf, sizeof(buf), "%llu", (unsigned long long)r->cpu_shares);
    rc |= cg_write(name, "cpu.weight", buf);

    /* ── Memory limits ──────────────────────────────────────────────────── */
    if (r->mem_limit_bytes >= 0) {
        snprintf(buf, sizeof(buf), "%lld", (long long)r->mem_limit_bytes);
        rc |= cg_write(name, "memory.max", buf);
    } else {
        rc |= cg_write(name, "memory.max", "max");
    }
    rc |= cg_write(name, "memory.swap.max", "0");   /* no swap — security */

    if (r->mem_low_bytes >= 0) {
        snprintf(buf, sizeof(buf), "%lld", (long long)r->mem_low_bytes);
        rc |= cg_write(name, "memory.low", buf);
    }

    /* ── PID limit ──────────────────────────────────────────────────────── */
    if (r->pids_max >= 0) {
        snprintf(buf, sizeof(buf), "%lld", (long long)r->pids_max);
        rc |= cg_write(name, "pids.max", buf);
    } else {
        rc |= cg_write(name, "pids.max", "max");
    }

    /* ── I/O weight ─────────────────────────────────────────────────────── */
    snprintf(buf, sizeof(buf), "%u", (unsigned)r->io_weight);
    rc |= cg_write(name, "io.weight", buf);

    if (rc != 0) {
        /* Any failed write → destroy the cgroup and refuse to start */
        sigma_log_err("[sigma-cgroup] failed to set limits for '%s' — destroying\n", name);
        sigma_cgroup_destroy(name);
        return -1;
    }

    sigma_log_info("[sigma-cgroup] created '%s' mem=%lldMB pids=%lld io_weight=%u\n",
                   name,
                   (long long)(r->mem_limit_bytes < 0 ? -1 : r->mem_limit_bytes / (1024*1024)),
                   (long long)r->pids_max,
                   (unsigned)r->io_weight);
    return 0;
}

int sigma_cgroup_enter(const char* name, pid_t pid) {
    char buf[32];
    snprintf(buf, sizeof(buf), "%d", (int)pid);
    int rc = cg_write(name, "cgroup.procs", buf);
    if (rc == 0)
        sigma_log_info("[sigma-cgroup] pid %d → cgroup '%s'\n", (int)pid, name);
    return rc;
}

int sigma_cgroup_destroy(const char* name) {
    char cg_dir[256];
    snprintf(cg_dir, sizeof(cg_dir), "%s/%s", SIGMA_CGROOT, name);
    if (rmdir(cg_dir) != 0 && errno != ENOENT) {
        sigma_log_warn("[sigma-cgroup] rmdir %s failed: %s\n", cg_dir, strerror(errno));
        return -errno;
    }
    sigma_log_info("[sigma-cgroup] destroyed '%s'\n", name);
    return 0;
}

int sigma_cgroup_stat(const char* name, sigma_cgroup_stat_t* out) {
    memset(out, 0, sizeof(*out));
    cg_read_u64(name, "memory.current",   &out->mem_current_bytes);
    cg_read_u64(name, "memory.peak",      &out->mem_peak_bytes);
    cg_read_u64(name, "pids.current",     &out->pids_current);
    /* cpu.stat and io.stat are multi-line — parse inline here */
    return 0;
}

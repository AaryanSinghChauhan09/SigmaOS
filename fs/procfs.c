/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: PROC-LIKE VIRTUAL FILESYSTEM
 * =============================================================================
 * Inspired by: Linux kernel fs/proc/ (procfs)
 *              Plan 9 synthetic file systems
 *              FreeBSD linprocfs
 * =============================================================================
 * Exposes kernel runtime state as readable virtual files in /proc/.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

#define PROCFS_MAX_ENTRIES  64
#define PROCFS_NAME_LEN     48
#define PROCFS_CONTENT_LEN  512

typedef int (*procfs_read_fn)(char* buf, sigma_u32 buf_size);

typedef struct {
    char          name[PROCFS_NAME_LEN];
    procfs_read_fn read_fn;
    sigma_bool    active;
} procfs_entry_t;

static procfs_entry_t procfs_table[PROCFS_MAX_ENTRIES];
static sigma_u32 procfs_count = 0;

/* --- Built-in /proc entries --- */

static int proc_version(char* buf, sigma_u32 buf_size) {
    const char* ver = "SigmaOS Zenith v15.2 (sovereign-microkernel) x86_64 #1 SMP SIGMA";
    sigma_u32 i = 0;
    while (i < buf_size - 1 && ver[i]) { buf[i] = ver[i]; i++; }
    buf[i] = '\0';
    return (int)i;
}

static int proc_uptime(char* buf, sigma_u32 buf_size) {
    /* Simulated uptime: 12345.67 seconds, 11000.00 idle */
    const char* upt = "12345.67 11000.00";
    sigma_u32 i = 0;
    while (i < buf_size - 1 && upt[i]) { buf[i] = upt[i]; i++; }
    buf[i] = '\0';
    return (int)i;
}

static int proc_meminfo(char* buf, sigma_u32 buf_size) {
    const char* info =
        "MemTotal:     2097152 kB\n"
        "MemFree:      1572864 kB\n"
        "MemAvailable: 1835008 kB\n"
        "Buffers:        65536 kB\n"
        "Cached:        262144 kB\n"
        "SwapTotal:          0 kB\n"
        "SwapFree:           0 kB\n";
    sigma_u32 i = 0;
    while (i < buf_size - 1 && info[i]) { buf[i] = info[i]; i++; }
    buf[i] = '\0';
    return (int)i;
}

static int proc_cpuinfo(char* buf, sigma_u32 buf_size) {
    const char* info =
        "processor\t: 0\n"
        "vendor_id\t: SigmaSilicon\n"
        "model name\t: Sovereign Zenith CPU @ 5.2GHz\n"
        "cpu MHz\t\t: 5200.000\n"
        "cache size\t: 16384 KB\n"
        "cpu cores\t: 8\n"
        "flags\t\t: fpu vme de pse tsc msr pqc avx512 rdrand\n";
    sigma_u32 i = 0;
    while (i < buf_size - 1 && info[i]) { buf[i] = info[i]; i++; }
    buf[i] = '\0';
    return (int)i;
}

static int proc_loadavg(char* buf, sigma_u32 buf_size) {
    const char* la = "0.15 0.10 0.05 1/128 42";
    sigma_u32 i = 0;
    while (i < buf_size - 1 && la[i]) { buf[i] = la[i]; i++; }
    buf[i] = '\0';
    return (int)i;
}

static int proc_filesystems(char* buf, sigma_u32 buf_size) {
    const char* fs =
        "nodev\tprocfs\n"
        "nodev\tsysfs\n"
        "nodev\ttmpfs\n"
        "\text4\n"
        "\tfat32\n"
        "\tsigmafs\n";
    sigma_u32 i = 0;
    while (i < buf_size - 1 && fs[i]) { buf[i] = fs[i]; i++; }
    buf[i] = '\0';
    return (int)i;
}

/* --- Public API --- */

void procfs_init(void) {
    sigma_memset(procfs_table, 0, sizeof(procfs_table));
    procfs_count = 0;

    /* Register built-in entries */
    procfs_register("version",     proc_version);
    procfs_register("uptime",      proc_uptime);
    procfs_register("meminfo",     proc_meminfo);
    procfs_register("cpuinfo",     proc_cpuinfo);
    procfs_register("loadavg",     proc_loadavg);
    procfs_register("filesystems", proc_filesystems);

    sigma_printf("[procfs] Virtual filesystem initialized (%u entries)\n", procfs_count);
}

int procfs_register(const char* name, procfs_read_fn fn) {
    if (procfs_count >= PROCFS_MAX_ENTRIES) return -1;
    procfs_entry_t* e = &procfs_table[procfs_count];
    sigma_u32 i = 0;
    while (i < PROCFS_NAME_LEN - 1 && name[i]) { e->name[i] = name[i]; i++; }
    e->name[i] = '\0';
    e->read_fn = fn;
    e->active  = SIGMA_TRUE;
    procfs_count++;
    return 0;
}

int procfs_read(const char* name, char* buf, sigma_u32 buf_size) {
    for (sigma_u32 i = 0; i < procfs_count; i++) {
        if (procfs_table[i].active) {
            /* strcmp inline */
            const char* a = procfs_table[i].name;
            const char* b = name;
            sigma_bool match = SIGMA_TRUE;
            while (*a && *b) {
                if (*a != *b) { match = SIGMA_FALSE; break; }
                a++; b++;
            }
            if (*a != *b) match = SIGMA_FALSE;

            if (match) {
                sigma_printf("[procfs] Reading /proc/%s\n", name);
                return procfs_table[i].read_fn(buf, buf_size);
            }
        }
    }
    sigma_printf("[procfs] ERR: /proc/%s not found\n", name);
    return -1;
}

void procfs_list(void) {
    sigma_printf("\n--- Σ /proc/ ENTRIES ---\n");
    for (sigma_u32 i = 0; i < procfs_count; i++) {
        if (procfs_table[i].active) {
            sigma_printf("  /proc/%s\n", procfs_table[i].name);
        }
    }
    sigma_printf("------------------------\n");
}

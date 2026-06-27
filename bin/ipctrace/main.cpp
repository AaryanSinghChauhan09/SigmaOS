// SPDX-License-Identifier: GPL-2.0-or-later
// ipctrace — IPC message tracer for SigmaOS
//
// Attaches to the kernel IPC tracer shard and prints every message
// exchanged between shards in real time.  Like strace but for IPC.
//
// Usage:
//   ipctrace                         # trace all IPC
//   ipctrace --shard sigma-netd      # trace a specific shard (by name)
//   ipctrace --opcode BLOCK_READ     # filter by opcode
//   ipctrace --from sigma-pkg        # only messages from a shard
//   ipctrace --to sigma-vault        # only messages to a shard
//   ipctrace --since 1000            # only events after monotonic ns 1000
//   ipctrace --json                  # JSON output for tooling
//
// Inspired by: strace, bpftrace, LTTng, DTrace

#include <stdint.h>
#include <stddef.h>
#include <string.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

// ── IPC event record ──────────────────────────────────────────────────────

typedef struct ipc_event {
    uint64_t  timestamp_ns;   // monotonic nanoseconds since boot
    uint32_t  src_shard;
    uint32_t  dst_shard;
    uint32_t  opcode;
    size_t    payload_len;
    uint32_t  flags;
    uint8_t   payload_preview[32]; // first 32 bytes of payload
    int       status;         // 0 = success, negative = error
    uint64_t  latency_ns;     // round-trip latency (for request/response)
} ipc_event_t;

// ── Opcode name table ─────────────────────────────────────────────────────

static const struct { uint32_t op; const char *name; } opcode_names[] = {
    { 0x1000, "DRV_REGISTER"    },
    { 0x1001, "DRV_UNREGISTER"  },
    { 0x1002, "DRV_IRQ_NOTIFY"  },
    { 0x2000, "BLOCK_READ"      },
    { 0x2001, "BLOCK_WRITE"     },
    { 0x2002, "BLOCK_FLUSH"     },
    { 0x2003, "BLOCK_TRIM"      },
    { 0x3000, "VFS_OPEN"        },
    { 0x3001, "VFS_READ"        },
    { 0x3002, "VFS_WRITE"       },
    { 0x3003, "VFS_CLOSE"       },
    { 0x3004, "VFS_STAT"        },
    { 0x4000, "NET_SEND"        },
    { 0x4001, "NET_RECV"        },
    { 0x4002, "NET_CONNECT"     },
    { 0x4003, "NET_ACCEPT"      },
    { 0x5000, "SCHED_KICK"      },
    { 0x5001, "SCHED_YIELD"     },
    { 0x6000, "NOTIF_SEND"      },
    { 0x7000, "BPF_PROG_LOAD"   },
    { 0x7001, "BPF_MAP_UPDATE"  },
    { 0x0100, "INIT_SPAWN"      },
    { 0x0200, "INIT_SHUTDOWN"   },
    { 0x0201, "INIT_REBOOT"     },
    { 0, NULL }
};

static const char *opcode_name(uint32_t op) {
    for (int i = 0; opcode_names[i].name; i++)
        if (opcode_names[i].op == op) return opcode_names[i].name;
    static char buf[16];
    snprintf(buf, sizeof(buf), "0x%04x", op);
    return buf;
}

// ── Shard name resolution ─────────────────────────────────────────────────

// In production: query sigma-ds (service discovery) via IPC.
// Here: static table for well-known shards.
static const struct { uint32_t id; const char *name; } shard_names[] = {
    {  1, "init"          },
    {  2, "sigma-macd"    },
    {  3, "sigma-busd"    },
    {  4, "sigma-netd"    },
    {  5, "sigma-timed"   },
    {  6, "sigma-healthd" },
    {  7, "sigma-ds"      },
    {  8, "sigma-rs"      },
    {  9, "sigma-trustd"  },
    { 10, "sigma-apid"    },
    { 11, "sigma-vault"   },
    { 12, "sigma-pkg"     },
    { 13, "sigma-updated" },
    { 14, "nvme-driver"   },
    { 15, "ahci-driver"   },
    { 16, "e1000-driver"  },
    { 17, "hda-driver"    },
    { 18, "zenith-wm"     },
    { 0, NULL }
};

static const char *shard_name(uint32_t id) {
    for (int i = 0; shard_names[i].name; i++)
        if (shard_names[i].id == id) return shard_names[i].name;
    static char buf[16];
    snprintf(buf, sizeof(buf), "shard#%u", id);
    return buf;
}

// ── Filter config ─────────────────────────────────────────────────────────

typedef struct filter {
    uint32_t src_shard;     // 0 = any
    uint32_t dst_shard;     // 0 = any
    uint32_t opcode;        // 0 = any
    uint64_t since_ns;      // 0 = all
    bool     json;
    bool     latency;       // show latency histogram
    bool     count;         // count messages per opcode
} filter_t;

static filter_t g_filter = {0};

// ── Message counts per opcode (for --count mode) ─────────────────────────

static struct { uint32_t op; uint64_t count; uint64_t total_latency_ns; }
    g_counts[256];
static int g_count_n = 0;

static void count_event(const ipc_event_t *e) {
    for (int i = 0; i < g_count_n; i++) {
        if (g_counts[i].op == e->opcode) {
            g_counts[i].count++;
            g_counts[i].total_latency_ns += e->latency_ns;
            return;
        }
    }
    if (g_count_n < 256) {
        g_counts[g_count_n].op               = e->opcode;
        g_counts[g_count_n].count            = 1;
        g_counts[g_count_n].total_latency_ns = e->latency_ns;
        g_count_n++;
    }
}

static void print_counts(void) {
    printf("\n%-20s  %10s  %12s\n", "OPCODE", "COUNT", "AVG_LATENCY");
    printf("%-20s  %10s  %12s\n", "------", "-----", "-----------");
    for (int i = 0; i < g_count_n; i++) {
        uint64_t avg = g_counts[i].count ?
            g_counts[i].total_latency_ns / g_counts[i].count : 0;
        printf("%-20s  %10llu  %9llu ns\n",
               opcode_name(g_counts[i].op),
               (unsigned long long)g_counts[i].count,
               (unsigned long long)avg);
    }
}

// ── Event printer ─────────────────────────────────────────────────────────

static void print_event(const ipc_event_t *e) {
    if (g_filter.src_shard && e->src_shard != g_filter.src_shard) return;
    if (g_filter.dst_shard && e->dst_shard != g_filter.dst_shard) return;
    if (g_filter.opcode    && e->opcode    != g_filter.opcode)    return;
    if (e->timestamp_ns < g_filter.since_ns) return;

    if (g_filter.count) {
        count_event(e);
        return;
    }

    if (g_filter.json) {
        printf("{\"ts\":%llu,\"src\":\"%s\",\"dst\":\"%s\",\"op\":\"%s\","
               "\"len\":%zu,\"status\":%d,\"latency_ns\":%llu}\n",
               (unsigned long long)e->timestamp_ns,
               shard_name(e->src_shard), shard_name(e->dst_shard),
               opcode_name(e->opcode), e->payload_len,
               e->status, (unsigned long long)e->latency_ns);
        return;
    }

    // Human-readable
    printf("[%10.6f] %-16s → %-16s  %-18s  len=%-4zu",
           (double)e->timestamp_ns / 1e9,
           shard_name(e->src_shard), shard_name(e->dst_shard),
           opcode_name(e->opcode), e->payload_len);
    if (e->latency_ns)
        printf("  lat=%llu ns", (unsigned long long)e->latency_ns);
    if (e->status)
        printf("  ERR=%d", e->status);
    printf("\n");
}

// ── Kernel IPC tracer connection (via sigma-bpf ring buffer) ──────────────

extern int sigma_bpf_attach_ipc_tracer(void);  // returns ring_buf_fd
extern int sigma_bpf_ringbuf_read(int fd, void *buf, size_t len, int timeout_ms);

// ── Argument parsing ──────────────────────────────────────────────────────

static void usage(const char *prog) {
    fprintf(stderr,
        "Usage: %s [options]\n"
        "  --shard NAME     Trace a specific shard (src or dst)\n"
        "  --from  NAME     Only messages from shard\n"
        "  --to    NAME     Only messages to shard\n"
        "  --opcode NAME    Filter by opcode name\n"
        "  --json           JSON output\n"
        "  --count          Count messages per opcode (summary at exit)\n"
        "  --latency        Show latency percentiles\n"
        "  --help\n", prog);
}

// ── Main ──────────────────────────────────────────────────────────────────

int main(int argc, char **argv) {
    for (int i = 1; i < argc; i++) {
        if (strcmp(argv[i], "--json")    == 0) g_filter.json    = true;
        if (strcmp(argv[i], "--count")   == 0) g_filter.count   = true;
        if (strcmp(argv[i], "--latency") == 0) g_filter.latency = true;
        if (strcmp(argv[i], "--help")    == 0) { usage(argv[0]); return 0; }
        if ((strcmp(argv[i], "--from") == 0 || strcmp(argv[i], "--shard") == 0)
             && i+1 < argc) {
            // Resolve shard name to ID
            for (int j = 0; shard_names[j].name; j++) {
                if (strcmp(shard_names[j].name, argv[i+1]) == 0)
                    g_filter.src_shard = shard_names[j].id;
            }
            i++;
        }
        if (strcmp(argv[i], "--to") == 0 && i+1 < argc) {
            for (int j = 0; shard_names[j].name; j++) {
                if (strcmp(shard_names[j].name, argv[i+1]) == 0)
                    g_filter.dst_shard = shard_names[j].id;
            }
            i++;
        }
    }

    printf("ipctrace: attaching to kernel IPC tracer...\n");
    int ring_fd = sigma_bpf_attach_ipc_tracer();
    if (ring_fd < 0) {
        fprintf(stderr, "ipctrace: failed to attach (need CAP_LOAD_MOD)\n");
        return 1;
    }
    printf("ipctrace: tracing IPC — press Ctrl+C to stop\n\n");
    if (!g_filter.json)
        printf("%-17s %-16s   %-16s  %-18s  %s\n",
               "TIMESTAMP", "SRC", "DST", "OPCODE", "LEN");

    ipc_event_t event;
    while (sigma_bpf_ringbuf_read(ring_fd, &event, sizeof(event), 100) >= 0) {
        print_event(&event);
    }

    if (g_filter.count) print_counts();
    return 0;
}

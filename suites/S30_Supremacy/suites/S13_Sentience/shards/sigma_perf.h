/*
 * =========================================================================
 * S SIGMAOS kernel/suites/S13_Sentience/shards/sigma_perf.h
 * =========================================================================
 * Sovereign Observability Engine — gap-closes:
 *   Linux  : perf_events, ftrace, eBPF, kprobes, tracepoints, /proc/pid
 *   macOS  : DTrace, Instruments, kdebug, os_log
 *   Windows: ETW (Event Tracing for Windows), PerfMon, WPR/WPA
 *   BSD    : hwpmc, ktrace, pmcstat
 *   Android: Perfetto, Simpleperf, systrace
 * =========================================================================
 */

#ifndef SIGMA_PERF_H
#define SIGMA_PERF_H

typedef unsigned long long pf_u64;
typedef unsigned int       pf_u32;
typedef signed   int       pf_i32;
typedef unsigned char      pf_bool;
#define PF_TRUE  ((pf_bool)1)
#define PF_FALSE ((pf_bool)0)
#define PF_NULL  ((void*)0)
#define PF_OK    ((pf_i32) 0)

/* ── Event types (perf_event_type parity) ───────────────────────────────── */
typedef enum {
    PERF_HW_CPU_CYCLES       = 0,
    PERF_HW_INSTRUCTIONS     = 1,
    PERF_HW_CACHE_REFS       = 2,
    PERF_HW_CACHE_MISSES     = 3,
    PERF_HW_BRANCH_INSTR     = 4,
    PERF_HW_BRANCH_MISS      = 5,
    PERF_SW_CONTEXT_SWITCHES = 6,
    PERF_SW_PAGE_FAULTS      = 7,
    PERF_SW_TASK_CLOCK       = 8,
    PERF_SW_CPU_MIGRATIONS   = 9,
    PERF_TRACEPOINT          = 10,  /* custom kernel tracepoint          */
    PERF_KPROBE              = 11,  /* dynamic kprobe                   */
    PERF_UPROBE              = 12   /* userspace uprobe                 */
} sigma_perf_type_t;

#define SIGMA_PERF_MAX_COUNTERS  64
#define SIGMA_PERF_MAX_SAMPLES  512
#define SIGMA_PERF_NAME_LEN      48

/* ── Performance counter ─────────────────────────────────────────────────── */
typedef struct {
    pf_u32            id;
    char              name[SIGMA_PERF_NAME_LEN];
    sigma_perf_type_t type;
    pf_u32            owner_pid;     /* 0 = system-wide                 */
    pf_u64            count;
    pf_u64            time_enabled;  /* ns counter was active           */
    pf_u64            time_running;  /* ns actually on PMU              */
    pf_bool           enabled;
    pf_bool           inherit;       /* inherit across fork()           */
} sigma_perf_counter_t;

/* ── Profiling sample (perf record parity) ───────────────────────────────── */
typedef struct {
    pf_u64 timestamp_ns;
    pf_u64 ip;           /* instruction pointer                         */
    pf_u32 pid;
    pf_u32 cpu;
    pf_u64 period;       /* sample period                               */
    pf_u64 callchain[8]; /* compact call chain                         */
    pf_u32 callchain_len;
} sigma_perf_sample_t;

/* ── ETW/Perfetto-style trace event ─────────────────────────────────────── */
typedef enum {
    TRACE_BEGIN  = 0,
    TRACE_END    = 1,
    TRACE_INSTANT= 2,
    TRACE_COUNTER= 3
} sigma_trace_phase_t;

typedef struct {
    char               name[SIGMA_PERF_NAME_LEN];
    char               category[24];
    sigma_trace_phase_t phase;
    pf_u64             timestamp_ns;
    pf_u32             pid;
    pf_u64             value;   /* for TRACE_COUNTER events              */
} sigma_trace_event_t;

#define SIGMA_TRACE_MAX 4096

/* ── Public API ─────────────────────────────────────────────────────────── */
void   sigma_perf_init(void);

/* Counters (perf_event_open equivalent) */
pf_i32 sigma_perf_counter_open(const char *name, sigma_perf_type_t type,
                                pf_u32 pid);
void   sigma_perf_counter_enable(pf_u32 id);
void   sigma_perf_counter_disable(pf_u32 id);
pf_u64 sigma_perf_counter_read(pf_u32 id);
void   sigma_perf_counter_reset(pf_u32 id);
void   sigma_perf_counter_close(pf_u32 id);
void   sigma_perf_counters_dump(void);

/* Sampling */
void   sigma_perf_sample_record(pf_u64 ip, pf_u32 pid, pf_u32 cpu);
void   sigma_perf_samples_dump(void);

/* Trace events (Perfetto/ETW style) */
void   sigma_trace_begin(const char *name, const char *cat, pf_u32 pid);
void   sigma_trace_end(const char *name, const char *cat, pf_u32 pid);
void   sigma_trace_instant(const char *name, const char *cat);
void   sigma_trace_counter(const char *name, pf_u64 value);
void   sigma_trace_dump_json(void);  /* exports Perfetto/Chrome trace JSON */

/* /proc equivalent */
void   sigma_proc_stat_print(pf_u32 pid);
void   sigma_system_stat_print(void);

#endif /* SIGMA_PERF_H */

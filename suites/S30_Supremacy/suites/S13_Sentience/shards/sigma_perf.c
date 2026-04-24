/*
 * =========================================================================
 * S SIGMAOS kernel/suites/S13_Sentience/shards/sigma_perf.c
 * =========================================================================
 */

#include "sigma_perf.h"
#include "sigma_libc.h"

static sigma_perf_counter_t s_counters[SIGMA_PERF_MAX_COUNTERS];
static pf_u32               s_counter_count = 0;
static pf_u32               s_next_id       = 1;

static sigma_perf_sample_t  s_samples[SIGMA_PERF_MAX_SAMPLES];
static pf_u32               s_sample_head   = 0;

static sigma_trace_event_t  s_trace[SIGMA_TRACE_MAX];
static pf_u32               s_trace_head    = 0;

static pf_u64               s_system_clock  = 0;

/* -- Simulated HW counter values (monotonically increasing) ---------------- */
static pf_u64 sim_read_hw(sigma_perf_type_t type) {
    s_system_clock += 1000;
    switch (type) {
        case PERF_HW_CPU_CYCLES:       return s_system_clock * 3;
        case PERF_HW_INSTRUCTIONS:     return s_system_clock * 2;
        case PERF_HW_CACHE_REFS:       return s_system_clock / 4;
        case PERF_HW_CACHE_MISSES:     return s_system_clock / 40;
        case PERF_HW_BRANCH_INSTR:     return s_system_clock / 8;
        case PERF_HW_BRANCH_MISS:      return s_system_clock / 80;
        case PERF_SW_CONTEXT_SWITCHES: return s_system_clock / 1000;
        case PERF_SW_PAGE_FAULTS:      return s_system_clock / 500;
        case PERF_SW_TASK_CLOCK:       return s_system_clock;
        default:                        return s_system_clock;
    }
}

/* -- Init ------------------------------------------------------------------ */
void sigma_perf_init(void) {
    sigma_sigma_memset(s_counters, 0, sizeof(s_counters));
    sigma_sigma_memset(s_samples,  0, sizeof(s_samples));
    sigma_sigma_memset(s_trace,    0, sizeof(s_trace));
    sigma_sigma_printf("S [PERF] Observability Engine initialized\n");
    sigma_sigma_printf("S [PERF] Counters: %u slots   Samples: %u slots   Trace: %u slots\n",
                 SIGMA_PERF_MAX_COUNTERS, SIGMA_PERF_MAX_SAMPLES, SIGMA_TRACE_MAX);
}

/* -- Counter lifecycle ----------------------------------------------------- */
pf_i32 sigma_perf_counter_open(const char *name, sigma_perf_type_t type, pf_u32 pid) {
    if (s_counter_count >= SIGMA_PERF_MAX_COUNTERS) return -1;
    sigma_perf_counter_t *c = &s_counters[s_counter_count++];
    sigma_sigma_memset(c, 0, sizeof(*c));
    c->id        = s_next_id++;
    c->type      = type;
    c->owner_pid = pid;
    c->enabled   = PF_FALSE;
    sigma_strncpy(c->name, name, SIGMA_PERF_NAME_LEN - 1);
    sigma_sigma_printf("S [PERF] OPEN: fd=%u name=%s pid=%u\n", c->id, name, pid);
    return (pf_i32)c->id;
}

static sigma_perf_counter_t *find_counter(pf_u32 id) {
    for (pf_u32 i = 0; i < s_counter_count; i++)
        if (s_counters[i].id == id) return &s_counters[i];
    return PF_NULL;
}

void sigma_perf_counter_enable(pf_u32 id) {
    sigma_perf_counter_t *c = find_counter(id);
    if (c) { c->enabled = PF_TRUE; c->time_enabled = s_system_clock; }
}

void sigma_perf_counter_disable(pf_u32 id) {
    sigma_perf_counter_t *c = find_counter(id);
    if (c) c->enabled = PF_FALSE;
}

pf_u64 sigma_perf_counter_read(pf_u32 id) {
    sigma_perf_counter_t *c = find_counter(id);
    if (!c || !c->enabled) return 0;
    c->count        = sim_read_hw(c->type);
    c->time_running = s_system_clock - c->time_enabled;
    return c->count;
}

void sigma_perf_counter_reset(pf_u32 id) {
    sigma_perf_counter_t *c = find_counter(id);
    if (c) { c->count = 0; c->time_enabled = s_system_clock; }
}

void sigma_perf_counter_close(pf_u32 id) {
    for (pf_u32 i = 0; i < s_counter_count; i++) {
        if (s_counters[i].id == id) {
            for (pf_u32 j = i; j < s_counter_count - 1; j++)
                s_counters[j] = s_counters[j+1];
            s_counter_count--;
            return;
        }
    }
}

void sigma_perf_counters_dump(void) {
    static const char *type_names[] = {
        "CPU_CYCLES","INSTR","CACHE_REF","CACHE_MISS","BRANCH","BR_MISS",
        "CTX_SW","PF","TASK_CLK","CPU_MIG","TRACEPOINT","KPROBE","UPROBE"
    };
    sigma_sigma_printf("\nS PERF COUNTERS (%u)\n", s_counter_count);
    sigma_sigma_printf("%-4s %-24s %-16s %-20s %s\n", "FD","NAME","TYPE","COUNT","TIME(ns)");
    for (pf_u32 i = 0; i < s_counter_count; i++) {
        sigma_perf_counter_t *c = &s_counters[i];
        sigma_sigma_printf("  %-2u %-24s %-16s %-20llu %llu\n",
                     c->id, c->name,
                     type_names[c->type],
                     (unsigned long long)c->count,
                     (unsigned long long)c->time_running);
    }
}

/* -- Sampling -------------------------------------------------------------- */
void sigma_perf_sample_record(pf_u64 ip, pf_u32 pid, pf_u32 cpu) {
    sigma_perf_sample_t *s = &s_samples[s_sample_head % SIGMA_PERF_MAX_SAMPLES];
    s->timestamp_ns = s_system_clock;
    s->ip           = ip;
    s->pid          = pid;
    s->cpu          = cpu;
    s->period       = 10000;
    s_sample_head++;
}

void sigma_perf_samples_dump(void) {
    pf_u32 n = s_sample_head < SIGMA_PERF_MAX_SAMPLES ? s_sample_head : SIGMA_PERF_MAX_SAMPLES;
    sigma_sigma_printf("\nS PERF SAMPLES (%u)\n", n);
    for (pf_u32 i = 0; i < n; i++) {
        sigma_perf_sample_t *s = &s_samples[i];
        sigma_sigma_printf("  t=%-12llu ip=0x%llx pid=%-5u cpu=%u\n",
                     (unsigned long long)s->timestamp_ns,
                     (unsigned long long)s->ip,
                     s->pid, s->cpu);
    }
}

/* -- Trace events ---------------------------------------------------------- */
static void trace_push(const char *name, const char *cat,
                        sigma_trace_phase_t ph, pf_u32 pid, pf_u64 val) {
    sigma_trace_event_t *e = &s_trace[s_trace_head % SIGMA_TRACE_MAX];
    sigma_strncpy(e->name,     name, SIGMA_PERF_NAME_LEN - 1);
    sigma_strncpy(e->category, cat,  23);
    e->phase        = ph;
    e->timestamp_ns = s_system_clock;
    e->pid          = pid;
    e->value        = val;
    s_trace_head++;
    s_system_clock += 100;
}

void sigma_trace_begin(const char *name, const char *cat, pf_u32 pid) {
    trace_push(name, cat, TRACE_BEGIN, pid, 0);
}
void sigma_trace_end(const char *name, const char *cat, pf_u32 pid) {
    trace_push(name, cat, TRACE_END, pid, 0);
}
void sigma_trace_instant(const char *name, const char *cat) {
    trace_push(name, cat, TRACE_INSTANT, 0, 0);
}
void sigma_trace_counter(const char *name, pf_u64 value) {
    trace_push(name, "counters", TRACE_COUNTER, 0, value);
}

void sigma_trace_dump_json(void) {
    static const char *ph_str[] = {"B","E","i","C"};
    sigma_sigma_printf("\n{\"traceEvents\":[\n");
    pf_u32 n = s_trace_head < SIGMA_TRACE_MAX ? s_trace_head : SIGMA_TRACE_MAX;
    for (pf_u32 i = 0; i < n; i++) {
        sigma_trace_event_t *e = &s_trace[i];
        sigma_sigma_printf("{\"name\":\"%s\",\"cat\":\"%s\",\"ph\":\"%s\","
                     "\"ts\":%llu,\"pid\":%u,\"args\":{\"v\":%llu}}%s\n",
                     e->name, e->category, ph_str[e->phase],
                     (unsigned long long)e->timestamp_ns, e->pid,
                     (unsigned long long)e->value,
                     i < n-1 ? "," : "");
    }
    sigma_sigma_printf("]}\n");
}

/* -- /proc stat equivalent ------------------------------------------------- */
void sigma_proc_stat_print(pf_u32 pid) {
    sigma_sigma_printf("\n/proc/%u/stat (sigma)\n", pid);
    sigma_sigma_printf("  cpu_cycles: %llu   instructions: %llu\n",
                 (unsigned long long)sim_read_hw(PERF_HW_CPU_CYCLES),
                 (unsigned long long)sim_read_hw(PERF_HW_INSTRUCTIONS));
    sigma_sigma_printf("  cache_miss: %llu   branch_miss: %llu\n",
                 (unsigned long long)sim_read_hw(PERF_HW_CACHE_MISSES),
                 (unsigned long long)sim_read_hw(PERF_HW_BRANCH_MISS));
}

void sigma_system_stat_print(void) {
    sigma_sigma_printf("\nS SYSTEM STATS\n");
    sigma_sigma_printf("  ctx_switches: %llu   page_faults: %llu\n",
                 (unsigned long long)sim_read_hw(PERF_SW_CONTEXT_SWITCHES),
                 (unsigned long long)sim_read_hw(PERF_SW_PAGE_FAULTS));
    sigma_sigma_printf("  task_clock:   %llu ns\n",
                 (unsigned long long)sim_read_hw(PERF_SW_TASK_CLOCK));
    sigma_perf_counters_dump();
}

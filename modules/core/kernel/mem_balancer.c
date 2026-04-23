#include <stdint.h>
#include <stddef.h>

// ---------------------------------------------------------
// SigmaOS Automated Memory Balancer
// Dynamically adjusts memory quotas based on
// process workload, priority, and token economy balance
// ---------------------------------------------------------

#define MAX_PROCESSES    128
#define MIN_PAGES        16
#define MAX_PAGES_TOTAL  65536   // 256 MB at 4KB/page

typedef struct {
    uint32_t pid;
    uint8_t  priority;       // 0 (idle) to 99 (real-time)
    uint32_t current_pages;
    uint32_t requested_pages;
    uint32_t last_page_faults;
    uint32_t workload_score;  // Computed by AI scheduler EMA
    uint8_t  active;
} proc_quota_t;

static proc_quota_t quotas[MAX_PROCESSES];
static uint32_t proc_count = 0;
static uint32_t total_allocated = 0;

extern void audit_chain_append(uint32_t pid, uint8_t level, const char* msg);

// Register a new process for memory balancing
int membal_register(uint32_t pid, uint8_t priority, uint32_t initial_pages) {
    if (proc_count >= MAX_PROCESSES) return -1;
    proc_quota_t* p = &quotas[proc_count++];
    p->pid              = pid;
    p->priority         = priority;
    p->current_pages    = initial_pages;
    p->requested_pages  = initial_pages;
    p->last_page_faults = 0;
    p->workload_score   = 50; // Start at mid-range
    p->active           = 1;
    total_allocated    += initial_pages;
    return 0;
}

// Update workload metrics from profiler/scheduler tick
void membal_update_metrics(uint32_t pid, uint32_t page_faults, uint32_t workload_score) {
    for (uint32_t i = 0; i < proc_count; i++) {
        if (quotas[i].pid == pid && quotas[i].active) {
            quotas[i].last_page_faults = page_faults;
            // EMA smoothing of workload score (alpha = 30%)
            quotas[i].workload_score = (quotas[i].workload_score * 7 + workload_score * 3) / 10;
            return;
        }
    }
}

// Compute ideal page allocation for a process
static uint32_t compute_ideal_pages(proc_quota_t* p) {
    // Base: proportional to priority
    uint32_t base = MIN_PAGES + (p->priority * 4);
    
    // Boost if experiencing page faults (process needs more RAM)
    if (p->last_page_faults > 100) base = (base * 3) / 2;
    
    // Reduce if workload is low
    if (p->workload_score < 20) base = (base * 2) / 3;
    
    // Hard cap at 8192 pages (32 MB) per process
    return (base > 8192) ? 8192 : base;
}

// Periodic rebalance — called by scheduler every N ticks
void membal_rebalance(void) {
    uint32_t new_total = 0;

    // Phase 1: compute ideal allocations
    for (uint32_t i = 0; i < proc_count; i++) {
        if (!quotas[i].active) continue;
        uint32_t ideal = compute_ideal_pages(&quotas[i]);
        quotas[i].requested_pages = ideal;
        new_total += ideal;
    }

    // Phase 2: scale down proportionally if system is over-committed
    if (new_total > MAX_PAGES_TOTAL) {
        uint32_t scale_num = MAX_PAGES_TOTAL;
        uint32_t scale_den = new_total;
        for (uint32_t i = 0; i < proc_count; i++) {
            if (!quotas[i].active) continue;
            quotas[i].requested_pages =
                (quotas[i].requested_pages * scale_num) / scale_den;
            if (quotas[i].requested_pages < MIN_PAGES)
                quotas[i].requested_pages = MIN_PAGES;
        }
    }

    // Phase 3: apply new quotas
    total_allocated = 0;
    for (uint32_t i = 0; i < proc_count; i++) {
        if (!quotas[i].active) continue;
        if (quotas[i].current_pages != quotas[i].requested_pages) {
            audit_chain_append(quotas[i].pid, 1, "MEM_QUOTA_ADJUSTED");
            quotas[i].current_pages = quotas[i].requested_pages;
        }
        total_allocated += quotas[i].current_pages;
    }
}

// Deregister process (called on exit → triggers cap auto-revocation)
void membal_deregister(uint32_t pid) {
    for (uint32_t i = 0; i < proc_count; i++) {
        if (quotas[i].pid == pid && quotas[i].active) {
            total_allocated -= quotas[i].current_pages;
            quotas[i].active = 0;
            audit_chain_append(pid, 1, "MEM_QUOTA_RELEASED");
            return;
        }
    }
}

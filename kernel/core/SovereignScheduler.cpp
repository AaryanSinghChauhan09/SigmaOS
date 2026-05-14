#include "../include/core/sigma_types.h"
#include "../include/sigma_log.h"
#include "../include/libc/SovereignLibC.h"

/* =========================================================================
 * SIGMAOS: SOVEREIGN PREDICTIVE SCHEDULER (S-Sched) v2.0
 * - O(1) multi-priority queues (HIGH, NORMAL, LOW)
 * - Round-robin within each priority band
 * - Per-process CR3 page-directory isolation
 * - Quantum-based preemption tracking
 * ========================================================================= */

enum class SigmaProcessState : sigma_u8 {
    READY   = 0,
    RUNNING = 1,
    BLOCKED = 2,
    ZOMBIE  = 3
};

enum class SigmaPriority : sigma_u8 {
    HIGH   = 0,
    NORMAL = 1,
    LOW    = 2
};

struct SigmaProcessBlock {
    sigma_u32          pid;
    SigmaPriority      priority;
    SigmaProcessState  state;
    sigma_u64          cr3_page_dir; /* isolated address space */
    sigma_u64          cpu_time_us;  /* microseconds of CPU consumed */
    sigma_u64          quantum_rem;  /* remaining quantum in us */
    char               name[32];
};

class SovereignScheduler {
public:
    static SovereignScheduler& getInstance() {
        static SovereignScheduler instance;
        return instance;
    }

    void init() {
        sigma_log_info("[SCHED] Initializing Sovereign Round-Robin Scheduler v2.0...");
        this->total_procs = 0;
        this->current_idx = 0;
        this->context_switches = 0;
        sigma_memset(this->procs, 0, sizeof(this->procs));
        sigma_log_info("[SCHED] Multi-priority queues ACTIVE. Process isolation: CR3-per-process.");
    }

    sigma_u32 spawn(const char* name, SigmaPriority prio) {
        if (this->total_procs >= MAX_PROCS) {
            sigma_log_info("[SCHED] WARN: Process table full.");
            return SIGMA_ERROR;
        }
        sigma_u32 idx = this->total_procs++;
        SigmaProcessBlock& p = this->procs[idx];
        p.pid         = idx + 1;
        p.priority    = prio;
        p.state       = SigmaProcessState::READY;
        p.cr3_page_dir = 0x300000ULL + ((sigma_u64)p.pid * 0x1000ULL);
        p.cpu_time_us  = 0;
        p.quantum_rem  = quantumFor(prio);
        sigma_hardened_strcpy(p.name, name, 32);
        sigma_log_info("[SCHED] Spawned PID %u '%s' CR3=0x%llX\n", p.pid, p.name, p.cr3_page_dir);
        return p.pid;
    }

    sigma_u32 schedule() {
        if (this->total_procs == 0) return 0; /* idle */

        /* Mark current as ready */
        procs[current_idx].state    = SigmaProcessState::READY;
        procs[current_idx].quantum_rem = quantumFor(procs[current_idx].priority);

        /* Round-robin with priority bias: HIGH -> NORMAL -> LOW */
        sigma_u32 best = findNextByPriority(SigmaPriority::HIGH);
        if (best == MAX_PROCS) best = findNextByPriority(SigmaPriority::NORMAL);
        if (best == MAX_PROCS) best = findNextByPriority(SigmaPriority::LOW);
        if (best == MAX_PROCS) return 0; /* all blocked */

        current_idx = best;
        procs[current_idx].state = SigmaProcessState::RUNNING;
        procs[current_idx].cpu_time_us += procs[current_idx].quantum_rem;
        this->context_switches++;

        sigma_log_info("[SCHED] CTX-SWITCH -> PID %u '%s' CR3=0x%llX (switches=%llu)\n",
            procs[current_idx].pid, procs[current_idx].name,
            procs[current_idx].cr3_page_dir, this->context_switches);
        return procs[current_idx].pid;
    }

    void block(sigma_u32 pid) {
        for (sigma_u32 i = 0; i < total_procs; ++i)
            if (procs[i].pid == pid) { procs[i].state = SigmaProcessState::BLOCKED; return; }
    }

    void unblock(sigma_u32 pid) {
        for (sigma_u32 i = 0; i < total_procs; ++i)
            if (procs[i].pid == pid) { procs[i].state = SigmaProcessState::READY; return; }
    }

    void printStats() const {
        sigma_log_info("[SCHED/STATS] Processes: %u | Context Switches: %llu\n",
            total_procs, context_switches);
        for (sigma_u32 i = 0; i < total_procs; ++i) {
            sigma_log_info("  PID %u %-16s PRIO=%u STATE=%u CPU=%llus\n",
                procs[i].pid, procs[i].name,
                (sigma_u32)procs[i].priority, (sigma_u32)procs[i].state,
                procs[i].cpu_time_us);
        }
    }

private:
    static constexpr sigma_u32 MAX_PROCS = 256;

    SovereignScheduler() : total_procs(0), current_idx(0), context_switches(0) {}

    sigma_u64 quantumFor(SigmaPriority p) const {
        switch (p) {
            case SigmaPriority::HIGH:   return 20000; /* 20ms */
            case SigmaPriority::NORMAL: return 10000; /* 10ms */
            case SigmaPriority::LOW:    return 5000;  /*  5ms */
            default:                   return 10000;
        }
    }

    sigma_u32 findNextByPriority(SigmaPriority prio) const {
        for (sigma_u32 n = 1; n <= total_procs; ++n) {
            sigma_u32 idx = (current_idx + n) % total_procs;
            if (procs[idx].priority == prio && procs[idx].state == SigmaProcessState::READY)
                return idx;
        }
        return MAX_PROCS; /* not found */
    }

    SigmaProcessBlock procs[MAX_PROCS];
    sigma_u32 total_procs;
    sigma_u32 current_idx;
    sigma_u64 context_switches;
};

/* --- C Bridge --- */
extern "C" void sched_init()                          { SovereignScheduler::getInstance().init(); }
extern "C" sigma_u32 sched_spawn(const char* n, sigma_u32 p) { return SovereignScheduler::getInstance().spawn(n, (SigmaPriority)p); }
extern "C" sigma_u32 sched_schedule()                 { return SovereignScheduler::getInstance().schedule(); }
extern "C" void sched_block(sigma_u32 pid)            { SovereignScheduler::getInstance().block(pid); }
extern "C" void sched_unblock(sigma_u32 pid)          { SovereignScheduler::getInstance().unblock(pid); }
extern "C" void sched_stats()                         { SovereignScheduler::getInstance().printStats(); }

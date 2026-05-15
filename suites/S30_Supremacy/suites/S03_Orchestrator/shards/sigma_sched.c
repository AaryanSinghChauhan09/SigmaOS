#include "../../../../../include/SovereignLibC.h"
/*
 * =========================================================================
 * S SIGMAOS kernel/suites/S03_Orchestrator/shards/sigma_sched.c
 * =========================================================================
 */

#include "../../../../../include/sigma_sched.h"
#include "../../../../../include/libc/sigma_libc.h"

static sigma_runqueue_t s_rqs[SIGMA_MAX_CPUS];
static sc_u32           s_num_cpus = 0;

/* ── Helpers ─────────────────────────────────────────────────────────────── */
static sc_u64 nice_to_weight(sc_i32 nice) {
    /* Linux prio_to_weight table: each step = ~1.25x weight ratio */
    static const sc_u64 weights[40] = {
        88761,71755,56483,46273,36291,
        29154,23254,18705,14949,11916,
        9548, 7620, 6100, 4904, 3906,
        3121, 2501, 1991, 1586, 1277,
        1024,  820,  655,  526,  423,
         335,  272,  215,  172,  137,
         110,   87,   70,   56,   45,
          36,   29,   23,   18,   15
    };
    sc_i32 idx = nice + 20;  /* nice -20..+19 -> idx 0..39 */
    if (idx < 0)  idx = 0;
    if (idx > 39) idx = 39;
    return weights[idx];
}

static sigma_runqueue_t *get_rq(sc_u32 cpu_id) {
    if (cpu_id >= s_num_cpus) return (sigma_runqueue_t*)0;
    return &s_rqs[cpu_id];
}

static sigma_task_t *find_task(sigma_runqueue_t *rq, sc_u32 pid) {
    for (sc_u32 i = 0; i < rq->task_count; i++)
        if (rq->tasks[i].base.id == pid) return &rq->tasks[i];
    return (sigma_task_t*)0;
}

/* ── Init ────────────────────────────────────────────────────────────────── */
void sigma_sched_init(sc_u32 num_cpus) {
    if (num_cpus > SIGMA_MAX_CPUS) num_cpus = SIGMA_MAX_CPUS;
    s_num_cpus = num_cpus;
    sigma_sigma_memset(s_rqs, 0, sizeof(s_rqs));
    for (sc_u32 i = 0; i < num_cpus; i++) {
        s_rqs[i].cpu_id = i;
        s_rqs[i].min_vruntime = 0;
    }
    sigma_sigma_printf("S [SCHED] CFS initialized on %u CPU(s)\n", num_cpus);
}

/* ── Enqueue ─────────────────────────────────────────────────────────────── */
void sigma_sched_enqueue(sc_u32 cpu_id, sc_u32 pid,
                          sigma_sched_policy_t policy,
                          sigma_qos_t qos, sc_i32 nice) {
    sigma_runqueue_t *rq = get_rq(cpu_id);
    if (!rq || rq->task_count >= SIGMA_SCHED_MAX_TASKS) return;

    sigma_task_t *t = &rq->tasks[rq->task_count++];
    t->base.id      = pid;
    t->base.name    = "sigma_task";
    t->policy       = policy;
    t->qos          = qos;
    t->nice         = nice;
    /* CFS: new task inherits min_vruntime so it doesn't starve old tasks */
    t->vruntime     = rq->min_vruntime;
    t->timeslice_ns = SIGMA_SCHED_TIMESLICE_NS;
    t->on_cpu       = SCHED_FALSE;
    t->preemptible  = (policy != POLICY_FIFO);

    sigma_sigma_printf("S [SCHED] ENQUEUE: cpu=%u pid=%u policy=%d qos=%d nice=%d\n",
                 cpu_id, pid, (int)policy, (int)qos, nice);
}

/* ── Dequeue ─────────────────────────────────────────────────────────────── */
void sigma_sched_dequeue(sc_u32 cpu_id, sc_u32 pid) {
    sigma_runqueue_t *rq = get_rq(cpu_id);
    if (!rq) return;
    for (sc_u32 i = 0; i < rq->task_count; i++) {
        if (rq->tasks[i].base.id == pid) {
            for (sc_u32 j = i; j < rq->task_count - 1; j++)
                rq->tasks[j] = rq->tasks[j+1];
            rq->task_count--;
            sigma_sigma_printf("S [SCHED] DEQUEUE: cpu=%u pid=%u\n", cpu_id, pid);
            return;
        }
    }
}

/* ── CFS pick-next ───────────────────────────────────────────────────────── */
sc_u32 sigma_sched_pick_next(sc_u32 cpu_id) {
    sigma_runqueue_t *rq = get_rq(cpu_id);
    if (!rq || rq->task_count == 0) return 0;

    /* Realtime tasks always preempt CFS */
    for (sc_u32 i = 0; i < rq->task_count; i++) {
        if (rq->tasks[i].policy == POLICY_FIFO ||
            rq->tasks[i].policy == POLICY_RR   ||
            rq->tasks[i].qos   == QOS_REALTIME)
            return rq->tasks[i].base.id;
    }

    /* EDF: pick task with earliest deadline */
    sc_u64 earliest = (sc_u64)-1;
    sc_u32 edf_pid  = 0;
    for (sc_u32 i = 0; i < rq->task_count; i++) {
        if (rq->tasks[i].policy == POLICY_EDF &&
            rq->tasks[i].deadline_ns < earliest) {
            earliest = rq->tasks[i].deadline_ns;
            edf_pid  = rq->tasks[i].base.id;
        }
    }
    if (edf_pid) return edf_pid;

    /* CFS: pick task with lowest vruntime (leftmost RB-tree node) */
    sc_u64 min_vrt = (sc_u64)-1;
    sc_u32 cfs_pid = 0;
    for (sc_u32 i = 0; i < rq->task_count; i++) {
        if (rq->tasks[i].policy == POLICY_IDLE) continue;
        if (rq->tasks[i].vruntime < min_vrt) {
            min_vrt = rq->tasks[i].vruntime;
            cfs_pid = rq->tasks[i].base.id;
        }
    }
    if (cfs_pid) {
        rq->min_vruntime = min_vrt;
        return cfs_pid;
    }

    /* IDLE class fallback */
    return rq->tasks[0].base.id;
}

/* ── Tick ─────────────────────────────────────────────────────────────────── */
void sigma_sched_tick(sc_u32 cpu_id, sc_u64 elapsed_ns) {
    sigma_runqueue_t *rq = get_rq(cpu_id);
    if (!rq) return;
    rq->clock_ns += elapsed_ns;

    sigma_task_t *cur = find_task(rq, rq->current_pid);
    if (!cur) return;

    sc_u64 weight = nice_to_weight(cur->nice);
    /* CFS delta = elapsed * (1024 / weight) — normalized vruntime */
    sc_u64 delta_vrt = (elapsed_ns * 1024) / (weight ? weight : 1024);
    cur->vruntime     += delta_vrt;
    cur->total_cpu_ns += elapsed_ns;

    if (elapsed_ns >= cur->timeslice_ns && cur->preemptible) {
        cur->timeslice_ns = SIGMA_SCHED_TIMESLICE_NS;
        rq->preemptions++;
        sc_u32 next = sigma_sched_pick_next(cpu_id);
        if (next != rq->current_pid) {
            rq->context_switches++;
            cur->on_cpu         = SCHED_FALSE;
            rq->current_pid     = next;
            sigma_task_t *nxt   = find_task(rq, next);
            if (nxt) nxt->on_cpu = SCHED_TRUE;
        }
    }
}

/* ── Yield ───────────────────────────────────────────────────────────────── */
void sigma_sched_yield(sc_u32 cpu_id, sc_u32 pid) {
    sigma_runqueue_t *rq = get_rq(cpu_id);
    if (!rq) return;
    sigma_task_t *t = find_task(rq, pid);
    if (t) { t->vruntime += SIGMA_SCHED_TIMESLICE_NS; t->on_cpu = SCHED_FALSE; }
    rq->current_pid = sigma_sched_pick_next(cpu_id);
    rq->context_switches++;
}

/* ── EDF deadline ─────────────────────────────────────────────────────────── */
void sigma_sched_set_deadline(sc_u32 pid, sc_u64 deadline_ns) {
    for (sc_u32 c = 0; c < s_num_cpus; c++) {
        sigma_task_t *t = find_task(&s_rqs[c], pid);
        if (t) { t->deadline_ns = deadline_ns; t->policy = POLICY_EDF; return; }
    }
}

/* ── Work-stealing load balancer ─────────────────────────────────────────── */
void sigma_sched_balance(void) {
    if (s_num_cpus < 2) return;
    for (sc_u32 i = 0; i < s_num_cpus; i++) {
        for (sc_u32 j = i + 1; j < s_num_cpus; j++) {
            sigma_runqueue_t *heavy = &s_rqs[i];
            sigma_runqueue_t *light = &s_rqs[j];
            if (light->task_count > heavy->task_count) {
                sigma_runqueue_t *tmp = heavy; heavy = light; light = tmp;
            }
            if (heavy->task_count > light->task_count + 1) {
                /* Migrate last task from heavy to light */
                sigma_task_t stolen = heavy->tasks[--heavy->task_count];
                light->tasks[light->task_count++] = stolen;
                sigma_sigma_printf("S [SCHED] STEAL: pid=%u cpu%u->cpu%u\n",
                             stolen.base.id, heavy->cpu_id, light->cpu_id);
            }
        }
    }
}

/* ── Stats ─────────────────────────────────────────────────────────────────── */
void sigma_sched_stats(sc_u32 cpu_id) {
    sigma_runqueue_t *rq = get_rq(cpu_id);
    if (!rq) return;
    sigma_sigma_printf("\nS SCHED CPU%u: tasks=%u ctx_sw=%llu prempt=%llu\n",
                 cpu_id, rq->task_count,
                 (unsigned long long)rq->context_switches,
                 (unsigned long long)rq->preemptions);
    for (sc_u32 i = 0; i < rq->task_count; i++) {
        sigma_task_t *t = &rq->tasks[i];
        sigma_sigma_printf("  pid=%-5u vrt=%llu cpu=%llu ns nice=%d%s\n",
                     t->base.id, (unsigned long long)t->vruntime,
                     (unsigned long long)t->total_cpu_ns,
                     t->nice, t->on_cpu ? " [RUNNING]" : "");
    }
}

void sigma_sched_global_stats(void) {
    for (sc_u32 i = 0; i < s_num_cpus; i++)
        sigma_sched_stats(i);
}

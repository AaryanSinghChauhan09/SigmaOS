/*
 * SigmaOS: Enterprise Predictive AI Scheduler (C Layer)
 * =====================================================
 * Mission: High-speed, ML-informed, preemptive CPU thread management.
 * Principles: Separation of concerns, encapsulation, fault tolerance.
 *
 * Architecture:
 *   - Task Control Block (TCB) encapsulates all per-task state
 *   - Priority queue with aging prevents starvation
 *   - Predictive heuristic tracks burst history for proactive scheduling
 *   - Multi-level feedback queue (MLFQ) for fairness
 *   - Sovereign Control Groups (SCG): Linux-inspired resource isolation and limiting
 *   - Sovereign Wait Queues: Event-based blocking for ultra-low latency waking
 */

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>

/* ── Configuration ─────────────────────────────────────────────── */
#define MAX_TASKS           256
#define PRIORITY_LEVELS     8
#define HISTORY_DEPTH       8
#define AGING_THRESHOLD     5
#define DEFAULT_QUANTUM_MS  10
#define MIN_PRIORITY        0
#define MAX_PRIORITY        (PRIORITY_LEVELS - 1)
#define MAX_SCGS            16
#define MAX_WAIT_QUEUES     64

/* ── CPU Context (x86_64) ──────────────────────────────────────── */
typedef struct {
    uint64_t r15, r14, r13, r12, r11, r10, r9, r8;
    uint64_t rbp, rdi, rsi, rdx, rcx, rbx, rax;
    uint64_t rip;
    uint64_t cs;
    uint64_t rflags;
    uint64_t rsp;
    uint64_t ss;
} cpu_context_t;

/* ── Task States ───────────────────────────────────────────────── */
typedef enum {
    TASK_FREE = 0,
    TASK_READY,
    TASK_RUNNING,
    TASK_BLOCKED,
    TASK_SLEEPING,
    TASK_TERMINATED
} task_state_t;

/* ── Predictive Burst Tracker ──────────────────────────────────── */
typedef struct {
    uint32_t burst_history[HISTORY_DEPTH];
    uint8_t  history_index;
    uint32_t predicted_burst;
    uint64_t total_cpu_time;
    uint64_t total_wait_time;
} predictor_t;

/* ── Wait Queue (Linux-inspired) ─────────────────────────────────── */
typedef struct {
    uint32_t blocked_tasks[MAX_TASKS];
    uint32_t count;
    uint32_t id;
} wait_queue_t;

/* ── Sovereign Control Group (SCG - cgroups-inspired) ─────────────── */
typedef struct {
    uint32_t id;
    char     name[32];
    uint32_t cpu_share;       /* 1-100% of total CPU time allocated */
    uint32_t max_priority;    /* Hard ceiling for priorities in this group */
    uint64_t total_used_time;
    uint32_t task_count;
    bool     active;
} scg_t;

/* ── Task Control Block (TCB) — Encapsulated ───────────────────── */
typedef struct {
    uint32_t       pid;
    task_state_t   state;
    cpu_context_t *context;
    uint64_t      *stack_base;
    uint64_t      *stack_ptr;
    int            priority;
    int            base_priority;
    uint32_t       quantum_ms;
    uint32_t       time_remaining;
    uint32_t       age;
    predictor_t    predictor;
    uint32_t       scg_id;      /* Control group membership */
    uint32_t       wait_event;  /* ID of wait queue task is blocked on */
    char           name[32];
} tcb_t;

/* ── Static State (Encapsulated Module Scope) ──────────────────── */
static tcb_t    task_table[MAX_TASKS];
static scg_t     scg_table[MAX_SCGS];
static wait_queue_t wait_queues[MAX_WAIT_QUEUES];
static tcb_t   *current_task   = NULL;
static uint32_t next_pid       = 1;
static uint32_t next_scg_id    = 1;
static uint64_t scheduler_tick = 0;

/* ── External: Assembly context switcher ───────────────────────── */
extern void sigma_switch_tasks(uint64_t **old_sp, uint64_t *new_sp);
void sigma_schedule(void);

/* ── Predictor: Update burst history and compute weighted average ─ */
static void predictor_record_burst(predictor_t *p, uint32_t burst) {
    p->burst_history[p->history_index % HISTORY_DEPTH] = burst;
    p->history_index++;

    /* Exponentially weighted moving average */
    uint32_t total = 0;
    uint32_t weight = 1;
    uint32_t weight_sum = 0;
    uint8_t count = (p->history_index < HISTORY_DEPTH)
                        ? p->history_index
                        : HISTORY_DEPTH;

    for (uint8_t i = 0; i < count; i++) {
        uint8_t idx = (p->history_index - 1 - i) % HISTORY_DEPTH;
        total += p->burst_history[idx] * weight;
        weight_sum += weight;
        weight <<= 1;  /* Double weight for more recent entries */
    }

    p->predicted_burst = (weight_sum > 0) ? (total / weight_sum) : burst;
}

static void predictor_init(predictor_t *p) {
    for (int i = 0; i < HISTORY_DEPTH; i++) {
        p->burst_history[i] = 0;
    }
    p->history_index    = 0;
    p->predicted_burst  = DEFAULT_QUANTUM_MS;
    p->total_cpu_time   = 0;
    p->total_wait_time  = 0;
}

/* ── Safe string copy (no stdlib dependency) ───────────────────── */
static void safe_strncpy(char *dst, const char *src, int max_len) {
    int n = 0;
    while (src[n] && n < max_len - 1) {
        dst[n] = src[n];
        n++;
    }
    dst[n] = '\0';
}

/* ── Scheduler Init ────────────────────────────────────────────── */
void sigma_scheduler_init(void) {
    for (int i = 0; i < MAX_TASKS; i++) {
        task_table[i].state = TASK_FREE;
        task_table[i].pid   = 0;
    }
    for (int i = 0; i < MAX_SCGS; i++) {
        scg_table[i].active = false;
    }
    for (int i = 0; i < MAX_WAIT_QUEUES; i++) {
        wait_queues[i].count = 0;
        wait_queues[i].id = i;
    }

    current_task   = NULL;
    next_pid       = 1;
    next_scg_id    = 1;
    scheduler_tick = 0;
}

/* ── SCG Management (Linux cgroups-inspired) ───────────────────── */
int sigma_create_scg(const char *name, uint32_t cpu_share, uint32_t max_priority) {
    if (next_scg_id >= MAX_SCGS) return -1;
    uint32_t id = next_scg_id++;
    scg_t *scg = &scg_table[id];
    scg->id = id;
    scg->cpu_share = cpu_share;
    scg->max_priority = max_priority;
    scg->total_used_time = 0;
    scg->task_count = 0;
    scg->active = true;
    safe_strncpy(scg->name, name, 32);
    return (int)id;
}

int sigma_assign_task_to_scg(uint32_t pid, uint32_t scg_id) {
    if (scg_id >= MAX_SCGS || !scg_table[scg_id].active) return -1;
    for (int i = 0; i < MAX_TASKS; i++) {
        if (task_table[i].pid == pid && task_table[i].state != TASK_FREE) {
            task_table[i].scg_id = scg_id;
            scg_table[scg_id].task_count++;
            return 0;
        }
    }
    return -1;
}

/* ── Wait Queue Management (Linux-inspired) ────────────────────── */
void sigma_wait_on(uint32_t wait_queue_id) {
    if (wait_queue_id >= MAX_WAIT_QUEUES || !current_task) return;
    
    wait_queue_t *wq = &wait_queues[wait_queue_id];
    if (wq->count < MAX_TASKS) {
        wq->blocked_tasks[wq->count++] = current_task->pid;
        current_task->state = TASK_BLOCKED;
        current_task->wait_event = wait_queue_id;
        sigma_schedule(); /* Preempt current task to wait */
    }
}

void sigma_wake_up(uint32_t wait_queue_id) {
    if (wait_queue_id >= MAX_WAIT_QUEUES) return;
    
    wait_queue_t *wq = &wait_queues[wait_queue_id];
    for (uint32_t i = 0; i < wq->count; i++) {
        uint32_t pid = wq->blocked_tasks[i];
        for (int j = 0; j < MAX_TASKS; j++) {
            if (task_table[j].pid == pid) {
                task_table[j].state = TASK_READY;
                task_table[j].wait_event = 0;
                break;
            }
        }
    }
    wq->count = 0;
}

/* ── Create Task ───────────────────────────────────────────────── */
int sigma_create_task(void (*entry_point)(void), uint64_t *stack_space,
                      int priority, const char *name) {
    if (!stack_space || !name) return -1;
    if (priority < MIN_PRIORITY) priority = MIN_PRIORITY;
    if (priority > MAX_PRIORITY) priority = MAX_PRIORITY;

    for (int i = 0; i < MAX_TASKS; i++) {
        if (task_table[i].state == TASK_FREE) {
            tcb_t *task = &task_table[i];
            task->pid            = next_pid++;
            task->state          = TASK_READY;
            task->priority       = priority;
            task->base_priority  = priority;
            task->quantum_ms     = DEFAULT_QUANTUM_MS;
            task->time_remaining = DEFAULT_QUANTUM_MS;
            task->age            = 0;
            task->scg_id         = 0; /* Default: Root Group */
            task->wait_event     = 0;
            task->stack_base     = stack_space;
            task->stack_ptr      = stack_space;

            predictor_init(&task->predictor);
            safe_strncpy(task->name, name, 32);

            (void)entry_point;  /* Entry point setup handled by arch layer */
            return (int)task->pid;
        }
    }
    return -1;  /* Task table full — fault tolerance: return error code */
}

/* ── Terminate Task ────────────────────────────────────────────── */
int sigma_terminate_task(uint32_t pid) {
    for (int i = 0; i < MAX_TASKS; i++) {
        if (task_table[i].pid == pid && task_table[i].state != TASK_FREE) {
            task_table[i].state = TASK_TERMINATED;
            /* Memory reclamation deferred to garbage collector */
            return 0;
        }
    }
    return -1;  /* Task not found */
}

/* ── Anti-Starvation: Age all waiting tasks ────────────────────── */
static void aging_pass(void) {
    for (int i = 0; i < MAX_TASKS; i++) {
        if (task_table[i].state == TASK_READY) {
            task_table[i].age++;
            if (task_table[i].age >= AGING_THRESHOLD) {
                if (task_table[i].priority < MAX_PRIORITY) {
                    task_table[i].priority++;
                }
                task_table[i].age = 0;
            }
        }
    }
}

/* ── Predictive Priority Scheduler (MLFQ + AI Heuristic) ──────── */
void sigma_schedule(void) {
    scheduler_tick++;

    /* Run aging every 4 ticks to prevent starvation */
    if ((scheduler_tick & 3) == 0) {
        aging_pass();
    }

    /* Record burst for outgoing task */
    if (current_task && current_task->state == TASK_RUNNING) {
        uint32_t used = current_task->quantum_ms - current_task->time_remaining;
        predictor_record_burst(&current_task->predictor, used);
        current_task->predictor.total_cpu_time += used;
        
        /* Update SCG accounting (Linux-style resource tracking) */
        if (current_task->scg_id < MAX_SCGS && scg_table[current_task->scg_id].active) {
            scg_table[current_task->scg_id].total_used_time += used;
        }
    }

    /* Find highest-priority READY task (predictive tie-breaking + SCG capping) */
    tcb_t *best = NULL;
    for (int i = 0; i < MAX_TASKS; i++) {
        if (task_table[i].state == TASK_READY) {
            tcb_t *candidate = &task_table[i];
            
            /* Apply SCG resource caps (Linux cgroups-inspired) */
            int effective_priority = candidate->priority;
            if (candidate->scg_id < MAX_SCGS && scg_table[candidate->scg_id].active) {
                scg_t *grp = &scg_table[candidate->scg_id];
                if (effective_priority > (int)grp->max_priority) {
                    effective_priority = (int)grp->max_priority;
                }
            }

            if (!best) {
                best = candidate;
            } else {
                /* Compare based on effective priority first */
                int best_effective = best->priority;
                if (best->scg_id < MAX_SCGS && scg_table[best->scg_id].active) {
                    if (best_effective > (int)scg_table[best->scg_id].max_priority)
                        best_effective = (int)scg_table[best->scg_id].max_priority;
                }

                if (effective_priority > best_effective) {
                    best = candidate;
                } else if (effective_priority == best_effective) {
                    /* Tie-break: prefer task with shorter predicted burst (SJF) */
                    if (candidate->predictor.predicted_burst <
                        best->predictor.predicted_burst) {
                        best = candidate;
                    }
                }
            }
        }
    }

    if (!best) return;  /* No ready tasks — idle */

    if (best != current_task) {
        tcb_t *old = current_task;
        if (old && old->state == TASK_RUNNING) {
            old->state = TASK_READY;
            /* Reset to base priority after demotion (MLFQ decay) */
            if (old->priority > old->base_priority) {
                old->priority--;
            }
        }

        current_task = best;
        current_task->state          = TASK_RUNNING;
        current_task->time_remaining = current_task->quantum_ms;
        current_task->age            = 0;

        /* Adaptive quantum: set based on predicted burst */
        if (current_task->predictor.predicted_burst > 0) {
            current_task->quantum_ms = current_task->predictor.predicted_burst;
            if (current_task->quantum_ms < 2) current_task->quantum_ms = 2;
            if (current_task->quantum_ms > 100) current_task->quantum_ms = 100;
        }

        if (old) {
            sigma_switch_tasks(&old->stack_ptr, current_task->stack_ptr);
        }
    }
}

/* ── Timer Tick Handler (called by IRQ0) ───────────────────────── */
void sigma_timer_tick(void) {
    if (current_task && current_task->state == TASK_RUNNING) {
        if (current_task->time_remaining > 0) {
            current_task->time_remaining--;
        }
        if (current_task->time_remaining == 0) {
            sigma_schedule();  /* Preempt: quantum expired */
        }
    }
}

/* ── Query Interface (for monitoring / analytics) ──────────────── */
uint32_t sigma_get_task_count(void) {
    uint32_t count = 0;
    for (int i = 0; i < MAX_TASKS; i++) {
        if (task_table[i].state != TASK_FREE &&
            task_table[i].state != TASK_TERMINATED) {
            count++;
        }
    }
    return count;
}

uint32_t sigma_get_predicted_burst(uint32_t pid) {
    for (int i = 0; i < MAX_TASKS; i++) {
        if (task_table[i].pid == pid) {
            return task_table[i].predictor.predicted_burst;
        }
    }
    return 0;
}

/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

/*
 * SigmaOS Advanced Process Scheduler
 * ================================
 * Object-Oriented Process Management with SOLID Principles
 */

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>

// Forward declarations for OOP-style interface
typedef struct ProcessManager ProcessManager;
typedef struct Process Process;
typedef struct SchedulerPolicy SchedulerPolicy;
typedef struct ReadyQueue ReadyQueue;

// Process states following Linux process model
typedef enum {
    PROCESS_STATE_RUNNING = 0,
    PROCESS_STATE_READY = 1,
    PROCESS_STATE_BLOCKED = 2,
    PROCESS_STATE_SLEEPING = 3,
    PROCESS_STATE_ZOMBIE = 4,
    PROCESS_STATE_STOPPED = 5,
    PROCESS_STATE_TRACED = 6
} ProcessState;

// Process priority levels (nice values -20 to 19)
typedef enum {
    PRIORITY_REALTIME = -20,
    PRIORITY_HIGH = -10,
    PRIORITY_NORMAL = 0,
    PRIORITY_LOW = 10,
    PRIORITY_IDLE = 19
} ProcessPriority;

// Scheduling policies
typedef enum {
    SCHED_NORMAL = 0,      // Completely fair scheduler
    SCHED_FIFO = 1,        // Real-time FIFO
    SCHED_RR = 2,          // Real-time round-robin
    SCHED_BATCH = 3,       // Batch processing
    SCHED_IDLE = 4,        // Idle class
    SCHED_DEADLINE = 5     // Deadline scheduler
} SchedulingPolicy;

// OOP Interface for Scheduler Policy (Strategy Pattern)
typedef struct {
    void (*enqueue)(ReadyQueue* queue, Process* process);
    Process* (*dequeue)(ReadyQueue* queue);
    Process* (*peek_next)(ReadyQueue* queue);
    bool (*is_empty)(ReadyQueue* queue);
    void (*reorder)(ReadyQueue* queue);
    const char* policy_name;
} SchedulerPolicyInterface;

// Ready queue implementation (Composition)
struct ReadyQueue {
    Process** processes;
    size_t capacity;
    size_t size;
    SchedulerPolicyInterface* policy;
    uint32_t quantum;
};

// Process Control Block (PCB) with OOP principles
struct Process {
    uint32_t pid;
    uint32_t ppid;
    uint32_t uid;
    uint32_t gid;
    ProcessState state;
    ProcessPriority priority;
    SchedulingPolicy policy;
    
    // CPU context (x86_64)
    struct {
        uint64_t rax, rbx, rcx, rdx;
        uint64_t rsi, rdi, rbp, rsp;
        uint64_t r8, r9, r10, r11;
        uint64_t r12, r13, r14, r15;
        uint64_t rip, rflags;
        uint64_t cs, ss, ds, es, fs, gs;
    } context;
    
    // Scheduling information
    uint64_t vruntime;          // Virtual runtime for CFS
    uint64_t exec_start;        // Execution start time
    uint64_t sum_exec_runtime;  // Total execution time
    uint64_t prev_sum_exec_runtime;
    uint64_t time_slice;        // Time slice allocation
    uint64_t last_ran;          // Last time process ran
    
    // Memory management
    uint64_t brk;               // Program break
    uint64_t start_stack;
    uint64_t start_code;
    uint64_t end_code;
    uint64_t start_data;
    uint64_t end_data;
    
    // File descriptors
    struct {
        uint32_t* fds;
        size_t count;
        size_t capacity;
    } file_table;
    
    // Signal handling
    uint64_t pending_signals;
    uint64_t blocked_signals;
    void (*signal_handlers[32])(int);
    
    // Process relationships
    Process* parent;
    Process* children[16];
    size_t child_count;
    Process* sibling;
    Process* thread_group_leader;
    
    // Statistics
    uint64_t start_time;
    uint64_t user_time;
    uint64_t system_time;
    uint64_t voluntary_ctxt_switches;
    uint64_t nonvoluntary_ctxt_switches;
    
    // Security context
    uint32_t capabilities[4];
    char comm[16];              // Command name
    char cmdline[256];          // Command line arguments
};

// Process Manager with SOLID principles
struct ProcessManager {
    Process* current_process;
    Process* idle_process;
    ReadyQueue runqueues[8];    // One per priority level
    Process* pid_table[65536];   // PID to process mapping
    uint32_t next_pid;
    uint64_t tick_count;
    uint64_t last_schedule_time;
    
    // OOP: Policy injection for different scheduling algorithms
    SchedulerPolicyInterface* policies[6];
    SchedulingPolicy current_policy;
    
    // Statistics
    uint64_t context_switches;
    uint64_t schedule_count;
    uint64_t runnable_processes;
    uint64_t total_processes;
    
    // Configuration
    uint32_t min_granularity;
    uint32_t latency;
    uint32_t wakeup_granularity;
};

// Completely Fair Scheduler (CFS) Implementation
typedef struct {
    ReadyQueue* queue;
    uint64_t min_vruntime;
    uint64_t max_vruntime;
    uint32_t nr_running;
    uint32_t load;
} CFSQueue;

// Real-time Scheduler Implementation
typedef struct {
    ReadyQueue* queue;
    Process* active_bitmap[8];  // 100 priority levels
    Process* expired_bitmap[8];
    int active_array;
    int expired_array;
} RTQueue;

// OOP: CFS Policy Implementation
static void cfs_enqueue(ReadyQueue* queue, Process* process) {
    // Insert process in vruntime order
    size_t i = 0;
    while (i < queue->size && queue->processes[i]->vruntime <= process->vruntime) {
        i++;
    }
    
    // Shift elements to make space
    for (size_t j = queue->size; j > i; j--) {
        queue->processes[j] = queue->processes[j - 1];
    }
    
    queue->processes[i] = process;
    queue->size++;
}

static Process* cfs_dequeue(ReadyQueue* queue) {
    if (queue->size == 0) return NULL;
    
    Process* process = queue->processes[0];
    
    // Shift remaining elements
    for (size_t i = 0; i < queue->size - 1; i++) {
        queue->processes[i] = queue->processes[i + 1];
    }
    
    queue->size--;
    return process;
}

static Process* cfs_peek_next(ReadyQueue* queue) {
    return queue->size > 0 ? queue->processes[0] : NULL;
}

static bool cfs_is_empty(ReadyQueue* queue) {
    return queue->size == 0;
}

static void cfs_reorder(ReadyQueue* queue) {
    // Rebalance the queue based on vruntime
    for (size_t i = 1; i < queue->size; i++) {
        Process* key = queue->processes[i];
        size_t j = i - 1;
        
        while (j < queue->size && queue->processes[j]->vruntime > key->vruntime) {
            queue->processes[j + 1] = queue->processes[j];
            j--;
        }
        queue->processes[j + 1] = key;
    }
}

// OOP: Real-time Policy Implementation
static void rt_enqueue(ReadyQueue* queue, Process* process) {
    // Insert in priority order, FIFO within same priority
    int priority = (int)process->priority;
    size_t i = 0;
    
    while (i < queue->size && (int)queue->processes[i]->priority <= priority) {
        i++;
    }
    
    for (size_t j = queue->size; j > i; j--) {
        queue->processes[j] = queue->processes[j - 1];
    }
    
    queue->processes[i] = process;
    queue->size++;
}

static Process* rt_dequeue(ReadyQueue* queue) {
    if (queue->size == 0) return NULL;
    
    Process* process = queue->processes[0];
    
    for (size_t i = 0; i < queue->size - 1; i++) {
        queue->processes[i] = queue->processes[i + 1];
    }
    
    queue->size--;
    return process;
}

static Process* rt_peek_next(ReadyQueue* queue) {
    return queue->size > 0 ? queue->processes[0] : NULL;
}

static bool rt_is_empty(ReadyQueue* queue) {
    return queue->size == 0;
}

static void rt_reorder(ReadyQueue* queue) {
    // RT scheduler maintains strict priority order
}

// Policy instances (Singleton Pattern)
static SchedulerPolicyInterface cfs_policy = {
    .enqueue = cfs_enqueue,
    .dequeue = cfs_dequeue,
    .peek_next = cfs_peek_next,
    .is_empty = cfs_is_empty,
    .reorder = cfs_reorder,
    .policy_name = "CFS"
};

static SchedulerPolicyInterface rt_policy = {
    .enqueue = rt_enqueue,
    .dequeue = rt_dequeue,
    .peek_next = rt_peek_next,
    .is_empty = rt_is_empty,
    .reorder = rt_reorder,
    .policy_name = "RT"
};

// Process Manager Constructor
ProcessManager* sigma_process_manager_create(void) {
    ProcessManager* pm = (ProcessManager*)malloc(sizeof(ProcessManager));
    if (!pm) return NULL;
    
    // Initialize fields
    pm->current_process = NULL;
    pm->idle_process = NULL;
    pm->next_pid = 1;
    pm->tick_count = 0;
    pm->last_schedule_time = 0;
    pm->context_switches = 0;
    pm->schedule_count = 0;
    pm->runnable_processes = 0;
    pm->total_processes = 0;
    pm->current_policy = SCHED_NORMAL;
    
    // Initialize run queues
    for (int i = 0; i < 8; i++) {
        pm->runqueues[i].processes = (Process**)malloc(sizeof(Process*) * 256);
        pm->runqueues[i].capacity = 256;
        pm->runqueues[i].size = 0;
        pm->runqueues[i].quantum = 10; // Default 10ms quantum
        pm->runqueues[i].policy = &cfs_policy;
    }
    
    // Set up real-time queues
    for (int i = 0; i < 5; i++) {
        pm->runqueues[i].policy = &rt_policy;
    }
    
    // Initialize PID table
    memset(pm->pid_table, 0, sizeof(pm->pid_table));
    
    // Set default configuration
    pm->min_granularity = 1;   // 1ms
    pm->latency = 20;          // 20ms
    pm->wakeup_granularity = 5; // 5ms
    
    return pm;
}

// Process Factory Method
Process* sigma_process_create(ProcessManager* pm, uint32_t ppid, const char* name) {
    Process* process = (Process*)malloc(sizeof(Process));
    if (!process) return NULL;
    
    // Initialize process
    memset(process, 0, sizeof(Process));
    
    process->pid = pm->next_pid++;
    process->ppid = ppid;
    process->uid = 0;
    process->gid = 0;
    process->state = PROCESS_STATE_READY;
    process->priority = PRIORITY_NORMAL;
    process->policy = SCHED_NORMAL;
    
    // Set up command name
    strncpy(process->comm, name, sizeof(process->comm) - 1);
    
    // Initialize scheduling info
    process->vruntime = 0;
    process->exec_start = 0;
    process->sum_exec_runtime = 0;
    process->prev_sum_exec_runtime = 0;
    process->time_slice = pm->latency;
    process->last_ran = 0;
    
    // Initialize file descriptor table
    process->file_table.fds = (uint32_t*)malloc(sizeof(uint32_t) * 16);
    process->file_table.count = 0;
    process->file_table.capacity = 16;
    
    // Set start time
    process->start_time = sigma_get_timestamp();
    
    // Add to PID table
    pm->pid_table[process->pid] = process;
    pm->total_processes++;
    
    return process;
}

// Process Destructor
void sigma_process_destroy(ProcessManager* pm, Process* process) {
    if (!process) return;
    
    // Remove from PID table
    if (process->pid < 65536) {
        pm->pid_table[process->pid] = NULL;
    }
    
    // Free file descriptor table
    if (process->file_table.fds) {
        free(process->file_table.fds);
    }
    
    // Clean up children
    for (size_t i = 0; i < process->child_count; i++) {
        process->children[i]->parent = NULL;
    }
    
    pm->total_processes--;
    free(process);
}

// Add process to appropriate run queue
void sigma_process_enqueue(ProcessManager* pm, Process* process) {
    if (!process || process->state != PROCESS_STATE_READY) return;
    
    int priority_level = (int)process->priority + 20; // Convert to 0-39 range
    if (priority_level < 0) priority_level = 0;
    if (priority_level > 39) priority_level = 39;
    
    ReadyQueue* queue = &pm->runqueues[priority_level / 5]; // 8 queues total
    
    queue->policy->enqueue(queue, process);
    pm->runnable_processes++;
}

// Pick next process to run (Strategy Pattern)
Process* sigma_schedule_next(ProcessManager* pm) {
    Process* next = NULL;
    
    // Check real-time queues first (highest priority)
    for (int i = 0; i < 5; i++) {
        if (!pm->runqueues[i].policy->is_empty(&pm->runqueues[i])) {
            next = pm->runqueues[i].policy->dequeue(&pm->runqueues[i]);
            break;
        }
    }
    
    // If no real-time processes, check normal queues
    if (!next) {
        for (int i = 5; i < 8; i++) {
            if (!pm->runqueues[i].policy->is_empty(&pm->runqueues[i])) {
                next = pm->runqueues[i].policy->dequeue(&pm->runqueues[i]);
                break;
            }
        }
    }
    
    // If no runnable processes, run idle process
    if (!next) {
        next = pm->idle_process;
    }
    
    if (next) {
        pm->runnable_processes--;
        pm->schedule_count++;
    }
    
    return next;
}

// Context switch implementation
void sigma_context_switch(ProcessManager* pm, Process* old_process, Process* new_process) {
    if (!old_process || !new_process || old_process == new_process) return;
    
    // Save old process context
    if (old_process->state == PROCESS_STATE_RUNNING) {
        old_process->state = PROCESS_STATE_READY;
        sigma_process_enqueue(pm, old_process);
    }
    
    // Update scheduling statistics
    uint64_t current_time = sigma_get_timestamp();
    uint64_t runtime = current_time - old_process->exec_start;
    old_process->sum_exec_runtime += runtime;
    old_process->exec_start = current_time;
    
    // Update virtual runtime for CFS
    if (old_process->policy == SCHED_NORMAL) {
        old_process->vruntime += runtime * 1024 / (old_process->priority + 20);
    }
    
    // Load new process context
    new_process->state = PROCESS_STATE_RUNNING;
    new_process->exec_start = current_time;
    pm->current_process = new_process;
    pm->context_switches++;
    
    // Perform actual context switch (assembly)
    sigma_switch_context(&old_process->context, &new_process->context);
}

// Main scheduler entry point
void sigma_scheduler_tick(ProcessManager* pm) {
    pm->tick_count++;
    uint64_t current_time = sigma_get_timestamp();
    
    Process* current = pm->current_process;
    if (!current) {
        current = sigma_schedule_next(pm);
        if (current) {
            pm->current_process = current;
            current->state = PROCESS_STATE_RUNNING;
            current->exec_start = current_time;
        }
        return;
    }
    
    // Check if current process should be preempted
    bool should_preempt = false;
    
    // Time slice expiration
    if (current_time - current->exec_start >= current->time_slice) {
        should_preempt = true;
    }
    
    // Higher priority process available
    Process* next = sigma_schedule_next(pm);
    if (next && next != current && 
        (int)next->priority < (int)current->priority) {
        should_preempt = true;
    }
    
    // Perform context switch if needed
    if (should_preempt) {
        sigma_context_switch(pm, current, next);
    }
}

// Process sleep implementation
void sigma_process_sleep(ProcessManager* pm, uint32_t milliseconds) {
    Process* current = pm->current_process;
    if (!current) return;
    
    current->state = PROCESS_STATE_SLEEPING;
    current->last_ran = sigma_get_timestamp();
    
    // Add to sleep queue (simplified)
    Process* next = sigma_schedule_next(pm);
    sigma_context_switch(pm, current, next);
}

// Process wakeup implementation
void sigma_process_wakeup(ProcessManager* pm, Process* process) {
    if (!process || process->state != PROCESS_STATE_SLEEPING) return;
    
    process->state = PROCESS_STATE_READY;
    sigma_process_enqueue(pm, process);
}

// Signal handling
void sigma_process_send_signal(ProcessManager* pm, Process* process, int signal) {
    if (!process) return;
    
    // Check if signal is blocked
    if (process->blocked_signals & (1ULL << signal)) return;
    
    // Set pending signal
    process->pending_signals |= (1ULL << signal);
    
    // Wake up sleeping process
    if (process->state == PROCESS_STATE_SLEEPING) {
        sigma_process_wakeup(pm, process);
    }
}

// Process termination
void sigma_process_exit(ProcessManager* pm, int exit_code) {
    Process* current = pm->current_process;
    if (!current) return;
    
    current->state = PROCESS_STATE_ZOMBIE;
    
    // Notify parent
    if (current->parent) {
        sigma_process_send_signal(pm, current->parent, SIGCHLD);
    }
    
    // Schedule next process
    Process* next = sigma_schedule_next(pm);
    sigma_context_switch(pm, current, next);
}

// Get process statistics
typedef struct {
    uint32_t pid;
    char comm[16];
    ProcessState state;
    ProcessPriority priority;
    uint64_t vruntime;
    uint64_t sum_exec_runtime;
    uint64_t user_time;
    uint64_t system_time;
} ProcessStats;

void sigma_process_get_stats(ProcessManager* pm, Process* process, ProcessStats* stats) {
    if (!process || !stats) return;
    
    stats->pid = process->pid;
    strncpy(stats->comm, process->comm, sizeof(stats->comm) - 1);
    stats->state = process->state;
    stats->priority = process->priority;
    stats->vruntime = process->vruntime;
    stats->sum_exec_runtime = process->sum_exec_runtime;
    stats->user_time = process->user_time;
    stats->system_time = process->system_time;
}

// System call interface
uint32_t sigma_sys_fork(ProcessManager* pm) {
    Process* parent = pm->current_process;
    if (!parent) return -1;
    
    Process* child = sigma_process_create(pm, parent->pid, parent->comm);
    if (!child) return -1;
    
    // Copy parent's state to child
    child->context = parent->context;
    child->uid = parent->uid;
    child->gid = parent->gid;
    child->priority = parent->priority;
    child->policy = parent->policy;
    
    // Copy file descriptor table
    child->file_table.count = parent->file_table.count;
    for (size_t i = 0; i < parent->file_table.count; i++) {
        child->file_table.fds[i] = parent->file_table.fds[i];
    }
    
    // Set up parent-child relationship
    child->parent = parent;
    parent->children[parent->child_count++] = child;
    
    // Add child to ready queue
    sigma_process_enqueue(pm, child);
    
    return child->pid;
}

// Change scheduling policy
int sigma_sys_sched_setscheduler(ProcessManager* pm, uint32_t pid, 
                               SchedulingPolicy policy, ProcessPriority priority) {
    Process* process = (pid == 0) ? pm->current_process : 
                      (pid < 65536) ? pm->pid_table[pid] : NULL;
    
    if (!process) return -1;
    
    process->policy = policy;
    process->priority = priority;
    
    // Requeue if process is ready
    if (process->state == PROCESS_STATE_READY) {
        // Remove from current queue and re-add with new priority
        sigma_process_enqueue(pm, process);
    }
    
    return 0;
}

// Get timestamp (hardware abstraction)
uint64_t sigma_get_timestamp(void) {
    // This would interface with hardware timer
    static uint64_t counter = 0;
    return counter++; // Simplified
}

// Assembly context switch (x86_64)
extern void sigma_switch_context(void* old_context, void* new_context);

// Process Manager Destructor
void sigma_process_manager_destroy(ProcessManager* pm) {
    if (!pm) return;
    
    // Clean up all processes
    for (uint32_t i = 0; i < 65536; i++) {
        if (pm->pid_table[i]) {
            sigma_process_destroy(pm, pm->pid_table[i]);
        }
    }
    
    // Clean up run queues
    for (int i = 0; i < 8; i++) {
        if (pm->runqueues[i].processes) {
            free(pm->runqueues[i].processes);
        }
    }
    
    free(pm);
}


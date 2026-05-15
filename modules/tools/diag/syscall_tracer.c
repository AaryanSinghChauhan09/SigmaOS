#include "../../../include/libc/sigma_libc.h"
#include "../../../include/libc/sigma_libc.h"
#include <string.h>

// ---------------------------------------------------------
// SigmaOS Dynamic Syscall Tracer
// Logs every syscall with PID, arguments, and anomaly detection
// ---------------------------------------------------------

#define MAX_TRACE_ENTRIES 1024
#define SYSCALL_NAME_LEN  24

typedef struct {
    uint64_t tick;
    uint32_t pid;
    uint32_t syscall_id;
    uint64_t args[4];
    int32_t  retval;
    uint8_t  anomaly; // 1 = flagged for anomaly
} syscall_trace_entry_t;

static syscall_trace_entry_t trace_ring[MAX_TRACE_ENTRIES];
static uint32_t trace_head = 0;   // Next write position (ring buffer)
static uint32_t trace_count = 0;  // Total entries ever recorded

extern void audit_chain_append(uint32_t pid, uint8_t level, const char* msg);

// Anomaly detection: syscalls per PID in the last N ticks
#define ANOMALY_WINDOW    100   // ticks
#define ANOMALY_THRESHOLD 50    // more than 50 syscalls = suspicious

static uint32_t count_recent_syscalls(uint32_t pid, uint64_t current_tick) {
    uint32_t count = 0;
    uint32_t entries = (trace_count < MAX_TRACE_ENTRIES) ? trace_count : MAX_TRACE_ENTRIES;
    for (uint32_t i = 0; i < entries; i++) {
        syscall_trace_entry_t* e = &trace_ring[i];
        if (e->pid == pid && (current_tick - e->tick) <= ANOMALY_WINDOW)
            count++;
    }
    return count;
}

// Called at the START of every syscall dispatch (kernel entry hook)
void syscall_trace_enter(uint32_t pid, uint32_t syscall_id,
                         uint64_t a0, uint64_t a1, uint64_t a2, uint64_t a3,
                         uint64_t current_tick) {
    syscall_trace_entry_t* e = &trace_ring[trace_head % MAX_TRACE_ENTRIES];
    e->tick       = current_tick;
    e->pid        = pid;
    e->syscall_id = syscall_id;
    e->args[0]    = a0;
    e->args[1]    = a1;
    e->args[2]    = a2;
    e->args[3]    = a3;
    e->retval     = 0;
    e->anomaly    = 0;

    // Anomaly detection: syscall storm check
    uint32_t recent = count_recent_syscalls(pid, current_tick);
    if (recent > ANOMALY_THRESHOLD) {
        e->anomaly = 1;
        audit_chain_append(pid, 3 /* LOG_WARN */, "SYSCALL_ANOMALY_DETECTED");
    }

    trace_head++;
    trace_count++;
}

// Called at the END of every syscall (record return value)
void syscall_trace_exit(uint32_t pid, uint32_t syscall_id, int32_t retval) {
    // Walk backwards to find matching entry (ring is small and recent)
    for (int i = trace_head - 1; i >= 0; i--) {
        syscall_trace_entry_t* e = &trace_ring[i % MAX_TRACE_ENTRIES];
        if (e->pid == pid && e->syscall_id == syscall_id && e->retval == 0) {
            e->retval = retval;
            break;
        }
    }
}

// Dump all flagged anomaly entries (for developer visualisation)
uint32_t syscall_trace_dump_anomalies(syscall_trace_entry_t* out, uint32_t max_out) {
    uint32_t found = 0;
    uint32_t entries = (trace_count < MAX_TRACE_ENTRIES) ? trace_count : MAX_TRACE_ENTRIES;
    for (uint32_t i = 0; i < entries && found < max_out; i++) {
        if (trace_ring[i].anomaly) {
            out[found++] = trace_ring[i];
        }
    }
    return found;
}

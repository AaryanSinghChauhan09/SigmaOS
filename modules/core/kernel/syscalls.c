#include <stdint.h>
#include <stddef.h>

// ---------------------------------------------------------
// SigmaOS Sovereign Syscall Interface
// USP: Ring-Buffer Batching (io_uring for everything)
// All syscalls are Capability-verified and Traced natively.
// ---------------------------------------------------------

#define MAX_SYSCALLS 256
#define SYSCALL_RING_SIZE 64

// Standard POSIX-style syscalls are considered legacy. 
// SigmaOS uses an asynchronous submission queue.
typedef struct {
    uint32_t syscall_id;
    uint64_t arg0;
    uint64_t arg1;
    uint64_t arg2;
    uint64_t arg3;
    uint32_t cap_token; // Every syscall requires a capability token
    uint8_t  status;    // 0: Pending, 1: Complete, 2: Error
    int64_t  result;
} syscall_submission_t;

typedef struct {
    syscall_submission_t ring[SYSCALL_RING_SIZE];
    uint32_t head; // Written by User
    uint32_t tail; // Read by Kernel
} syscall_queue_t;

// Example Syscall IDs
#define SYS_MEM_LEASE     10
#define SYS_MEM_DELEGATE  11
#define SYS_IPC_SEND      20
#define SYS_CAP_MINT      30

// External hooks
extern int cap_registry_verify(uint32_t cap_id, uint32_t pid, uint8_t required_rights);
extern void syscall_trace_enter(uint32_t pid, uint32_t syscall_id, uint64_t a0, uint64_t a1, uint64_t a2, uint64_t a3, uint64_t current_tick);
extern void syscall_trace_exit(uint32_t pid, uint32_t syscall_id, int32_t retval);
extern void watchdog_trigger_fault(uint32_t pid, const char* reason);
extern int mem_contract_lease(uint32_t pid, uint32_t base_page, uint32_t num_pages, uint64_t duration_ticks);

// Master Syscall Dispatcher Table
typedef int64_t (*syscall_handler_t)(uint32_t pid, uint64_t a0, uint64_t a1, uint64_t a2, uint64_t a3);
static syscall_handler_t syscall_table[MAX_SYSCALLS] = {0};

// -- Syscall Implementations --

static int64_t sys_mem_lease_handler(uint32_t pid, uint64_t base, uint64_t pages, uint64_t duration, uint64_t unused) {
    (void)unused;
    return mem_contract_lease(pid, (uint32_t)base, (uint32_t)pages, duration);
}

// Boot init
void syscall_init(void) {
    syscall_table[SYS_MEM_LEASE] = sys_mem_lease_handler;
    // ... register others ...
}

// Kernel worker that processes the asynchronous syscall rings for a process
// This runs in kernel space during the scheduler tick or a dedicated kernel thread
void syscall_process_queue(uint32_t pid, syscall_queue_t* queue, uint64_t current_tick) {
    if (!queue) return;

    while (queue->tail != queue->head) {
        syscall_submission_t* sub = &queue->ring[queue->tail % SYSCALL_RING_SIZE];
        
        if (sub->status != 0) {
            queue->tail++;
            continue;
        }

        uint32_t sys_id = sub->syscall_id;
        
        // Trace Enter
        syscall_trace_enter(pid, sys_id, sub->arg0, sub->arg1, sub->arg2, sub->arg3, current_tick);

        // USP: Cryptographic Capability Verification
        // User must provide a valid capability token for this specific syscall.
        // E.g., to call SYS_MEM_LEASE, you need the Memory Allocator Capability.
        if (!cap_registry_verify(sub->cap_token, pid, 0x01)) {
            sub->result = -1; // EPERM
            sub->status = 2;  // Error
            watchdog_trigger_fault(pid, "SYSCALL_UNAUTHORIZED_CAPABILITY");
        } else {
            // Dispatch
            if (sys_id < MAX_SYSCALLS && syscall_table[sys_id]) {
                sub->result = syscall_table[sys_id](pid, sub->arg0, sub->arg1, sub->arg2, sub->arg3);
                sub->status = 1; // Complete
            } else {
                sub->result = -38; // ENOSYS
                sub->status = 2;   // Error
            }
        }

        // Trace Exit
        syscall_trace_exit(pid, sys_id, (int32_t)sub->result);
        
        queue->tail++;
    }
}

// Legacy Synchronous Syscall Entry Point (Interrupt 0x80 or SYSCALL instruction)
// Handled for backwards compatibility, but mapped internally to the async engine.
int64_t syscall_sync_entry(uint32_t pid, uint32_t syscall_id, uint64_t arg0, uint64_t arg1, uint64_t arg2, uint64_t arg3, uint32_t cap_token, uint64_t tick) {
    syscall_trace_enter(pid, syscall_id, arg0, arg1, arg2, arg3, tick);
    
    int64_t result = -38; // ENOSYS

    if (!cap_registry_verify(cap_token, pid, 0x01)) {
        watchdog_trigger_fault(pid, "SYSCALL_UNAUTHORIZED_CAPABILITY");
        result = -1; // EPERM
    } else if (syscall_id < MAX_SYSCALLS && syscall_table[syscall_id]) {
        result = syscall_table[syscall_id](pid, arg0, arg1, arg2, arg3);
    }

    syscall_trace_exit(pid, syscall_id, (int32_t)result);
    return result;
}

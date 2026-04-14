// SigmaOS Sovereign System Call Interface (SCI)
// Absorbs Linux syscall table + Windows NT Executive + Mach Traps
// Zero-dependency, hardware-trap-based, C11 pure.

#include <sigma_types.h>

#define SIGMA_SYSCALL_MAX   512

typedef int64_t (*SyscallHandler)(uint64_t arg0, uint64_t arg1,
                                   uint64_t arg2, uint64_t arg3,
                                   uint64_t arg4, uint64_t arg5);

static SyscallHandler syscall_table[SIGMA_SYSCALL_MAX];

// ---- Core Syscall Definitions ----
#define SYS_SIGMA_EXIT        0
#define SYS_SIGMA_FORK        1
#define SYS_SIGMA_READ        2
#define SYS_SIGMA_WRITE       3
#define SYS_SIGMA_OPEN        4
#define SYS_SIGMA_CLOSE       5
#define SYS_SIGMA_MMAP        9
#define SYS_SIGMA_IPC_SEND   20
#define SYS_SIGMA_IPC_RECV   21
#define SYS_SIGMA_SHARD_LOAD 50  // Load a new sovereign shard at runtime
#define SYS_SIGMA_CAPABILITY 80  // Request a zero-trust capability token
#define SYS_SIGMA_BIOMETRIC  81  // Invoke biometric authentication gate

// Register a syscall handler (called during kernel init)
void sci_register_handler(uint32_t syscall_id, SyscallHandler handler);

// Central dispatch — called by the hardware trap vector (INT 0x80 / SYSCALL MSR)
int64_t sci_dispatch(uint32_t syscall_id, uint64_t a0, uint64_t a1,
                     uint64_t a2, uint64_t a3, uint64_t a4, uint64_t a5);

// Initialize the syscall table with all built-in implementations
void sci_init_table(void);




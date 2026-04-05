/* 
 Σ SIGMAOS ZENITH: SOVEREIGN SYSCALL DISPATCHER (v1600.0)
 Mission: User-to-Kernel Mode Transition & ABI Fulfillment.
*/

#include "../libc/SovereignLibC.h"

// Σ SYSCALL ABI TABLE
#define SIGMA_SYS_READ  0x00
#define SIGMA_SYS_WRITE 0x01
#define SIGMA_SYS_OPEN  0x02
#define SIGMA_SYS_CLOSE 0x03
#define SIGMA_SYS_EXIT  0x3C

// Σ THE CENTRAL SYSCALL HANDLER
// (RDI: OpCode, RSI, RDX, R10...: Parameters)
void sigma_syscall_handler(uint64_t opcode, uint64_t arg1, uint64_t arg2, uint64_t arg3) {
    switch (opcode) {
        case SIGMA_SYS_WRITE:
            break;
            
        case SIGMA_SYS_EXIT:
            // Terminate current process
            // (Placeholder: halt current task)
            break;
            
        default:
            break;
    }
}

// Σ INT 0x80 / SYSCALL Vector Setup
void sigma_syscall_init() {
}

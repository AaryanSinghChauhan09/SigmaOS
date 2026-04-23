#include <stdint.h>
#include <stddef.h>

// ---------------------------------------------------------
// SigmaOS System Call Layer Prototype
// ---------------------------------------------------------

#define MAX_SYSCALLS 256

// Array of function pointers for syscalls
static void* syscall_table[MAX_SYSCALLS];

// Register a syscall handler (called during kernel init)
void register_syscall(int num, void* handler) {
    if (num < MAX_SYSCALLS) {
        syscall_table[num] = handler;
    }
}

// Syscall Dispatcher (called by architecture specific interrupt handlers like int 0x80)
void syscall_dispatcher(uint32_t syscall_num, void* arg1, void* arg2, void* arg3) {
    if (syscall_num >= MAX_SYSCALLS || syscall_table[syscall_num] == NULL) {
        // Return ENOSYS (Function not implemented)
        return;
    }
    
    // Cast and execute handler
    void (*handler)(void*, void*, void*) = syscall_table[syscall_num];
    handler(arg1, arg2, arg3);
}

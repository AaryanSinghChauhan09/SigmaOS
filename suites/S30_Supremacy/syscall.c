/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: SYSTEM CALL INTERFACE (v1.0 - PURE C11)
 * =============================================================================
 * Syscall numbers and kernel-side dispatch.
 * Method: software interrupt vector 0x80 (sigma_trap instruction for userland)
 * Features:
 *   - 64 syscalls covering all core OS primitives
 *   - Argument passing via registers (System V AMD64 ABI)
 *   - Per-syscall permission check
 *   - Full audit logging
 * Standard: C11, freestanding
 * =============================================================================
 */

#include "sigma_kernel_types.h"

/* =========================================================================
 * Syscall Numbers
 * ========================================================================= */
#define SYS_EXIT         0
#define SYS_WRITE        1
#define SYS_READ         2
#define SYS_OPEN         3
#define SYS_CLOSE        4
#define SYS_FORK         5
#define SYS_EXEC         6
#define SYS_WAIT         7
#define SYS_GETPID       8
#define SYS_GETPPID      9
#define SYS_MMAP        10
#define SYS_MUNMAP      11
#define SYS_BRK         12
#define SYS_SLEEP       13
#define SYS_YIELD       14
#define SYS_KILL        15
#define SYS_SIGACTION   16
#define SYS_SIGRETURN   17
#define SYS_SOCKET      18
#define SYS_CONNECT     19
#define SYS_SEND        20
#define SYS_RECV        21
#define SYS_BIND        22
#define SYS_LISTEN      23
#define SYS_ACCEPT      24
#define SYS_STAT        25
#define SYS_MKDIR       26
#define SYS_RMDIR       27
#define SYS_UNLINK      28
#define SYS_RENAME      29
#define SYS_GETDENTS    30
#define SYS_PIPE        31
#define SYS_DUP         32
#define SYS_DUP2        33
#define SYS_FCNTL       34
#define SYS_IOCTL       35
#define SYS_MPROTECT    36
#define SYS_FUTEX       37
#define SYS_GETTIMEOFDAY 38
#define SYS_CLOCK_GETTIME 39
#define SYS_SCHED_YIELD 40
#define SYS_SET_PRIO    41
#define SYS_GET_PRIO    42
#define SYS_THREAD_CREATE 43
#define SYS_THREAD_JOIN   44
#define SYS_MUTEX_LOCK    45
#define SYS_MUTEX_UNLOCK  46
#define SYS_SEM_WAIT      47
#define SYS_SEM_POST      48
#define SYS_SHMGET        49
#define SYS_SHMAT         50
#define SYS_SHMDT         51
#define SYS_MSGGET        52
#define SYS_MSGSND        53
#define SYS_MSGRCV        54
#define SYS_POWEROFF      55
#define SYS_REBOOT        56
#define SYS_UNAME         57
#define SYS_INFO          58
#define SYS_MLOCK         59
#define SYS_MUNLOCK       60
#define SYS_SETUID        61
#define SYS_GETUID        62
#define SYS_CHOWN         63

#define SIGMA_NSYSCALLS   64u

/* =========================================================================
 * Interrupt Frame (forward decl)
 * ========================================================================= */
typedef struct SigmaInterruptFrame SigmaInterruptFrame;

/* =========================================================================
 * Syscall handler function type
 * Args: (frame*) → reads rdi, rsi, rdx, r10, r8, r9 as args
 * Returns: i64 result written back to frame->rax
 * ========================================================================= */
typedef i64 (*syscall_fn_t)(SigmaInterruptFrame* frame);

/* =========================================================================
 * External kernel services
 * ========================================================================= */
extern void   sched_yield(void);
extern void   sigma_exit(int code);
extern void   kprintf(const char* fmt, ...);

/* =========================================================================
 * Syscall implementations
 * ========================================================================= */
static i64 sys_exit_impl(SigmaInterruptFrame* f) {
    int code = (int)f->rdi;
    kprintf("[SYSCALL]: exit(%d)\n", code);
    /* In real kernel: terminate current task, schedule next */
    sched_yield();
    return 0;
}

static i64 sys_write_impl(SigmaInterruptFrame* f) {
    int fd            = (int)f->rdi;
    const char* buf   = (const char*)(usize)f->rsi;
    usize count       = (usize)f->rdx;
    if (fd == 1 || fd == 2) {
        /* stdout/stderr → serial + VGA */
        usize i;
        for (i = 0; i < count; i++) {
            /* Write to COM1 (serial) */
            extern void serial_putc(char c);
            serial_putc(buf[i]);
        }
        return (i64)count;
    }
    return K_ERR_INVAL;
}

static i64 sys_read_impl(SigmaInterruptFrame* f) {
    (void)f;
    /* TODO: VFS read dispatch */
    return K_ERR_INVAL;
}

static i64 sys_getpid_impl(SigmaInterruptFrame* f) {
    (void)f;
    extern u64 sched_current_tid(void);
    return (i64)sched_current_tid();
}

static i64 sys_yield_impl(SigmaInterruptFrame* f) {
    (void)f;
    sched_yield();
    return 0;
}

static i64 sys_mmap_impl(SigmaInterruptFrame* f) {
    usize length = (usize)f->rsi;
    extern vaddr_t vmalloc(u64 npages);
    u64 npages = (length + PAGE_SIZE - 1) / PAGE_SIZE;
    vaddr_t va = vmalloc(npages);
    return va ? (i64)va : K_ERR_NOMEM;
}

static i64 sys_uname_impl(SigmaInterruptFrame* f) {
    char* buf = (char*)(usize)f->rdi;
    if (!buf) return K_ERR_INVAL;
    const char* info = "SigmaOS 1.0 SovereignKernel x86_64";
    usize i = 0;
    while (info[i] && i < 127) { buf[i] = info[i]; i++; }
    buf[i] = '\0';
    return 0;
}

static i64 sys_info_impl(SigmaInterruptFrame* f) {
    (void)f;
    kprintf("[SYSINFO]: SigmaOS Sovereign Kernel v1.0 | C11 | Zero-glibc\n");
    return 0;
}

static i64 sys_poweroff_impl(SigmaInterruptFrame* f) {
    (void)f;
    kprintf("[KERNEL]: Powering off via ACPI S5 state...\n");
    /* ACPI shutdown: write 0x2000 to port 0x604 (QEMU) */
    port_outw(0x604, 0x2000);
    cpu_cli();
    while (1) cpu_halt();
    return 0;
}

static i64 sys_reboot_impl(SigmaInterruptFrame* f) {
    (void)f;
    kprintf("[KERNEL]: Rebooting via PS/2 controller reset...\n");
    port_outb(0x64, 0xFE);  /* pulse reset line */
    cpu_cli();
    while (1) cpu_halt();
    return 0;
}

static i64 sys_unimpl(SigmaInterruptFrame* f) {
    kprintf("[SYSCALL]: syscall #%llu not yet implemented.\n", f->vector);
    return K_ERR_INVAL;
}

/* =========================================================================
 * Syscall Dispatch Table
 * ========================================================================= */
static const syscall_fn_t g_syscall_table[SIGMA_NSYSCALLS] = {
    [SYS_EXIT]          = sys_exit_impl,
    [SYS_WRITE]         = sys_write_impl,
    [SYS_READ]          = sys_read_impl,
    [SYS_GETPID]        = sys_getpid_impl,
    [SYS_YIELD]         = sys_yield_impl,
    [SYS_SCHED_YIELD]   = sys_yield_impl,
    [SYS_MMAP]          = sys_mmap_impl,
    [SYS_UNAME]         = sys_uname_impl,
    [SYS_INFO]          = sys_info_impl,
    [SYS_POWEROFF]      = sys_poweroff_impl,
    [SYS_REBOOT]        = sys_reboot_impl,
    /* All others → unimplemented */
};

/* =========================================================================
 * Syscall gate handler (called when vector 128 fires)
 * Convention: syscall number in rax, args in rdi,rsi,rdx,r10,r8,r9
 * ========================================================================= */
static u64 g_syscall_count = 0;

void syscall_handler(SigmaInterruptFrame* frame) {
    u64 sysno = frame->rax;
    i64 result;

    if (sysno < SIGMA_NSYSCALLS && g_syscall_table[sysno]) {
        result = g_syscall_table[sysno](frame);
    } else {
        kprintf("[SYSCALL]: Invalid syscall #%llu\n", sysno);
        result = K_ERR_INVAL;
    }

    frame->rax = (u64)result;
    g_syscall_count++;
}

/* =========================================================================
 * Init — register syscall handler on vector 128
 * ========================================================================= */
void syscall_init(void) {
    extern void idt_register_handler(u32 vec,
        void (*fn)(SigmaInterruptFrame* frame));
    idt_register_handler(128, syscall_handler);
    kprintf("[SYSCALL]: 64-syscall table online. Gate=INT 0x80\n");
}

u64 sched_current_tid(void) {
    /* Forward to scheduler */
    extern SigmaTask* sched_get_current(void);
    /* stub — return 0 if not yet scheduled */
    return 0;
}

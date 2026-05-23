/*
 * Σ SigmaOS — sigma_strace: Sovereign Syscall Tracer
 * Absorbs: Linux strace(1), ftrace, eBPF syscall tracing, Solaris truss
 * Features: Hook and log SigmaOS syscall dispatch ring; decode syscall names,
 *           args, return values with minimal overhead ring-buffer recording.
 * Zero-Dependency: No libc. Sovereign ring-buffer + syscall table.
 */

typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;
typedef unsigned long long u64;
typedef long long          s64;

extern "C" void sigma_vga_puts(const char* s);
extern "C" void sigma_vga_putchar(char c);
extern "C" void sigma_vga_printf(const char* fmt, ...);

/* ─────────────── SigmaOS Syscall Numbers ─────────────── */
/* Mirrors kernel/syscall/sigma_syscall.h, absorbed from Linux x86_64 ABI */
#define SYS_READ         0
#define SYS_WRITE        1
#define SYS_OPEN         2
#define SYS_CLOSE        3
#define SYS_STAT         4
#define SYS_FSTAT        5
#define SYS_LSTAT        6
#define SYS_POLL         7
#define SYS_LSEEK        8
#define SYS_MMAP         9
#define SYS_MPROTECT     10
#define SYS_MUNMAP       11
#define SYS_BRK          12
#define SYS_SIGACTION    13
#define SYS_SIGPROCMASK  14
#define SYS_IOCTL        16
#define SYS_PREAD64      17
#define SYS_PWRITE64     18
#define SYS_READV        19
#define SYS_WRITEV       20
#define SYS_PIPE         22
#define SYS_SELECT       23
#define SYS_SCHED_YIELD  24
#define SYS_FORK         57
#define SYS_EXEC         59
#define SYS_EXIT         60
#define SYS_WAIT4        61
#define SYS_KILL         62
#define SYS_GETPID       39
#define SYS_GETPPID      110
#define SYS_SOCKET       41
#define SYS_CONNECT      42
#define SYS_ACCEPT       43
#define SYS_SENDTO       44
#define SYS_RECVFROM     45
#define SYS_BIND         49
#define SYS_LISTEN       50
#define SYS_GETSOCKNAME  51
#define SYS_CLONE        56
#define SYS_OPENAT       257
#define SYS_MKDIRAT      258
#define SYS_UNLINKAT     263
#define SYS_RENAMEAT     264
#define SYS_FACCESSAT   269
#define SYS_SIGMA_SHMALLOC 400 /* SigmaOS custom */
#define SYS_SIGMA_IPCPOST  401
#define SYS_SIGMA_SVCREQ   402

/* ─────────────── Syscall Name Table ─────────────── */
struct SyscallName { u32 num; const char* name; };

static const SyscallName syscall_names[] = {
    { SYS_READ,        "read"         }, { SYS_WRITE,       "write"       },
    { SYS_OPEN,        "open"         }, { SYS_CLOSE,       "close"       },
    { SYS_STAT,        "stat"         }, { SYS_FSTAT,       "fstat"       },
    { SYS_LSTAT,       "lstat"        }, { SYS_POLL,        "poll"        },
    { SYS_LSEEK,       "lseek"        }, { SYS_MMAP,        "mmap"        },
    { SYS_MPROTECT,    "mprotect"     }, { SYS_MUNMAP,      "munmap"      },
    { SYS_BRK,         "brk"          }, { SYS_SIGACTION,   "rt_sigaction"},
    { SYS_SIGPROCMASK, "rt_sigprocmask" }, { SYS_IOCTL,     "ioctl"       },
    { SYS_PREAD64,     "pread64"      }, { SYS_PWRITE64,    "pwrite64"    },
    { SYS_READV,       "readv"        }, { SYS_WRITEV,      "writev"      },
    { SYS_PIPE,        "pipe"         }, { SYS_SELECT,      "select"      },
    { SYS_SCHED_YIELD, "sched_yield"  }, { SYS_FORK,        "fork"        },
    { SYS_EXEC,        "execve"       }, { SYS_EXIT,        "exit"        },
    { SYS_WAIT4,       "wait4"        }, { SYS_KILL,        "kill"        },
    { SYS_GETPID,      "getpid"       }, { SYS_GETPPID,     "getppid"     },
    { SYS_SOCKET,      "socket"       }, { SYS_CONNECT,     "connect"     },
    { SYS_ACCEPT,      "accept"       }, { SYS_SENDTO,      "sendto"      },
    { SYS_RECVFROM,    "recvfrom"     }, { SYS_BIND,        "bind"        },
    { SYS_LISTEN,      "listen"       }, { SYS_GETSOCKNAME, "getsockname" },
    { SYS_CLONE,       "clone"        }, { SYS_OPENAT,      "openat"      },
    { SYS_MKDIRAT,     "mkdirat"      }, { SYS_UNLINKAT,    "unlinkat"    },
    { SYS_RENAMEAT,    "renameat"     }, { SYS_FACCESSAT,   "faccessat"   },
    { SYS_SIGMA_SHMALLOC,"sigma_shmalloc" },
    { SYS_SIGMA_IPCPOST, "sigma_ipc_post" },
    { SYS_SIGMA_SVCREQ,  "sigma_svc_request" },
    { 0xFFFFFFFF, nullptr }
};

static const char* syscall_name(u32 num) {
    for (u32 i = 0; syscall_names[i].name; i++)
        if (syscall_names[i].num == num) return syscall_names[i].name;
    return "unknown";
}

/* ─────────────── Trace Ring Buffer ─────────────── */
#define TRACE_RING_SIZE 1024

struct TraceEntry {
    u32 pid;
    u32 syscall_num;
    u64 args[6];
    s64 retval;
    bool has_retval;
    u64 timestamp; /* TSC-based, sovereign clock */
};

static TraceEntry trace_ring[TRACE_RING_SIZE];
static u32 trace_head = 0;
static u32 trace_tail = 0;
static bool trace_active = false;
static u32  trace_filter_pid = 0; /* 0 = trace all */

/* ─────────────── TSC Timestamp ─────────────── */
static u64 read_tsc() {
    u32 lo, hi;
    __asm__ volatile("rdtsc" : "=a"(lo), "=d"(hi));
    return ((u64)hi << 32) | lo;
}

/* ─────────────── Public Trace API (called from kernel syscall dispatcher) ─────────────── */
extern "C" void sigma_strace_record_entry(u32 pid, u32 syscall_num,
                                           u64 a0, u64 a1, u64 a2,
                                           u64 a3, u64 a4, u64 a5) {
    if (!trace_active) return;
    if (trace_filter_pid && pid != trace_filter_pid) return;

    u32 idx = trace_head % TRACE_RING_SIZE;
    trace_ring[idx].pid         = pid;
    trace_ring[idx].syscall_num = syscall_num;
    trace_ring[idx].args[0]     = a0;
    trace_ring[idx].args[1]     = a1;
    trace_ring[idx].args[2]     = a2;
    trace_ring[idx].args[3]     = a3;
    trace_ring[idx].args[4]     = a4;
    trace_ring[idx].args[5]     = a5;
    trace_ring[idx].has_retval  = false;
    trace_ring[idx].timestamp   = read_tsc();
    trace_head++;
}

extern "C" void sigma_strace_record_exit(u32 pid, u32 syscall_num, s64 retval) {
    if (!trace_active) return;
    /* Walk back to find the matching entry */
    u32 i = trace_head;
    u32 count = 0;
    while (count < TRACE_RING_SIZE) {
        i--;
        u32 idx = i % TRACE_RING_SIZE;
        if (trace_ring[idx].pid == pid &&
            trace_ring[idx].syscall_num == syscall_num &&
            !trace_ring[idx].has_retval) {
            trace_ring[idx].retval    = retval;
            trace_ring[idx].has_retval = true;
            return;
        }
        count++;
    }
}

/* ─────────────── Print Helpers ─────────────── */
static void print_hex64(u64 v) {
    static const char h[] = "0123456789abcdef";
    sigma_vga_puts("0x");
    for (int i = 60; i >= 0; i -= 4)
        sigma_vga_putchar(h[(v >> i) & 0xF]);
}

static void print_s64(s64 v) {
    if (v < 0) { sigma_vga_putchar('-'); v = -v; }
    if (v >= 10) print_s64(v / 10);
    sigma_vga_putchar('0' + (v % 10));
}

static void dump_entry(const TraceEntry* e) {
    /* Format: [PID] syscall_name(a0, a1, a2, ...) = retval */
    sigma_vga_printf("[%4u] %s(", e->pid, syscall_name(e->syscall_num));

    /* Print first 3 args (most syscalls use ≤3) */
    for (u32 i = 0; i < 3; i++) {
        if (i > 0) sigma_vga_puts(", ");
        print_hex64(e->args[i]);
    }

    sigma_vga_puts(")");
    if (e->has_retval) {
        sigma_vga_puts(" = ");
        print_s64(e->retval);
        if (e->retval < 0) sigma_vga_puts(" (error)");
    } else {
        sigma_vga_puts(" ...");
    }
    sigma_vga_putchar('\n');
}

/* ─────────────── Main ─────────────── */
extern "C" int sigma_strace_main(int argc, char** argv) {
    bool opt_dump   = false;
    bool opt_start  = false;
    bool opt_stop   = false;
    bool opt_clear  = false;

    for (int i = 1; i < argc; i++) {
        const char* a = argv[i];
        if (a[0] == '-') {
            switch (a[1]) {
                case 'd': opt_dump  = true; break;
                case 's': opt_start = true; break;
                case 'S': opt_stop  = true; break;
                case 'c': opt_clear = true; break;
                case 'p': /* -p PID: filter by process */
                    if (i + 1 < argc) {
                        u32 pid = 0;
                        for (u32 j = 0; argv[i+1][j]; j++)
                            pid = pid * 10 + (argv[i+1][j] - '0');
                        trace_filter_pid = pid;
                        i++;
                    }
                    break;
            }
        }
    }

    if (opt_start) {
        trace_active = true;
        sigma_vga_puts("strace: tracing enabled\n");
    }
    if (opt_stop) {
        trace_active = false;
        sigma_vga_puts("strace: tracing disabled\n");
    }
    if (opt_clear) {
        trace_head = trace_tail = 0;
        sigma_vga_puts("strace: trace buffer cleared\n");
    }
    if (opt_dump) {
        u32 count = trace_head - trace_tail;
        if (count > TRACE_RING_SIZE) count = TRACE_RING_SIZE;
        sigma_vga_printf("strace: %u entries\n", count);

        for (u32 i = 0; i < count; i++) {
            u32 idx = (trace_tail + i) % TRACE_RING_SIZE;
            dump_entry(&trace_ring[idx]);
        }
    }

    if (!opt_start && !opt_stop && !opt_clear && !opt_dump) {
        sigma_vga_puts("sigma-strace — Sovereign Syscall Tracer\n");
        sigma_vga_puts("Usage:\n");
        sigma_vga_puts("  strace -s         Start tracing\n");
        sigma_vga_puts("  strace -S         Stop tracing\n");
        sigma_vga_puts("  strace -d         Dump trace buffer\n");
        sigma_vga_puts("  strace -c         Clear trace buffer\n");
        sigma_vga_puts("  strace -p <pid>   Filter by PID\n");
    }
    return 0;
}

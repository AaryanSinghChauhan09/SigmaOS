# C Programming → SigmaOS Developer API Layer

> Maps the C Programming syllabus to SigmaOS kernel C APIs, developer tools, and the SigmaCLI runtime.

---

## Unit I: Pre-Programming & Getting Started

### Language Classification in SigmaOS Context

| Language Level | Examples | SigmaOS Layer |
|---|---|---|
| Machine Language | Binary opcodes | CPU ISA (x86, ARM) |
| Assembly | NASM, AT&T syntax | Kernel boot (`sigma_boot.asm`) |
| High-Level Procedural | C | Kernel C shards, tools |
| High-Level OOP | C++17 | Core kernel modules |
| Non-Procedural | SQL, HTML | SigmaDB, SigmaWeb |

### Algorithm → SigmaOS Flow

```
Problem → Algorithm → Flowchart → C Code → Kernel Module
    ↓           ↓           ↓          ↓           ↓
 Spec         Pseudocode   Diagram   .c file   .ko shard
```

### Basic C Structure in SigmaOS

```c
/* kernel/tools/sigma_example.c
 * SigmaOS C coding standard:
 * - No stdlib malloc (use sigma_kmalloc)
 * - No printf (use sigma_klog)
 * - No exit() (use sigma_panic)
 */
#include "sigma_kernel_types.h"
#include "sigma_klog.h"

/* Character set: letters, digits, special chars, whitespace */
/* Tokens: keywords, identifiers, constants, operators, strings */

/* Keywords (C11): auto, break, case, char, const, continue,
   default, do, double, else, enum, extern, float, for, goto,
   if, inline, int, long, register, restrict, return, short,
   signed, sizeof, static, struct, switch, typedef, union,
   unsigned, void, volatile, while, _Bool, _Complex */

/* Data Types */
sigma_u8    byte_val   = 255;         /* unsigned 8-bit  */
sigma_i32   count      = -1;          /* signed 32-bit   */
sigma_u64   address    = 0xFFFF0000;  /* unsigned 64-bit */
float       ratio      = 3.14f;
double      precise    = 3.14159265;
char        ch         = 'A';         /* ASCII value 65  */

/* Constants */
#define SIGMA_MAX_PROCS  4096
const sigma_u32 PAGE_SIZE = 4096;

/* Type Casting */
sigma_u64 ptr_val = (sigma_u64)some_pointer;  /* pointer to integer */
int truncated = (int)3.99;  /* → 3 */
```

---

## Unit II: Operators, Expressions & Control Structures

```c
/* Operator Precedence (high → low):
   () []  →  Postfix
   ++ -- ! ~ (type) * & sizeof  →  Unary
   * / %  →  Multiplicative
   + -    →  Additive
   << >>  →  Shift
   < <= > >=  →  Relational
   == !=  →  Equality
   &  →  Bitwise AND
   ^  →  Bitwise XOR
   |  →  Bitwise OR
   && →  Logical AND
   || →  Logical OR
   ?: →  Ternary
   = += -= *= /= %=  →  Assignment
*/

/* Bitwise ops critical in kernel */
sigma_u32 flags = 0;
flags |= (1 << 3);   /* Set bit 3   */
flags &= ~(1 << 3);  /* Clear bit 3 */
flags ^= (1 << 3);   /* Toggle bit 3*/

/* Console I/O — kernel version */
sigma_klog(LOG_INFO, "Value: %d\n", count);  /* No printf in kernel */

/* Control Structures */
if (privilege_level == RING0) {
    grant_access();
} else if (privilege_level == RING3) {
    request_syscall();
} else {
    sigma_panic("Invalid ring level");
}

switch (syscall_id) {
    case SYS_READ:  do_read(args);  break;
    case SYS_WRITE: do_write(args); break;
    default:        return -ENOSYS;
}

/* Loops */
for (int i = 0; i < MAX_PROCS; i++) proc_table[i].state = PROC_DEAD;

int retry = 0;
while (retry < 3 && !device_ready()) { retry++; sigma_sleep_ms(100); }

do { flush_cache(); } while (cache_dirty());

/* Jumping Statements */
for (int i = 0; i < n; i++) {
    if (arr[i] == target) { found = i; break; }
    if (arr[i] < 0) continue;  /* skip negatives */
}
```

---

## Unit III: Arrays, Pointers & Strings

```c
/* Arrays */
sigma_u8 page_buffer[4096];                    /* 1D array  */
sigma_u32 page_table[512][512];                /* 2D array  */
sigma_u8 framebuffer[1080][1920][4];           /* 3D: H×W×RGBA */

/* Initialization */
int primes[] = {2, 3, 5, 7, 11, 13};
char kernel_name[] = "SigmaOS Zenith";

/* Pointers — fundamental to kernel programming */
sigma_u64* cr3_register = (sigma_u64*)0xFFFFFF80;  /* physical addr */
*cr3_register = page_dir_base;                       /* write to addr */

/* Pointer arithmetic */
sigma_u8* ptr = page_buffer;
ptr += 512;  /* advance 512 bytes */

/* Array of pointers (process table) */
Process* proc_table[MAX_PROCS];

/* Pointer to array */
int (*row_ptr)[512] = page_table;  /* points to entire row */

/* Pointer to function — used for driver vtables */
typedef int (*read_fn_t)(void* buf, size_t len);
read_fn_t driver_read = &nvme_read;
driver_read(buffer, 512);

/* Strings */
char greeting[64] = "SigmaOS";
size_t len = sigma_strlen(greeting);     /* kernel strlen */
sigma_strcpy(dest, src, sizeof(dest));   /* safe strcpy  */
sigma_strcat(greeting, " Zenith", 64);
int cmp = sigma_strcmp(str1, str2);
char* found = sigma_strstr(haystack, "kernel");
```

---

## Unit IV: Functions, Structures, Unions & File Handling

```c
/* Function categories */

/* 1. Standard Library (kernel equivalents) */
void* sigma_kmalloc(size_t size);           /* malloc equivalent */
void  sigma_kfree(void* ptr);               /* free equivalent   */
void  sigma_memcpy(void* dst, const void* src, size_t n);
void  sigma_memset(void* ptr, int val, size_t n);

/* 2. User-Defined Functions */
static int calc_checksum(const sigma_u8* data, size_t len) {
    sigma_u32 sum = 0;
    for (size_t i = 0; i < len; i++) sum += data[i];
    return (int)(sum & 0xFF);
}

/* 3. Recursion — used in tree traversal */
sigma_u64 factorial(int n) {
    if (n <= 1) return 1;
    return n * factorial(n - 1);  /* Stack depth bounded by kernel stack */
}

/* Call by Value vs Call by Reference */
void increment_val(int x)    { x++; }          /* copy — no effect outside */
void increment_ref(int* x)   { (*x)++; }       /* pointer — modifies original */

/* Storage Classes */
static  int module_count = 0;    /* file-scope only     */
extern  int global_tick;         /* defined elsewhere   */
register int loop_var;           /* hint: use register  */
/* auto: default for local vars */

/* Structures */
typedef struct {
    sigma_u32  pid;
    char       name[64];
    ProcessState state;
    sigma_u64  stack_base;
    sigma_u64  stack_top;
} Process;

Process p1 = { .pid=1, .name="init", .state=PROC_RUNNING };
p1.state = PROC_SLEEPING;

/* Pointer to Structure */
Process* proc_ptr = &p1;
proc_ptr->state = PROC_READY;       /* arrow operator */
(*proc_ptr).pid = 2;                /* equivalent     */

/* Union — overlapping memory (used in registers, packets) */
typedef union {
    sigma_u64 raw;
    struct { sigma_u32 lo; sigma_u32 hi; } parts;
    sigma_u8 bytes[8];
} Register64;

Register64 rax;
rax.raw = 0xDEADBEEFCAFEBABE;
/* rax.bytes[0] = 0xBE (little-endian) */

/* File Handling — VFS wrappers */
SigmaFile* f = sigma_fopen("/sigma/log/kernel.log", "a");
sigma_fprintf(f, "[%s] Boot complete\n", sigma_timestamp());
int ch = sigma_getc(f);
sigma_putc('X', f);
sigma_fclose(f);

/* Binary I/O */
SigmaFile* bin = sigma_fopen("/sigma/data/proc.bin", "rb");
sigma_fread(&process_data, sizeof(Process), 1, bin);
sigma_fwrite(&process_data, sizeof(Process), 1, output);
sigma_fclose(bin);
```

---

## SigmaOS C Developer API Summary

| C Concept | SigmaOS API | File |
|---|---|---|
| malloc/free | `sigma_kmalloc` / `sigma_kfree` | `kernel/core/SovereignAllocator.h` |
| printf | `sigma_klog(level, fmt, ...)` | `kernel/core/sigma_klog.h` |
| string.h | `sigma_str*` functions | `kernel/libc/sigma_string.h` |
| fopen/fclose | `sigma_fopen` / `sigma_fclose` | `kernel/fs/sigma_vfs.h` |
| assert | `SIGMA_ASSERT(cond, msg)` | `kernel/core/sigma_debug.h` |

*Last updated: 2026-05-18 | SigmaOS Zenith v15.1*

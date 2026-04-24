/* sigma_kernel_types.h — Sovereign canonical shim */
#ifndef SIGMA_KERNEL_TYPES_H
#define SIGMA_KERNEL_TYPES_H
typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;
typedef unsigned long long u64;
typedef signed char        i8;
typedef signed short       i16;
typedef signed int         i32;
typedef signed long long   i64;
typedef unsigned long long usize;
typedef long long          isize;
typedef u64                paddr_t;
typedef u64                vaddr_t;
typedef int                bool_t;
typedef unsigned char      sigma_u8;
typedef unsigned short     sigma_u16;
typedef unsigned int       sigma_u32;
typedef unsigned long long sigma_u64;
typedef signed long long   sigma_ssize_t;
typedef unsigned long long sigma_size_t;
typedef int                sigma_bool;
typedef sigma_u32          uint32_t;
typedef sigma_u64          uint64_t;
typedef sigma_u8           uint8_t;
typedef sigma_u16          uint16_t;
#ifndef TRUE
#define TRUE  1
#define FALSE 0
#endif
#ifndef NULL
#define NULL ((void*)0)
#endif
#define PAGE_SIZE    4096ULL
#define PAGE_SHIFT   12u
#define K_OK         0
#define K_ERR_NOMEM -1
#define K_ERR_INVAL -2
#define K_ERR_NODEV -3
#define K_ERR_NOTFOUND -4
typedef int k_status;

#define BIT(n)       (1ULL << (n))
#define KERNEL_VMA   0xFFFFFFFF80000000ULL

/* Jail type for virtualization shards */
typedef struct sigma_jail {
    sigma_u32 id;
    sigma_u32 flags;
    const char* namespace_root;
} sigma_jail_t;

/* Unit type for orchestration shards */
typedef struct sigma_unit {
    const char* name;
    sigma_u32 state;
} sigma_unit_t;

static inline void cpu_halt(void)  { __asm__ __volatile__("cli; hlt"); }
static inline void cpu_pause(void) { __asm__ __volatile__("pause"); }
void sigma_panic(const char* msg, u64 rip, u64 rsp);
#define SIGMA_ASSERT(cond, msg) do { if (!(cond)) sigma_panic(msg, 0, 0); } while(0)
#endif /* SIGMA_KERNEL_TYPES_H */

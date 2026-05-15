/* sigma_kernel_types.h — Sovereign canonical shim */
#ifndef SIGMA_KERNEL_TYPES_H
#define SIGMA_KERNEL_TYPES_H

/* Standard types */
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

/* Sigma-prefixed types */
typedef u8  sigma_u8;
typedef u16 sigma_u16;
typedef u32 sigma_u32;
typedef u64 sigma_u64;

typedef i8  sigma_i8;
typedef i16 sigma_i16;
typedef i32 sigma_i32;
typedef i64 sigma_i64;

typedef u64 sigma_size_t;
typedef i64 sigma_ssize_t;
typedef u64 sigma_usize;
typedef i64 sigma_isize;

typedef u64 sigma_addr_t;
typedef u64 paddr_t;
typedef u64 vaddr_t;

typedef int sigma_bool;
typedef int bool_t;

#ifndef TRUE
#define TRUE  1
#define FALSE 0
#endif

#ifndef SIGMA_TRUE
#define SIGMA_TRUE  1u
#define SIGMA_FALSE 0u
#endif

#ifndef NULL
#ifdef __cplusplus
#define NULL nullptr
#else
#define NULL ((void*)0)
#endif
#endif

#ifndef SIGMA_NULL
#define SIGMA_NULL NULL
#endif

#define PAGE_SHIFT   12u

#define K_OK         0
#define K_ERR_NOMEM -1
#define K_ERR_INVAL -2
typedef int k_status;

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

#ifdef __cplusplus
extern "C" {
#endif

void sigma_panic(const char* msg, u64 rip, u64 rsp);

#ifdef __cplusplus
}
#endif

#define SIGMA_ASSERT(cond, msg) do { if (!(cond)) sigma_panic(msg, 0, 0); } while(0)

#endif /* SIGMA_KERNEL_TYPES_H */

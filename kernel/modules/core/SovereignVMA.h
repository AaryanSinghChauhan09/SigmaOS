#ifndef SOVEREIGN_VMA_H
#define SOVEREIGN_VMA_H

#include "../../../include/sigma_kernel.h"

/* Permission bits (like PROT_*) */
#define VM_NONE     0x00
#define VM_READ     0x01
#define VM_WRITE    0x02
#define VM_EXEC     0x04
#define VM_SHARED   0x08
#define VM_ANON     0x10
#define VM_STACK    0x20
#define VM_HEAP     0x40
#define VM_COW      0x80   /* copy-on-write pending */

#define MAX_VMAS_PER_PROC  256

typedef struct SigmaVMA {
    sigma_u64  start;       /* inclusive, page-aligned */
    sigma_u64  end;         /* exclusive, page-aligned */
    sigma_u32  flags;       /* VM_READ | VM_WRITE | VM_EXEC | … */
    char       name[32];    /* "[stack]", "[heap]", filename, etc. */
} SigmaVMA_t;

/* Forward declaration of AddressSpace to avoid circular dependency if needed, 
   but for now we just need the VMA logic. */
struct SigmaAddressSpace;

SigmaVMA_t *vma_find(struct SigmaAddressSpace *as, sigma_u64 addr);
SigmaVMA_t *vma_insert(struct SigmaAddressSpace *as,
                        sigma_u64 start, sigma_u64 end,
                        sigma_u32 flags, const char *name);

#endif /* SOVEREIGN_VMA_H */

#include "sigma_base.h"
#include "suites/S20_Interconnect/shards/SovereignInterconnect.h"

#include "SovereignPMM.h"

static sigma_u64 s_frame_bitmap[FRAME_BITMAP_WORDS]; /* 1=free, 0=used */
static sigma_u32 s_free_frames = (sigma_u32)TOTAL_FRAMES;
static sigma_u32 s_next_frame_hint = 0;

void pmm_init(void) {
    /* Mark all frames free */
    for (sigma_u32 i = 0; i < FRAME_BITMAP_WORDS; i++)
        s_frame_bitmap[i] = ~0ULL;
    /* Reserve first 2 MB (kernel identity map) */
    for (sigma_u32 i = 0; i < 8; i++)
        s_frame_bitmap[i] = 0;
    s_free_frames = (sigma_u32)TOTAL_FRAMES - 512;
    sigma_printf("S [PMM]: %u frames available (%u MB)\n",
                 s_free_frames, s_free_frames * 4 / 1024);
}

sigma_u64 pmm_alloc_frame(void) {
    for (sigma_u32 word = s_next_frame_hint / 64;
         word < FRAME_BITMAP_WORDS; word++) {
        if (s_frame_bitmap[word] == 0) continue;
        for (int bit = 0; bit < 64; bit++) {
            if ((s_frame_bitmap[word] >> bit) & 1ULL) {
                s_frame_bitmap[word] &= ~(1ULL << bit);
                s_free_frames--;
                sigma_u64 frame = (sigma_u64)(word * 64 + bit);
                sigma_u64 phys = frame * PAGE_SIZE;
                s_next_frame_hint = (sigma_u32)frame + 1;
                
                /* Interconnect: Notify lattice of allocation */
                sigma_u64 payload[4] = {phys, s_free_frames, 0, 0};
                OmniFabric_Send(SUITE_MEMORY, SUITE_SECURITY, MSG_TYPE_MEM_ALLOC, payload);
                
                return phys;
            }
        }
    }
    return 0; /* out of memory */
}

void pmm_free_frame(sigma_u64 phys) {
    sigma_u64 frame = phys / PAGE_SIZE;
    sigma_u64 word  = frame / 64;
    sigma_u64 bit   = frame % 64;
    if (word < FRAME_BITMAP_WORDS && !((s_frame_bitmap[word] >> bit) & 1)) {
        s_frame_bitmap[word] |= (1ULL << bit);
        s_free_frames++;
    }
}

sigma_u32 pmm_get_free_count(void) {
    return s_free_frames;
}

#include "SovereignVMA.h"
#include "SovereignAddrSpace.h"

SigmaVMA_t *vma_find(SigmaAddressSpace_t *as, sigma_u64 addr) {
    if (!as) return SIGMA_NULL;
    for (sigma_u32 i = 0; i < as->vma_count; i++) {
        if (addr >= as->vmas[i].start && addr < as->vmas[i].end)
            return &as->vmas[i];
    }
    return SIGMA_NULL;
}

SigmaVMA_t *vma_insert(SigmaAddressSpace_t *as,
                               sigma_u64 start, sigma_u64 end,
                               sigma_u32 flags, const char *name) {
    if (!as || as->vma_count >= MAX_VMAS_PER_PROC) return SIGMA_NULL;
    SigmaVMA_t *v = &as->vmas[as->vma_count++];
    v->start = start; v->end = end; v->flags = flags;
    sigma_strcpy(v->name, name);
    return v;
}

#include "SovereignAddrSpace.h"

#define SHADOW_POOL_SIZE  (MAX_PROCESSES * 256)  /* 256 entries per process */
static SigmaPageEntry_t s_shadow_pool[SHADOW_POOL_SIZE];
static sigma_u32        s_shadow_next = 0;

static SigmaAddressSpace_t s_spaces[MAX_PROCESSES];
static sigma_u32           s_space_count = 0;

void vmm_as_init(void) {
    s_space_count = 0;
    s_shadow_next = 0;
    sigma_memset(s_spaces, 0, sizeof(s_spaces));
    sigma_memset(s_shadow_pool, 0, sizeof(s_shadow_pool));
}

SigmaAddressSpace_t *vmm_get_space(sigma_u32 pid) {
    for (sigma_u32 i = 0; i < s_space_count; i++) {
        if (s_spaces[i].in_use && s_spaces[i].pid == pid)
            return &s_spaces[i];
    }
    return SIGMA_NULL;
}

SigmaAddressSpace_t *vmm_create_space(sigma_u32 pid) {
    if (s_space_count >= MAX_PROCESSES) return SIGMA_NULL;
    SigmaAddressSpace_t *as = &s_spaces[s_space_count++];
    sigma_memset(as, 0, sizeof(*as));
    as->pid        = pid;
    as->in_use     = SIGMA_TRUE;
    as->mmap_base  = 0x0000700000000000ULL; /* mmap region (ASLR base) */
    as->stack_top  = 0x00007FFFFFFFE000ULL;
    as->brk        = 0x0000000010000000ULL; /* heap starts at 256 MB */
    /* Assign shadow table slice */
    if (s_shadow_next + 256 <= SHADOW_POOL_SIZE) {
        as->shadow = &s_shadow_pool[s_shadow_next];
        s_shadow_next += 256;
    }
    return as;
}





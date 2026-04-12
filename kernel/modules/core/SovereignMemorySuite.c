/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN MEMORY MANAGEMENT SUITE (v2.0 - INTEGRATED)
 * =========================================================================
 */

#include "../../../include/sigma_base.h"

/* --- Sub-Module 1: Physical Memory Manager (PMM) --- */
#define MAX_FRAMES 1048576 
static sigma_u8 s_frame_bitmap[MAX_FRAMES / 8];
static sigma_u32 s_free_frames = MAX_FRAMES;

void pmm_init(void) {
    sigma_memset(s_frame_bitmap, 0, sizeof(s_frame_bitmap));
    s_free_frames = MAX_FRAMES;
}

sigma_u64 pmm_alloc_frame(void) {
    for (sigma_u32 i = 0; i < MAX_FRAMES; i++) {
        if (!(s_frame_bitmap[i / 8] & (1 << (i % 8)))) {
            s_frame_bitmap[i / 8] |= (1 << (i % 8));
            s_free_frames--;
            return (sigma_u64)i * 4096;
        }
    }
    return 0;
}

void pmm_free_frame(sigma_u64 phys) {
    sigma_u32 frame = (sigma_u32)(phys / 4096);
    if (frame < MAX_FRAMES) {
        s_frame_bitmap[frame / 8] &= ~(1 << (frame % 8));
        s_free_frames++;
    }
}

sigma_u32 pmm_get_free_count(void) { return s_free_frames; }

/* --- Sub-Module 2: Virtual Memory Area (VMA) --- */
#define VM_READ   (1<<0)
#define VM_WRITE  (1<<1)
#define VM_EXEC   (1<<2)
#define VM_STACK  (1<<3)
#define VM_HEAP   (1<<4)
#define VM_ANON   (1<<5)
#define VM_COW    (1<<6)

typedef struct {
    sigma_u64 start, end;
    sigma_u32 flags;
    char name[32];
} SigmaVMA_t;

typedef struct {
    sigma_u64 phys;
    sigma_u32 flags;
    sigma_bool present;
    sigma_bool dirty;
} SigmaPageEntry_t;

typedef struct {
    sigma_u32 pid;
    SigmaVMA_t vmas[16];
    sigma_u32 vma_count;
    sigma_u64 brk;
    sigma_u64 mmap_base;
    sigma_u64 stack_top;
    SigmaPageEntry_t *shadow;
    sigma_u32 page_faults;
    sigma_u32 cow_breaks;
} SigmaAddressSpace_t;

SigmaVMA_t* vma_find(SigmaAddressSpace_t *as, sigma_u64 addr) {
    for (sigma_u32 i = 0; i < as->vma_count; i++) {
        if (addr >= as->vmas[i].start && addr < as->vmas[i].end) return &as->vmas[i];
    }
    return SIGMA_NULL;
}

SigmaVMA_t* vma_insert(SigmaAddressSpace_t *as, sigma_u64 start, sigma_u64 end, sigma_u32 flags, const char *name) {
    if (as->vma_count >= 16) return SIGMA_NULL;
    SigmaVMA_t *v = &as->vmas[as->vma_count++];
    v->start = start; v->end = end; v->flags = flags;
    sigma_strncpy(v->name, name, 32);
    return v;
}

/* --- Sub-Module 3: Address Space Manager --- */
static SigmaAddressSpace_t s_address_spaces[8];

void vmm_as_init(void) {
    sigma_memset(s_address_spaces, 0, sizeof(s_address_spaces));
}

SigmaAddressSpace_t* vmm_get_space(sigma_u32 pid) {
    for (int i = 0; i < 8; i++) if (s_address_spaces[i].pid == pid) return &s_address_spaces[i];
    return SIGMA_NULL;
}

SigmaAddressSpace_t* vmm_create_space(sigma_u32 pid) {
    for (int i = 0; i < 8; i++) {
        if (s_address_spaces[i].pid == 0) {
            s_address_spaces[i].pid = pid;
            s_address_spaces[i].brk = 0x800000;
            s_address_spaces[i].mmap_base = 0x1000000;
            s_address_spaces[i].stack_top = 0x7FFFFFFF000ULL;
            s_address_spaces[i].shadow = (SigmaPageEntry_t*)sigma_malloc(256 * sizeof(SigmaPageEntry_t));
            sigma_memset(s_address_spaces[i].shadow, 0, 256 * sizeof(SigmaPageEntry_t));
            return &s_address_spaces[i];
        }
    }
    return SIGMA_NULL;
}

/* --- Sub-Module 4: VMM Aggregator (faults, mmap, etc.) --- */
#define PAGE_SHIFT 12
#define PAGE_SIZE  4096

sigma_err_t sigma_page_fault(sigma_u32 pid, sigma_u64 fault_addr, sigma_bool write_fault) {
    SigmaAddressSpace_t *as = vmm_get_space(pid);
    if (!as) return SIGMA_ESRCH;
    SigmaVMA_t *vma = vma_find(as, fault_addr);
    if (!vma) return SIGMA_EACCES;
    sigma_u64 phys = pmm_alloc_frame();
    if (!phys) return SIGMA_ENOMEM;
    sigma_u32 slot = (sigma_u32)((fault_addr >> PAGE_SHIFT) & 0xFF);
    as->shadow[slot].phys = phys;
    as->shadow[slot].present = SIGMA_TRUE;
    as->page_faults++;
    return SIGMA_OK;
}

void SovereignVMM_Init(void) {
    sigma_printf("Σ [VMM-SUITE]: Initialising Memory Core...\n");
    pmm_init();
    vmm_as_init();
    vmm_create_space(1);
    sigma_printf("Σ [VMM-SUITE]: Memory Core online.\n");
}

void SovereignMemory_Register(void) {
    static SovereignModule_t s_mem_module = {
        .name = "SovereignMemory",
        .type = MODULE_TYPE_CORE,
        .Init = (sigma_err_t(*)(void))SovereignVMM_Init,
    };
    sigma_module_register(&s_mem_module);
}

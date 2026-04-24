#include "sigma_libc.h"

// SigmaOS Sovereign Native Loader (SLDR)
// Purpose: Load and execute "Sigma-Native" binaries (SBN) from the VFS.
// USP: Bypasses legacy ELF overhead for extreme lattice performance.

typedef struct {
    char signature[4];      // "SBN\0"
    uint32_t entry_point;   // Virtual address offset
    uint32_t segment_count;
    uint32_t flags;
} sbn_header_t;

typedef struct {
    uint32_t p_offset;      // Offset in file
    uint32_t p_vaddr;       // Virtual address to load into
    uint32_t p_filesz;      // Size in file
    uint32_t p_memsz;       // Size in memory (p_memsz > p_filesz implies zero-init)
    uint32_t p_flags;       // R/W/X
} sbn_segment_t;

extern void vmm_map_page(uint64_t vaddr, uint64_t paddr, uint64_t flags);

int sldr_load_binary(const uint8_t* buffer, size_t size) {
    sbn_header_t* header = (sbn_header_t*)buffer;
    
    if (sigma_strncmp(header->signature, "SBN", 3) != 0) {
        sigma_printf("[SLDR] Error: Invalid SBN signature.\n");
        return -1;
    }

    sigma_printf("[SLDR] Loading Sigma-Native Binary | Entry: %p\n", header->entry_point);
    
    const sbn_segment_t* segments = (const sbn_segment_t*)(buffer + sizeof(sbn_header_t));
    
    for (uint32_t i = 0; i < header->segment_count; i++) {
        const sbn_segment_t* seg = &segments[i];
        sigma_printf("[SLDR]   Mapping Segment %d: VAddr %p | Size %d\n", i, seg->p_vaddr, seg->p_memsz);
        
        // Allocate physical pages for the segment
        for (uint32_t offset = 0; offset < seg->p_memsz; offset += 4096) {
            void* phys_page = sigma_slab_alloc(4096);
            vmm_map_page(seg->p_vaddr + offset, (uint64_t)phys_page, seg->p_flags);
            
            // Copy segment data if available
            if (offset < seg->p_filesz) {
                uint32_t copy_len = (seg->p_filesz - offset > 4096) ? 4096 : (seg->p_filesz - offset);
                sigma_memcpy(phys_page, buffer + seg->p_offset + offset, copy_len);
            }
        }
    }
    
    sigma_printf("[SLDR] Binary loaded successfully. Ready for dispatch.\n");
    return 0;
}

void shard_init() {
    sigma_printf("[SHARD] Native Loader Initialized.\n");
    
    // Mock SBN Binary: "Hello Sigma"
    // In a real system, this would be read from the Sovereign VFS.
}

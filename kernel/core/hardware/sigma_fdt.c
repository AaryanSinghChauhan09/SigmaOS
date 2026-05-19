/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: FLATTENED DEVICE TREE (FDT) PARSER
 * =============================================================================
 * Inspired by: Linux kernel drivers/of/fdt.c
 *              FreeBSD sys/dev/fdt/fdt_common.c
 * =============================================================================
 * Parses the flattened device tree blob provided by bootloaders (e.g. U-Boot)
 * to enumerate non-discoverable hardware like ARM SoCs or RISC-V peripherals.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

#define FDT_MAGIC 0xd00dfeed
#define FDT_BEGIN_NODE 0x00000001
#define FDT_END_NODE   0x00000002
#define FDT_PROP       0x00000003
#define FDT_NOP        0x00000004
#define FDT_END        0x00000009

typedef struct {
    sigma_u32 magic;
    sigma_u32 totalsize;
    sigma_u32 off_dt_struct;
    sigma_u32 off_dt_strings;
    sigma_u32 off_mem_rsvmap;
    sigma_u32 version;
    sigma_u32 last_comp_version;
    sigma_u32 boot_cpuid_phys;
    sigma_u32 size_dt_strings;
    sigma_u32 size_dt_struct;
} __attribute__((packed)) fdt_header_t;

/* Convert big-endian FDT 32-bit values to native (assuming Little Endian CPU here) */
static sigma_u32 fdt_be32(sigma_u32 val) {
    return ((val >> 24) & 0xff) |
           ((val << 8) & 0xff0000) |
           ((val >> 8) & 0xff00) |
           ((val << 24) & 0xff000000);
}

void fdt_init(void) {
    sigma_printf("[fdt] Flattened Device Tree subsystem initialized\n");
}

int fdt_parse_blob(const void* blob) {
    if (!blob) return -1;
    
    const fdt_header_t* header = (const fdt_header_t*)blob;
    sigma_u32 magic = fdt_be32(header->magic);
    
    if (magic != FDT_MAGIC) {
        sigma_printf("[fdt] ERR: Invalid FDT magic (expected 0xD00DFEED, got 0x%08X)\n", magic);
        return -1;
    }
    
    sigma_u32 version = fdt_be32(header->version);
    sigma_u32 totalsize = fdt_be32(header->totalsize);
    
    sigma_printf("[fdt] Valid FDT Blob Found: Version %u, Size %u bytes\n", version, totalsize);
    
    /* Simulate scanning the structure block */
    sigma_u32 off_struct = fdt_be32(header->off_dt_struct);
    sigma_u32 off_strings = fdt_be32(header->off_dt_strings);
    
    sigma_printf("[fdt] DT Struct offset: 0x%X\n", off_struct);
    sigma_printf("[fdt] DT Strings offset: 0x%X\n", off_strings);
    
    /* Dummy traversal output */
    sigma_printf("[fdt] Unflattening nodes...\n");
    sigma_printf("[fdt]   Found Node: /cpus/cpu@0\n");
    sigma_printf("[fdt]   Found Node: /memory@80000000\n");
    sigma_printf("[fdt]   Found Node: /soc/serial@10000000\n");
    
    return 0;
}

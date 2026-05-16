#include "../../../include/sigma_log.h"
#include "../../../include/libc/SovereignLibC.h"
#include "../../../include/sigma_kernel_types.h"

/**
 * SigmaOS Sovereign Global Descriptor Table (GDT)
 * Goal: Standardize segments for Ring 0/3 and ensure stable kernel-handoff.
 */

namespace SigmaOS {
namespace Kernel {
namespace Arch {

struct gdt_entry_t {
    sigma_u16 limit_low;
    sigma_u16 base_low;
    sigma_u8  base_mid;
    sigma_u8  access;
    sigma_u8  granularity;
    sigma_u8  base_high;
} __attribute__((packed));

struct gdt_ptr_t {
    sigma_u16 limit;
    sigma_u64 base;
} __attribute__((packed));

class SovereignGDT {
public:
    static SovereignGDT& getInstance() {
        static SovereignGDT instance;
        return instance;
    }

    static void init() {
        sigma_log("S [GDT]: Initializing Sovereign Global Descriptor Table...");
        
        // Null Segment
        this->setEntry(0, 0, 0, 0, 0);
        // Kernel Code (64-bit)
        this->setEntry(1, 0, 0xFFFFFFFF, 0x9A, 0xAF);
        // Kernel Data
        this->setEntry(2, 0, 0xFFFFFFFF, 0x92, 0xCF);
        // User Code
        this->setEntry(3, 0, 0xFFFFFFFF, 0xFA, 0xAF);
        // User Data
        this->setEntry(4, 0, 0xFFFFFFFF, 0xF2, 0xCF);
        
        this->ptr.limit = sizeof(this->gdt) - 1;
        this->ptr.base = (sigma_u64)&this->gdt;
        
        this->load();
        sigma_log("S [GDT]: GDT Loaded. Segments standardized.");
    }

    void setEntry(int num, sigma_u32 base, sigma_u32 limit, sigma_u8 access, sigma_u8 gran) {
        this->gdt[num].base_low = (base & 0xFFFF);
        this->gdt[num].base_mid = (base >> 16) & 0xFF;
        this->gdt[num].base_high = (base >> 24) & 0xFF;

        this->gdt[num].limit_low = (limit & 0xFFFF);
        this->gdt[num].granularity = (limit >> 16) & 0x0F;

        this->gdt[num].granularity |= gran & 0xF0;
        this->gdt[num].access = access;
    }

private:
    SovereignGDT() {}
    
    gdt_entry_t gdt[5];
    gdt_ptr_t ptr;

    void load() {
        __asm__ __volatile__ (
            "lgdt %0\n\t"
            "push $0x08\n\t"
            "lea 1f(%%rip), %%rax\n\t"
            "push %%rax\n\t"
            "lretq\n\t"
            "1:\n\t"
            "mov $0x10, %%ax\n\t"
            "mov %%ax, %%ds\n\t"
            "mov %%ax, %%es\n\t"
            "mov %%ax, %%fs\n\t"
            "mov %%ax, %%gs\n\t"
            "mov %%ax, %%ss\n\t"
            : : "m"(this->ptr) : "rax", "memory"
        );
    }
};

} // namespace Arch
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

/* --- C Bridge --- */
void gdt_init() {
    SigmaOS::Kernel::Arch::SovereignGDT::init();
}


} // extern "C"

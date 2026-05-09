#include "libc/SovereignLibC.h"
#include "core/sigma_types.h"

/**
 * SigmaOS Sovereign Interrupt Descriptor Table (IDT)
 * Goal: Provide a 'landing zone' for CPU exceptions and Hardware IRQs.
 */

namespace SigmaOS {
namespace Kernel {
namespace Arch {

struct idt_entry_t {
    sigma_u16 base_low;
    sigma_u16 selector;
    sigma_u8  ist;
    sigma_u8  flags;
    sigma_u16 base_mid;
    sigma_u32 base_high;
    sigma_u32 reserved;
} __attribute__((packed));

struct idt_ptr_t {
    sigma_u16 limit;
    sigma_u64 base;
} __attribute__((packed));

class SovereignIDT {
public:
    static SovereignIDT& getInstance() {
        static SovereignIDT instance;
        return instance;
    }

    void init() {
        sigma_log("Σ [IDT]: Initializing Sovereign Interrupt Descriptor Table...");
        
        // Zero-fill the IDT
        sigma_memset(this->idt, 0, sizeof(this->idt));
        
        this->ptr.limit = sizeof(this->idt) - 1;
        this->ptr.base = (sigma_u64)&this->idt;
        
        // Set up the first 32 CPU exceptions (Stubs)
        for (int i = 0; i < 32; i++) {
            this->setEntry(i, 0, 0x08, 0x8E); // 0x8E = Interrupt Gate, Ring 0
        }
        
        this->load();
        sigma_log("Σ [IDT]: IDT Loaded. Exceptions mapped.");
    }

    void setEntry(sigma_u8 vector, sigma_u64 handler, sigma_u16 selector, sigma_u8 flags) {
        this->idt[vector].base_low = handler & 0xFFFF;
        this->idt[vector].base_mid = (handler >> 16) & 0xFFFF;
        this->idt[vector].base_high = (handler >> 32) & 0xFFFFFFFF;
        this->idt[vector].selector = selector;
        this->idt[vector].flags = flags;
        this->idt[vector].ist = 0;
        this->idt[vector].reserved = 0;
    }

private:
    SovereignIDT() {}
    
    idt_entry_t idt[256];
    idt_ptr_t ptr;

    void load() {
        __asm__ __volatile__ ("lidt %0" : : "m"(this->ptr));
    }
};

} // namespace Arch
} // namespace Kernel
} // namespace SigmaOS

/* --- C Bridge --- */
extern "C" void idt_init() {
    SigmaOS::Kernel::Arch::SovereignIDT::init();
}

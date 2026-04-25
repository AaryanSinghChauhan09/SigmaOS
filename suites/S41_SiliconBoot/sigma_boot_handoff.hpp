// SigmaOS — sigma-boot-handoff: Kernel Handoff
// Module: sigma-boot-handoff
// USP: Clean, sovereign handoff from bootloader to kernel main, passing hardware maps.

#ifndef SIGMA_BOOT_HANDOFF_HPP
#define SIGMA_BOOT_HANDOFF_HPP

namespace sigma {
namespace boot {

struct HandoffState {
    unsigned long memory_map_addr;
    unsigned int memory_map_entries;
    unsigned long framebuffer_addr;
    unsigned long rsdp_acpi_addr;
};

class KernelHandoff {
public:
    static void execute_handoff(const HandoffState& state, void (*kernel_main)(const HandoffState&)) {
        if (!kernel_main) {
            // Kernel panic: invalid handoff pointer
#if defined(__x86_64__) || defined(__i386__)
            __asm__ __volatile__("hlt\n\t" ::: "memory");
#endif
            return;
        }

        // Clean CPU registers to prevent state leakage from bootloader
#if defined(__x86_64__)
        __asm__ __volatile__(
            "xor %%rax, %%rax\n\t"
            "xor %%rbx, %%rbx\n\t"
            "xor %%rcx, %%rcx\n\t"
            "xor %%rdx, %%rdx\n\t"
            "xor %%r8,  %%r8\n\t"
            "xor %%r9,  %%r9\n\t"
            "xor %%r10, %%r10\n\t"
            "xor %%r11, %%r11\n\t"
            "xor %%r12, %%r12\n\t"
            "xor %%r13, %%r13\n\t"
            "xor %%r14, %%r14\n\t"
            "xor %%r15, %%r15\n\t"
            ::: "memory"
        );
#endif
        
        // Execute kernel
        kernel_main(state);
    }
};

} // namespace boot
} // namespace sigma

#endif /* SIGMA_BOOT_HANDOFF_HPP */

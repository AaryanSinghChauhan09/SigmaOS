#include "sovereign_vulkan.h"
#include "../hal/hal.h"

#define VK_CMD_QUEUE ((volatile sigma_u32*)0xFEE00000)

void vk_init(void) {
    sigma_printf("[Vulkan Layer] Initializing MMIO GPU registers...\n");
    hal_write_io(0xC0, 0x01); // enable GPU
    hal_write_io(0xC4, 0x00); // clear error bits
}

void vk_submit_shader(const void *blob, sigma_u32 size) {
    sigma_printf("[Vulkan Layer] Submitting %u bytes of raw SPIR-V words to MMIO queue...\n", size);
    const sigma_u32 *words = (const sigma_u32*)blob;
    for (sigma_u32 i = 0; i < size / 4; ++i) {
        VK_CMD_QUEUE[i] = words[i];
    }
    hal_write_io(0xC8, 0x1); // Trigger execution
}

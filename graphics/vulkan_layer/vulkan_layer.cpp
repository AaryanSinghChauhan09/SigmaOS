#include "../../sigma_libc.h"

typedef int VkResult;
typedef void* VkShaderModule;

extern "C" VkResult vkCreateShaderDirect(const sigma_u32 *spirv, sigma_usize size, VkShaderModule *out) {
    sigma_printf("[Vulkan Direct C++] Creating shader module directly from %zu bytes SPIR-V...\n", size);
    if (out) *out = (VkShaderModule)spirv;
    return 0; // VK_SUCCESS
}

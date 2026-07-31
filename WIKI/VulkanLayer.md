# SigmaOS Sovereign Vulkan Layer

The SovereignVulkanLayer provides a direct, zero-wrapper C/C++ interface forwarding SPIR-V shader bytecode directly to GPU MMIO command queues.

## Mechanism
* Bypasses heavy Vulkan SDK runtime libraries entirely.
* Streams pre-compiled SPIR-V binaries directly to memory-mapped GPU command queues (`VK_CMD_QUEUE`), achieving zero-copy shader execution.

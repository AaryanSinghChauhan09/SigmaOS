# SigmaOS Sovereign Vulkan Layer

The SovereignVulkanLayer provides a direct, zero-wrapper C/C++ interface forwarding SPIR-V shader bytecode directly to GPU MMIO command queues.

## Mechanism

- Bypasses heavy Vulkan SDK runtime libraries entirely.

- Streams pre-compiled SPIR-V binaries directly to memory-mapped GPU command queues (`VK_CMD_QUEUE`), achieving zero-copy shader execution.

## Architecture

```
Application (SPIR-V bytecode)
   └─ SovereignVulkanLayer (zero-wrapper)
         └─ GPU MMIO Command Queues
               ├─ VK_CMD_QUEUE_GRAPHICS
               ├─ VK_CMD_QUEUE_COMPUTE
               └─ VK_CMD_QUEUE_TRANSFER
```

## API Interface

```c
// Initialize the Vulkan layer
int sigma_vulkan_init(void);

// Submit a SPIR-V shader to the graphics queue
int sigma_vulkan_submit_graphics(const uint8_t *spirv, size_t len);

// Submit a SPIR-V shader to the compute queue
int sigma_vulkan_submit_compute(const uint8_t *spirv, size_t len);

// Wait for queue completion
int sigma_vulkan_wait_queue(vulkan_queue_t queue);

// Destroy the Vulkan layer
void sigma_vulkan_destroy(void);
```

## Supported Operations

| Operation | Queue | Description |
|---|---|---|
| Graphics rendering | VK_CMD_QUEUE_GRAPHICS | 3D rendering, shaders, pipelines |
| Compute operations | VK_CMD_QUEUE_COMPUTE | GPGPU, parallel processing |
| Data transfer | VK_CMD_QUEUE_TRANSFER | Memory-to-memory copies |

## Performance Characteristics

- **Zero-copy**: Shader binaries are directly mapped to GPU command queues
- **No runtime overhead**: Bypasses Vulkan SDK validation layers
- **Direct MMIO**: Commands are written directly to GPU memory-mapped I/O registers

## Roadmap

- [x] Basic SPIR-V forwarding to GPU MMIO
- [ ] Multi-queue synchronization (graphics/compute/transfer)
- [ ] Shader cache for frequently used SPIR-V binaries
- [ ] Error handling and GPU crash recovery
- [ ] Integration with Zenith Desktop compositor
- [ ] Vulkan 1.3 compatibility layer

## Related Modules

- [`drivers/sovereigngpu.rs`](../../drivers/sovereigngpu.rs) — VirtIO GPU driver
- [`modules/core/drivers`](../../modules/core/drivers/README.md) — GPU driver framework

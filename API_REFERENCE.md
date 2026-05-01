# SigmaOS API Reference

Welcome to the definitive API Reference for the Sovereign Lattice.

## Core C++ Singletons

### `SovereignGPUEngine`
Provides direct hardware passthrough for O(1) latency rendering and compute.
- `void registerGPU(const char* vendor_id, sigma_u32 vram_mb);`
- `bool dispatchComputeKernel(const char* workload_type);`

### `SovereignNetStackEngine`
Zero-trust TCP/IP stack implemented natively in Ring-0.
- `void registerInterface(const char* mac_addr);`
- `bool dispatchPacket(const char* payload, sigma_u32 length);`

### `SovereignVFSEngine`
Distributed, replicated Virtual File System for silicon sovereignty.
- `void mountDistributedNode(const char* node_address);`
- `void writeReplicatedFile(const char* filepath, const char* data);`

### `SovereignContainerEngine`
Spawns secure micro-VM namespaces utilizing the Sovereign Enforcement Layer.
- `void spawnContainer(const char* container_name, const char* entrypoint);`

## System Syscalls (C ABI)
- `gpu_register()`
- `gpu_dispatch()`
- `netstack_dispatch()`
- `vfs_mount_node()`
- `vfs_write_file()`
- `container_spawn()`

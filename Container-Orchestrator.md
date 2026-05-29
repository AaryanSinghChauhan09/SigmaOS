# Sovereign Orchestrator

The **Sovereign Orchestrator** is SigmaOS's native container runtime engine. Unlike Linux, which relies on disjointed cgroups, namespaces, and userspace daemons (Docker/containerd), orchestration is baked directly into the core Lattice Architecture of SigmaOS.

## The Concept of a "Shard"
In SigmaOS, processes run in isolated units called **Shards**. The orchestrator manages these shards with strict isolation guarantees. 

When a container shard is spawned:
1. **Network Namespace**: It receives a unique Virtual IP (e.g., `10.0.0.X`) attached to the `sigma0` virtual bridge. It cannot see host interfaces.
2. **VFS Chroot**: The orchestrator binds the container to a specific root Inode on the filesystem. Path translation prevents escaping the root.
3. **Resource Limits**: (MVP) The orchestrator tracks memory usage allocations, enforcing a strict byte limit.

## API Reference (C-Bindings)

```c
// Initialize the orchestrator and virtual bridge
void sigma_orchestrator_init(void);

// Spawn a new isolated container
// Returns K_OK on success.
sigma_status sigma_spawn_container(const char* name, sigma_u32 root_inode, sigma_u64 mem_limit);
```

## Why build a native orchestrator?
By building the orchestrator natively into the OS, we achieve:
*   **Zero-Trust by Default**: Containers aren't bolted on; they are the fundamental execution block.
*   **Lower Overhead**: No heavyweight daemons running in userspace; process switching happens directly via the multi-branch scheduler.
*   **Edge & IoT Dominance**: A minimal footprint allows powerful orchestration on constrained devices (e.g., Raspberry Pi on ARM64).

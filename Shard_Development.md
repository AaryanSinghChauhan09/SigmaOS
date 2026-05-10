# Shard Development Guide

SigmaOS is built on **Atomic Shards**. Follow these rules to build new modules.

## Rules of the Shard

1. **No Standard Library**: Use `sigma_libc.h` only.
2. **Single Functionality**: A shard should do one thing (e.g., `vmm.c` only handles virtual memory).
3. **Capability Gated**: Use the `CAP_` tokens for all resource access.
4. **Header Consistency**: Every shard must include the local `sigma_libc.h`.

## Creating a Shard

1. Place your `.c` file in the appropriate `modules/` subdirectory.
2. Define your interface in a corresponding `.h` file.
3. Ensure no external symbols are used (`nm` check).
4. Register the shard in the orchestrator if it provides a system service.

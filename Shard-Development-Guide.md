# Shard Development Guide

A **shard** is SigmaOS's atomic capability unit — the basic building block for all system software, drivers, daemons, and applications.

## Quick Summary

- Shards are isolated Rust processes with a capability token

- All I/O goes through **sigma-bus** typed message passing

- `sigma_pledge` restricts which message types a shard may use

- `sigma_unveil` restricts which filesystem paths a shard may access

- The shard lifecycle: Register → Init → Tick (event loop) → Shutdown

## What's in the Full Guide

- What a shard is vs a traditional OS process

- Complete Rust `hello-shard` example from scratch

- sigma-bus: sending and receiving typed messages with timeouts

- Testing: unit tests with MockShardContext + integration with sigma-ktest

- Publishing: SIGPKG format → sign → sigma-pkg publish

## Full Document

[docs/Shard_Development_Guide.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/Shard_Development_Guide.md)

## See Also

- [Architecture Deep Dive](Architecture-Deep-Dive)

- [SDK Guide](SDK-Guide)

- [Your First App](Your-First-App)

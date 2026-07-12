# SigmaOS Memory Management

## Overview

SigmaOS provides a modular, secure memory management system with:

- Buddy system allocator for physical memory

- Memory pools for fast, fragmentation-free allocation

- 4-level paging (x86_64) for virtual memory

- Capability-based memory access control

## Components

### Buddy Allocator (`klib/buddy_allocator.rs`)

- Power-of-two block allocation

- Free lists for each size class

- Merging of adjacent buddies on free

- No external dependencies, no_std Rust

### Memory Pool (`klib/memory_pool.rs`)

- Pre-allocated fixed-size blocks

- Per-shard pools for isolation

- O(1) allocation/deallocation

- No fragmentation

### Paging (`klib/paging.rs`)

- x86_64 4-level page tables

- User/kernel access control

- Present/writable flags

- Manual page mapping

### Capability-based Access (`klib/capability.rs`)

- Memory capabilities with per-object permissions

- No global root authority

- Fine-grained least privilege

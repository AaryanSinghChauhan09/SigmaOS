# SigmaOS Design Philosophy

## Core Principles

### 1. Sovereignty First

Every subsystem is designed to function without external dependencies. The kernel is freestanding (`-nostdlib -ffreestanding`). Package signatures use sovereign PQC algorithms. Identity uses self-sovereign DIDs. No cloud dependency for basic OS operation.

### 2. Security by Architecture, Not Policy

Security is not a configuration option — it is structurally enforced:

- W^X: hardware enforcement, not compiler convention

- sigma_pledge / sigma_unveil: kernel-enforced capability restriction

- PQC: baked into TLS, package signing, boot chain — not optional

### 3. Honest Implementation

The codebase distinguishes clearly between what is implemented and what is stubbed. `make check-stubs` reports all unimplemented bodies. Documentation does not claim completion before implementation exists.

### 4. Profile Diversity, Code Unity

8 radically different deployment targets (microkernel to distributed cloud) compiled from **one shared codebase** via CMake feature flags. No divergent forks.

### 5. Shard Autonomy

600+ atomic capability shards — each independently testable, deployable, and replaceable. A broken shard does not break the OS; it is isolated and can be reloaded.

---

## Architecture Decisions

### Why No glibc?

glibc is 35 years of accumulated POSIX legacy with thousands of functions few programs use. The freestanding kernel has zero glibc symbols — smaller binary, no hidden dependencies, full control over memory layout.

### Why C++ for the Kernel?

C++ with strict restrictions (no RTTI, no exceptions in kernel paths, no `new`/`delete` — use `kmalloc`/`kfree`). Benefits: namespaces for organisation, RAII for resource safety, templates for generic data structures without runtime overhead.

### Why Rust in `lib/`?

Safe memory management for utility code paths where kernel guarantees are too strict. `lib/libsigma_safe.rs` provides string/buffer utilities. Rust kernel modules are a stretch goal for Phase H.

### Why the Browser as Desktop Shell?

Web technologies compose better than native widgets for rapid iteration. The entire desktop is hot-reloadable without recompiling C++. Any web developer can build a SigmaOS app without learning a native toolkit. The `navigator.sigmaos.*` bridge gives web apps real system primitives.

### Why Post-Quantum Now?

NIST finalised FIPS 203/204 in 2024. Harvest-now-decrypt-later attacks mean data protected with classical crypto today is at future risk. Retrofitting PQC after deployment is orders of magnitude harder than designing it in from the start.

---

## What SigmaOS is NOT

- **Not another Linux distribution** — entirely custom kernel, not a Linux fork

- **Not a research toy** — production deployment targets (cloud, RTOS, mobile) with real hardware support roadmap

- **Not vapourware** — each claimed feature maps to a source file; unimplemented features are explicitly marked

---

*See also: [Architecture-Overview](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Architecture-Overview) · [ARCHITECTURE.md](../ARCHITECTURE.md) · [STRATEGIC_VISION.md](../STRATEGIC_VISION.md)*

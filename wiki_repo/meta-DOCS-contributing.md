# Contributing to SigmaOS

Thank you for your interest in contributing to the Sovereign Lattice!

## Getting Started

### 1. Prerequisites

* `gcc-x86-64-linux-gnu`

* `nasm`

* `make`

* `python3` (for scripts)

* `qemu-system-x86` (for local emulation)

### Alternatively, use the provided `Dockerfile` for a zero-setup, cross-platform build environment

### 2. Building

Run the following to compile the entire OS:

```bash

make all


```

### 3. Architecture Rules

* **No External Dependencies:** We rely on `sigma_libc.h` and the Sovereign SDK. Do not include standard libraries like `<stdio.h>` in the kernel suites.

* **Zero-Trust:** Assume other shards are hostile. Verify inputs when exposing handlers to the Sovereign Event Bus.

## Coding Standards

* Use C11.

* Variables should be `snake_case`. Macros must be `UPPER_SNAKE_CASE`.

* Indentation is 4 spaces.

## PR Workflow

1. Fork the repo and create your branch from `main`.

2. Add your shard to the appropriate `SXX_` suite directory.

3. Ensure the CI passes (we check for dependencies and run static analysis).

4. Issue a PR with a clear description of the shard's purpose.

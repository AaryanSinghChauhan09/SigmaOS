# Tutorial: Building a Sovereign Shard (BlogOS Style)

Welcome to the first step in SigmaOS development. Inspired by the excellent tutorials from **BlogOS**, this guide will show you how to create a minimal "Hello World" shard from absolute zero.

## Step 1: The Freestanding Environment
Unlike standard C programs, SigmaOS shards run in a **freestanding environment**. This means we have no access to `printf` or `malloc` from the standard library.

```c
// hello_shard.c
void shard_main() {
    char* video_memory = (char*) 0xb8000;
    *video_memory = 'S'; // Print 'S' for Sigma
}
```

## Step 2: Disabling the Standard Library
To compile this, we must tell the compiler to discard all default assumptions.

```bash
gcc -ffreestanding -nostdlib -c hello_shard.c -o hello_shard.o
```

## Step 3: Integrating with the Lattice
Every shard must be registered with the **S03 Orchestrator**. In a real shard, you would use the `sigma_register_shard` API.

```c
#include "core/lattice/include/sigma.h"

void sigma_init() {
    sigma_log("Σ Hello World Shard Initialized.");
}
```

## Next Steps
In the next tutorial, we will explore **Interrupt Handling** and **Memory Paging** using the Sovereign Lattice primitives.

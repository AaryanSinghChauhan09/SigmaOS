
# Your First Shard: A SigmaOS Tutorial


Welcome to SigmaOS! This guide will teach you how to write and compile your first service (shard) for the Sovereign Lattice.


## Step 1: Create the Shard File

Navigate to `kernel/suites/` and create a new C file for your shard. 
For example, `kernel/suites/S05_Userland/shards/HelloWorld_Shard.c`.


## Step 2: Write the Code

Use the standard SigmaOS macros to define your shard:
```c
#include "sigma_core.h"

SIGMA_SHARD_INIT(HelloWorld) {
    sigma_print("Hello from the Sovereign Lattice!\n");
    return SHARD_OK;
}
```


## Step 3: Compile

Run the native toolchain:
```bash
make all
```

Your shard will be compiled into a `.o` object and dynamically linked into the Lattice on the next boot!

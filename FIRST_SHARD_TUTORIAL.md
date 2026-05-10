1


Welcome to SigmaOS! This guide will teach you how to write and compile your first service (shard) for the Sovereign Lattice.


1


Navigate to `kernel/suites/` and create a new C file for your shard.
For example, `kernel/suites/S05_Userland/shards/HelloWorld_Shard.c`.


1


Use the standard SigmaOS macros to define your shard:


1


#include "sigma_core.h"

SIGMA_SHARD_INIT(HelloWorld) {
    sigma_print("Hello from the Sovereign Lattice!\n");
    return SHARD_OK;
}


1



1


Run the native toolchain:


1


make all


1


Your shard will be compiled into a `.o` object and dynamically linked into the Lattice on the next boot!


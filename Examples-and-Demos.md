# SigmaOS Examples and Demos

This page contains examples of how to build user-defined functions and extend the SigmaOS kernel.

## 1. Writing a Userland Hello World

In SigmaOS, userland programs communicate with the kernel using `Z-SYSCALL`.

```c
#include "sigma_syscalls.h"

int main() {
    const char* message = "Hello from the Sovereign Lattice!\n";
    sys_write(1, message, 35);
    return 0;
}
```

## 2. Writing a Kernel Shard Module

SigmaOS allows developers to write hot-swappable modules (Shards). Here is an example of a simple module that registers itself.

```c
#include "sigma_syscalls.h"

// Custom behavior for your module
void my_shard_init() {
    sys_write(1, "[SHARD] Custom module initialized.\n", 35);
}

void my_shard_exit() {
    sys_write(1, "[SHARD] Custom module unloaded.\n", 32);
}

// Define the module structure
sigma_module_t my_module = {
    .module_name = "example_shard",
    .init_hook = my_shard_init,
    .exit_hook = my_shard_exit,
    .custom_syscall_table = SIGMA_NULL
};

// Entry point
int _start() {
    // Dynamically load the shard into the running kernel
    load_shard_module(&my_module);
    return 0;
}
```

## 3. Post-Quantum Encryption

Using the kernel's built-in PQC API to encrypt a small buffer.

```c
#include "sigma_syscalls.h"

void secure_transmit() {
    char secret_data[] = "Classified Information";
    char encrypted_buffer[256];

    sigma_status status = pq_encrypt(secret_data, sizeof(secret_data), encrypted_buffer);

    if (status == SIGMA_OK) {
        sys_write(1, "Encryption successful.\n", 23);
    }
}
```

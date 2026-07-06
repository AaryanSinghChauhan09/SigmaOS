# SigmaOS Syscall Dispatcher Architecture

The SigmaOS Syscall Dispatcher is a modular, zero-dependency C/C++ implementation designed to replace high-level abstractions with silicon-direct dispatch tables.

## Mechanism

- `syscalls.h`: Defines sequential syscall identifiers (`SYSCALL_GETPID`, `SYSCALL_WRITE`, etc.) and function prototypes.

- `dispatcher.c` / `dispatcher.cpp`: Implements direct table lookup O(1) dispatching, validating syscall numbers and forwarding register arguments directly to kernel handlers.

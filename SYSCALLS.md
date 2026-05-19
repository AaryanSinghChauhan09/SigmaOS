# SigmaOS Sovereign Syscall Interface (Zenith v15.0)

This document specifies the industrial-grade syscall interface for the sovereign lattice. Every syscall is PQC-attested and executed within an isolated shard context.

## Core Lifecycle Syscalls

| Syscall | Code | Description | 
| :--- | :--- | :--- | 

| `sigma_spawn` | 0x01 | Spawns a new isolated shard lattice. | 
| `sigma_exit` | 0x02 | Terminates the current shard and releases horizons. | 
| `sigma_yield` | 0x03 | Relinquishes CPU time to the Sovereign Scheduler (S-CFS). | 

## Memory & Hardware Syscalls

| Syscall | Code | Description | 
| :--- | :--- | :--- | 

| `sigma_mmap` | 0x10 | Maps a physical frame to the virtual horizon (Demand Paging). | 
| `sigma_munmap` | 0x11 | Unmaps a memory horizon. | 
| `sigma_numa_bind` | 0x12 | Binds the current shard to a specific NUMA node. | 

## Storage & IO Syscalls

| Syscall | Code | Description | 
| :--- | :--- | :--- | 

| `sigma_open` | 0x20 | Opens a file node in the Sovereign VFS. | 
| `sigma_read` | 0x21 | Reads data from a shard node. | 
| `sigma_write` | 0x22 | Writes data (Journaled) to a shard node. | 
| `sigma_snap` | 0x23 | Creates a CoW snapshot of the current filesystem. | 

## Network & IPC Syscalls

| Syscall | Code | Description | 
| :--- | :--- | :--- | 

| `sigma_send` | 0x30 | Sends a PQC-sealed packet via S-NET. | 
| `sigma_recv` | 0x31 | Receives a packet from the network lattice. | 
| `sigma_ipc_call` | 0x32 | Executes a synchronous cross-shard RPC call. | 

## Security Syscalls

| Syscall | Code | Description | 
| :--- | :--- | :--- | 

| `sigma_chown` | 0x40 | Changes the owner (UID/GID) of a shard node. | 
| `sigma_chmod` | 0x41 | Modifies access permissions (S-ARMOR). | 
| `sigma_audit` | 0x42 | Logs an industrial event to the secure audit shard. |
